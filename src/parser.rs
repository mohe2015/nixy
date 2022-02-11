use crate::{lexer::{NixLexer, NixToken, NixTokenType}, ast::{ASTVisitor, ASTBuilder}};
use core::fmt;
use itertools::MultiPeek;
use std::{
    fmt::Debug,
    mem::discriminant,
    process::{Command, ExitStatus, Stdio}, marker::PhantomData,
};
//#[cfg(debug_assertions)]
use tracing::instrument;

// TODO FIXME call lexer.reset_peek(); everywhere

// TODO FIXME right-associativity and no associativity

// https://matklad.github.io/2020/03/22/fast-simple-rust-interner.html

const BUILTIN_UNARY_NOT: &[u8] = b"__builtin_unary_not";
const BUILTIN_PATH_CONCATENATE: &[u8] = b"__builtin_path_concatenate";
const BUILTIN_SELECT: &[u8] = b"__builtin_select";
const BUILTIN_IF: &[u8] = b"__builtin_if";
const BUILTIN_STRING_CONCATENATE: &[u8] = b"__builtin_string_concatenate";

const BUILTIN_UNARY_MINUS: &[u8] = b"__builtin_unary_minus";

#[derive(PartialEq)]
pub enum AST<'a> {
    Identifier(&'a [u8]),
    String(&'a [u8]),
    PathSegment(&'a [u8]), // merge into String
    Integer(i64),
    Float(f64),
    Let(Box<AST<'a>>, Box<AST<'a>>, Box<AST<'a>>),
    Call(Box<AST<'a>>, Box<AST<'a>>),
}

impl<'a> Debug for AST<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !f.alternate() {
            // ugly hack for tracing macros
            write!(f, "{:#?}", self)
        } else {
            match self {
                Self::Identifier(arg0) => f
                    .debug_tuple("Identifier")
                    .field(&std::str::from_utf8(arg0).unwrap())
                    .finish(),
                Self::String(arg0) => f
                    .debug_tuple("String")
                    .field(&std::str::from_utf8(arg0).unwrap())
                    .finish(),
                Self::PathSegment(arg0) => f
                    .debug_tuple("PathSegment")
                    .field(&std::str::from_utf8(arg0).unwrap())
                    .finish(),
                Self::Integer(arg0) => f.debug_tuple("Integer").field(arg0).finish(),
                Self::Float(arg0) => f.debug_tuple("Float").field(arg0).finish(),
                Self::Let(arg0, arg1, arg2) => f
                    .debug_tuple("Let")
                    .field(arg0)
                    .field(arg1)
                    .field(arg2)
                    .finish(),
                Self::Call(arg0, arg1) => f.debug_tuple("Call").field(arg0).field(arg1).finish(),
            }
        }
    }
}

pub struct Parser<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug, R: std::fmt::Debug, A: ASTVisitor<'a, R>> {
    pub lexer: MultiPeek<I>,
    pub visitor: A,
    pub phantom: PhantomData<R> // https://github.com/rust-lang/rust/issues/23246
}

