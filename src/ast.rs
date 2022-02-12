use std::io::Write;
use std::{io::BufWriter, marker::PhantomData};

use crate::{
    lexer::NixTokenType,
    parser::{
        BindType, Parser, AST, BUILTIN_IF, BUILTIN_PATH_CONCATENATE, BUILTIN_SELECT,
        BUILTIN_UNARY_MINUS, BUILTIN_UNARY_NOT,
    },
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

    fn visit_if_before(&mut self);

    fn visit_if_after_condition(&mut self, condition: &R);

    fn visit_if_after_true_case(&mut self, condition: &R, true_case: &R);

    fn visit_if(&mut self, condition: R, true_case: R, false_case: R) -> R;

    fn visit_attrpath_part(&mut self, begin: R, last: R) -> R;

    fn visit_path_concatenate(&mut self, begin: R, last: R) -> R;

    fn visit_path_segment(&mut self, segment: &'a [u8]) -> R;

    fn visit_string(&mut self, string: &'a [u8]) -> R;

    fn visit_string_concatenate(&mut self, begin: R, last: R) -> R;

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

    fn visit_let_push_bind(&mut self, binds: Option<R>, bind: R) -> R;

    fn visit_let_before_body(&mut self, binds: &Option<R>);

    fn visit_let(&mut self, binds: Option<R>, body: R) -> R;

    fn visit_attrset_before(&mut self, binds: &Option<R>);

    fn visit_attrset_bind_push(&mut self, begin: Option<R>, last: R) -> R;

    fn visit_attrset(&mut self, binds: Option<R>) -> R;
}

pub struct ASTJavaTranspiler<'a, W: Write> {
    writer: &'a mut W,
}

// cargo test ast::test_java_transpiler -- --nocapture
#[test]
pub fn test_java_transpiler() {
    // https://learnxinyminutes.com/docs/nix/
    test_java_transpiler_code(b"(true && false)");
    test_java_transpiler_code(b"(true || false)");
    test_java_transpiler_code(br#"(if 3 < 4 then "a" else "b")"#);
    test_java_transpiler_code(br#"(4 + 6 + 12 - 2)"#);
    test_java_transpiler_code(br#"(4 - 2.5)"#);
    test_java_transpiler_code(br#"(7 / 2)"#);
    test_java_transpiler_code(br#"(7 / 2.0)"#);
    test_java_transpiler_code(br#""Strings literals are in double quotes.""#);
    test_java_transpiler_code(br#""
    String literals can span
    multiple lines.
  ""#);
    test_java_transpiler_code(br#"''
    This is called an "indented string" literal.
    It intelligently strips leading whitespace.
  ''"#);
    test_java_transpiler_code(br#"''
    a
      b
  ''"#);
    test_java_transpiler_code(br#"("ab" + "cd")"#);
    test_java_transpiler_code(br#"("Your home directory is ${1}")"#);
    test_java_transpiler_code(br#"/tmp/tutorials/learn.nix"#);
    test_java_transpiler_code(br#"7/2"#);
    test_java_transpiler_code(br#"(7 / 2)"#);
    test_java_transpiler_code(br#"(import /tmp/foo.nix)"#);
    test_java_transpiler_code(br#"(let x = "a"; in
    x + x + x)"#);
    test_java_transpiler_code(br#" (let y = x + "b";
    x = "a"; in
 y + "c")"#);
    test_java_transpiler_code(br#"(let a = 1; in
        let a = 2; in
          a)"#);
    test_java_transpiler_code(br#"(n: n + 1)"#);
    test_java_transpiler_code(br#"((n: n + 1) 5)"#);
    test_java_transpiler_code(br#"(let succ = (n: n + 1); in succ 5)"#);
    test_java_transpiler_code(br#"((x: y: x + "-" + y) "a" "b")"#);
    test_java_transpiler_code(br#"length [1 2 3 "x"])"#);
    test_java_transpiler_code(br#"([1 2 3] ++ [4 5])"#);
    test_java_transpiler_code(br#"(concatLists [[1 2] [3 4] [5]])"#);
    test_java_transpiler_code(br#"(head [1 2 3])"#);
    test_java_transpiler_code(br#"(tail [1 2 3])"#);
    test_java_transpiler_code(br#"(elemAt ["a" "b" "c" "d"] 2)"#);
    test_java_transpiler_code(br#"(elem 2 [1 2 3])"#);
    test_java_transpiler_code(br#"(elem 5 [1 2 3])"#);
    test_java_transpiler_code(br#"(filter (n: n < 3) [1 2 3 4])"#);
    test_java_transpiler_code(br#"{ foo = [1 2]; bar = "x"; }"#);
    test_java_transpiler_code(br#"{ a = 1; b = 2; }.a"#);
    test_java_transpiler_code(br#"({ a = 1; b = 2; } ? a)"#);
    test_java_transpiler_code(br#"({ a = 1; b = 2; } ? c)"#);
    test_java_transpiler_code(br#"({ a = 1; } // { b = 2; })"#);
    test_java_transpiler_code(br#"({ a = 1; b = 2; } // { a = 3; c = 4; })"#);
    test_java_transpiler_code(br#"(let a = 1; in { a = 2; b = a; }.b)"#);
    test_java_transpiler_code(br#"(let a = 1; in rec { a = 2; b = a; }.b)"#);
    test_java_transpiler_code(br#"{
        a.b = 1;
        a.c.d = 2;
        a.c.e = 3;
      }.a.c"#);
    test_java_transpiler_code(br#"{
        a = { b = 1; };
        a.c = 2;
      }"#);
    test_java_transpiler_code(br#"(with { a = 1; b = 2; };
        a + b)"#);
    test_java_transpiler_code(br#"(with { a = 1; b = 2; };
        (with { a = 5; };
          a + b))"#);
    test_java_transpiler_code(br#"(args: args.x + "-" + args.y) { x = "a"; y = "b"; }"#);
    test_java_transpiler_code(br#"({x, y}: x + "-" + y) { x = "a"; y = "b"; }"#);
    test_java_transpiler_code(br#"({x, y, ...}: x + "-" + y) { x = "a"; y = "b"; z = "c"; }"#);
    test_java_transpiler_code(br#"(assert 1 < 2; 42)"#);
    test_java_transpiler_code(br#"(tryEval 42)"#);

    test_java_transpiler_code(b"{ a = 1; b = 10; }");
    test_java_transpiler_code(b"let a = 5; b = 7; in a + b");
    test_java_transpiler_code(b"(a: a + 1) 2");
    test_java_transpiler_code(br#"["1" "true" "yes"]"#);
    test_java_transpiler_code(b"1");
    test_java_transpiler_code(b"1 + 1");
    test_java_transpiler_code(b"if 1 == 1 then 1 + 1 else 2 + 2");
    test_java_transpiler_code(b"a: a + 1");
}

impl<'a, W: Write> ASTVisitor<'a, ()> for ASTJavaTranspiler<'a, W> {
    fn visit_file_start(&mut self) {
        write!(
            self.writer,
            r#"
public class MainClosure implements NixLazy {{

    public NixValue force() {{
        return "#
        )
        .unwrap();
    }

    fn visit_file_end(&mut self) {
        write!(
            self.writer,
            r#".force();
    }}

    public static void main(String[] args) {{
		System.out.println(new MainClosure().force());
	}}
}}
        "#
        )
        .unwrap();
    }

    // probably also make functions lazy?
    fn visit_function_before(&mut self) {
        write!(self.writer, "NixLambda.createFunction(").unwrap();
    }

    fn visit_function_enter(&mut self, arg: &()) {
        write!(
            self.writer,
            r#" -> {{
            return 
        "#
        )
        .unwrap();
    }

    fn visit_function_exit(&mut self, arg: (), body: ()) -> () {
        write!(
            self.writer,
            ".force();
}})"
        )
        .unwrap();
    }

    fn visit_identifier(&mut self, id: &'a [u8]) -> () {
        // I think just because of the with statement
        // we need to make this completely dynamic
        write!(self.writer, "__nixy_vars.get(\"{}\")", std::str::from_utf8(id).unwrap()).unwrap();
    }

    fn visit_integer(&mut self, integer: i64) -> () {
        write!(self.writer, "NixInteger.create({})", integer).unwrap();
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
            NixTokenType::Subtraction => {
                write!(self.writer, ".subtract(").unwrap();
            }
            NixTokenType::Multiplication => {
                write!(self.writer, ".multiply(").unwrap();
            }
            NixTokenType::Division => {
                write!(self.writer, ".divide(").unwrap();
            }
            NixTokenType::Equals => {
                write!(self.writer, ".eq(").unwrap();
            }
            NixTokenType::NotEquals => {
                write!(self.writer, ".neq(").unwrap();
            }
            NixTokenType::LessThan => {
                write!(self.writer, ".lt(").unwrap();
            }
            NixTokenType::LessThanOrEqual => {
                write!(self.writer, ".lte(").unwrap();
            }
            NixTokenType::GreaterThan => {
                write!(self.writer, ".gt(").unwrap();
            }
            NixTokenType::GreaterThanOrEqual => {
                write!(self.writer, ".gte(").unwrap();
            }
            NixTokenType::And => {
                write!(self.writer, ".land(").unwrap();
            }
            NixTokenType::Or => {
                write!(self.writer, ".lor(").unwrap();
            }
            _ => todo!(),
        }
    }

    fn visit_infix_operation(&mut self, left: (), right: (), operator: NixTokenType<'a>) -> () {
        write!(self.writer, ")").unwrap();
    }

    fn visit_prefix_operation(&mut self, operator: NixTokenType<'a>, expr: ()) -> () {
        todo!()
    }

    fn visit_if_before(&mut self) {
        write!(self.writer, r#"NixLazy.createIf("#).unwrap();
    }

    fn visit_if_after_condition(&mut self, condition: &()) {
        write!(self.writer, r#","#).unwrap();
    }

    fn visit_if_after_true_case(&mut self, condition: &(), true_case: &()) {
        write!(self.writer, r#","#).unwrap();
    }

    fn visit_if(&mut self, condition: (), true_case: (), false_case: ()) -> () {
        write!(self.writer, r#")"#).unwrap();
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
        // https://www.vojtechruzicka.com/raw-strings/
        write!(
            self.writer,
            "NixString.create(\"\"\"\n{}\"\"\")",
            std::str::from_utf8(string).unwrap()
        )
        .unwrap();
    }

    fn visit_string_concatenate(&mut self, begin: (), last: ()) -> () {
        todo!()
    }

    fn visit_array_start(&mut self) {
        write!(self.writer, r#"NixArray.create(java.util.Arrays.asList("#,).unwrap();
    }

    fn visit_array_push_before(&mut self, begin: &Option<()>) {
        if let Some(_) = begin {
            write!(self.writer, r#","#,).unwrap();
        }
    }

    fn visit_array_push(&mut self, begin: Option<()>, last: ()) -> () {}

    fn visit_array_end(&mut self, array: ()) -> () {
        write!(self.writer, r#"))"#,).unwrap();
    }

    fn visit_call_maybe(&mut self, expr: &Option<()>) {
        match expr {
            Some(_) => {
                write!(self.writer, r#".createCall("#,).unwrap();
            }
            None => {
                // write!(self.writer, r#"NixLambda.createCall("#, ).unwrap();
            }
        }
    }

    fn visit_call_maybe_not(&mut self) {
        write!(self.writer, r#")"#,).unwrap();
    }

    fn visit_call(&mut self, function: (), parameter: ()) -> () {
        write!(self.writer, r#")"#,).unwrap();
    }

    fn visit_bind_before(&mut self, bind_type: BindType) {
        match bind_type {
            BindType::Let => write!(self.writer, r#"NixLazy "#,).unwrap(),
            BindType::Attrset => write!(self.writer, r#"this.put(""#,).unwrap(),
        }
    }

    fn visit_bind_between(&mut self, bind_type: BindType, attrpath: &()) {
        match bind_type {
            BindType::Let => write!(self.writer, r#" = "#,).unwrap(),
            BindType::Attrset => write!(self.writer, r#"".intern(), "#,).unwrap(),
        }
    }

    fn visit_bind_after(&mut self, bind_type: BindType, attrpath: (), expr: ()) -> () {
        match bind_type {
            BindType::Let => write!(self.writer, ";\n",).unwrap(),
            BindType::Attrset => write!(self.writer, ");",).unwrap(),
        }
    }

    fn visit_let_before(&mut self) {
        write!(
            self.writer,
            r#"((NixLazy) () -> {{
			/* head */"#,
        )
        .unwrap();
    }

    fn visit_let_push_bind(&mut self, binds: Option<()>, bind: ()) -> () {}

    fn visit_let_before_body(&mut self, binds: &Option<()>) {
        write!(self.writer, "\n/* body */ \nreturn ",).unwrap();
    }

    fn visit_let(&mut self, binds: Option<()>, body: ()) -> () {
        write!(self.writer, ".force(); }})",).unwrap();
    }

    fn visit_attrset_before(&mut self, binds: &Option<()>) {
        if let None = binds {
            write!(
                self.writer,
                "NixAttrset.create(new java.util.IdentityHashMap<String, NixLazy>() {{{{\n",
            )
            .unwrap();
        }
    }

    fn visit_attrset_bind_push(&mut self, begin: Option<()>, last: ()) -> () {}

    fn visit_attrset(&mut self, binds: Option<()>) -> () {
        write!(self.writer, r#"}}}})"#,).unwrap();
    }
}

fn test_java_transpiler_code(code: &[u8]) {
    let mut data = Vec::new();
    let transpiler = ASTJavaTranspiler { writer: &mut data };

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

    std::fs::write("/tmp/MainClosure.java", data).expect("Unable to write file");

    let mut fmt_cmd = std::process::Command::new("google-java-format");

    fmt_cmd.arg("--replace").arg("/tmp/MainClosure.java");

    let fmt_output = fmt_cmd.output().unwrap();

    println!(
        "java formatter exited with {} {} {}",
        fmt_output.status,
        String::from_utf8(fmt_output.stderr).unwrap(),
        String::from_utf8(fmt_output.stdout).unwrap()
    );

    if !fmt_output.status.success() {
        panic!("invalid syntax (according to java formatter)");
    }

    let mut compile_cmd = std::process::Command::new("javac");

    compile_cmd
        .arg("-cp")
        .arg("java/")
        .arg("/tmp/MainClosure.java");

    let compile_output = compile_cmd.output().unwrap();
    println!(
        "java compiler exited with {} {} {}",
        compile_output.status,
        String::from_utf8(compile_output.stderr).unwrap(),
        String::from_utf8(compile_output.stdout).unwrap()
    );

    if !compile_output.status.success() {
        panic!("invalid syntax (according to java compiler)");
    }

    let mut run_cmd = std::process::Command::new("java");

    run_cmd.arg("-cp").arg("/tmp:java/").arg("MainClosure");

    let run_cmd = run_cmd.output().unwrap();
    println!(
        "java program exited with {} {} {}",
        run_cmd.status,
        String::from_utf8(run_cmd.stderr).unwrap(),
        String::from_utf8(run_cmd.stdout).unwrap()
    );

    if !run_cmd.status.success() {
        panic!("failed to run java program");
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

    fn visit_string_concatenate(&mut self, _begin: AST<'a>, _last: AST<'a>) -> AST<'a> {
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
        todo!()
    }

    fn visit_call_maybe_not(&mut self) {
        todo!()
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
}
