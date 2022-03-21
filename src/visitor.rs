use crate::{lexer::NixTokenType, parser::BindType};

#[derive(PartialEq, Debug)]
pub enum WithOrLet {
    With, Let
}

pub trait ASTVisitor<'a, R: std::fmt::Debug, FORMALS: std::fmt::Debug, BIND: std::fmt::Debug, IDENTIFIER: std::fmt::Debug> {
    fn visit_identifier(&mut self, id: &'a [u8]) -> IDENTIFIER;

    fn visit_integer(&mut self, integer: i64) -> R;

    fn visit_float(&mut self, float: f64) -> R;

    fn visit_todo(&mut self) -> R;

    fn visit_select(&mut self, expr: R, attrpath: Vec<R>, default: Option<R>) -> R;

    fn visit_infix_lhs(&mut self, operator: NixTokenType<'a>, left: &R);

    fn visit_infix_operation(&mut self, left: R, right: R, operator: NixTokenType<'a>) -> R;

    fn visit_prefix_operation(&mut self, operator: NixTokenType<'a>, expr: R) -> R;

    fn visit_if(&mut self, condition: R, true_case: R, false_case: R) -> R;

    fn visit_path_concatenate(&mut self, begin: R, last: R) -> R;

    fn visit_path_segment(&mut self, segment: &'a [u8]) -> R;

    fn visit_string(&mut self, string: &'a [u8]) -> R;

    fn visit_string_concatenate(&mut self, begin: Option<R>, last: R) -> R;

    fn visit_string_concatenate_end(&mut self, result: Option<R>) -> R;

    fn visit_array_end(&mut self, array: Vec<R>) -> R;

    fn visit_call(&mut self, function: R, parameter: R) -> R;

    fn visit_function_exit(&mut self, arg: IDENTIFIER, body: R) -> R;

    fn visit_bind_after(&mut self, bind_type: BindType, attrpath: R, expr: R) -> BIND;

    fn visit_attrset(&mut self, binds: Vec<R>) -> R;

    fn visit_formal(&mut self, formals: Option<R>, identifier: &'a [u8], default: Option<R>) -> R;

    fn visit_formals(
        &mut self,
        formals: Option<R>,
        at_identifier: Option<IDENTIFIER>,
        ellipsis: bool,
    ) -> FORMALS;

    fn visit_inherit(&mut self, attrs: Vec<R>) -> R;

    fn visit_with_or_let(&mut self, with_or_let: WithOrLet, with_expr: R, expr: R) -> R;
}
