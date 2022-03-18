use std::{io::Write, marker::PhantomData};

use crate::{
    lexer::NixTokenType,
    parser::{BindType, Parser},
    visitor::ASTVisitor,
};

pub struct ASTJavaTranspiler<'a, W: Write> {
    pub writer: &'a mut W,
}

// cargo test ast::test_java_transpiler -- --nocapture
#[test]
pub fn test_java_transpiler() {
    // https://learnxinyminutes.com/docs/nix/

    // probably use the ast to codegen
    // then this should be easy using the proxy implementation
    // using a Map to store all variables will get extremely annoying
    // because then I have to implement variable capturing etc

    /*test_java_transpiler_code(
           br#" (let y = x + "b";
       x = "a"; in
    y + "c")"#,
       );*/
    test_java_transpiler_code(br#"with builtins; (length [1 2 3 "x"])"#);
    test_java_transpiler_code(
        br#"(let a = 1; in
        let a = 2; in
          a)"#,
    );
    test_java_transpiler_code(br#"(import /tmp/foo.nix)"#);
    test_java_transpiler_code(br#"/tmp/tutorials/learn.nix"#);
    test_java_transpiler_code(br#"("Your home directory is ${1} ${1}")"#);
    test_java_transpiler_code(b"(true && false)");
    test_java_transpiler_code(b"(true || false)");
    test_java_transpiler_code(br#"(if 3 < 4 then "a" else "b")"#);
    test_java_transpiler_code(br#"(4 + 6 + 12 - 2)"#);
    test_java_transpiler_code(br#"(4 - 2.5)"#);
    test_java_transpiler_code(br#"(7 / 2)"#);
    test_java_transpiler_code(br#"(7 / 2.0)"#);
    test_java_transpiler_code(br#""Strings literals are in double quotes.""#);
    test_java_transpiler_code(
        br#""
    String literals can span
    multiple lines.
  ""#,
    );
    test_java_transpiler_code(
        br#"''
    This is called an "indented string" literal.
    It intelligently strips leading whitespace.
  ''"#,
    );
    test_java_transpiler_code(
        br#"''
    a
      b
  ''"#,
    );
    test_java_transpiler_code(br#"("ab" + "cd")"#);
    test_java_transpiler_code(br#"7/2"#);
    test_java_transpiler_code(br#"(7 / 2)"#);
    test_java_transpiler_code(
        br#"(let x = "a"; in
    x + x + x)"#,
    );
    test_java_transpiler_code(br#"(n: n + 1)"#);
    test_java_transpiler_code(br#"((n: n + 1) 5)"#);
    test_java_transpiler_code(br#"(let succ = (n: n + 1); in succ 5)"#);
    test_java_transpiler_code(br#"((x: y: x + "-" + y) "a" "b")"#);
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
    test_java_transpiler_code(
        br#"{
        a.b = 1;
        a.c.d = 2;
        a.c.e = 3;
      }.a.c"#,
    );
    test_java_transpiler_code(
        br#"{
        a = { b = 1; };
        a.c = 2;
      }"#,
    );
    test_java_transpiler_code(
        br#"(with { a = 1; b = 2; };
        a + b)"#,
    );
    test_java_transpiler_code(
        br#"(with { a = 1; b = 2; };
        (with { a = 5; };
          a + b))"#,
    );
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
public class MainClosure extends NixLazyBase {{

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

    fn visit_function_enter(&mut self, _arg: &()) {
        write!(
            self.writer,
            r#" -> {{
            return 
        "#
        )
        .unwrap();
    }

    fn visit_function_exit(&mut self, _arg: (), _body: ()) {
        write!(
            self.writer,
            ".force();
}})"
        )
        .unwrap();
    }

    fn visit_identifier(&mut self, id: &'a [u8]) {
        // I think just because of the with statement
        // we need to make this completely dynamic
        write!(self.writer, "{}_", std::str::from_utf8(id).unwrap()).unwrap();
    }

    fn visit_integer(&mut self, integer: i64) {
        write!(self.writer, "NixInteger.create({})", integer).unwrap();
    }

    fn visit_float(&mut self, float: f64) {
        write!(self.writer, "NixFloat.create({}f)", float).unwrap();
    }

    fn visit_todo(&mut self) {
        todo!()
    }

    fn visit_select(&mut self, _expr: (), _attrpath: (), _default: Option<()>) {
        todo!()
    }

    fn visit_infix_lhs(&mut self, operator: NixTokenType<'a>, _left: &()) {
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

    fn visit_infix_operation(&mut self, _left: (), _right: (), _operator: NixTokenType<'a>) {
        write!(self.writer, ")").unwrap();
    }

    fn visit_prefix_operation(&mut self, _operator: NixTokenType<'a>, _expr: ()) {
        todo!()
    }

    fn visit_if_before(&mut self) {
        write!(self.writer, r#"NixLazy.createIf("#).unwrap();
    }

    fn visit_if_after_condition(&mut self, _condition: &()) {
        write!(self.writer, r#","#).unwrap();
    }

    fn visit_if_after_true_case(&mut self, _condition: &(), _true_case: &()) {
        write!(self.writer, r#","#).unwrap();
    }

    fn visit_if(&mut self, _condition: (), _true_case: (), _false_case: ()) {
        write!(self.writer, r#")"#).unwrap();
    }

    fn visit_attrpath_part(&mut self, _begin: (), _last: ()) {
        todo!()
    }

    fn visit_path_concatenate(&mut self, _begin: (), _last: ()) {
        todo!()
    }

    fn visit_path_segment(&mut self, segment: &'a [u8]) {
        write!(
            self.writer,
            "NixPath.create(\"\"\"\n{}\"\"\")",
            std::str::from_utf8(segment).unwrap()
        )
        .unwrap();
    }

    fn visit_string(&mut self, string: &'a [u8]) {
        // https://www.vojtechruzicka.com/raw-strings/
        write!(
            self.writer,
            "NixString.create(\"\"\"\n{}\"\"\")",
            std::str::from_utf8(string).unwrap()
        )
        .unwrap();
    }

    fn visit_string_concatenate(&mut self, begin: Option<()>, _last: ()) {
        match begin {
            Some(_) => {
                write!(self.writer, r#").add("#,).unwrap();
            }
            None => {
                write!(self.writer, r#".add("#,).unwrap();
            }
        }
    }

    fn visit_string_concatenate_end(&mut self, _result: Option<()>) {
        write!(self.writer, r#")"#,).unwrap();
    }

    fn visit_array_start(&mut self) {
        write!(self.writer, r#"NixArray.create(java.util.Arrays.asList("#,).unwrap();
    }

    fn visit_array_push_before(&mut self, begin: &[()]) {
        if !begin.is_empty() {
            write!(self.writer, r#","#,).unwrap();
        }
    }

    fn visit_array_push(&mut self, _begin: &[()], _last: ()) {}

    fn visit_array_end(&mut self, _array: Vec<()>) {
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

    fn visit_call(&mut self, _function: (), _parameter: ()) {
        write!(self.writer, r#")"#,).unwrap();
    }

    fn visit_bind_before(&mut self, bind_type: BindType) {
        match bind_type {
            BindType::Let => write!(self.writer, r#"NixLazy "#,).unwrap(),
            BindType::Attrset => write!(self.writer, r#"this.put(""#,).unwrap(),
        }
    }

    fn visit_bind_between(&mut self, bind_type: BindType, _attrpath: &()) {
        match bind_type {
            BindType::Let => write!(self.writer, r#" = "#,).unwrap(),
            BindType::Attrset => write!(self.writer, r#"".intern(), "#,).unwrap(),
        }
    }

    fn visit_bind_after(&mut self, bind_type: BindType, _attrpath: (), _expr: ()) {
        match bind_type {
            BindType::Let => writeln!(self.writer, ";",).unwrap(),
            BindType::Attrset => write!(self.writer, ");",).unwrap(),
        }
    }

    fn visit_let_before(&mut self) {
        write!(
            self.writer,
            "(new NixLazy() {{

                @Override
                public NixValue force() {{
			/* head */\n",
        )
        .unwrap();
    }

    fn visit_let_bind_push(&mut self, _binds: &[()], _bind: ()) {}

    fn visit_let_before_body(&mut self, _binds: &[()]) {
        write!(self.writer, "\n/* body */ \nreturn ",).unwrap();
    }

    fn visit_let(&mut self, _binds: Vec<()>, _body: ()) {
        write!(self.writer, ".force(); }}}})",).unwrap();
    }

    fn visit_attrset_before(&mut self, binds: &[()]) {
        if binds.is_empty() {
            writeln!(
                self.writer,
                "NixAttrset.create(new java.util.IdentityHashMap<String, NixLazy>() {{{{",
            )
            .unwrap();
        }
    }

    fn visit_attrset_bind_push(&mut self, _begin: &[()], _last: ()) {}

    fn visit_attrset(&mut self, _binds: Vec<()>) {
        write!(self.writer, r#"}}}})"#,).unwrap();
    }

    fn visit_formal(&mut self, _formals: Option<()>, _identifier: &'a [u8], _default: Option<()>) {
        todo!()
    }

    fn visit_formals(
        &mut self,
        _formals: Option<()>,
        _at_identifier: Option<&'a [u8]>,
        _ellipsis: bool,
    ) {
        todo!()
    }

    fn visit_inherit(&mut self, _attrs: Vec<()>) {
        todo!()
    }

    fn visit_with(&mut self, with_expr: (), expr: ()) -> () {
        todo!()
    }
}

fn test_java_transpiler_code(code: &[u8]) {
    let mut data = Vec::new();
    let transpiler = ASTJavaTranspiler { writer: &mut data };

    let lexer = crate::lexer::NixLexer::new(code).filter(|t| {
        !matches!(
            t.token_type,
            NixTokenType::Whitespace(_)
                | NixTokenType::SingleLineComment(_)
                | NixTokenType::MultiLineComment(_)
        )
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

    /*
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
    }*/

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
