use crate::lexer::{NixLexer, NixToken, NixTokenType};
use core::fmt;
use itertools::MultiPeek;
use std::{
    fmt::Debug,
    mem::discriminant,
    process::{Command, Stdio, ExitStatus},
};
use tracing::instrument;

// TODO FIXME call lexer.reset_peek(); everywhere

// TODO FIXME right-associativity and no associativity

// TODO https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
// TODO https://matklad.github.io/2020/04/15/from-pratt-to-dijkstra.html
// https://matklad.github.io/2020/03/22/fast-simple-rust-interner.html
// https://eli.thegreenplace.net/2009/03/14/some-problems-of-recursive-descent-parsers/
// https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing

/*
expr_op hard

3+4*5

addition = (multiplication {+ multiplication})
multiplication = (number {* number})

addition = multiplication {+ multiplication}
addition = number {* number } {+ multiplication}
addition = number + multiplication
addition = number + number * number

3+4*5+3
addition = multiplication {+ multiplication}
addition = number {+ multiplication}
addition = number + multiplication {+ multiplication}
addition = number + (number * number) {+ multiplication}

*/

const BUILTIN_UNARY_NOT: &[u8] = b"__builtin_unary_not";
const BUILTIN_PATH_CONCATENATE: &[u8] = b"__builtin_path_concatenate";
const BUILTIN_SELECT: &[u8] = b"__builtin_select";
const BUILTIN_IF: &[u8] = b"__builtin_if";
const BUILTIN_STRING_CONCATENATE: &[u8] = b"__builtin_string_concatenate";

#[derive(PartialEq)]
pub enum AST<'a> {
    Identifier(&'a [u8]),
    String(&'a [u8]),
    PathSegment(&'a [u8]), // merge into String
    Integer(i64),
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

#[instrument(name = "expect", skip_all, ret)]
pub fn expect<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
    t: NixTokenType<'a>,
) {
    let token = lexer.next();
    if discriminant(&token.as_ref().unwrap().token_type) != discriminant(&t) {
        panic!("expected {:?} but got {:?}", t, &token)
    }
}

#[instrument(name = "attrpath", skip_all, ret)]
pub fn parse_attrpath<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> AST<'a> {
    let mut result: Option<AST<'a>> = None;
    loop {
        match lexer.peek() {
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

                lexer.next();
            }
            Some(NixToken {
                token_type: NixTokenType::Select,
            }) => {
                expect(lexer, NixTokenType::Select);
            }
            Some(NixToken {
                token_type: NixTokenType::StringStart,
            }) => {
                lexer.reset_peek();
                let res =
                    parse_some_string(lexer, NixTokenType::StringStart, NixTokenType::StringEnd)
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
                expect(lexer, NixTokenType::InterpolateStart);
                let expr = parse_expr(lexer).unwrap();
                expect(lexer, NixTokenType::CurlyClose);
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
                lexer.reset_peek();
                break;
            }
        }
    }
    result.unwrap()
}

#[instrument(name = "bind", skip_all, ret)]
pub fn parse_bind<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> (Box<AST<'a>>, Box<AST<'a>>) {
    match lexer.peek() {
        Some(NixToken {
            token_type: NixTokenType::Inherit,
        }) => {
            expect(lexer, NixTokenType::Inherit);
            match lexer.peek() {
                Some(NixToken {
                    token_type: NixTokenType::ParenOpen,
                }) => {
                    expect(lexer, NixTokenType::ParenOpen);
                    let expr = parse_expr(lexer);
                    expect(lexer, NixTokenType::ParenClose);
                }
                _ => {
                    lexer.reset_peek();
                }
            }
            let mut attrs = Vec::new();
            loop {
                match lexer.peek() {
                    Some(NixToken {
                        token_type: NixTokenType::Identifier(attr),
                    }) => {
                        attrs.push(AST::Identifier(attr));
                        lexer.next();
                    }
                    _ => {
                        // TODO string attrs missing
                        lexer.reset_peek();
                        break;
                    }
                }
            }
            expect(lexer, NixTokenType::Semicolon);
            (
                Box::new(AST::Identifier(b"TODO inherit")),
                Box::new(AST::Identifier(b"TODO inherit")),
            )
        }
        other => {
            lexer.reset_peek();
            let attrpath = parse_attrpath(lexer);
            expect(lexer, NixTokenType::Assign);

            //println!("TEST {:?}", lexer.peek());
            //lexer.reset_peek();

            let expr = parse_expr(lexer).expect("expected expression in binding at");
            expect(lexer, NixTokenType::Semicolon);

            (Box::new(attrpath), Box::new(expr))
        }
    }
}

#[instrument(name = "let", skip_all, ret)]
pub fn parse_let<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    expect(lexer, NixTokenType::Let);
    let mut binds: Vec<(Box<AST<'a>>, Box<AST<'a>>)> = Vec::new();
    loop {
        match lexer.peek() {
            Some(NixToken {
                token_type: NixTokenType::In,
            }) => {
                lexer.next();

                let body = parse_expr_function(lexer).expect("failed to parse body of let binding");

                break Some(binds.into_iter().fold(body, |accum, item| {
                    AST::Let(item.0, item.1, Box::new(accum))
                }));
            }
            _ => {
                lexer.reset_peek();
                let bind = parse_bind(lexer);

                binds.push(bind);
            }
        }
    }
}