impl<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug, R: std::fmt::Debug, A: ASTVisitor<'a, R>> Parser<'a, I, R, A> {
    #[cfg_attr(debug_assertions, instrument(name = "expect", skip_all, ret))]
    pub fn expect(&mut self, t: NixTokenType<'a>) {
        let token = self.lexer.next();
        if discriminant(&token.as_ref().unwrap().token_type) != discriminant(&t) {
            panic!("expected {:?} but got {:?}", t, &token)
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "attrpath", skip_all, ret))]
    pub fn parse_attrpath(&mut self) -> Option<AST<'a>> {
        let mut result: Option<AST<'a>> = None;
        loop {
            match self.lexer.peek() {
                Some(NixToken {
                    token_type: NixTokenType::Identifier(id),
                }) => {
                    match result {
                        Some(a) => {
                            result = Some(AST::Call(
                                Box::new(AST::Call(
                                    Box::new(AST::Identifier(BUILTIN_SELECT)),
                                    Box::new(a),
                                )),
                                Box::new(AST::Identifier(id)),
                            ));
                        }
                        None => result = Some(AST::Identifier(id)),
                    }

                    self.lexer.next();
                }
                Some(NixToken {
                    token_type: NixTokenType::Select,
                }) => {
                    self.expect(NixTokenType::Select);
                }
                Some(NixToken {
                    token_type: NixTokenType::StringStart,
                }) => {
                    self.lexer.reset_peek();
                    let res = self.parse_some_string(
                        NixTokenType::StringStart,
                        NixTokenType::StringEnd,
                    )
                    .unwrap();
                    match result {
                        Some(a) => {
                            result = Some(AST::Call(
                                Box::new(AST::Call(
                                    Box::new(AST::Identifier(BUILTIN_SELECT)),
                                    Box::new(a),
                                )),
                                Box::new(res),
                            ));
                        }
                        None => result = Some(res),
                    }
                }
                Some(NixToken {
                    token_type: NixTokenType::InterpolateStart,
                }) => {
                    self.expect(NixTokenType::InterpolateStart);
                    let expr = self.parse_expr().unwrap();
                    self.expect(NixTokenType::CurlyClose);
                    match result {
                        Some(a) => {
                            result = Some(AST::Call(
                                Box::new(AST::Call(
                                    Box::new(AST::Identifier(BUILTIN_SELECT)),
                                    Box::new(a),
                                )),
                                Box::new(expr),
                            ));
                        }
                        None => result = Some(expr),
                    }
                }
                _ => {
                    self.lexer.reset_peek();
                    break;
                }
            }
        }
        result
    }

    #[cfg_attr(debug_assertions, instrument(name = "bind", skip_all, ret))]
    pub fn parse_bind(&mut self) -> (Box<AST<'a>>, Box<AST<'a>>) {
        match self.lexer.peek() {
            Some(NixToken {
                token_type: NixTokenType::Inherit,
            }) => {
                self.expect(NixTokenType::Inherit);
                match self.lexer.peek() {
                    Some(NixToken {
                        token_type: NixTokenType::ParenOpen,
                    }) => {
                        self.expect(NixTokenType::ParenOpen);
                        let expr = self.parse_expr();
                        self.expect(NixTokenType::ParenClose);
                    }
                    _ => {
                        self.lexer.reset_peek();
                    }
                }
                let mut attrs = Vec::new();
                loop {
                    match self.lexer.peek() {
                        Some(NixToken {
                            token_type: NixTokenType::Identifier(attr),
                        }) => {
                            attrs.push(AST::Identifier(attr));
                            self.lexer.next();
                        }
                        _ => {
                            // TODO string attrs missing
                            self.lexer.reset_peek();
                            break;
                        }
                    }
                }
                self.expect(NixTokenType::Semicolon);
                (
                    Box::new(AST::Identifier(b"TODO inherit")),
                    Box::new(AST::Identifier(b"TODO inherit")),
                )
            }
            other => {
                self.lexer.reset_peek();
                let attrpath = self.parse_attrpath();
                self.expect(NixTokenType::Assign);

                //println!("TEST {:?}", lexer.peek());
                //lexer.reset_peek();

                let expr = self.parse_expr().expect("expected expression in binding at");
                self.expect(NixTokenType::Semicolon);

                (Box::new(attrpath.unwrap()), Box::new(expr))
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "let", skip_all, ret))]
    pub fn parse_let(&mut self) -> Option<AST<'a>> {
        self.expect(NixTokenType::Let);
        let mut binds: Vec<(Box<AST<'a>>, Box<AST<'a>>)> = Vec::new();
        loop {
            match self.lexer.peek() {
                Some(NixToken {
                    token_type: NixTokenType::In,
                }) => {
                    self.lexer.next();

                    let body =
                        self.parse_expr_function().expect("failed to parse body of let binding");

                    break Some(binds.into_iter().fold(body, |accum, item| {
                        AST::Let(item.0, item.1, Box::new(accum))
                    }));
                }
                _ => {
                    self.lexer.reset_peek();
                    let bind = self.parse_bind();

                    binds.push(bind);
                }
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "path", skip_all, ret))]
    pub fn parse_path(&mut self) -> Option<AST<'a>> {
        self.expect(NixTokenType::PathStart);
        let mut result: Option<AST<'a>> = None;
        loop {
            let val = self.lexer.next();
            match val {
                Some(NixToken {
                    token_type: NixTokenType::PathEnd,
                }) => {
                    break result;
                }
                Some(NixToken {
                    token_type: NixTokenType::InterpolateStart,
                }) => {
                    let expr = self.parse_expr().unwrap();
                    self.expect(NixTokenType::CurlyClose);
                    match result {
                        Some(a) => {
                            result = Some(AST::Call(
                                Box::new(AST::Call(
                                    Box::new(AST::Identifier(BUILTIN_PATH_CONCATENATE)),
                                    Box::new(a),
                                )),
                                Box::new(expr),
                            ))
                        }
                        None => result = Some(expr),
                    }
                }
                Some(NixToken {
                    token_type: NixTokenType::PathSegment(segment),
                }) => match result {
                    Some(a) => {
                        result = Some(AST::Call(
                            Box::new(AST::Call(
                                Box::new(AST::Identifier(BUILTIN_PATH_CONCATENATE)),
                                Box::new(a),
                            )),
                            Box::new(AST::PathSegment(segment)),
                        ))
                    }
                    None => result = Some(AST::PathSegment(segment)),
                },
                _ => {
                    todo!();
                }
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "str", skip_all, ret))]
    pub fn parse_some_string(&mut self,
        start: NixTokenType<'a>,
        end: NixTokenType<'a>,
    ) -> Option<AST<'a>> {
        self.expect(start);
        let mut accum = AST::String(b"");
        loop {
            match self.lexer.next() {
                Some(NixToken {
                    token_type: NixTokenType::String(string),
                }) => {
                    accum = AST::Call(
                        Box::new(AST::Call(
                            Box::new(AST::Identifier(BUILTIN_STRING_CONCATENATE)),
                            Box::new(accum),
                        )),
                        Box::new(AST::String(string)),
                    )
                }
                Some(NixToken {
                    token_type: NixTokenType::IndentedString(string),
                }) => {
                    accum = AST::Call(
                        Box::new(AST::Call(
                            Box::new(AST::Identifier(BUILTIN_STRING_CONCATENATE)),
                            Box::new(accum),
                        )),
                        Box::new(AST::String(string)),
                    )
                }
                Some(NixToken {
                    token_type: NixTokenType::InterpolateStart,
                }) => {
                    let expr = self.parse_expr().unwrap();
                    self.expect(NixTokenType::CurlyClose);
                    accum = AST::Call(
                        Box::new(AST::Call(
                            Box::new(AST::Identifier(BUILTIN_STRING_CONCATENATE)),
                            Box::new(accum),
                        )),
                        Box::new(expr),
                    )
                }
                Some(NixToken { token_type: end }) => break Some(accum),
                v => panic!("unexpected {:?}", v),
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "attrs", skip_all, ret))]
    pub fn parse_attrset(&mut self) -> Option<AST<'a>> {
        self.expect(NixTokenType::CurlyOpen);

        let mut binds: Vec<(Box<AST<'a>>, Box<AST<'a>>)> = Vec::new();
        loop {
            match self.lexer.peek() {
                Some(NixToken {
                    token_type: NixTokenType::CurlyClose,
                }) => {
                    self.expect(NixTokenType::CurlyClose);

                    break Some(
                        binds
                            .into_iter()
                            .fold(AST::Identifier(b"TODO attrset"), |accum, item| {
                                AST::Let(item.0, item.1, Box::new(accum))
                            }),
                    );
                }
                _ => {
                    self.lexer.reset_peek();
                    let bind = self.parse_bind();

                    binds.push(bind);
                }
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "simple", skip_all, ret))]
    pub fn parse_expr_simple(&mut self) -> Option<R> {
        let val = self.lexer.peek();
        match val {
            Some(NixToken {
                token_type: NixTokenType::Identifier(id),
            }) => {
                let ret = Some(self.visitor.visit_identifier(id));
                self.lexer.next();
                ret
            }
            Some(NixToken {
                token_type: NixTokenType::Integer(integer),
            }) => {
                let ret = Some(self.visitor.visit_integer(*integer));
                self.lexer.next();
                ret
            }
            Some(NixToken {
                token_type: NixTokenType::Float(float),
            }) => {
                let ret = Some(self.visitor.visit_float(*float));
                self.lexer.next();
                ret
            }
            Some(NixToken {
                token_type: NixTokenType::PathStart,
            }) => {
                self.lexer.reset_peek();
                self.parse_path();
                Some(self.visitor.visit_todo())
            }
            Some(NixToken {
                token_type: NixTokenType::IndentedStringStart,
            }) => {
                self.lexer.reset_peek();
                self.parse_some_string(
                    NixTokenType::IndentedStringStart,
                    NixTokenType::IndentedStringEnd,
                );
                Some(self.visitor.visit_todo())
            }
            Some(NixToken {
                token_type: NixTokenType::StringStart,
            }) => {
                self.lexer.reset_peek();
                self.parse_some_string(NixTokenType::StringStart, NixTokenType::StringEnd);
                self.visitor.visit_todo()
            }
            Some(NixToken {
                token_type: NixTokenType::ParenOpen,
            }) => {
                self.expect(NixTokenType::ParenOpen);
                let expr = self.parse_expr();
                self.expect(NixTokenType::ParenClose);
                expr;
                self.visitor.visit_todo()
            }
            Some(NixToken {
                token_type: NixTokenType::CurlyOpen,
            }) => {
                self.parse_attrset();
                self.visitor.visit_todo()
            },
            Some(NixToken {
                token_type: NixTokenType::BracketOpen,
            }) => {
                self.expect( NixTokenType::BracketOpen);
                let mut array = Vec::new();
                loop {
                    match self.lexer.peek() {
                        Some(NixToken {
                            token_type: NixTokenType::BracketClose,
                        }) => {
                            self.lexer.next();
                            break;
                        }
                        token => {
                            self.lexer.reset_peek();
                            array.push(self.parse_expr_select().unwrap());
                        }
                    }
                }
                Some(
                    array
                        .into_iter()
                        .fold(AST::Identifier(b"cons"), |accum, item| {
                            AST::Call(Box::new(accum), Box::new(item))
                        }),
                );
                self.visitor.visit_todo()
            }
            Some(NixToken {
                token_type: NixTokenType::Let,
            }) => {
                self.expect(NixTokenType::Let);
                self.parse_attrset();
                self.visitor.visit_todo()
            }
            Some(NixToken {
                token_type: NixTokenType::Rec,
            }) => {
                self.expect(NixTokenType::Rec);
                self.parse_attrset();
                self.visitor.visit_todo()
            }
            _ => {
                self.lexer.reset_peek();
                None
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "", skip_all, ret))]
    pub fn parse_expr_infix<F: FnMut(&mut Self) -> Option<AST<'a>> + Copy>(&mut self,
        f: F,
        operators: &[NixTokenType],
    ) -> Option<AST<'a>> {
        self.parse_expr_infix_split( f, f, operators)
    }

    #[cfg_attr(debug_assertions, instrument(name = "", skip_all, ret))]
    pub fn parse_expr_infix_split<F1: FnMut(&mut Self) -> Option<AST<'a>>, F2: FnMut(&mut Self) -> Option<AST<'a>>>(&mut self,
        mut flhs: F1,
        mut frhs: F2,
        operators: &[NixTokenType],
    ) -> Option<AST<'a>> {
        let mut result = flhs(self)?;
        loop {
            let next_token = self.lexer.peek();
            if next_token.is_none() {
                self.lexer.reset_peek();
                return Some(result);
            }
            if operators.contains(&next_token.unwrap().token_type) {
                let token = self.lexer.next().unwrap();
                let rhs = frhs(self).expect(&format!(
                    "expected right hand side after {:?} but got nothing",
                    token.token_type
                ));
                // TODO FIXME replace leaking by match to function name
                result = AST::Call(
                    Box::new(AST::Call(
                        Box::new(AST::Identifier(match token.token_type {
                            NixTokenType::If => b"if",
                            NixTokenType::Then => b"then",
                            NixTokenType::Else => b"else",
                            NixTokenType::Assert => b"assert",
                            NixTokenType::With => b"with",
                            NixTokenType::Let => b"let",
                            NixTokenType::In => b"in",
                            NixTokenType::Rec => b"rec",
                            NixTokenType::Inherit => b"inherit",
                            NixTokenType::Or => b"or",
                            NixTokenType::Ellipsis => b"ellipsis",
                            NixTokenType::Equals => b"equals",
                            NixTokenType::NotEquals => b"notequals",
                            NixTokenType::LessThanOrEqual => b"lessthanorequal",
                            NixTokenType::GreaterThanOrEqual => b"greaterthanorequal",
                            NixTokenType::LessThan => b"lessthan",
                            NixTokenType::GreaterThan => b"greaterthan",
                            NixTokenType::And => b"and",
                            NixTokenType::Implies => b"implies",
                            NixTokenType::Update => b"update",
                            NixTokenType::Concatenate => b"concatenate",
                            NixTokenType::Assign => b"assign",
                            NixTokenType::Semicolon => b"semicolon",
                            NixTokenType::Colon => b"colon",
                            NixTokenType::Select => b"select",
                            NixTokenType::Comma => b"comman",
                            NixTokenType::AtSign => b"atsign",
                            NixTokenType::QuestionMark => b"questionmark",
                            NixTokenType::ExclamationMark => b"exclamationmark",
                            NixTokenType::Addition => b"addition",
                            NixTokenType::Subtraction => b"subtractoin",
                            NixTokenType::Multiplication => b"multiplication",
                            NixTokenType::Division => b"division",
                            _ => todo!(),
                        })),
                        Box::new(result),
                    )),
                    Box::new(rhs),
                );
            } else {
                self.lexer.reset_peek();
                return Some(result);
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "sel", skip_all, ret))]
    pub fn parse_expr_select(&mut self) -> Option<AST<'a>> {
        let expr = self.parse_expr_simple()?;
        let peeked = self.lexer.peek();
        if let Some(NixToken {
            token_type: NixTokenType::Select,
        }) = peeked
        {
            self.expect(NixTokenType::Select);
            // TODO FIXME we probably need to fix that method (use a custom one because of function application order)
            let attrpath = self.parse_attrpath();
            // we need to parse it specially because evaluation needs to abort if the attrpath does not exist and there is no or
            let value = AST::Call(
                Box::new(AST::Call(
                    Box::new(AST::Identifier(BUILTIN_SELECT)),
                    Box::new(expr),
                )),
                Box::new(attrpath.unwrap()),
            );
            if let Some(NixToken {
                token_type: NixTokenType::Identifier(b"or"),
            }) = self.lexer.peek()
            {
                self.lexer.next();
                let default = self.parse_expr_simple().unwrap();
                Some(AST::Call(
                    Box::new(AST::Call(
                        Box::new(AST::Identifier(b"__value_or_default")),
                        Box::new(value),
                    )),
                    Box::new(default),
                ))
            } else {
                self.lexer.reset_peek();
                // also add abort call
                // TODO FIXME replace all inner calls in parse_attrpath for early abort (also mentions more accurate location then)
                Some(AST::Call(
                    Box::new(AST::Identifier(b"__abort_invalid_attrpath")),
                    Box::new(value),
                ))
            }
        } else {
            self.lexer.reset_peek();
            Some(expr)
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "app", skip_all, ret))]
    pub fn parse_expr_app(&mut self ) -> Option<AST<'a>> {
        let mut result: Option<AST<'a>> = None;
        loop {
            let jo = self.parse_expr_select();
            match jo {
                Some(expr) => {
                    match result {
                        Some(a) => {
                            result = Some(AST::Call(Box::new(a), Box::new(expr)));
                        }
                        None => result = Some(expr),
                    }

                    //lexer.next();
                }
                None => {
                    break;
                }
            }
        }
        result
    }

    #[cfg_attr(debug_assertions, instrument(name = "-", skip_all, ret))]
    pub fn parse_expr_arithmetic_negation(&mut self) -> Option<AST<'a>> {
        if let Some(NixToken {
            token_type: NixTokenType::Subtraction,
        }) = self.lexer.peek()
        {
            self.expect(NixTokenType::Subtraction);
            Some(AST::Call(
                Box::new(AST::Identifier(BUILTIN_UNARY_MINUS)),
                Box::new(
                    self.parse_expr_app().expect("failed to parse arithmetic minus expression"),
                ),
            ))
        } else {
            self.lexer.reset_peek();
            self.parse_expr_app()
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "?", skip_all, ret))]
    pub fn parse_expr_has_attribute(&mut self) -> Option<AST<'a>> {
        // TODO FIXME RHS needs to be attrpath
        self.parse_expr_infix_split(
            Parser::parse_expr_arithmetic_negation,
            Parser::parse_attrpath,
            &[NixTokenType::QuestionMark],
        )
    }

    #[cfg_attr(debug_assertions, instrument(name = "++", skip_all, ret))]
    pub fn parse_expr_list_concatenation(&mut self) -> Option<AST<'a>> {
        self.parse_expr_infix(
            Parser::parse_expr_has_attribute,
            &[NixTokenType::Concatenate],
        )
    }

    #[cfg_attr(debug_assertions, instrument(name = "*/", skip_all, ret))]
    pub fn parse_expr_arithmetic_mul_div(&mut self) -> Option<AST<'a>> {
        self.parse_expr_infix(
            Parser::parse_expr_list_concatenation,
            &[NixTokenType::Multiplication, NixTokenType::Division],
        )
    }

    #[cfg_attr(debug_assertions, instrument(name = "+-", skip_all, ret))]
    pub fn parse_expr_arithmetic_or_concatenate(&mut self) -> Option<AST<'a>> {
        self.parse_expr_infix(
            Parser::parse_expr_arithmetic_mul_div,
            &[NixTokenType::Addition, NixTokenType::Subtraction],
        )
    }

    #[cfg_attr(debug_assertions, instrument(name = "!", skip_all, ret))]
    pub fn parse_expr_not(&mut self) -> Option<AST<'a>> {
        if let Some(NixToken {
            token_type: NixTokenType::ExclamationMark,
        }) = self.lexer.peek()
        {
            self.expect(NixTokenType::ExclamationMark);
            Some(AST::Call(
                Box::new(AST::Identifier(BUILTIN_UNARY_NOT)),
                Box::new(
                    self.parse_expr_arithmetic_or_concatenate()
                        .expect("failed to parse negated expression"),
                ),
            ))
        } else {
            self.lexer.reset_peek();
            self.parse_expr_arithmetic_or_concatenate()
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "//", skip_all, ret))]
    pub fn parse_expr_update(&mut self ) -> Option<AST<'a>> {
        self.parse_expr_infix(Parser::parse_expr_not, &[NixTokenType::Update])
    }

    #[cfg_attr(debug_assertions, instrument(name = "<=>", skip_all, ret))]
    pub fn parse_expr_comparison(&mut self ) -> Option<AST<'a>> {
        self.parse_expr_infix(
            Parser::parse_expr_update,
            &[
                NixTokenType::LessThan,
                NixTokenType::LessThanOrEqual,
                NixTokenType::GreaterThan,
                NixTokenType::GreaterThanOrEqual,
            ],
        )
    }

    #[cfg_attr(debug_assertions, instrument(name = "=!=", skip_all, ret))]
    pub fn parse_expr_inequality_or_equality(&mut self) -> Option<AST<'a>> {
        self.parse_expr_infix(
            Parser::parse_expr_comparison,
            &[NixTokenType::Equals, NixTokenType::NotEquals],
        )
    }

    #[cfg_attr(debug_assertions, instrument(name = "&&", skip_all, ret))]
    pub fn parse_expr_logical_and(&mut self) -> Option<AST<'a>> {
        self.parse_expr_infix(
            Parser::parse_expr_inequality_or_equality,
            &[NixTokenType::And],
        )
    }

    #[cfg_attr(debug_assertions, instrument(name = "||", skip_all, ret))]
    pub fn parse_expr_logical_or(&mut self) -> Option<AST<'a>> {
        self.parse_expr_infix( Parser::parse_expr_logical_and, &[NixTokenType::Or])
    }

    #[cfg_attr(debug_assertions, instrument(name = "->", skip_all, ret))]
    pub fn parse_expr_logical_implication(&mut self) -> Option<AST<'a>> {
        self.parse_expr_infix(Parser::parse_expr_logical_or, &[NixTokenType::Implies])
    }

    #[cfg_attr(debug_assertions, instrument(name = "op", skip_all, ret))]
    pub fn parse_expr_op(&mut self ) -> Option<AST<'a>> {
        self.parse_expr_logical_implication()
    }

    #[cfg_attr(debug_assertions, instrument(name = "if", skip_all, ret))]
    pub fn parse_expr_if(&mut self ) -> Option<AST<'a>> {
        match self.lexer.peek() {
            Some(NixToken {
                token_type: NixTokenType::If,
            }) => {
                self.expect(NixTokenType::If);
                let condition = self.parse_expr().expect("failed to parse if condition");
                self.expect(NixTokenType::Then);
                let true_case = self.parse_expr().expect("failed to parse if true case");
                self.expect( NixTokenType::Else);
                let false_case = self.parse_expr().expect("failed to parse if false case");
                Some(AST::Call(
                    Box::new(AST::Call(
                        Box::new(AST::Call(
                            Box::new(AST::Identifier(BUILTIN_IF)),
                            Box::new(condition),
                        )),
                        Box::new(true_case),
                    )),
                    Box::new(false_case),
                ))
            }
            _ => {
                self.lexer.reset_peek();
                self.parse_expr_op()
            }
        }
    }

    // this returns none for some reason
    #[cfg_attr(debug_assertions, instrument(name = "args", skip_all, ret))]
    pub fn parse_formals(&mut self) -> Option<AST<'a>> {
        // we need quite some peekahead here do differentiate between attrsets
        // this is probably the most complicated function in here
        let formals: Vec<AST<'a>> = Vec::new();
        let mut parsed_first = false;
        if let Some(NixToken {
            token_type: NixTokenType::CurlyOpen,
        }) = self.lexer.peek()
        {
            loop {
                match self.lexer.peek() {
                    Some(NixToken {
                        token_type: NixTokenType::Identifier(a),
                    }) => {
                        let token = self.lexer.peek();
                        if let Some(NixToken {
                            token_type: NixTokenType::QuestionMark,
                        }) = token
                        {
                            if !parsed_first {
                                self.expect( NixTokenType::CurlyOpen);
                                parsed_first = true;
                            }
                            self.expect( NixTokenType::Identifier(b""));
                            self.expect( NixTokenType::QuestionMark);
                            self.parse_expr();
                        } else if let Some(NixToken {
                            token_type: NixTokenType::Comma,
                        }) = token
                        {
                            if !parsed_first {
                                self.expect( NixTokenType::CurlyOpen);
                                parsed_first = true;
                            }
                            self.expect( NixTokenType::Identifier(b""));
                            self.expect( NixTokenType::Comma);
                        } else if let Some(NixToken {
                            token_type: NixTokenType::CurlyClose,
                        }) = token
                        {
                            if !parsed_first {
                                self.expect(NixTokenType::CurlyOpen);
                                parsed_first = true;
                            }
                            self.expect( NixTokenType::Identifier(b""));
                            self.expect( NixTokenType::CurlyClose);
                            return Some(AST::Identifier(b"TODO formals")); // TODO FIXME
                        } else {
                            // probably an attrset
                            self.lexer.reset_peek();
                            return None;
                        }
                    }
                    Some(NixToken {
                        token_type: NixTokenType::Inherit,
                    }) => {
                        // attrset
                        self.lexer.reset_peek();
                        return None;
                    }
                    Some(NixToken {
                        token_type: NixTokenType::Comma,
                    }) => {
                        if !parsed_first {
                            self.expect( NixTokenType::CurlyOpen);
                            parsed_first = true;
                        }
                        self.expect( NixTokenType::Comma);
                    }
                    Some(NixToken {
                        token_type: NixTokenType::Ellipsis,
                    }) => {
                        if !parsed_first {
                            self.expect( NixTokenType::CurlyOpen);
                            parsed_first = true;
                        }
                        self.expect( NixTokenType::Ellipsis);
                    }
                    Some(NixToken {
                        token_type: NixTokenType::CurlyClose,
                    }) => {
                        if !parsed_first {
                            match self.lexer.peek() {
                                Some(NixToken {
                                    token_type: NixTokenType::Colon,
                                }) => {
                                    // empty function
                                    self.expect( NixTokenType::CurlyOpen);
                                    self.expect( NixTokenType::CurlyClose);
                                    self.lexer.reset_peek();
                                    return Some(AST::Identifier(b"TODO formals"));
                                    // TODO FIXME
                                }
                                Some(NixToken {
                                    token_type: NixTokenType::AtSign,
                                }) => {
                                    // empty function in stupid
                                    self.expect( NixTokenType::CurlyOpen);
                                    self.expect( NixTokenType::CurlyClose);
                                    self.expect( NixTokenType::AtSign);
                                    self.expect( NixTokenType::Identifier(b""));
                                    self.lexer.reset_peek();
                                    return Some(AST::Identifier(b"TODO formals"));
                                    // TODO FIXME
                                }
                                _ => {
                                    // potentially empty attrset
                                    self.lexer.reset_peek();
                                    return None;
                                }
                            }
                        }
                        self.expect( NixTokenType::CurlyClose);
                        return Some(AST::Identifier(b"TODO formals")); // TODO FIXME
                    }
                    Some(NixToken {
                        token_type: NixTokenType::StringStart,
                    })
                    | Some(NixToken {
                        token_type: NixTokenType::InterpolateStart,
                    }) => {
                        // that's not how formals look like
                        self.lexer.reset_peek();
                        return None;
                    }
                    token => panic!("{:?}", token),
                }
            }
        } else {
            self.lexer.reset_peek();
            None
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "fn", skip_all, ret))]
    pub fn parse_expr_function(&mut self) -> Option<AST<'a>> {
        let token = self.lexer.peek();
        match token.map(|t| &t.token_type) {
            Some(NixTokenType::Let) => {
                self.lexer.reset_peek();
                self.parse_let()
            }
            Some(NixTokenType::CurlyOpen) => {
                self.lexer.reset_peek();
                let formals = self.parse_formals();
                if let None = formals {
                    // not a function, probably an attrset
                    return self.parse_expr_if();
                }
                match self.lexer.next() {
                    Some(NixToken {
                        token_type: NixTokenType::Colon,
                    }) => {}
                    Some(NixToken {
                        token_type: NixTokenType::AtSign,
                    }) => {
                        let ident = self.expect( NixTokenType::Identifier(b""));
                        self.expect( NixTokenType::Colon);
                    }
                    _ => todo!(),
                }
                self.parse_expr_function()
            }
            Some(NixTokenType::Identifier(ident)) => {
                match self.lexer.peek() {
                    Some(NixToken {
                        token_type: NixTokenType::Colon,
                    }) => {
                        // function call
                        // TODO parameter
                        let ident = self.lexer.next();
                        self.expect( NixTokenType::Colon);
                        self.parse_expr_function()
                    }
                    Some(NixToken {
                        token_type: NixTokenType::AtSign,
                    }) => {
                        // function call
                        let ident = self.lexer.next();
                        self.expect( NixTokenType::AtSign);
                        let formals = self.parse_formals().unwrap();
                        self.expect( NixTokenType::Colon);
                        self.parse_expr_function()
                    }
                    _ => {
                        self.lexer.reset_peek();
                        self.parse_expr_if()
                    }
                }
            }
            Some(NixTokenType::Assert) => {
                self.expect( NixTokenType::Assert);
                let assert_expr = self.parse_expr();
                self.expect( NixTokenType::Semicolon);
                let body = self.parse_expr();
                body // TODO FIXME
            }
            Some(NixTokenType::With) => {
                self.expect( NixTokenType::With);
                let with_expr = self.parse_expr();
                self.expect( NixTokenType::Semicolon);
                let body = self.parse_expr();

                body // TODO FIXME
            }
            _ => {
                self.lexer.reset_peek();
                self.parse_expr_if()
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "e", skip_all, ret))]
    pub fn parse_expr(&mut self ) -> Option<AST<'a>> {
        self.parse_expr_function()
    }

    #[cfg_attr(debug_assertions, instrument(name = "p", skip_all, ret))]
    pub fn parse(&mut self  ) -> Option<AST<'a>> {
        let result = self.parse_expr();
        assert_eq!(None, self.lexer.next());
        result
    }
}

