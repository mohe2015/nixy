use crate::lexer::{NixToken, NixTokenType};
use itertools::{multipeek, MultiPeek};

#[derive(Debug)]
pub enum AST<'a> {
    Select(Box<AST<'a>>, Box<AST<'a>>),
    Identifier(&'a [u8]),
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
                    None => {
                        result = Some(AST::Identifier(id))
                    }
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
                let attrpath = parse_attrpath(lexer);
                println!("{:?}", attrpath);
                todo!();
            }
        }
    }
}

pub fn parse_let<'a, I: Iterator<Item = NixToken<'a>>>(lexer: &mut MultiPeek<I>) {
    loop {
        match lexer.peek() {
            Some(NixToken {
                token_type: NixTokenType::Identifier(b"in"),
            }) => {
                lexer.next();
                break;
            }
            _ => {
                parse_bind(lexer);
            }
        }
    }
}

pub fn parse_expr<'a, I: Iterator<Item = NixToken<'a>>>(lexer: &mut MultiPeek<I>) {
    let token = lexer.next();
    match token.map(|t| t.token_type) {
        Some(NixTokenType::Identifier(b"let")) => {
            println!("letttt");
            parse_let(lexer);
        }
        _ => todo!(),
    }

    assert_eq!(None, lexer.next())
}

pub fn parse<'a, I: Iterator<Item = NixToken<'a>>>(lexer: &mut I) {
    parse_expr(&mut multipeek(lexer))
}