#[instrument(name = "path", skip_all, ret)]
pub fn parse_path<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    expect(lexer, NixTokenType::PathStart);
    let mut result: Option<AST<'a>> = None;
    loop {
        let val = lexer.next();
        match val {
            Some(NixToken {
                token_type: NixTokenType::PathEnd,
            }) => {
                break result;
            }
            Some(NixToken {
                token_type: NixTokenType::InterpolateStart,
            }) => {
                let expr = parse_expr(lexer).unwrap();
                expect(lexer, NixTokenType::CurlyClose);
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

#[instrument(name = "str", skip_all, ret)]
pub fn parse_some_string<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
    start: NixTokenType<'a>,
    end: NixTokenType<'a>,
) -> Option<AST<'a>> {
    expect(lexer, start);
    let mut accum = AST::String(b"");
    loop {
        match lexer.next() {
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
                let expr = parse_expr(lexer).unwrap();
                expect(lexer, NixTokenType::CurlyClose);
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

#[instrument(name = "attrs", skip_all, ret)]
pub fn parse_attrset<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    expect(lexer, NixTokenType::CurlyOpen);

    let mut binds: Vec<(Box<AST<'a>>, Box<AST<'a>>)> = Vec::new();
    loop {
        match lexer.peek() {
            Some(NixToken {
                token_type: NixTokenType::CurlyClose,
            }) => {
                expect(lexer, NixTokenType::CurlyClose);

                break Some(
                    binds
                        .into_iter()
                        .fold(AST::Identifier(b"TODO attrset"), |accum, item| {
                            AST::Let(item.0, item.1, Box::new(accum))
                        }),
                );
            }
            _ => {
                lexer.reset_peek();
                let bind = parse_bind(lexer);

                binds.push(bind);
            }
        }
    }
}

#[instrument(name = "simple", skip_all, ret)]
pub fn parse_expr_simple<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    let val = lexer.peek();
    match val {
        Some(NixToken {
            token_type: NixTokenType::Identifier(id),
        }) => {
            let ret = Some(AST::Identifier(id));
            lexer.next();
            ret
        }
        Some(NixToken {
            token_type: NixTokenType::Integer(integer),
        }) => {
            let ret = Some(AST::Integer(*integer));
            lexer.next();
            ret
        }
        Some(NixToken {
            token_type: NixTokenType::PathStart,
        }) => {
            lexer.reset_peek();
            parse_path(lexer)
        }
        Some(NixToken {
            token_type: NixTokenType::IndentedStringStart,
        }) => {
            lexer.reset_peek();
            parse_some_string(
                lexer,
                NixTokenType::IndentedStringStart,
                NixTokenType::IndentedStringEnd,
            )
        }
        Some(NixToken {
            token_type: NixTokenType::StringStart,
        }) => {
            lexer.reset_peek();
            parse_some_string(lexer, NixTokenType::StringStart, NixTokenType::StringEnd)
        }
        Some(NixToken {
            token_type: NixTokenType::ParenOpen,
        }) => {
            expect(lexer, NixTokenType::ParenOpen);
            let expr = parse_expr(lexer);
            expect(lexer, NixTokenType::ParenClose);
            expr
        }
        Some(NixToken {
            token_type: NixTokenType::CurlyOpen,
        }) => parse_attrset(lexer),
        Some(NixToken {
            token_type: NixTokenType::BracketOpen,
        }) => {
            expect(lexer, NixTokenType::BracketOpen);
            let mut array = Vec::new();
            loop {
                match lexer.peek() {
                    Some(NixToken {
                        token_type: NixTokenType::BracketClose,
                    }) => {
                        lexer.next();
                        break;
                    }
                    token => {
                        lexer.reset_peek();
                        array.push(parse_expr_select(lexer).unwrap());
                    }
                }
            }
            return Some(
                array
                    .into_iter()
                    .fold(AST::Identifier(b"cons"), |accum, item| {
                        AST::Call(Box::new(accum), Box::new(item))
                    }),
            );
        }
        Some(NixToken {
            token_type: NixTokenType::Let,
        }) => {
            expect(lexer, NixTokenType::Let);
            parse_attrset(lexer)
        }
        Some(NixToken {
            token_type: NixTokenType::Rec,
        }) => {
            expect(lexer, NixTokenType::Rec);
            parse_attrset(lexer)
        }
        _ => {
            lexer.reset_peek();
            None
        }
    }
}

#[instrument(name = "", skip_all, ret)]
pub fn parse_expr_infix<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
    fun: fn(&mut MultiPeek<I>) -> Option<AST<'a>>,
    operators: &[NixTokenType],
) -> Option<AST<'a>> {
    let mut result = fun(lexer)?;
    loop {
        let next_token = lexer.peek();
        if next_token.is_none() {
            lexer.reset_peek();
            return Some(result);
        }
        if operators.contains(&next_token.unwrap().token_type) {
            let token = lexer.next().unwrap();
            let rhs = fun(lexer).expect(&format!(
                "expected right hand side after {:?} but got nothing",
                token.token_type
            ));
            // TODO FIXME replace leaking by match to function name
            result = AST::Call(
                Box::new(AST::Call(
                    Box::new(AST::Identifier(Vec::leak(
                        format!("{:?}", token.token_type).into_bytes(),
                    ))),
                    Box::new(result),
                )),
                Box::new(rhs),
            );
        } else {
            lexer.reset_peek();
            return Some(result);
        }
    }
}

