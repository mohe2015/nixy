use crate::parser::AST;

pub trait ASTVisitor<'a, R: std::fmt::Debug> {

    fn visit_identifier(self, id: &'a [u8]) -> R;

    fn visit_integer(self, integer: i64) -> R;

    fn visit_float(self, float: f64) -> R;

    fn visit_todo(self) -> R;
}




pub struct ASTBuilder;

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
}