use crate::lexer::{NixToken, NixTokenType};
use core::fmt;
use itertools::MultiPeek;
use std::{fmt::Debug, mem::discriminant};
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
                todo!()
            }
            Some(NixToken {
                token_type: NixTokenType::InterpolateStart,
            }) => {
                todo!()
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
            token_type: NixTokenType::Identifier(b"inherit"),
        }) => {
            lexer.next();
            todo!();
        }
        other => {
            //println!("TEST {:?}", other);
            lexer.reset_peek();
            let attrpath = parse_attrpath(lexer);
            expect(lexer, NixTokenType::Assign);
            let expr = parse_expr(lexer).unwrap();
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

#[instrument(name = "ind_str", skip_all, ret)]
pub fn parse_indented_string<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    expect(lexer, NixTokenType::IndentedStringStart);
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
            Some(NixToken {
                token_type: NixTokenType::IndentedStringEnd,
            }) => break Some(accum),
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
            parse_indented_string(lexer)
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
                        array.push(parse_expr_simple(lexer).unwrap());
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
                        expect(lexer, NixTokenType::CurlyOpen);
                        parsed_first = true;
                    }
                    expect(lexer, NixTokenType::CurlyClose);
                    return Some(AST::Identifier(b"TODO formals")); // TODO FIXME
                }
                _ => todo!(),
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
            expect(lexer, NixTokenType::Colon);
            parse_expr_function(lexer)
        }
        /*Some(NixTokenType::Identifier(_)) => {
            todo!(); // for this we need two lookahead because otherwise this is parse_expr_if
        }*/
        Some(NixTokenType::Assert) => {
            todo!();
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

#[test]
fn test_operators() {
    let subscriber = tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

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

    let text = "{
        members = [];
    }";

    let lexer = crate::lexer::NixLexer::new(text.as_bytes()).filter(|t| match t.token_type {
        NixTokenType::Whitespace(_)
        | NixTokenType::SingleLineComment(_)
        | NixTokenType::MultiLineComment(_) => false,
        _ => true,
    });

    let result = parse(&mut itertools::multipeek(lexer));
    assert_eq!(result, None);
}