#[cfg(test)]
fn can_parse(code: &str) {
    use crate::ast::ASTBuilder;

    std::fs::write("/tmp/foo", code).expect("Unable to write file");

    let mut cmd = Command::new("nix");

    cmd.arg("eval").arg("-f").arg("/tmp/foo");

    let output = cmd.output().unwrap();

    println!(
        "exited with {} {} {}",
        output.status,
        String::from_utf8(output.stderr).unwrap(),
        String::from_utf8(output.stdout).unwrap()
    );

    if !output.status.success() {
        panic!("invalid expr (according to the official nix evaluator)");
    }

    let lexer = crate::lexer::NixLexer::new(code.as_bytes()).filter(|t| match t.token_type {
        NixTokenType::Whitespace(_)
        | NixTokenType::SingleLineComment(_)
        | NixTokenType::MultiLineComment(_) => false,
        _ => true,
    });

    for token in lexer.clone() {
        println!("{:?}", token.token_type);
    }

    let mut parser = Parser {
        lexer: itertools::multipeek(lexer),
        visitor: ASTBuilder,
        phantom: PhantomData,
    };

    let result = parser.parse();
}

#[test]
fn test_operators() {
    let subscriber = tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    // will get desugared so don't care
    //can_parse(r##"let { body = 1; }"##);

    can_parse(r##"2.39583"##);

    can_parse(
        r##"{k}:
    (i: i ? ${k})
    "##,
    );

    can_parse(
        r##"
    {param}:
    with param;
      !pkgs.stdenv.hostPlatform.isAarch64 || cfg.version >= 3

    
    "##,
    );

    can_parse("1");

    can_parse(r##"-1"##);

    can_parse(
        r#"{
        "str" = 1;
      }"#,
    );

    can_parse(r#"{}@a: 1"#);

    can_parse(r#"a@{}: 1"#);

    can_parse(r#"1 != 1"#);

    can_parse(r#"a: 1"#);

    can_parse(
        r#"
        {param}:
        with param; [
            attrpath.withdot
          ]
        "#,
    );

    can_parse(
        r#"
{
  src = 1 + 1;
}
"#,
    );

    // another lookahead issue
    can_parse("{ }: 1");
    can_parse("{ }");

    can_parse(r#"{ a = "b"; }"#);

    can_parse("{ ... }: 1");

    can_parse(
        "{
        members = [];
    }",
    );

    let mut parser = Parser { lexer: itertools::multipeek(
        [
            NixToken {
                token_type: NixTokenType::Integer(1),
            },
            NixToken {
                token_type: NixTokenType::Addition,
            },
            NixToken {
                token_type: NixTokenType::Integer(41),
            },
        ]
        .into_iter()),
        visitor: ASTBuilder,
        phantom: PhantomData, };
    let r = parser.parse_expr_op()
    .unwrap();
    assert_eq!(
        AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Identifier(b"addition")),
                Box::new(AST::Integer(1))
            )),
            Box::new(AST::Integer(41))
        ),
        r
    );
}
