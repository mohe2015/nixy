use std::io::Write;
use std::{io::BufWriter, marker::PhantomData};
use core::fmt::Debug;

use crate::{
    lexer::{NixTokenType, NixToken},
    parser::{
        BindType, Parser, BUILTIN_IF, BUILTIN_PATH_CONCATENATE, BUILTIN_SELECT,
        BUILTIN_UNARY_MINUS, BUILTIN_UNARY_NOT,
    }, visitor::ASTVisitor,
};


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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

    fn visit_select(
        &mut self,
        expr: AST<'a>,
        attrpath: AST<'a>,
        default: Option<AST<'a>>,
    ) -> AST<'a> {
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

    fn visit_string_concatenate(&mut self, _begin: Option<AST<'a>>, _last: AST<'a>) -> AST<'a> {
        todo!()
    }
    fn visit_array_start(&mut self) {
        todo!()
    }

    fn visit_array_push_before(&mut self, begin: &Option<AST<'a>>) {
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

    fn visit_function_before(&mut self) {
        todo!()
    }

    fn visit_if_before(&mut self) {
        todo!()
    }

    fn visit_if_after_condition(&mut self, condition: &AST<'a>) {
        todo!()
    }

    fn visit_if_after_true_case(&mut self, condition: &AST<'a>, true_case: &AST<'a>) {
        todo!()
    }

    fn visit_call_maybe(&mut self, expr: &Option<AST<'a>>) {
    }

    fn visit_call_maybe_not(&mut self) {
    }

    fn visit_bind_before(&mut self, bind_type: BindType) {
        todo!()
    }

    fn visit_bind_between(&mut self, bind_type: BindType, attrpath: &AST<'a>) {
        todo!()
    }

    fn visit_bind_after(
        &mut self,
        bind_type: BindType,
        attrpath: AST<'a>,
        expr: AST<'a>,
    ) -> AST<'a> {
        todo!()
    }

    fn visit_let_before(&mut self) {
        todo!()
    }

    fn visit_let_push_bind(&mut self, binds: Option<AST<'a>>, bind: AST<'a>) -> AST<'a> {
        todo!()
    }

    fn visit_let(&mut self, binds: Option<AST<'a>>, body: AST<'a>) -> AST<'a> {
        todo!()
    }

    fn visit_let_before_body(&mut self, binds: &Option<AST<'a>>) {
        todo!()
    }

    fn visit_attrset_before(&mut self, binds: &Option<AST<'a>>) {
        todo!()
    }

    fn visit_attrset(&mut self, binds: Option<AST<'a>>) -> AST<'a> {
        todo!()
    }

    fn visit_string_concatenate_end(&mut self, result: Option<AST<'a>>) -> AST<'a> {
        todo!()
    }

    fn visit_formal(&mut self, formals: Option<AST<'a>>, identifier: &'a [u8], default: Option<AST<'a>>) -> AST<'a> {
        todo!()
    }

    fn visit_formals(&mut self, formals: Option<AST<'a>>, at_identifier: Option<&'a [u8]>, ellipsis: bool) -> AST<'a> {
        todo!()
    }
}
