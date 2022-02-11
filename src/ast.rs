use crate::{parser::{AST, BUILTIN_IF, BUILTIN_PATH_CONCATENATE, BUILTIN_UNARY_MINUS, BUILTIN_UNARY_NOT}, lexer::NixTokenType};

pub trait ASTVisitor<'a, R: std::fmt::Debug> {

    fn visit_identifier(&self, id: &'a [u8]) -> R;

    fn visit_integer(&self, integer: i64) -> R;

    fn visit_float(&self, float: f64) -> R;

    fn visit_todo(&self) -> R;

    fn visit_select(&self, expr: R, attrpath: R, default: Option<R>) -> R;

    fn visit_infix_operation(&self, left: R, right: R, operator: NixTokenType<'a>) -> R;

    fn visit_prefix_operation(&self, operator: NixTokenType<'a>, expr: R) -> R;

    fn visit_if(&self, condition: R, true_case: R, false_case: R) -> R;

    fn visit_attrpath_part(&self, begin: R, last: R) -> R;

    fn visit_path_concatenate(&self, begin: R, last: R) -> R;

    fn visit_path_segment(&self, segment: &'a [u8]) -> R;

    fn visit_string(&self, string: &'a [u8]) -> R;

    fn visit_string_concatenate(&self, begin: R, last: R) -> R;

    fn visit_array_push(&self, begin: Option<R>, last: R) -> R;

    /// This is always called after `visit_array_push` and may help some implementations.
    fn visit_array_end(&self, array: R) -> R;

    fn visit_call(&self, function: R, parameter: R) -> R;

    fn visit_attrset_bind_push(&self, begin: Option<R>, last: R) -> R;
}




pub struct ASTBuilder;

const BUILTIN_SELECT: &[u8] = b"__builtin_select";

impl<'a> ASTVisitor<'a, AST<'a>> for ASTBuilder {

    fn visit_identifier(&self, id: &'a [u8]) -> AST<'a> {
        AST::Identifier(id)
    }
    
    fn visit_integer(&self, integer: i64) -> AST<'a> {
        AST::Integer(integer)
    }

    fn visit_float(&self, float: f64) -> AST<'a> {
        AST::Float(float)
    }

    fn visit_todo(&self) -> AST<'a> {
        todo!()
    }

    fn visit_select(&self, expr: AST<'a>, attrpath: AST<'a>, default: Option<AST<'a>>) -> AST<'a> {
        let value =  AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Identifier(BUILTIN_SELECT)),
                Box::new(expr),
            )),
            Box::new(attrpath),
        );
        match default {
            Some(default) => {
                AST::Call(
                    Box::new(AST::Call(
                        Box::new(AST::Identifier(b"__value_or_default")),
                        Box::new(value),
                    )),
                    Box::new(default),
                )
            }
            None => {
                AST::Call(
                    Box::new(AST::Identifier(b"__abort_invalid_attrpath")),
                    Box::new(value),
                )
            }
        }
    }

    fn visit_infix_operation(&self, left: AST<'a>, right: AST<'a>, operator: NixTokenType<'a>) -> AST<'a> {
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

    fn visit_if(&self, condition: AST<'a>, true_case: AST<'a>, false_case: AST<'a>) -> AST<'a> {
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

    fn visit_attrpath_part(&self, begin: AST<'a>, last: AST<'a>) -> AST<'a> {
        AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Identifier(BUILTIN_SELECT)),
                Box::new(begin),
            )),
            Box::new(last),
        )
    }

    fn visit_path_concatenate(&self, begin: AST<'a>, last: AST<'a>) -> AST<'a> {
        AST::Call(
            Box::new(AST::Call(
                Box::new(AST::Identifier(BUILTIN_PATH_CONCATENATE)),
                Box::new(begin),
            )),
            Box::new(last),
        )
    }

    fn visit_path_segment(&self, segment: &'a [u8]) -> AST<'a> {
        AST::PathSegment(segment)
    }

    fn visit_string(&self, string: &'a [u8]) -> AST<'a> {
        AST::String(string)
    }

    fn visit_string_concatenate(&self, begin: AST<'a>, last: AST<'a>) -> AST<'a> {
        todo!()
    }

    fn visit_array_push(&self, begin: Option<AST<'a>>, last: AST<'a>) -> AST<'a> {
        match begin {
            Some(begin) => {
                AST::Call(Box::new(AST::Identifier(b"cons")), Box::new(AST::Call(Box::new(begin), Box::new(last))))
            }
            None => {
                AST::Call(Box::new(AST::Identifier(b"cons")), Box::new(last))
            }
        }
    }

    fn visit_array_end(&self, array: AST<'a>) -> AST<'a> {
        AST::Call(Box::new(array), Box::new(AST::Identifier(b"nil")))
    }

    fn visit_call(&self, function: AST<'a>, parameter: AST<'a>) -> AST<'a> {
        AST::Call(Box::new(function), Box::new(parameter))
    }

    fn visit_prefix_operation(&self, operator: NixTokenType<'a>, expr: AST<'a>) -> AST<'a> {
        AST::Call(
            Box::new(AST::Identifier(match operator {
                NixTokenType::Subtraction => BUILTIN_UNARY_MINUS,
                NixTokenType::ExclamationMark => BUILTIN_UNARY_NOT,
                _ => todo!()
            })),
            Box::new(
                expr
            ),
        )
    }

    fn visit_attrset_bind_push(&self, begin: Option<AST<'a>>, last: AST<'a>) -> AST<'a> {
        //AST::Let(item.0, item.1, Box::new(accum))
        todo!()
    }
}