use crate::lexer::{NixToken, NixTokenType};
use core::fmt;
use itertools::{multipeek, MultiPeek};
use std::mem::discriminant;
use tracing::instrument;

// TODO FIXME expect token primitive

pub struct ASTBind<'a> {
    path: Box<AST<'a>>,
    value: Box<AST<'a>>
}

pub struct ASTLet<'a> {
    bind: ASTBind<'a>,
    body: Box<AST<'a>>,
}

pub struct ASTPathSegment<'a>(&'a [u8]);

pub struct ASTConcatenate<'a> {
    first: Box<AST<'a>>,
    rest: Box<AST<'a>>,
}

pub struct ASTSelect<'a> {
    first: Box<AST<'a>>,
    rest: Box<AST<'a>>,
}

pub struct ASTIdentifier<'a>(&'a [u8]);

pub enum AST<'a> {
    Select(ASTSelect<'a>),
    Identifier(ASTIdentifier<'a>),
    PathConcatenate(ASTConcatenate<'a>),
    PathSegment(ASTPathSegment<'a>),
    Bind(ASTBind<'a>),
    Let(ASTLet<'a>)
}

impl<'a> fmt::Debug for AST<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Select(arg0, arg1) => f.debug_tuple("Select").field(arg0).field(arg1).finish(),
            Self::Identifier(arg0) => f
                .debug_tuple("Identifier")
                .field(&std::str::from_utf8(arg0).unwrap().to_owned())
                .finish(),
            Self::PathConcatenate(arg0, arg1) => f
                .debug_tuple("PathConcatenate")
                .field(arg0)
                .field(arg1)
                .finish(),
                Self::Bind(arg0, arg1) => f
                .debug_tuple("Bind")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::PathSegment(arg0) => f
                .debug_tuple("PathSegment")
                .field(&std::str::from_utf8(arg0).unwrap().to_owned())
                .finish(),
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
        panic!("expected {:?} but got {:?}", &token, t)
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

#[instrument(name = "bind", skip_all, ret)]
pub fn parse_bind<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> AST<'a> {
    match lexer.peek() {
        Some(NixToken {
            token_type: NixTokenType::Identifier(b"inherit"),
        }) => {
            lexer.next();
            todo!();
        }
        _ => {
            lexer.reset_peek();
            let attrpath = parse_attrpath(lexer);
            println!("{:?}", attrpath);
            expect(lexer, NixTokenType::Assign);
            let expr = parse_expr(lexer).unwrap();
            expect(lexer, NixTokenType::Semicolon);

            AST::Bind(Box::new(attrpath), Box::new(expr))
        }
    }
}

#[instrument(name = "let", skip_all, ret)]
pub fn parse_let<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    let result = None;
    let current = None;
    loop {
        match lexer.peek() {
            Some(NixToken {
                token_type: NixTokenType::In,
            }) => {
                lexer.next();

                let body = parse_expr_function(lexer);

                current.1 = body;
                break result;
            }
            _ => {
                lexer.reset_peek();
                let bind = parse_bind(lexer);

                match result {
                    Some(a) => {
                        current.1 = AST::Let(bind, fakeBody);
                        current = current.1;
                    }
                    None => {
                        result = current = AST::Let(bind, fakeBody);
                    },
                }
            }
        }
    }
    result
}

#[instrument(name = "path", skip_all, ret)]
pub fn parse_path<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    expect(lexer, NixTokenType::PathStart);
    let mut result = None;
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
                    Some(a) => result = Some(AST::PathConcatenate(Box::new(a), Box::new(expr))),
                    None => result = Some(expr),
                }
            }
            Some(NixToken {
                token_type: NixTokenType::PathSegment(segment),
            }) => match result {
                Some(a) => {
                    result = Some(AST::PathConcatenate(
                        Box::new(a),
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
        },
        Some(NixToken {
            token_type: NixTokenType::PathStart,
        }) => parse_path(lexer),
        other => None
    }
}

#[instrument(name = "sel", skip_all, ret)]
pub fn parse_expr_select<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_simple(lexer)
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
                    // TODO FIXME apply?
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

#[instrument(name = "op", skip_all, ret)]
pub fn parse_expr_op<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_app(lexer)
}

#[instrument(name = "if", skip_all, ret)]
pub fn parse_expr_if<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_op(lexer)
}

#[instrument(name = "fn", skip_all, ret)]
pub fn parse_expr_function<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    let token = lexer.next();
    match token.map(|t| t.token_type) {
        Some(NixTokenType::Let) => {
            println!("letttt");
            parse_let(lexer)
        }
        _ => parse_expr_if(lexer),
    }
}

#[instrument(name = "e", skip_all, ret)]
pub fn parse_expr<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    let result = parse_expr_function(lexer);
    assert_eq!(None, lexer.next());
    result
}

#[instrument(name = "p", skip_all, ret)]
pub fn parse<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut I,
) -> Option<AST<'a>> {
    tracing::trace!("Hello, world!");

    parse_expr(&mut multipeek(lexer))
}