#[instrument(name = "sel", skip_all, ret)]
pub fn parse_expr_select<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    let expr = parse_expr_simple(lexer)?;
    let peeked = lexer.peek();
    if let Some(NixToken {
        token_type: NixTokenType::Select,
    }) = peeked
    {
        expect(lexer, NixTokenType::Select);
        // TODO FIXME we probably need to fix that method (use a custom one because of function application order)
        let attrpath = parse_attrpath(lexer);
        // we need to parse it specially because evaluation needs to abort if the attrpath does not exist and there is no or
        let value = AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Identifier(BUILTIN_SELECT)),
                Box::new(expr),
            )),
            Box::new(attrpath),
        );
        if let Some(NixToken {
            token_type: NixTokenType::Identifier(b"or"),
        }) = lexer.peek()
        {
            lexer.next();
            let default = parse_expr_simple(lexer).unwrap();
            Some(AST::Call(
                Box::new(AST::Call(
                    Box::new(AST::Identifier(b"__value_or_default")),
                    Box::new(value),
                )),
                Box::new(default),
            ))
        } else {
            lexer.reset_peek();
            // also add abort call
            // TODO FIXME replace all inner calls in parse_attrpath for early abort (also mentions more accurate location then)
            Some(AST::Call(
                Box::new(AST::Identifier(b"__abort_invalid_attrpath")),
                Box::new(value),
            ))
        }
    } else {
        lexer.reset_peek();
        Some(expr)
    }
}

#[instrument(name = "app", skip_all, ret)]
pub fn parse_expr_app<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    let mut result: Option<AST<'a>> = None;
    loop {
        let jo = parse_expr_select(lexer);
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

#[instrument(name = "-", skip_all, ret)]
pub fn parse_expr_arithmetic_negation<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    // TODO FIXME
    parse_expr_app(lexer)
}

#[instrument(name = "?", skip_all, ret)]
pub fn parse_expr_has_attribute<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_infix(
        lexer,
        parse_expr_arithmetic_negation,
        &[NixTokenType::QuestionMark],
    )
}

#[instrument(name = "++", skip_all, ret)]
pub fn parse_expr_list_concatenation<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_infix(
        lexer,
        parse_expr_has_attribute,
        &[NixTokenType::Concatenate],
    )
}

#[instrument(name = "*/", skip_all, ret)]
pub fn parse_expr_arithmetic_mul_div<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_infix(
        lexer,
        parse_expr_list_concatenation,
        &[NixTokenType::Multiplication, NixTokenType::Division],
    )
}

#[instrument(name = "+-", skip_all, ret)]
pub fn parse_expr_arithmetic_or_concatenate<
    'a,
    I: Iterator<Item = NixToken<'a>> + std::fmt::Debug,
>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_infix(
        lexer,
        parse_expr_arithmetic_mul_div,
        &[NixTokenType::Addition, NixTokenType::Subtraction],
    )
}

#[instrument(name = "!", skip_all, ret)]
pub fn parse_expr_not<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    if let Some(NixToken {
        token_type: NixTokenType::ExclamationMark,
    }) = lexer.peek()
    {
        expect(lexer, NixTokenType::ExclamationMark);
        Some(AST::Call(
            Box::new(AST::Identifier(BUILTIN_UNARY_NOT)),
            Box::new(
                parse_expr_arithmetic_or_concatenate(lexer)
                    .expect("failed to parse negated expression"),
            ),
        ))
    } else {
        lexer.reset_peek();
        parse_expr_arithmetic_or_concatenate(lexer)
    }
}

#[instrument(name = "//", skip_all, ret)]
pub fn parse_expr_update<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_infix(lexer, parse_expr_not, &[NixTokenType::Update])
}

#[instrument(name = "<=>", skip_all, ret)]
pub fn parse_expr_comparison<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_infix(
        lexer,
        parse_expr_update,
        &[
            NixTokenType::LessThan,
            NixTokenType::LessThanOrEqual,
            NixTokenType::GreaterThan,
            NixTokenType::GreaterThanOrEqual,
        ],
    )
}

#[instrument(name = "=!=", skip_all, ret)]
pub fn parse_expr_inequality_or_equality<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_infix(
        lexer,
        parse_expr_comparison,
        &[NixTokenType::Equals, NixTokenType::NotEquals],
    )
}

#[instrument(name = "&&", skip_all, ret)]
pub fn parse_expr_logical_and<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_infix(
        lexer,
        parse_expr_inequality_or_equality,
        &[NixTokenType::And],
    )
}

#[instrument(name = "||", skip_all, ret)]
pub fn parse_expr_logical_or<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_infix(lexer, parse_expr_logical_and, &[NixTokenType::Or])
}

#[instrument(name = "->", skip_all, ret)]
pub fn parse_expr_logical_implication<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_infix(lexer, parse_expr_logical_or, &[NixTokenType::Implies])
}

#[instrument(name = "op", skip_all, ret)]
pub fn parse_expr_op<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_logical_implication(lexer)
}

#[instrument(name = "if", skip_all, ret)]
pub fn parse_expr_if<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    match lexer.peek() {
        Some(NixToken {
            token_type: NixTokenType::If,
        }) => {
            expect(lexer, NixTokenType::If);
            let condition = parse_expr(lexer).expect("failed to parse if condition");
            expect(lexer, NixTokenType::Then);
            let true_case = parse_expr(lexer).expect("failed to parse if true case");
            expect(lexer, NixTokenType::Else);
            let false_case = parse_expr(lexer).expect("failed to parse if false case");
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
            lexer.reset_peek();
            parse_expr_op(lexer)
        }
    }
}

