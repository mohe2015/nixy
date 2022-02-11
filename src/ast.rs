use crate::parser::AST;

pub trait ASTVisitor<'a, R: std::fmt::Debug> {

    fn visit_identifier(self, id: &'a [u8]) -> R;

    fn visit_integer(self, integer: i64) -> R;

    fn visit_float(self, float: f64) -> R;

    fn visit_todo(self) -> R;

    fn visit_select(self, expr: R, attrpath: R, default: Option<R>) -> R;
}




pub struct ASTBuilder;

const BUILTIN_SELECT: &[u8] = b"__builtin_select";

impl<'a> ASTVisitor<'a, AST<'a>> for ASTBuilder {

    fn visit_identifier(self, id: &'a [u8]) -> AST<'a> {
        AST::Identifier(id)
    }
    
    fn visit_integer(self, integer: i64) -> AST<'a> {
        AST::Integer(integer)
    }

    fn visit_float(self, float: f64) -> AST<'a> {
        AST::Float(float)
    }

    fn visit_todo(self) -> AST<'a> {
        todo!()
    }

    fn visit_select(self, expr: AST<'a>, attrpath: AST<'a>, default: Option<AST<'a>>) -> AST<'a> {
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
}