use crate::{lexer::NixTokenType, parser::BindType};

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

    fn visit_if_before(&mut self);

    fn visit_if_after_condition(&mut self, condition: &R);

    fn visit_if_after_true_case(&mut self, condition: &R, true_case: &R);

    fn visit_if(&mut self, condition: R, true_case: R, false_case: R) -> R;

    fn visit_attrpath_part(&mut self, begin: R, last: R) -> R;

    fn visit_path_concatenate(&mut self, begin: R, last: R) -> R;

    fn visit_path_segment(&mut self, segment: &'a [u8]) -> R;

    fn visit_string(&mut self, string: &'a [u8]) -> R;

    fn visit_string_concatenate(&mut self, begin: Option<R>, last: R) -> R;

    fn visit_string_concatenate_end(&mut self, result: Option<R>) -> R;

    fn visit_array_start(&mut self);

    fn visit_array_push_before(&mut self, begin: &Option<R>);

    fn visit_array_push(&mut self, begin: Option<R>, last: R) -> R;

    /// This is always called after `visit_array_push` and may help some implementations.
    fn visit_array_end(&mut self, array: R) -> R;

    fn visit_call_maybe(&mut self, expr: &Option<R>);

    fn visit_call_maybe_not(&mut self);

    fn visit_call(&mut self, function: R, parameter: R) -> R;

    fn visit_function_enter(&mut self, arg: &R);

    fn visit_function_exit(&mut self, arg: R, body: R) -> R;

    fn visit_function_before(&mut self);

    fn visit_bind_before(&mut self, bind_type: BindType);

    fn visit_bind_between(&mut self, bind_type: BindType, attrpath: &R);

    fn visit_bind_after(&mut self, bind_type: BindType, attrpath: R, expr: R) -> R;

    fn visit_let_before(&mut self);

    fn visit_let_push_bind(&mut self, binds: &Vec<R>, bind: R) -> R;

    fn visit_let_before_body(&mut self, binds: &Vec<R>);

    fn visit_let(&mut self, binds: Vec<R>, body: R) -> R;

    fn visit_attrset_before(&mut self, binds: &Option<R>);

    fn visit_attrset_bind_push(&mut self, begin: Option<R>, last: R) -> R;

    fn visit_attrset(&mut self, binds: Option<R>) -> R;

    fn visit_formal(&mut self, formals: Option<R>, identifier: &'a [u8], default: Option<R>) -> R;

    fn visit_formals(
        &mut self,
        formals: Option<R>,
        at_identifier: Option<&'a [u8]>,
        ellipsis: bool,
    ) -> R;
}
