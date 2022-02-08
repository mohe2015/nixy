use core::fmt;
use std::mem::{discriminant, Discriminant};
use tracing::{instrument, info_span};
use crate::lexer::{NixToken, NixTokenType};
use itertools::{multipeek, MultiPeek};

// TODO FIXME expect token primitive

pub enum AST<'a> {
    Select(Box<AST<'a>>, Box<AST<'a>>),
    Identifier(&'a [u8]),
}

impl<'a> fmt::Debug for AST<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Select(arg0, arg1) => f.debug_tuple("Select").field(arg0).field(arg1).finish(),
            Self::Identifier(arg0) => f
                .debug_tuple("Identifier")
                .field(&std::str::from_utf8(arg0).unwrap().to_owned())
                .finish(),
        }
    }
}

#[instrument(name="expect", skip_all)]
pub fn expect<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(lexer: &mut MultiPeek<I>, t: NixTokenType<'a>) {
    tracing::trace!("Hello, world!");

    let token = lexer.next();
    if discriminant(&token.as_ref().unwrap().token_type) != discriminant(&t) {
        panic!("expected {:?} but got {:?}", &token, t)
    }
}

#[instrument(name="attrpath", skip_all)]
pub fn parse_attrpath<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(lexer: &mut MultiPeek<I>) -> AST<'a> {
    tracing::trace!("Hello, world!");

    let mut result: Option<AST<'a>> = None;
    loop {
        match lexer.peek() {
            Some(NixToken {
                token_type: NixTokenType::Identifier(id),
            }) => {
                match result {
                    Some(a) => {
                        result = Some(AST::Select(Box::new(a), Box::new(AST::Identifier(id))))
                    }
                    None => result = Some(AST::Identifier(id)),
                }

                lexer.next();
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

#[instrument(name="bind", skip_all)]
pub fn parse_bind<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(lexer: &mut MultiPeek<I>) {
    tracing::trace!("Hello, world!");

    loop {
        match lexer.peek() {
            Some(NixToken {
                token_type: NixTokenType::Identifier(b"inherit"),
            }) => {
                lexer.next();
                todo!();
                break;
            }
            _ => {
                lexer.reset_peek();
                let attrpath = parse_attrpath(lexer);
                println!("{:?}", attrpath);
                expect(lexer, NixTokenType::Assign);
                parse_expr(lexer);
                expect(lexer, NixTokenType::Semicolon);

                todo!();
            }
        }
    }
}

#[instrument(name="let", skip_all)]
pub fn parse_let<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    tracing::trace!("Hello, world!");

    loop {
        match lexer.peek() {
            Some(NixToken {
                token_type: NixTokenType::Identifier(b"in"),
            }) => {
                lexer.next();
                break None;
            }
            _ => {
                lexer.reset_peek();
                parse_bind(lexer);
            }
        }
    }
}

#[instrument(name="path", skip_all)]
pub fn parse_path<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    tracing::trace!("Hello, world!");

    loop {
        match lexer.next() {
            Some(NixToken {
                token_type: NixTokenType::PathEnd,
            }) => {
                break None;
            }
            Some(NixToken {
                token_type: NixTokenType::InterpolateStart,
            }) => {
                parse_expr(lexer);
                expect(lexer, NixTokenType::CurlyClose);
            }
            Some(NixToken {
                token_type: NixTokenType::PathSegment(segment),
            }) => {}
            _ => {
                todo!();
            }
        }
    }
}

#[instrument(name="simple", skip_all)]
pub fn parse_expr_simple<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    tracing::trace!("Hello, world!");

    let val = lexer.next();
    tracing::trace!("{:?}", val);
    match val {
        Some(NixToken {
            token_type: NixTokenType::Identifier(id),
        }) => Some(AST::Identifier(id)),
        Some(NixToken {
            token_type: NixTokenType::PathStart,
        }) => parse_path(lexer),
        other => {
            // TODO FIXME return None
            panic!("{:?}", other);
        }
    }
}

#[instrument(name="select", skip_all)]
pub fn parse_expr_select<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    tracing::trace!("Hello, world!");

    let val = parse_expr_simple(lexer);
    tracing::trace!("{:?}", val);
    val
}

#[instrument(name="app", skip_all)]
pub fn parse_expr_app<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    tracing::trace!("Hello, world!");

    let mut result: Option<AST<'a>> = None;
    loop {
        let jo = parse_expr_select(lexer);
        match jo {
            Some(expr) => {
                match result {
                    Some(a) => result = Some(AST::Select(Box::new(a), Box::new(expr))),
                    None => result = Some(expr),
                }

                lexer.next();
            }
            None => {
                break;
            }
        }
    }
    result
}

#[instrument(name="op", skip_all)]
pub fn parse_expr_op<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    tracing::trace!("Hello, world!");

    parse_expr_app(lexer)
}

#[instrument(name="if", skip_all)]
pub fn parse_expr_if<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    tracing::trace!("Hello, world!");

    parse_expr_op(lexer)
}

#[instrument(name="function", skip_all)]
pub fn parse_expr_function<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    tracing::trace!("Hello, world!");

    let token = lexer.next();
    match token.map(|t| t.token_type) {
        Some(NixTokenType::Identifier(b"let")) => {
            println!("letttt");
            parse_let(lexer)
        }
        _ => parse_expr_if(lexer),
    }
}

#[instrument(name="expr", skip_all)]
pub fn parse_expr<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    tracing::trace!("Hello, world!");

    let result = parse_expr_function(lexer);
    assert_eq!(None, lexer.next());
    result
}

#[instrument(name="p", skip_all)]
pub fn parse<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(lexer: &mut I) -> Option<AST<'a>> {
    tracing::trace!("Hello, world!");

    parse_expr(&mut multipeek(lexer))
}
