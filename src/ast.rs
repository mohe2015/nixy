use crate::parser::AST;

pub trait ASTVisitor<R> {
    fn visit_code_before() -> R;
}




pub struct ASTBuilder;

impl<'a> ASTVisitor<AST<'a>> for ASTBuilder {

    fn visit_code_before() -> AST<'a> {
        todo!()
    }
}