// this returns none for some reason
#[instrument(name = "args", skip_all, ret)]
pub fn parse_formals<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    // we need quite some peekahead here do differentiate between attrsets
    // this is probably the most complicated function in here
    let formals: Vec<AST<'a>> = Vec::new();
    let mut parsed_first = false;
    if let Some(NixToken {
        token_type: NixTokenType::CurlyOpen,
    }) = lexer.peek()
    {
        loop {
            match lexer.peek() {
                Some(NixToken {
                    token_type: NixTokenType::Identifier(a),
                }) => {
                    let token = lexer.peek();
                    if let Some(NixToken {
                        token_type: NixTokenType::QuestionMark,
                    }) = token
                    {
                        if !parsed_first {
                            expect(lexer, NixTokenType::CurlyOpen);
                            parsed_first = true;
                        }
                        expect(lexer, NixTokenType::Identifier(b""));
                        expect(lexer, NixTokenType::QuestionMark);
                        parse_expr(lexer);
                    } else if let Some(NixToken {
                        token_type: NixTokenType::Comma,
                    }) = token
                    {
                        if !parsed_first {
                            expect(lexer, NixTokenType::CurlyOpen);
                            parsed_first = true;
                        }
                        expect(lexer, NixTokenType::Identifier(b""));
                        expect(lexer, NixTokenType::Comma);
                    } else if let Some(NixToken {
                        token_type: NixTokenType::CurlyClose,
                    }) = token
                    {
                        if !parsed_first {
                            expect(lexer, NixTokenType::CurlyOpen);
                            parsed_first = true;
                        }
                        expect(lexer, NixTokenType::Identifier(b""));
                        expect(lexer, NixTokenType::CurlyClose);
                        return Some(AST::Identifier(b"TODO formals")); // TODO FIXME
                    } else {
                        // probably an attrset
                        lexer.reset_peek();
                        return None;
                    }
                }
                Some(NixToken {
                    token_type: NixTokenType::Inherit,
                }) => {
                    // attrset
                    lexer.reset_peek();
                    return None;
                }
                Some(NixToken {
                    token_type: NixTokenType::Comma,
                }) => {
                    if !parsed_first {
                        expect(lexer, NixTokenType::CurlyOpen);
                        parsed_first = true;
                    }
                    expect(lexer, NixTokenType::Comma);
                }
                Some(NixToken {
                    token_type: NixTokenType::Ellipsis,
                }) => {
                    if !parsed_first {
                        expect(lexer, NixTokenType::CurlyOpen);
                        parsed_first = true;
                    }
                    expect(lexer, NixTokenType::Ellipsis);
                }
                Some(NixToken {
                    token_type: NixTokenType::CurlyClose,
                }) => {
                    if !parsed_first {
                        match lexer.peek() {
                            Some(NixToken {
                                token_type: NixTokenType::Colon,
                            }) => {
                                // empty function
                                expect(lexer, NixTokenType::CurlyOpen);
                                expect(lexer, NixTokenType::CurlyClose);
                                lexer.reset_peek();
                                return Some(AST::Identifier(b"TODO formals")); // TODO FIXME
                            }
                            Some(NixToken {
                                token_type: NixTokenType::AtSign,
                            }) => {
                                // empty function in stupid
                                expect(lexer, NixTokenType::CurlyOpen);
                                expect(lexer, NixTokenType::CurlyClose);
                                expect(lexer, NixTokenType::AtSign);
                                expect(lexer, NixTokenType::Identifier(b""));
                                lexer.reset_peek();
                                return Some(AST::Identifier(b"TODO formals")); // TODO FIXME
                            }
                            _ => {
                                // potentially empty attrset
                                lexer.reset_peek();
                                return None;
                            }
                        }
                    }
                    expect(lexer, NixTokenType::CurlyClose);
                    return Some(AST::Identifier(b"TODO formals")); // TODO FIXME
                }
                Some(NixToken {
                    token_type: NixTokenType::StringStart,
                })
                | Some(NixToken {
                    token_type: NixTokenType::InterpolateStart,
                }) => {
                    // that's not how formals look like
                    lexer.reset_peek();
                    return None;
                }
                token => panic!("{:?}", token),
            }
        }
    } else {
        lexer.reset_peek();
        None
    }
}

#[instrument(name = "fn", skip_all, ret)]
pub fn parse_expr_function<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    let token = lexer.peek();
    match token.map(|t| &t.token_type) {
        Some(NixTokenType::Let) => {
            lexer.reset_peek();
            parse_let(lexer)
        }
        Some(NixTokenType::CurlyOpen) => {
            lexer.reset_peek();
            let formals = parse_formals(lexer);
            if let None = formals {
                // not a function, probably an attrset
                return parse_expr_if(lexer);
            }
            match lexer.next() {
                Some(NixToken {
                    token_type: NixTokenType::Colon,
                }) => {}
                Some(NixToken {
                    token_type: NixTokenType::AtSign,
                }) => {
                    let ident = expect(lexer, NixTokenType::Identifier(b""));
                    expect(lexer, NixTokenType::Colon);
                }
                _ => todo!(),
            }
            parse_expr_function(lexer)
        }
        Some(NixTokenType::Identifier(ident)) => {
            match lexer.peek() {
                Some(NixToken {
                    token_type: NixTokenType::Colon,
                }) => {
                    // function call
                    // TODO parameter
                    let ident = lexer.next();
                    expect(lexer, NixTokenType::Colon);
                    parse_expr_function(lexer)
                }
                Some(NixToken {
                    token_type: NixTokenType::AtSign,
                }) => {
                    // function call
                    let ident = lexer.next();
                    expect(lexer, NixTokenType::AtSign);
                    let formals = parse_formals(lexer).unwrap();
                    expect(lexer, NixTokenType::Colon);
                    parse_expr_function(lexer)
                }
                _ => {
                    lexer.reset_peek();
                    parse_expr_if(lexer)
                }
            }
        }
        Some(NixTokenType::Assert) => {
            expect(lexer, NixTokenType::Assert);
            let assert_expr = parse_expr(lexer);
            expect(lexer, NixTokenType::Semicolon);
            let body = parse_expr(lexer);
            body // TODO FIXME
        }
        Some(NixTokenType::With) => {
            expect(lexer, NixTokenType::With);
            let with_expr = parse_expr(lexer);
            expect(lexer, NixTokenType::Semicolon);
            let body = parse_expr(lexer);

            body // TODO FIXME
        }
        _ => {
            lexer.reset_peek();
            parse_expr_if(lexer)
        }
    }
}

