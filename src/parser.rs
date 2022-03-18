use crate::{
    ast::AST,
    lexer::{NixToken, NixTokenType},
    visitor::ASTVisitor,
};

use itertools::MultiPeek;
use std::{marker::PhantomData, mem::discriminant};
//#[cfg(debug_assertions)]
use tracing::instrument;

// TODO FIXME call lexer.reset_peek(); everywhere

// TODO FIXME right-associativity and no associativity

// https://matklad.github.io/2020/03/22/fast-simple-rust-interner.html

pub const BUILTIN_UNARY_NOT: &[u8] = b"__builtin_unary_not";
pub const BUILTIN_PATH_CONCATENATE: &[u8] = b"__builtin_path_concatenate";
pub const BUILTIN_IF: &[u8] = b"__builtin_if";
pub const BUILTIN_STRING_CONCATENATE: &[u8] = b"__builtin_string_concatenate";
pub const BUILTIN_UNARY_MINUS: &[u8] = b"__builtin_unary_minus";
pub const BUILTIN_SELECT: &[u8] = b"__builtin_select";

pub struct Parser<
    'a,
    I: Iterator<Item = NixToken<'a>> + std::fmt::Debug,
    R: std::fmt::Debug,
    A: ASTVisitor<'a, R>,
> {
    pub lexer: MultiPeek<I>,
    pub visitor: A,
    pub phantom: PhantomData<R>, // https://github.com/rust-lang/rust/issues/23246
}

#[derive(Copy, Clone)]
pub enum BindType {
    Let,
    Attrset,
}

