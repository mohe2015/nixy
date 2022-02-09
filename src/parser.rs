use crate::lexer::{NixToken, NixTokenType};
use core::fmt;
use itertools::MultiPeek;
use std::{mem::discriminant, fmt::{Display, Debug}};
use tracing::instrument;

// TODO FIXME call lexer.reset_peek(); everywhere

// TODO https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
// TODO https://matklad.github.io/2020/04/15/from-pratt-to-dijkstra.html
// https://matklad.github.io/2020/03/22/fast-simple-rust-interner.html

const BUILTIN_UNARY_NOT: &[u8] = b"__builtin_unary_not";
const BUILTIN_PATH_CONCATENATE: &[u8] = b"__builtin_path_concatenate";
const BUILTIN_SELECT: &[u8] = b"__builtin_select";
const BUILTIN_IF: &[u8] = b"__builtin_if";

pub struct ASTBind<'a> {
    path: Box<AST<'a>>,
    value: Box<AST<'a>>
}

impl<'a> Debug for ASTBind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} = {:?}", self.path, self.value)
    }
}

pub struct ASTLet<'a> {
    bind: ASTBind<'a>,
    body: Box<AST<'a>>,
}


impl<'a> Debug for ASTLet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let {:?} in {:?}", self.bind, self.body)
    }
}

pub struct ASTPathSegment<'a>(&'a [u8]);

impl<'a> Debug for ASTPathSegment<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",  std::str::from_utf8(self.0).unwrap())
    }
}

pub struct ASTCall<'a>(Box<AST<'a>>,Box<AST<'a>>);

impl<'a> Debug for ASTCall<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?} {:?})", self.0, self.1)
    }
}

pub struct ASTIdentifier<'a>(&'a [u8]);

impl<'a> Debug for ASTIdentifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(self.0).unwrap())
    }
}

pub enum AST<'a> {
    Identifier(ASTIdentifier<'a>),
    PathSegment(ASTPathSegment<'a>),
    Bind(ASTBind<'a>),
    Let(ASTLet<'a>),
    Call(ASTCall<'a>),
}

impl<'a> Debug for AST<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
            AST::Identifier(v) => write!(f, "{:?}", v),
            AST::PathSegment(v) =>  write!(f, "{:?}", v),
            AST::Bind(v) =>  write!(f, "{:?}", v),
            AST::Let(v) =>  write!(f, "{:?}", v),
            AST::Call(v) =>  write!(f, "{:?}", v),
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
                        result = Some(AST::Call(ASTCall(Box::new(AST::Call(ASTCall(Box::new(AST::Identifier(ASTIdentifier(BUILTIN_SELECT))), Box::new(a)))), Box::new(AST::Identifier(ASTIdentifier(id))))));
                    }
                    None => result = Some(AST::Identifier(ASTIdentifier(id))),
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
) -> ASTBind<'a> {
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

            ASTBind {path: Box::new(attrpath), value: Box::new(expr)}
        }
    }
}

#[instrument(name = "let", skip_all, ret)]
pub fn parse_let<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    expect(lexer, NixTokenType::Let);
    let mut binds = Vec::new();
    loop {
        match lexer.peek() {
            Some(NixToken {
                token_type: NixTokenType::In,
            }) => {
                lexer.next();

                let body = parse_expr_function(lexer).expect("failed to parse body of let binding");

                break Some(binds.into_iter().fold(body, |accum, item| {
                    AST::Let(ASTLet {
                        bind: item,
                        body: Box::new(accum)
                    })
                }))
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
                        result = Some(AST::Call(ASTCall(Box::new(AST::Call(ASTCall(Box::new(AST::Identifier(ASTIdentifier(BUILTIN_PATH_CONCATENATE))), Box::new(a)))), Box::new(expr))))
                    }
                    None => result = Some(expr),
                }
            }
            Some(NixToken {
                token_type: NixTokenType::PathSegment(segment),
            }) => match result {
                Some(a) => {
                    result = Some(AST::Call(ASTCall(Box::new(AST::Call(ASTCall(Box::new(AST::Identifier(ASTIdentifier(BUILTIN_PATH_CONCATENATE))), Box::new(a)))), Box::new(AST::PathSegment(ASTPathSegment(segment))))))

                }
                None => result = Some(AST::PathSegment(ASTPathSegment(segment))),
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
            let ret = Some(AST::Identifier(ASTIdentifier(id)));
            lexer.next();
            ret
        },
        Some(NixToken {
            token_type: NixTokenType::PathStart,
        }) => {
            lexer.reset_peek();
            parse_path(lexer)
        },
        other => {
            lexer.reset_peek();
            None
        }
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
                    Some(a) => {
                        result = Some(AST::Call(ASTCall(Box::new(a),  Box::new(expr))));
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

#[instrument(name = "op", skip_all, ret)]
pub fn parse_expr_op<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    match lexer.peek() {
        Some(NixToken {
            token_type: NixTokenType::ExclamationMark
        }) => {
            expect(lexer, NixTokenType::ExclamationMark);
            let expr = parse_expr(lexer).expect("failed to parse negated expression");
            Some(AST::Call(ASTCall(Box::new(AST::Identifier(ASTIdentifier(BUILTIN_UNARY_NOT))), Box::new(expr))))
        }
        _ => {
            lexer.reset_peek();
            parse_expr_app(lexer)
        }
    }
}

#[instrument(name = "if", skip_all, ret)]
pub fn parse_expr_if<'a, I: Iterator<Item = NixToken<'a>> + std::fmt::Debug>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    match lexer.peek() {
        Some(NixToken {
            token_type: NixTokenType::If
        }) => {
            expect(lexer, NixTokenType::If);
            let condition = parse_expr(lexer).expect("failed to parse if condition");
            expect(lexer, NixTokenType::Then);
            let true_case = parse_expr(lexer).expect("failed to parse if true case");
            expect(lexer, NixTokenType::Else);
            let false_case = parse_expr(lexer).expect("failed to parse if false case");
            Some(AST::Call(ASTCall(Box::new(AST::Call(ASTCall(Box::new(AST::Call(ASTCall(Box::new(AST::Identifier(ASTIdentifier(BUILTIN_IF))), Box::new(condition)))), Box::new(true_case)))), Box::new(false_case))))
        }
        _ => {
            lexer.reset_peek();
            parse_expr_op(lexer)
        }
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
    let result = parse_expr_function(lexer);
    result
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