#[instrument(name = "e", skip_all, ret)]
pub fn parse_expr<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_function(lexer)
}

#[instrument(name = "p", skip_all, ret)]
pub fn parse<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    tracing::trace!("Hello, world!");

    let result = parse_expr(lexer);
    assert_eq!(None, lexer.next());
    result
}

#[cfg(test)]
fn can_parse(code: &str) {
    std::fs::write("/tmp/foo", code).expect("Unable to write file");

    let exit = Command::new("nix")
    .arg("eval")
    .arg("-f")
    .arg("/tmp/foo")
    .status().unwrap();

    println!("exited with {}", exit);

    exit.exit_ok().expect("invalid expr (according to the official nix evaluator)");

    let lexer = crate::lexer::NixLexer::new(code.as_bytes()).filter(|t| match t.token_type {
        NixTokenType::Whitespace(_)
        | NixTokenType::SingleLineComment(_)
        | NixTokenType::MultiLineComment(_) => false,
        _ => true,
    });

    for token in lexer.clone() {
        println!("{:?}", token.token_type);
    }

    let result = parse(&mut itertools::multipeek(lexer));
}

#[test]
fn test_operators() {
    let subscriber = tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    can_parse("1");

    can_parse(
        r##"{
        
        { config, lib, pkgs, ... }:

with lib;

let

  cfg = config.boot.loader.grub;

  efi = config.boot.loader.efi;

  grubPkgs =
    # Package set of targeted architecture
    if cfg.forcei686 then pkgs.pkgsi686Linux else pkgs;

  realGrub = if cfg.version == 1 then grubPkgs.grub
    else if cfg.zfsSupport then grubPkgs.grub2.override { zfsSupport = true; }
    else if cfg.trustedBoot.enable
         then if cfg.trustedBoot.isHPLaptop
              then grubPkgs.trustedGrub-for-HP
              else grubPkgs.trustedGrub
         else grubPkgs.grub2;

  grub =
    # Don't include GRUB if we're only generating a GRUB menu (e.g.,
    # in EC2 instances).
    if cfg.devices == ["nodev"]
    then null
    else realGrub;

  grubEfi =
    # EFI version of Grub v2
    if cfg.efiSupport && (cfg.version == 2)
    then realGrub.override { efiSupport = cfg.efiSupport; }
    else null;

  f = x: if x == null then "" else "" + x;

  grubConfig = args:
    let
      efiSysMountPoint = if args.efiSysMountPoint == null then args.path else args.efiSysMountPoint;
      efiSysMountPoint' = replaceChars [ "/" ] [ "-" ] efiSysMountPoint;
    in
    pkgs.writeText "grub-config.xml" (builtins.toXML
    { splashImage = f cfg.splashImage;
      splashMode = f cfg.splashMode;
      backgroundColor = f cfg.backgroundColor;
      grub = f grub;
      grubTarget = f (grub.grubTarget or "");
      shell = "${pkgs.runtimeShell}";
      fullName = lib.getName realGrub;
      fullVersion = lib.getVersion realGrub;
      grubEfi = f grubEfi;
      grubTargetEfi = if cfg.efiSupport && (cfg.version == 2) then f (grubEfi.grubTarget or "") else "";
      bootPath = args.path;
      storePath = config.boot.loader.grub.storePath;
      bootloaderId = if args.efiBootloaderId == null then "NixOS${efiSysMountPoint'}" else args.efiBootloaderId;
      timeout = if config.boot.loader.timeout == null then -1 else config.boot.loader.timeout;
      users = if cfg.users == {} || cfg.version != 1 then cfg.users else throw "GRUB version 1 does not support user accounts.";
      theme = f cfg.theme;
      inherit efiSysMountPoint;
      inherit (args) devices;
      inherit (efi) canTouchEfiVariables;
      inherit (cfg)
        version extraConfig extraPerEntryConfig extraEntries forceInstall useOSProber
        extraGrubInstallArgs
        extraEntriesBeforeNixOS extraPrepareConfig configurationLimit copyKernels
        default fsIdentifier efiSupport efiInstallAsRemovable gfxmodeEfi gfxmodeBios gfxpayloadEfi gfxpayloadBios;
      path = with pkgs; makeBinPath (
        [ coreutils gnused gnugrep findutils diffutils btrfs-progs util-linux mdadm ]
        ++ optional (cfg.efiSupport && (cfg.version == 2)) efibootmgr
        ++ optionals cfg.useOSProber [ busybox os-prober ]);
      font = if cfg.font == null then ""
        else (if lib.last (lib.splitString "." cfg.font) == "pf2"
             then cfg.font
             else "${convertedFont}");
    });

  bootDeviceCounters = foldr (device: attr: attr // { ${device} = (attr.${device} or 0) + 1; }) {}
    (concatMap (args: args.devices) cfg.mirroredBoots);

  convertedFont = (pkgs.runCommand "grub-font-converted.pf2" {}
           (builtins.concatStringsSep " "
             ([ "${realGrub}/bin/grub-mkfont"
               cfg.font
               "--output" "$out"
             ] ++ (optional (cfg.fontSize!=null) "--size ${toString cfg.fontSize}")))
         );

  defaultSplash = pkgs.nixos-artwork.wallpapers.simple-dark-gray-bootloader.gnomeFilePath;
in

{

  ###### interface

  options = {

    boot.loader.grub = {

      enable = mkOption {
        default = !config.boot.isContainer;
        defaultText = literalExpression "!config.boot.isContainer";
        type = types.bool;
        description = ''
          Whether to enable the GNU GRUB boot loader.
        '';
      };

      version = mkOption {
        default = 2;
        example = 1;
        type = types.int;
        description = ''
          The version of GRUB to use: <literal>1</literal> for GRUB
          Legacy (versions 0.9x), or <literal>2</literal> (the
          default) for GRUB 2.
        '';
      };

      device = mkOption {
        default = "";
        example = "/dev/disk/by-id/wwn-0x500001234567890a";
        type = types.str;
        description = ''
          The device on which the GRUB boot loader will be installed.
          The special value <literal>nodev</literal> means that a GRUB
          boot menu will be generated, but GRUB itself will not
          actually be installed.  To install GRUB on multiple devices,
          use <literal>boot.loader.grub.devices</literal>.
        '';
      };

      devices = mkOption {
        default = [];
        example = [ "/dev/disk/by-id/wwn-0x500001234567890a" ];
        type = types.listOf types.str;
        description = ''
          The devices on which the boot loader, GRUB, will be
          installed. Can be used instead of <literal>device</literal> to
          install GRUB onto multiple devices.
        '';
      };

      users = mkOption {
        default = {};
        example = {
          root = { hashedPasswordFile = "/path/to/file"; };
        };
        description = ''
          User accounts for GRUB. When specified, the GRUB command line and
          all boot options except the default are password-protected.
          All passwords and hashes provided will be stored in /boot/grub/grub.cfg,
          and will be visible to any local user who can read this file. Additionally,
          any passwords and hashes provided directly in a Nix configuration
          (as opposed to external files) will be copied into the Nix store, and
          will be visible to all local users.
        '';
        type = with types; attrsOf (submodule {
          options = {
            hashedPasswordFile = mkOption {
              example = "/path/to/file";
              default = null;
              type = with types; uniq (nullOr str);
              description = ''
                Specifies the path to a file containing the password hash
                for the account, generated with grub-mkpasswd-pbkdf2.
                This hash will be stored in /boot/grub/grub.cfg, and will
                be visible to any local user who can read this file.
              '';
            };
            hashedPassword = mkOption {
              example = "grub.pbkdf2.sha512.10000.674DFFDEF76E13EA...2CC972B102CF4355";
              default = null;
              type = with types; uniq (nullOr str);
              description = ''
                Specifies the password hash for the account,
                generated with grub-mkpasswd-pbkdf2.
                This hash will be copied to the Nix store, and will be visible to all local users.
              '';
            };
            passwordFile = mkOption {
              example = "/path/to/file";
              default = null;
              type = with types; uniq (nullOr str);
              description = ''
                Specifies the path to a file containing the
                clear text password for the account.
                This password will be stored in /boot/grub/grub.cfg, and will
                be visible to any local user who can read this file.
              '';
            };
            password = mkOption {
              example = "Pa$$w0rd!";
              default = null;
              type = with types; uniq (nullOr str);
              description = ''
                Specifies the clear text password for the account.
                This password will be copied to the Nix store, and will be visible to all local users.
              '';
            };
          };
        });
      };

      mirroredBoots = mkOption {
        default = [ ];
        example = [
          { path = "/boot1"; devices = [ "/dev/disk/by-id/wwn-0x500001234567890a" ]; }
          { path = "/boot2"; devices = [ "/dev/disk/by-id/wwn-0x500009876543210a" ]; }
        ];
        description = ''
          Mirror the boot configuration to multiple partitions and install grub
          to the respective devices corresponding to those partitions.
        '';

        type = with types; listOf (submodule {
          options = {

            path = mkOption {
              example = "/boot1";
              type = types.str;
              description = ''
                The path to the boot directory where GRUB will be written. Generally
                this boot path should double as an EFI path.
              '';
            };

            efiSysMountPoint = mkOption {
              default = null;
              example = "/boot1/efi";
              type = types.nullOr types.str;
              description = ''
                The path to the efi system mount point. Usually this is the same
                partition as the above path and can be left as null.
              '';
            };

            efiBootloaderId = mkOption {
              default = null;
              example = "NixOS-fsid";
              type = types.nullOr types.str;
              description = ''
                The id of the bootloader to store in efi nvram.
                The default is to name it NixOS and append the path or efiSysMountPoint.
                This is only used if <literal>boot.loader.efi.canTouchEfiVariables</literal> is true.
              '';
            };

            devices = mkOption {
              default = [ ];
              example = [ "/dev/disk/by-id/wwn-0x500001234567890a" "/dev/disk/by-id/wwn-0x500009876543210a" ];
              type = types.listOf types.str;
              description = ''
                The path to the devices which will have the GRUB MBR written.
                Note these are typically device paths and not paths to partitions.
              '';
            };

          };
        });
      };

      configurationName = mkOption {
        default = "";
        example = "Stable 2.6.21";
        type = types.str;
        description = ''
          GRUB entry name instead of default.
        '';
      };

      storePath = mkOption {
        default = "/nix/store";
        type = types.str;
        description = ''
          Path to the Nix store when looking for kernels at boot.
          Only makes sense when copyKernels is false.
        '';
      };

      extraPrepareConfig = mkOption {
        default = "";
        type = types.lines;
        description = ''
          Additional bash commands to be run at the script that
          prepares the GRUB menu entries.
        '';
      };

      extraConfig = mkOption {
        default = "";
        example = ''
          serial --unit=0 --speed=115200 --word=8 --parity=no --stop=1
          terminal_input --append serial
          terminal_output --append serial
        '';
        type = types.lines;
        description = ''
          Additional GRUB commands inserted in the configuration file
          just before the menu entries.
        '';
      };

      extraGrubInstallArgs = mkOption {
        default = [ ];
        example = [ "--modules=nativedisk ahci pata part_gpt part_msdos diskfilter mdraid1x lvm ext2" ];
        type = types.listOf types.str;
        description = ''
          Additional arguments passed to <literal>grub-install</literal>.

          A use case for this is to build specific GRUB2 modules
          directly into the GRUB2 kernel image, so that they are available
          and activated even in the <literal>grub rescue</literal> shell.

          They are also necessary when the BIOS/UEFI is bugged and cannot
          correctly read large disks (e.g. above 2 TB), so GRUB2's own
          <literal>nativedisk</literal> and related modules can be used
          to use its own disk drivers. The example shows one such case.
          This is also useful for booting from USB.
          See the
          <link xlink:href="http://git.savannah.gnu.org/cgit/grub.git/tree/grub-core/commands/nativedisk.c?h=grub-2.04#n326">
          GRUB source code
          </link>
          for which disk modules are available.

          The list elements are passed directly as <literal>argv</literal>
          arguments to the <literal>grub-install</literal> program, in order.
        '';
      };

      extraInstallCommands = mkOption {
        default = "";
        example = ''
          # the example below generates detached signatures that GRUB can verify
          # https://www.gnu.org/software/grub/manual/grub/grub.html#Using-digital-signatures
          ''${pkgs.findutils}/bin/find /boot -not -path "/boot/efi/*" -type f -name '*.sig' -delete
          old_gpg_home=$GNUPGHOME
          export GNUPGHOME="$(mktemp -d)"
          ''${pkgs.gnupg}/bin/gpg --import ''${priv_key} > /dev/null 2>&1
          ''${pkgs.findutils}/bin/find /boot -not -path "/boot/efi/*" -type f -exec ''${pkgs.gnupg}/bin/gpg --detach-sign "{}" \; > /dev/null 2>&1
          rm -rf $GNUPGHOME
          export GNUPGHOME=$old_gpg_home
        '';
        type = types.lines;
        description = ''
          Additional shell commands inserted in the bootloader installer
          script after generating menu entries.
        '';
      };

      extraPerEntryConfig = mkOption {
        default = "";
        example = "root (hd0)";
        type = types.lines;
        description = ''
          Additional GRUB commands inserted in the configuration file
          at the start of each NixOS menu entry.
        '';
      };

      extraEntries = mkOption {
        default = "";
        type = types.lines;
        example = ''
          # GRUB 1 example (not GRUB 2 compatible)
          title Windows
            chainloader (hd0,1)+1

          # GRUB 2 example
          menuentry "Windows 7" {
            chainloader (hd0,4)+1
          }

          # GRUB 2 with UEFI example, chainloading another distro
          menuentry "Fedora" {
            set root=(hd1,1)
            chainloader /efi/fedora/grubx64.efi
          }
        '';
        description = ''
          Any additional entries you want added to the GRUB boot menu.
        '';
      };

      extraEntriesBeforeNixOS = mkOption {
        default = false;
        type = types.bool;
        description = ''
          Whether extraEntries are included before the default option.
        '';
      };

      extraFiles = mkOption {
        type = types.attrsOf types.path;
        default = {};
        example = literalExpression ''
          { "memtest.bin" = "''${pkgs.memtest86plus}/memtest.bin"; }
        '';
        description = ''
          A set of files to be copied to <filename>/boot</filename>.
          Each attribute name denotes the destination file name in
          <filename>/boot</filename>, while the corresponding
          attribute value specifies the source file.
        '';
      };

      useOSProber = mkOption {
        default = false;
        type = types.bool;
        description = ''
          If set to true, append entries for other OSs detected by os-prober.
        '';
      };

      splashImage = mkOption {
        type = types.nullOr types.path;
        example = literalExpression "./my-background.png";
        description = ''
          Background image used for GRUB.
          Set to <literal>null</literal> to run GRUB in text mode.

          <note><para>
          For grub 1:
          It must be a 640x480,
          14-colour image in XPM format, optionally compressed with
          <command>gzip</command> or <command>bzip2</command>.
          </para></note>

          <note><para>
          For grub 2:
          File must be one of .png, .tga, .jpg, or .jpeg. JPEG images must
          not be progressive.
          The image will be scaled if necessary to fit the screen.
          </para></note>
        '';
      };

      backgroundColor = mkOption {
        type = types.nullOr types.str;
        example = "#7EBAE4";
        default = null;
        description = ''
          Background color to be used for GRUB to fill the areas the image isn't filling.

          <note><para>
          This options has no effect for GRUB 1.
          </para></note>
        '';
      };

      theme = mkOption {
        type = types.nullOr types.path;
        example = literalExpression "pkgs.nixos-grub2-theme";
        default = null;
        description = ''
          Grub theme to be used.

          <note><para>
          This options has no effect for GRUB 1.
          </para></note>
        '';
      };

      splashMode = mkOption {
        type = types.enum [ "normal" "stretch" ];
        default = "stretch";
        description = ''
          Whether to stretch the image or show the image in the top-left corner unstretched.

          <note><para>
          This options has no effect for GRUB 1.
          </para></note>
        '';
      };

      font = mkOption {
        type = types.nullOr types.path;
        default = "${realGrub}/share/grub/unicode.pf2";
        defaultText = literalExpression ''"''${pkgs.grub2}/share/grub/unicode.pf2"'';
        description = ''
          Path to a TrueType, OpenType, or pf2 font to be used by Grub.
        '';
      };

      fontSize = mkOption {
        type = types.nullOr types.int;
        example = 16;
        default = null;
        description = ''
          Font size for the grub menu. Ignored unless <literal>font</literal>
          is set to a ttf or otf font.
        '';
      };

      gfxmodeEfi = mkOption {
        default = "auto";
        example = "1024x768";
        type = types.str;
        description = ''
          The gfxmode to pass to GRUB when loading a graphical boot interface under EFI.
        '';
      };

      gfxmodeBios = mkOption {
        default = "1024x768";
        example = "auto";
        type = types.str;
        description = ''
          The gfxmode to pass to GRUB when loading a graphical boot interface under BIOS.
        '';
      };

      gfxpayloadEfi = mkOption {
        default = "keep";
        example = "text";
        type = types.str;
        description = ''
          The gfxpayload to pass to GRUB when loading a graphical boot interface under EFI.
        '';
      };

      gfxpayloadBios = mkOption {
        default = "text";
        example = "keep";
        type = types.str;
        description = ''
          The gfxpayload to pass to GRUB when loading a graphical boot interface under BIOS.
        '';
      };

      configurationLimit = mkOption {
        default = 100;
        example = 120;
        type = types.int;
        description = ''
          Maximum of configurations in boot menu. GRUB has problems when
          there are too many entries.
        '';
      };

      copyKernels = mkOption {
        default = false;
        type = types.bool;
        description = ''
          Whether the GRUB menu builder should copy kernels and initial
          ramdisks to /boot.  This is done automatically if /boot is
          on a different partition than /.
        '';
      };

      default = mkOption {
        default = "0";
        type = types.either types.int types.str;
        apply = toString;
        description = ''
          Index of the default menu item to be booted.
          Can also be set to "saved", which will make GRUB select
          the menu item that was used at the last boot.
        '';
      };

      fsIdentifier = mkOption {
        default = "uuid";
        type = types.enum [ "uuid" "label" "provided" ];
        description = ''
          Determines how GRUB will identify devices when generating the
          configuration file. A value of uuid / label signifies that grub
          will always resolve the uuid or label of the device before using
          it in the configuration. A value of provided means that GRUB will
          use the device name as show in <command>df</command> or
          <command>mount</command>. Note, zfs zpools / datasets are ignored
          and will always be mounted using their labels.
        '';
      };

      zfsSupport = mkOption {
        default = false;
        type = types.bool;
        description = ''
          Whether GRUB should be built against libzfs.
          ZFS support is only available for GRUB v2.
          This option is ignored for GRUB v1.
        '';
      };

      efiSupport = mkOption {
        default = false;
        type = types.bool;
        description = ''
          Whether GRUB should be built with EFI support.
          EFI support is only available for GRUB v2.
          This option is ignored for GRUB v1.
        '';
      };

      efiInstallAsRemovable = mkOption {
        default = false;
        type = types.bool;
        description = ''
          Whether to invoke <literal>grub-install</literal> with
          <literal>--removable</literal>.</para>

          <para>Unless you turn this on, GRUB will install itself somewhere in
          <literal>boot.loader.efi.efiSysMountPoint</literal> (exactly where
          depends on other config variables). If you've set
          <literal>boot.loader.efi.canTouchEfiVariables</literal> *AND* you
          are currently booted in UEFI mode, then GRUB will use
          <literal>efibootmgr</literal> to modify the boot order in the
          EFI variables of your firmware to include this location. If you are
          *not* booted in UEFI mode at the time GRUB is being installed, the
          NVRAM will not be modified, and your system will not find GRUB at
          boot time. However, GRUB will still return success so you may miss
          the warning that gets printed ("<literal>efibootmgr: EFI variables
          are not supported on this system.</literal>").</para>

          <para>If you turn this feature on, GRUB will install itself in a
          special location within <literal>efiSysMountPoint</literal> (namely
          <literal>EFI/boot/boot$arch.efi</literal>) which the firmwares
          are hardcoded to try first, regardless of NVRAM EFI variables.</para>

          <para>To summarize, turn this on if:
          <itemizedlist>
            <listitem><para>You are installing NixOS and want it to boot in UEFI mode,
            but you are currently booted in legacy mode</para></listitem>
            <listitem><para>You want to make a drive that will boot regardless of
            the NVRAM state of the computer (like a USB "removable" drive)</para></listitem>
            <listitem><para>You simply dislike the idea of depending on NVRAM
            state to make your drive bootable</para></listitem>
          </itemizedlist>
        '';
      };

      enableCryptodisk = mkOption {
        default = false;
        type = types.bool;
        description = ''
          Enable support for encrypted partitions. GRUB should automatically
          unlock the correct encrypted partition and look for filesystems.
        '';
      };

      forceInstall = mkOption {
        default = false;
        type = types.bool;
        description = ''
          Whether to try and forcibly install GRUB even if problems are
          detected. It is not recommended to enable this unless you know what
          you are doing.
        '';
      };

      forcei686 = mkOption {
        default = false;
        type = types.bool;
        description = ''
          Whether to force the use of a ia32 boot loader on x64 systems. Required
          to install and run NixOS on 64bit x86 systems with 32bit (U)EFI.
        '';

};


    };

  };





}
        
        "##,
    );

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
          [
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

    let r = parse_expr_op(&mut itertools::multipeek(
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
    ))
    .unwrap();
    assert_eq!(
        AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Identifier(b"Addition")),
                Box::new(AST::Integer(1))
            )),
            Box::new(AST::Integer(41))
        ),
        r
    );
}