impl<
        'a,
        I: Iterator<Item = NixToken<'a>> + std::fmt::Debug,
        R: std::fmt::Debug,
        A: ASTVisitor<'a, R>,
    > Parser<'a, I, R, A>
{
    #[cfg_attr(debug_assertions, instrument(name = "expect", skip_all, ret))]
    pub fn expect(&mut self, t: NixTokenType<'a>) -> NixToken {
        let token = self.lexer.next();
        if discriminant(&token.as_ref().unwrap().token_type) != discriminant(&t) {
            panic!("expected {:?} but got {:?}", t, &token)
        }
        token.unwrap()
    }

    #[cfg_attr(debug_assertions, instrument(name = "attrpath", skip_all, ret))]
    pub fn parse_attrpath(&mut self) -> Option<R> {
        let mut result: Option<R> = None;
        loop {
            match self.lexer.peek() {
                Some(NixToken {
                    token_type: NixTokenType::Identifier(id),
                }) => {
                    let id_ast = self.visitor.visit_identifier(id);
                    match result {
                        Some(a) => {
                            result = Some(self.visitor.visit_attrpath_part(a, id_ast));
                        }
                        None => result = Some(id_ast),
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
                    let res = self
                        .parse_some_string(NixTokenType::StringStart, NixTokenType::StringEnd)
                        .unwrap();
                    match result {
                        Some(a) => {
                            result = Some(self.visitor.visit_attrpath_part(a, res));
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
                            result = Some(self.visitor.visit_attrpath_part(a, expr));
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
    pub fn parse_bind(&mut self, bind_type: BindType) -> R {
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
                        let _expr = self.parse_expr();
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
                self.visitor.visit_todo()
            }
            _other => {
                self.lexer.reset_peek();

                self.visitor.visit_bind_before(bind_type);

                let attrpath = self.parse_attrpath().unwrap();
                self.expect(NixTokenType::Assign);

                self.visitor.visit_bind_between(bind_type, &attrpath);

                let expr = self
                    .parse_expr()
                    .expect("expected expression in binding at");
                self.expect(NixTokenType::Semicolon);

                self.visitor.visit_bind_after(bind_type, attrpath, expr)
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "let", skip_all, ret))]
    pub fn parse_let(&mut self) -> Option<R> {
        self.expect(NixTokenType::Let);

        self.visitor.visit_let_before();

        // maybe do this like the method after? so the let has a third parameter which is the body and which we can then concatenate afterwards
        let mut binds: Vec<R> = Vec::new();
        loop {
            match self.lexer.peek() {
                Some(NixToken {
                    token_type: NixTokenType::In,
                }) => {
                    self.lexer.next();

                    self.visitor.visit_let_before_body(&binds);

                    let body = self
                        .parse_expr_function()
                        .expect("failed to parse body of let binding");

                    return Some(self.visitor.visit_let(binds, body));
                }
                _ => {
                    self.lexer.reset_peek();
                    let bind = self.parse_bind(BindType::Let);

                    binds.push(self.visitor.visit_let_push_bind(&binds, bind));
                }
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "path", skip_all, ret))]
    pub fn parse_path(&mut self) -> Option<R> {
        self.expect(NixTokenType::PathStart);
        let mut result: Option<R> = None;
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
                            result = Some(self.visitor.visit_path_concatenate(a, expr));
                        }
                        None => result = Some(expr),
                    }
                }
                Some(NixToken {
                    token_type: NixTokenType::PathSegment(segment),
                }) => match result {
                    Some(a) => {
                        let segment = self.visitor.visit_path_segment(segment);
                        result = Some(self.visitor.visit_path_concatenate(a, segment));
                    }
                    None => result = Some(self.visitor.visit_path_segment(segment)),
                },
                _ => {
                    todo!();
                }
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "str", skip_all, ret))]
    pub fn parse_some_string(
        &mut self,
        start: NixTokenType<'a>,
        _end: NixTokenType<'a>,
    ) -> Option<R> {
        self.expect(start);
        let mut accum: Option<R> = None;
        loop {
            match self.lexer.next() {
                Some(NixToken {
                    token_type: NixTokenType::String(string),
                }) => {
                    let string = self.visitor.visit_string(string);
                    accum = Some(self.visitor.visit_string_concatenate(accum, string));
                }
                Some(NixToken {
                    token_type: NixTokenType::IndentedString(string),
                }) => {
                    let string = self.visitor.visit_string(string);
                    accum = Some(self.visitor.visit_string_concatenate(accum, string));
                }
                Some(NixToken {
                    token_type: NixTokenType::InterpolateStart,
                }) => {
                    let expr = self.parse_expr().unwrap();
                    self.expect(NixTokenType::CurlyClose);
                    accum = Some(self.visitor.visit_string_concatenate(accum, expr));
                }
                Some(NixToken { token_type: _end }) => {
                    break Some(self.visitor.visit_string_concatenate_end(accum))
                }
                v => panic!("unexpected {:?}", v),
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "attrs", skip_all, ret))]
    pub fn parse_attrset(&mut self) -> Option<R> {
        // caller needs to do
        // self.expect(NixTokenType::CurlyOpen);

        let mut binds: Option<R> = None;
        loop {
            self.visitor.visit_attrset_before(&binds);
            match self.lexer.peek() {
                Some(NixToken {
                    token_type: NixTokenType::CurlyClose,
                }) => {
                    self.expect(NixTokenType::CurlyClose);

                    break Some(self.visitor.visit_attrset(binds));
                }
                _ => {
                    self.lexer.reset_peek();
                    let bind = self.parse_bind(BindType::Attrset);

                    binds = Some(self.visitor.visit_attrset_bind_push(binds, bind));
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
                self.parse_path()
            }
            Some(NixToken {
                token_type: NixTokenType::IndentedStringStart,
            }) => {
                self.lexer.reset_peek();
                self.parse_some_string(
                    NixTokenType::IndentedStringStart,
                    NixTokenType::IndentedStringEnd,
                )
            }
            Some(NixToken {
                token_type: NixTokenType::StringStart,
            }) => {
                self.lexer.reset_peek();
                self.parse_some_string(NixTokenType::StringStart, NixTokenType::StringEnd)
            }
            Some(NixToken {
                token_type: NixTokenType::ParenOpen,
            }) => {
                self.expect(NixTokenType::ParenOpen);
                let expr = self.parse_expr();
                self.expect(NixTokenType::ParenClose);
                expr
            }
            Some(NixToken {
                token_type: NixTokenType::CurlyOpen,
            }) => {
                self.expect(NixTokenType::CurlyOpen);
                self.parse_attrset()
            }
            Some(NixToken {
                token_type: NixTokenType::BracketOpen,
            }) => {
                // array
                self.expect(NixTokenType::BracketOpen);
                self.visitor.visit_array_start();
                let mut array = None;
                loop {
                    match self.lexer.peek() {
                        Some(NixToken {
                            token_type: NixTokenType::BracketClose,
                        }) => {
                            self.lexer.next();
                            break;
                        }
                        _tokens => {
                            self.lexer.reset_peek();

                            self.visitor.visit_array_push_before(&array);
                            let last = self.parse_expr_select().unwrap();
                            array = Some(self.visitor.visit_array_push(array, last))
                        }
                    }
                }
                Some(self.visitor.visit_array_end(array.unwrap()))
            }
            Some(NixToken {
                token_type: NixTokenType::Let,
            }) => {
                self.expect(NixTokenType::Let);
                self.expect(NixTokenType::CurlyOpen);
                self.parse_attrset();
                Some(self.visitor.visit_todo())
            }
            Some(NixToken {
                token_type: NixTokenType::Rec,
            }) => {
                self.expect(NixTokenType::Rec);
                self.expect(NixTokenType::CurlyOpen);
                self.parse_attrset();
                Some(self.visitor.visit_todo())
            }
            _ => {
                self.lexer.reset_peek();
                None
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "", skip_all, ret))]
    pub fn parse_expr_infix<F: FnMut(&mut Self) -> Option<R> + Copy>(
        &mut self,
        f: F,
        operators: &[NixTokenType],
    ) -> Option<R> {
        self.parse_expr_infix_split(f, f, operators)
    }

    #[cfg_attr(debug_assertions, instrument(name = "", skip_all, ret))]
    pub fn parse_expr_infix_split<
        F1: FnMut(&mut Self) -> Option<R>,
        F2: FnMut(&mut Self) -> Option<R>,
    >(
        &mut self,
        mut flhs: F1,
        mut frhs: F2,
        operators: &[NixTokenType],
    ) -> Option<R> {
        let mut result = flhs(self)?;
        loop {
            let next_token = self.lexer.peek();
            if next_token.is_none() {
                self.lexer.reset_peek();
                return Some(result);
            }
            if operators.contains(&next_token.unwrap().token_type) {
                let token = self.lexer.next().unwrap();
                self.visitor.visit_infix_lhs(token.token_type, &result);
                let rhs = frhs(self).unwrap_or_else(|| {
                    panic!(
                        "expected right hand side after {:?} but got nothing",
                        token.token_type
                    )
                });
                // TODO FIXME replace leaking by match to function name
                result = self
                    .visitor
                    .visit_infix_operation(result, rhs, token.token_type);
            } else {
                self.lexer.reset_peek();
                return Some(result);
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "sel", skip_all, ret))]
    pub fn parse_expr_select(&mut self) -> Option<R> {
        let expr = self.parse_expr_simple()?;
        let peeked = self.lexer.peek();
        if let Some(NixToken {
            token_type: NixTokenType::Select,
        }) = peeked
        {
            self.expect(NixTokenType::Select);
            // TODO FIXME we probably need to fix that method (use a custom one because of function application order)
            let attrpath = self.parse_attrpath().unwrap();
            // we need to parse it specially because evaluation needs to abort if the attrpath does not exist and there is no or
            if let Some(NixToken {
                token_type: NixTokenType::Identifier(b"or"),
            }) = self.lexer.peek()
            {
                self.lexer.next();
                let default = self.parse_expr_simple().unwrap();

                Some(self.visitor.visit_select(expr, attrpath, Some(default)))
            } else {
                self.lexer.reset_peek();
                // also add abort call
                // TODO FIXME replace all inner calls in parse_attrpath for early abort (also mentions more accurate location then)
                Some(self.visitor.visit_select(expr, attrpath, None))
            }
        } else {
            self.lexer.reset_peek();
            Some(expr)
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "app", skip_all, ret))]
    pub fn parse_expr_app(&mut self) -> Option<R> {
        let mut result: Option<R> = None;
        loop {
            self.visitor.visit_call_maybe(&result);
            let jo = self.parse_expr_select();
            match jo {
                Some(expr) => {
                    match result {
                        Some(a) => {
                            result = Some(self.visitor.visit_call(a, expr));
                        }
                        None => result = Some(expr),
                    }
                    //lexer.next();
                }
                None => {
                    self.visitor.visit_call_maybe_not();
                    break;
                }
            }
        }
        result
    }

    #[cfg_attr(debug_assertions, instrument(name = "-", skip_all, ret))]
    pub fn parse_expr_arithmetic_negation(&mut self) -> Option<R> {
        if let Some(NixToken {
            token_type: NixTokenType::Subtraction,
        }) = self.lexer.peek()
        {
            self.expect(NixTokenType::Subtraction);
            let expr = self
                .parse_expr_app()
                .expect("failed to parse arithmetic minus expression");
            Some(
                self.visitor
                    .visit_prefix_operation(NixTokenType::Subtraction, expr),
            )
        } else {
            self.lexer.reset_peek();
            self.parse_expr_app()
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "?", skip_all, ret))]
    pub fn parse_expr_has_attribute(&mut self) -> Option<R> {
        // TODO FIXME RHS needs to be attrpath
        self.parse_expr_infix_split(
            Parser::parse_expr_arithmetic_negation,
            Parser::parse_attrpath,
            &[NixTokenType::QuestionMark],
        )
    }

    #[cfg_attr(debug_assertions, instrument(name = "++", skip_all, ret))]
    pub fn parse_expr_list_concatenation(&mut self) -> Option<R> {
        self.parse_expr_infix(
            Parser::parse_expr_has_attribute,
            &[NixTokenType::Concatenate],
        )
    }

    #[cfg_attr(debug_assertions, instrument(name = "*/", skip_all, ret))]
    pub fn parse_expr_arithmetic_mul_div(&mut self) -> Option<R> {
        self.parse_expr_infix(
            Parser::parse_expr_list_concatenation,
            &[NixTokenType::Multiplication, NixTokenType::Division],
        )
    }

    #[cfg_attr(debug_assertions, instrument(name = "+-", skip_all, ret))]
    pub fn parse_expr_arithmetic_or_concatenate(&mut self) -> Option<R> {
        self.parse_expr_infix(
            Parser::parse_expr_arithmetic_mul_div,
            &[NixTokenType::Addition, NixTokenType::Subtraction],
        )
    }

    #[cfg_attr(debug_assertions, instrument(name = "!", skip_all, ret))]
    pub fn parse_expr_not(&mut self) -> Option<R> {
        if let Some(NixToken {
            token_type: NixTokenType::ExclamationMark,
        }) = self.lexer.peek()
        {
            self.expect(NixTokenType::ExclamationMark);
            let expr = self
                .parse_expr_arithmetic_or_concatenate()
                .expect("failed to parse negated expression");
            Some(
                self.visitor
                    .visit_prefix_operation(NixTokenType::ExclamationMark, expr),
            )
        } else {
            self.lexer.reset_peek();
            self.parse_expr_arithmetic_or_concatenate()
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "//", skip_all, ret))]
    pub fn parse_expr_update(&mut self) -> Option<R> {
        self.parse_expr_infix(Parser::parse_expr_not, &[NixTokenType::Update])
    }

    #[cfg_attr(debug_assertions, instrument(name = "<=>", skip_all, ret))]
    pub fn parse_expr_comparison(&mut self) -> Option<R> {
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
    pub fn parse_expr_inequality_or_equality(&mut self) -> Option<R> {
        self.parse_expr_infix(
            Parser::parse_expr_comparison,
            &[NixTokenType::Equals, NixTokenType::NotEquals],
        )
    }

    #[cfg_attr(debug_assertions, instrument(name = "&&", skip_all, ret))]
    pub fn parse_expr_logical_and(&mut self) -> Option<R> {
        self.parse_expr_infix(
            Parser::parse_expr_inequality_or_equality,
            &[NixTokenType::And],
        )
    }

    #[cfg_attr(debug_assertions, instrument(name = "||", skip_all, ret))]
    pub fn parse_expr_logical_or(&mut self) -> Option<R> {
        self.parse_expr_infix(Parser::parse_expr_logical_and, &[NixTokenType::Or])
    }

    #[cfg_attr(debug_assertions, instrument(name = "->", skip_all, ret))]
    pub fn parse_expr_logical_implication(&mut self) -> Option<R> {
        self.parse_expr_infix(Parser::parse_expr_logical_or, &[NixTokenType::Implies])
    }

    #[cfg_attr(debug_assertions, instrument(name = "op", skip_all, ret))]
    pub fn parse_expr_op(&mut self) -> Option<R> {
        self.parse_expr_logical_implication()
    }

    #[cfg_attr(debug_assertions, instrument(name = "if", skip_all, ret))]
    pub fn parse_expr_if(&mut self) -> Option<R> {
        match self.lexer.peek() {
            Some(NixToken {
                token_type: NixTokenType::If,
            }) => {
                self.expect(NixTokenType::If);
                self.visitor.visit_if_before();
                let condition = self.parse_expr().expect("failed to parse if condition");
                self.visitor.visit_if_after_condition(&condition);
                self.expect(NixTokenType::Then);
                let true_case = self.parse_expr().expect("failed to parse if true case");
                self.visitor
                    .visit_if_after_true_case(&condition, &true_case);
                self.expect(NixTokenType::Else);
                let false_case = self.parse_expr().expect("failed to parse if false case");
                Some(self.visitor.visit_if(condition, true_case, false_case))
            }
            _ => {
                self.lexer.reset_peek();
                self.parse_expr_op()
            }
        }
    }

    // this returns none for some reason
    #[cfg_attr(debug_assertions, instrument(name = "args", skip_all, ret))]
    pub fn parse_formals(&mut self) -> Option<R> {
        // destructured function parameters

        self.expect(NixTokenType::CurlyOpen);

        // check whether this is a formal or attrset
        match self.lexer.peek() {
            Some(NixToken {
                token_type: NixTokenType::Identifier(_a),
            }) => {
                match self.lexer.peek() {
                    Some(NixToken {
                        token_type: NixTokenType::QuestionMark,
                    }) => {
                        // { a ? jo }:
                        self.lexer.reset_peek();
                    }
                    Some(NixToken {
                        token_type: NixTokenType::Comma,
                    }) => {
                        // {a, b}:
                        self.lexer.reset_peek();
                    }
                    Some(NixToken {
                        token_type: NixTokenType::CurlyClose,
                    }) => {
                        // { a }:
                        // { a }@jo:
                        self.lexer.reset_peek();
                    }
                    _ => {
                        // {a=1;}
                        // probably an attrset
                        self.lexer.reset_peek();
                        return self.parse_attrset();
                    }
                }
            }
            Some(NixToken {
                token_type: NixTokenType::CurlyClose,
            }) => {
                match self.lexer.peek() {
                    Some(NixToken {
                        token_type: NixTokenType::Colon,
                    }) => {
                        // {}:
                        self.lexer.reset_peek();
                    }
                    Some(NixToken {
                        token_type: NixTokenType::AtSign,
                    }) => {
                        // {}@jo:
                        // empty function in stupid
                        self.expect(NixTokenType::CurlyClose);
                        self.expect(NixTokenType::AtSign);
                        match self.lexer.next() {
                            Some(NixToken {
                                token_type: NixTokenType::Identifier(_a),
                            }) => {
                                self.lexer.reset_peek();
                                return Some(self.visitor.visit_formals(None, Some(_a), false));
                            }
                            _ => todo!(),
                        }
                    }
                    _ => {
                        // {}
                        // potentially empty attrset
                        self.lexer.reset_peek();
                        return self.parse_attrset();
                    }
                }
            }
            // {inherit a;}
            // {"a"=1;}
            // {${"a"} = 1;}
            _ => {
                // attrset
                self.lexer.reset_peek();
                return self.parse_attrset();
            }
        }

        // this second part here actually parses these formals
        let mut formals: Option<R> = None;

        loop {
            match self.lexer.next() {
                Some(NixToken {
                    token_type: NixTokenType::Identifier(_a),
                }) => {
                    let token = self.lexer.next();
                    if let Some(NixToken {
                        token_type: NixTokenType::QuestionMark,
                    }) = token
                    {
                        let expr = self.parse_expr().unwrap();
                        formals = Some(self.visitor.visit_formal(formals, _a, Some(expr)));
                    } else if let Some(NixToken {
                        token_type: NixTokenType::Comma,
                    }) = token
                    {
                        formals = Some(self.visitor.visit_formal(formals, _a, None));
                    } else if let Some(NixToken {
                        token_type: NixTokenType::CurlyClose,
                    }) = token
                    {
                        return Some(self.visitor.visit_formal(formals, _a, None));
                    } else {
                        panic!();
                    }
                }
                Some(NixToken {
                    token_type: NixTokenType::Comma,
                }) => {}
                Some(NixToken {
                    token_type: NixTokenType::Ellipsis,
                }) => {
                    return Some(self.visitor.visit_formals(None, None, true));
                }
                Some(NixToken {
                    token_type: NixTokenType::CurlyClose,
                }) => {
                    return Some(self.visitor.visit_todo()); //return Some(AST::Identifier(b"TODO formals")); // TODO FIXME
                }
                token => panic!("{:?}", token),
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "fn", skip_all, ret))]
    pub fn parse_expr_function(&mut self) -> Option<R> {
        let token = self.lexer.peek();
        match token.map(|t| &t.token_type) {
            Some(NixTokenType::Let) => {
                self.lexer.reset_peek();
                self.parse_let()
            }
            Some(NixTokenType::CurlyOpen) => {
                self.lexer.reset_peek();
                let formals = self.parse_formals();
                if formals.is_none() {
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
                        let _ident = self.expect(NixTokenType::Identifier(b""));
                        self.expect(NixTokenType::Colon);
                    }
                    _ => todo!(),
                }
                self.parse_expr_function()
            }
            Some(NixTokenType::Identifier(_ident)) => {
                match self.lexer.peek() {
                    Some(NixToken {
                        token_type: NixTokenType::Colon,
                    }) => {
                        // function call
                        let ident = self.lexer.next().unwrap();
                        match ident {
                            NixToken {
                                token_type: NixTokenType::Identifier(ident),
                            } => {
                                self.visitor.visit_function_before();

                                let arg = self.visitor.visit_identifier(ident);

                                self.visitor.visit_function_enter(&arg);

                                self.expect(NixTokenType::Colon);
                                let body = self.parse_expr_function().unwrap();

                                Some(self.visitor.visit_function_exit(arg, body))
                            }
                            _ => todo!(),
                        }
                    }
                    Some(NixToken {
                        token_type: NixTokenType::AtSign,
                    }) => {
                        // function call
                        let _ident = self.lexer.next();
                        self.expect(NixTokenType::AtSign);
                        let _formals = self.parse_formals().unwrap();
                        self.expect(NixTokenType::Colon);
                        self.parse_expr_function()
                    }
                    _ => {
                        self.lexer.reset_peek();
                        self.parse_expr_if()
                    }
                }
            }
            Some(NixTokenType::Assert) => {
                self.expect(NixTokenType::Assert);
                let _assert_expr = self.parse_expr();
                self.expect(NixTokenType::Semicolon);

                self.parse_expr() // TODO FIXME
            }
            Some(NixTokenType::With) => {
                self.expect(NixTokenType::With);
                let _with_expr = self.parse_expr();
                self.expect(NixTokenType::Semicolon);

                self.parse_expr() // TODO FIXME
            }
            _ => {
                self.lexer.reset_peek();
                self.parse_expr_if()
            }
        }
    }

    #[cfg_attr(debug_assertions, instrument(name = "e", skip_all, ret))]
    pub fn parse_expr(&mut self) -> Option<R> {
        self.parse_expr_function()
    }

    #[cfg_attr(debug_assertions, instrument(name = "p", skip_all, ret))]
    pub fn parse(&mut self) -> Option<R> {
        self.visitor.visit_file_start();
        let result = self.parse_expr();
        assert_eq!(None, self.lexer.next());
        self.visitor.visit_file_end();
        result
    }
}

#[cfg(test)]
fn can_parse(code: &str) {
    use crate::ast::ASTBuilder;

    std::fs::write("/tmp/foo", code).expect("Unable to write file");

    let mut cmd = std::process::Command::new("nix");

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

    let lexer = crate::lexer::NixLexer::new(code.as_bytes()).filter(|t| {
        !matches!(
            t.token_type,
            NixTokenType::Whitespace(_)
                | NixTokenType::SingleLineComment(_)
                | NixTokenType::MultiLineComment(_)
        )
    });

    for token in lexer.clone() {
        println!("{:?}", token.token_type);
    }

    let mut parser = Parser {
        lexer: itertools::multipeek(lexer),
        visitor: ASTBuilder,
        phantom: PhantomData,
    };

    let _result = parser.parse();
}

// cargo test parser::test_operators -- --nocapture
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

    let mut parser = Parser {
        lexer: itertools::multipeek(
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
            .into_iter(),
        ),
        visitor: crate::ast::ASTBuilder,
        phantom: PhantomData,
    };
    let r = parser.parse_expr_op().unwrap();
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
