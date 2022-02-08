use core::fmt;

use crate::lexer::{NixToken, NixTokenType};
use itertools::{multipeek, MultiPeek};

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

pub fn parse_attrpath<'a, I: Iterator<Item = NixToken<'a>>>(lexer: &mut MultiPeek<I>) -> AST<'a> {
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

pub fn parse_bind<'a, I: Iterator<Item = NixToken<'a>>>(lexer: &mut MultiPeek<I>) {
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
                match lexer.next() {
                    Some(NixToken {
                        token_type: NixTokenType::Assign,
                    }) => {}
                    _ => panic!("unexpected token"),
                }
                println!("{:?}", attrpath);
                parse_expr(lexer);
                match lexer.next() {
                    Some(NixToken {
                        token_type: NixTokenType::Semicolon,
                    }) => {}
                    _ => panic!("unexpected token"),
                }

                todo!();
            }
        }
    }
}

pub fn parse_let<'a, I: Iterator<Item = NixToken<'a>>>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
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

pub fn parse_expr_simple<'a, I: Iterator<Item = NixToken<'a>>>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    match lexer.next() {
        Some(NixToken {
            token_type: NixTokenType::Identifier(id),
        }) => Some(AST::Identifier(id)),
        _ => {
            // TODO FIXME return None
            todo!();
        }
    }
}

pub fn parse_expr_select<'a, I: Iterator<Item = NixToken<'a>>>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_simple(lexer)
}

pub fn parse_expr_app<'a, I: Iterator<Item = NixToken<'a>>>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
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

pub fn parse_expr_op<'a, I: Iterator<Item = NixToken<'a>>>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_app(lexer)
}

pub fn parse_expr_if<'a, I: Iterator<Item = NixToken<'a>>>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    parse_expr_op(lexer)
}

pub fn parse_expr_function<'a, I: Iterator<Item = NixToken<'a>>>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    let token = lexer.next();
    match token.map(|t| t.token_type) {
        Some(NixTokenType::Identifier(b"let")) => {
            println!("letttt");
            parse_let(lexer)
        }
        _ => parse_expr_if(lexer),
    }
}

pub fn parse_expr<'a, I: Iterator<Item = NixToken<'a>>>(
    lexer: &mut MultiPeek<I>,
) -> Option<AST<'a>> {
    let result = parse_expr_function(lexer);
    assert_eq!(None, lexer.next());
    result
}

pub fn parse<'a, I: Iterator<Item = NixToken<'a>>>(lexer: &mut I) -> Option<AST<'a>> {
    parse_expr(&mut multipeek(lexer))
}
