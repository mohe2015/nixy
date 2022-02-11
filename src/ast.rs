use std::{io::BufWriter, marker::PhantomData};
use std::io::Write;

use crate::{
    lexer::NixTokenType,
    parser::{AST, BUILTIN_IF, BUILTIN_PATH_CONCATENATE, BUILTIN_UNARY_MINUS, BUILTIN_UNARY_NOT, BUILTIN_SELECT, Parser},
};

pub trait ASTVisitor<'a, R: std::fmt::Debug> {
    fn visit_file_start(&mut self) {}

    fn visit_file_end(&mut self) {}

    fn visit_identifier(&mut self, id: &'a [u8]) -> R;

    fn visit_integer(&mut self, integer: i64) -> R;

    fn visit_float(&mut self, float: f64) -> R;

    fn visit_todo(&mut self) -> R;

    fn visit_select(&mut self, expr: R, attrpath: R, default: Option<R>) -> R;

    fn visit_infix_lhs(&mut self, operator: NixTokenType<'a>, left: &R);

    fn visit_infix_operation(&mut self, left: R, right: R, operator: NixTokenType<'a>) -> R;

    fn visit_prefix_operation(&mut self, operator: NixTokenType<'a>, expr: R) -> R;

    fn visit_if(&mut self, condition: R, true_case: R, false_case: R) -> R;

    fn visit_attrpath_part(&mut self, begin: R, last: R) -> R;

    fn visit_path_concatenate(&mut self, begin: R, last: R) -> R;

    fn visit_path_segment(&mut self, segment: &'a [u8]) -> R;

    fn visit_string(&mut self, string: &'a [u8]) -> R;

    fn visit_string_concatenate(&mut self, begin: R, last: R) -> R;

    fn visit_array_push(&mut self, begin: Option<R>, last: R) -> R;

    /// This is always called after `visit_array_push` and may help some implementations.
    fn visit_array_end(&mut self, array: R) -> R;

    fn visit_call(&mut self, function: R, parameter: R) -> R;

    fn visit_attrset_bind_push(&mut self, begin: Option<R>, last: R) -> R;

    fn visit_function_enter(&mut self, arg: &R);

    fn visit_function_exit(&mut self, arg: R, body: R) -> R;
}

pub struct ASTJavaTranspiler<'a, W: Write> {
    writer: &'a mut W
}

impl<'a, W: Write> ASTVisitor<'a, ()> for ASTJavaTranspiler<'a, W> {
    fn visit_file_start(&mut self) {
        write!(self.writer, "
        test
        
        ").unwrap();
    }

    fn visit_file_end(&mut self) {
        
    }

    fn visit_function_enter(&mut self, arg: &()) {
        write!(self.writer, "ff").unwrap();

    }

    fn visit_function_exit(&mut self, arg: (), body: ()) -> () {
        write!(self.writer, "}}").unwrap();
    }
    
    fn visit_identifier(&mut self, id: &'a [u8]) -> () {
        write!(self.writer, "{}", std::str::from_utf8(id).unwrap()).unwrap();
    }

    fn visit_integer(&mut self, integer: i64) -> () {
        write!(self.writer, "(new NixInteger({}))", integer).unwrap();
    }

    fn visit_float(&mut self, float: f64) -> () {
        todo!()
    }

    fn visit_todo(&mut self) -> () {
        todo!()
    }

    fn visit_select(&mut self, expr: (), attrpath: (), default: Option<()>) -> () {
        todo!()
    }

    fn visit_infix_lhs(&mut self, operator: NixTokenType<'a>, left: &()) {
        match operator {
            NixTokenType::Addition => {
                write!(self.writer, ".add(").unwrap();
            }
            _ => todo!()
        }
    }

    fn visit_infix_operation(&mut self, left: (), right: (), operator: NixTokenType<'a>) -> () {
        write!(self.writer, ")").unwrap();
    }

    fn visit_prefix_operation(&mut self, operator: NixTokenType<'a>, expr: ()) -> () {
        todo!()
    }

    fn visit_if(&mut self, condition: (), true_case: (), false_case: ()) -> () {
        todo!()
    }

    fn visit_attrpath_part(&mut self, begin: (), last: ()) -> () {
        todo!()
    }

    fn visit_path_concatenate(&mut self, begin: (), last: ()) -> () {
        todo!()
    }

    fn visit_path_segment(&mut self, segment: &'a [u8]) -> () {
        todo!()
    }

    fn visit_string(&mut self, string: &'a [u8]) -> () {
        todo!()
    }

    fn visit_string_concatenate(&mut self, begin: (), last: ()) -> () {
        todo!()
    }

    fn visit_array_push(&mut self, begin: Option<()>, last: ()) -> () {
        todo!()
    }

    fn visit_array_end(&mut self, array: ()) -> () {
        todo!()
    }

    fn visit_call(&mut self, function: (), parameter: ()) -> () {
        todo!()
    }

    fn visit_attrset_bind_push(&mut self, begin: Option<()>, last: ()) -> () {
        todo!()
    }
}

fn test_java_transpiler_code(code: &[u8]) {
    let mut data = Vec::new();
    let transpiler = ASTJavaTranspiler {
        writer: &mut data
    };

    let lexer = crate::lexer::NixLexer::new(code).filter(|t| match t.token_type {
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
        visitor: transpiler,
        phantom: PhantomData,
    };

    parser.parse();

    println!("code: {}", std::str::from_utf8(&data).unwrap());
}

// cargo test ast::test_java_transpiler -- --nocapture
#[test]
pub fn test_java_transpiler() {
    test_java_transpiler_code(b"1");
    test_java_transpiler_code(b"1 + 1");
    test_java_transpiler_code(b"a: a + 1");
}

pub struct ASTBuilder;

impl<'a> ASTVisitor<'a, AST<'a>> for ASTBuilder {
    fn visit_identifier(&mut self, id: &'a [u8]) -> AST<'a> {
        AST::Identifier(id)
    }

    fn visit_integer(&mut self, integer: i64) -> AST<'a> {
        AST::Integer(integer)
    }

    fn visit_float(&mut self, float: f64) -> AST<'a> {
        AST::Float(float)
    }

    fn visit_todo(&mut self) -> AST<'a> {
        todo!()
    }

    fn visit_select(&mut self, expr: AST<'a>, attrpath: AST<'a>, default: Option<AST<'a>>) -> AST<'a> {
        let value = AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Identifier(BUILTIN_SELECT)),
                Box::new(expr),
            )),
            Box::new(attrpath),
        );
        match default {
            Some(default) => AST::Call(
                Box::new(AST::Call(
                    Box::new(AST::Identifier(b"__value_or_default")),
                    Box::new(value),
                )),
                Box::new(default),
            ),
            None => AST::Call(
                Box::new(AST::Identifier(b"__abort_invalid_attrpath")),
                Box::new(value),
            ),
        }
    }

    fn visit_infix_lhs(&mut self, operator: NixTokenType<'a>, left: &AST<'a>) {
        todo!()
    }

    fn visit_infix_operation(
        &mut self,
        left: AST<'a>,
        right: AST<'a>,
        operator: NixTokenType<'a>,
    ) -> AST<'a> {
        AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Identifier(match operator {
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
                Box::new(left),
            )),
            Box::new(right),
        )
    }

    fn visit_prefix_operation(&mut self, operator: NixTokenType<'a>, expr: AST<'a>) -> AST<'a> {
        AST::Call(
            Box::new(AST::Identifier(match operator {
                NixTokenType::Subtraction => BUILTIN_UNARY_MINUS,
                NixTokenType::ExclamationMark => BUILTIN_UNARY_NOT,
                _ => todo!(),
            })),
            Box::new(expr),
        )
    }

    fn visit_if(&mut self, condition: AST<'a>, true_case: AST<'a>, false_case: AST<'a>) -> AST<'a> {
        AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Call(
                    Box::new(AST::Identifier(BUILTIN_IF)),
                    Box::new(condition),
                )),
                Box::new(true_case),
            )),
            Box::new(false_case),
        )
    }

    fn visit_attrpath_part(&mut self, begin: AST<'a>, last: AST<'a>) -> AST<'a> {
        AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Identifier(BUILTIN_SELECT)),
                Box::new(begin),
            )),
            Box::new(last),
        )
    }

    fn visit_path_concatenate(&mut self, begin: AST<'a>, last: AST<'a>) -> AST<'a> {
        AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Identifier(BUILTIN_PATH_CONCATENATE)),
                Box::new(begin),
            )),
            Box::new(last),
        )
    }

    fn visit_path_segment(&mut self, segment: &'a [u8]) -> AST<'a> {
        AST::PathSegment(segment)
    }

    fn visit_string(&mut self, string: &'a [u8]) -> AST<'a> {
        AST::String(string)
    }

    fn visit_string_concatenate(&mut self, _begin: AST<'a>, _last: AST<'a>) -> AST<'a> {
        todo!()
    }

    fn visit_array_push(&mut self, begin: Option<AST<'a>>, last: AST<'a>) -> AST<'a> {
        match begin {
            Some(begin) => AST::Call(
                Box::new(AST::Identifier(b"cons")),
                Box::new(AST::Call(Box::new(begin), Box::new(last))),
            ),
            None => AST::Call(Box::new(AST::Identifier(b"cons")), Box::new(last)),
        }
    }

    fn visit_array_end(&mut self, array: AST<'a>) -> AST<'a> {
        AST::Call(Box::new(array), Box::new(AST::Identifier(b"nil")))
    }

    fn visit_call(&mut self, function: AST<'a>, parameter: AST<'a>) -> AST<'a> {
        AST::Call(Box::new(function), Box::new(parameter))
    }

    fn visit_attrset_bind_push(&mut self, _begin: Option<AST<'a>>, _last: AST<'a>) -> AST<'a> {
        //AST::Let(item.0, item.1, Box::new(accum))
        todo!()
    }

    fn visit_function_enter(&mut self, arg: &AST<'a>) {
        todo!()
    }

    fn visit_function_exit(&mut self, arg: AST<'a>, body: AST<'a>) -> AST<'a> {
        todo!()
    }
}
