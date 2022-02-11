use crate::parser::AST;

pub trait ASTVisitor<'a, R> {

    fn visit_identifier(id: &'a [u8]) -> R;

    fn visit_integer(integer: i64) -> R;

    fn visit_float(float: f64) -> R;

    fn visit_todo() -> R;
}




pub struct ASTBuilder;

impl<'a> ASTVisitor<'a, AST<'a>> for ASTBuilder {

    fn visit_identifier(id: &'a [u8]) -> AST<'a> {
        AST::Identifier(id)
    }
    
    fn visit_integer(integer: i64) -> AST<'a> {
        AST::Integer(integer)
    }

    fn visit_float(float: f64) -> AST<'a> {
        AST::Float(float)
    }

    fn visit_todo() -> AST<'a> {
        todo!()
    }
}