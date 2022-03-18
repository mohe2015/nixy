use itertools::Itertools;
use rand::thread_rng;

use crate::{
    ast::{ASTBuilder, AST},
    lexer::NixTokenType,
    parser::Parser,
};
use std::{io::Write, marker::PhantomData};

pub struct JavaCodegen<'a, W: Write> {
    pub writer: &'a mut W,
    pub withs: Vec<(String, &'a AST<'a>)>,
    pub variables: Vec<(&'a str, &'a AST<'a>)>, // TODO FIXME hashmap and stack
}

// TODO FIXME in java, memorize forced values

impl<'a, W: Write> JavaCodegen<'a, W> {
    fn codegen_expr(&mut self, expr: &'a AST<'a>) {
        match expr {
            AST::Identifier(value) => {
                if self.variables.iter().any(|f| f.0 == *value) {
                    write!(self.writer, "{}_", value).unwrap()
                } else {
                    for (ident, with) in self.withs.clone().iter().rev() {
                        write!(self.writer, r#"{}.get("{}")"#, ident, value).unwrap()
                    }
                }
            }
            AST::Integer(value) => write!(self.writer, "NixInteger.create({})", value).unwrap(),
            AST::Float(value) => write!(self.writer, "NixFloat.create({}f)", value).unwrap(),
            AST::String(value) => {
                write!(self.writer, "NixString.create(\"\"\"\n{}\"\"\")", value).unwrap()
            }
            AST::Call(function, param) => {
                self.codegen_expr(function);
                write!(self.writer, r#".createCall("#,).unwrap();
                self.codegen_expr(param);
                write!(self.writer, r#")"#,).unwrap();
            }
            AST::Array(array) => {
                write!(self.writer, r#"NixArray.create(java.util.Arrays.asList("#,).unwrap();
                for (i, x) in array.iter().enumerate() {
                    if i != 0 {
                        write!(self.writer, r#","#,).unwrap();
                    }
                    self.codegen_expr(x);
                }
                write!(self.writer, r#"))"#,).unwrap();
            }
            AST::With(with_expr, expr) => {
                // in theory I think we could run the with_expr and then from the results get all local variables so with could be handled completely statically
                use rand::Rng;
                const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
                let mut rng = rand::thread_rng();

                let random_identifer: String = (0..10)
                    .map(|_| {
                        let idx = rng.gen_range(0..CHARSET.len());
                        CHARSET[idx] as char
                    })
                    .collect();

                write!(
                    self.writer,
                    r#"((NixLazy) () -> {{
                    NixLazy {} = "#,
                    random_identifer
                )
                .unwrap();
                self.codegen_expr(with_expr);
                self.withs.push((random_identifer, with_expr));
                write!(
                    self.writer,
                    r#";
                  
                return 
    "#
                )
                .unwrap();
                self.codegen_expr(expr);
                write!(
                    self.writer,
                    r#".force();
        }})"#
                )
                .unwrap();
                self.withs.pop();
            }
            AST::Let(binds, expr) => {
                write!(self.writer, r#"((NixLazy) () -> {{
                    "#).unwrap();
                for bind in binds {
                    match bind {
                        // fuck everything is an attrset
                        AST::Bind(variable, value) => {
                            write!(self.writer, r#"NixProxy x_ = new NixProxy();
    "#).unwrap();
                        }
                        _ => panic!()
                    }
                }
            
        write!(self.writer, r#"y_.proxy = "#).unwrap();
    write!(self.writer, r#"
            return .force();
    }}"#).unwrap();
            }
            ast => panic!("{:?}", ast),
        }
    }

    fn codegen(&mut self, expr: &'a AST<'a>) {
        write!(
            self.writer,
            r#"
public class MainClosure extends NixLazyBase {{

    public NixValue force() {{
        return "#
        )
        .unwrap();

        self.codegen_expr(expr);

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
}

fn test_codegen<'a>(code: &'a [u8]) {
    let mut data = Vec::new();
    let mut transpiler = JavaCodegen {
        writer: &mut data,
        withs: Vec::new(),
        variables: vec![("builtins", &AST::Builtins)],
    };
    let ast_builder = ASTBuilder {};

    let lexer = crate::lexer::NixLexer::new(code).filter(|t| {
        !matches!(
            t.token_type,
            NixTokenType::Whitespace(_)
                | NixTokenType::SingleLineComment(_)
                | NixTokenType::MultiLineComment(_)
        )
    });

    let mut parser = Parser {
        lexer: itertools::multipeek(lexer),
        visitor: ast_builder,
        phantom: PhantomData,
    };

    let expr = parser.parse().unwrap();

    transpiler.codegen(&expr);

    std::fs::write("/tmp/MainClosure.java", data).expect("Unable to write file");

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

#[test]
fn test_codegen_basic() {
    test_codegen(br#"let ${"hi"} = 1; in hi"#);
    test_codegen(br"1");
    test_codegen(br#"with builtins; (length [1 2 3 "x"])"#);
    test_codegen(
        br#" (let y = x + "b";
       x = "a"; in
    y + "c")"#,
    );
    test_codegen(br#"with builtins; (length [1 2 3 "x"])"#);
    test_codegen(
        br#"(let a = 1; in
           let a = 2; in
             a)"#,
    );
    test_codegen(br#"(import /tmp/foo.nix)"#);
    test_codegen(br#"/tmp/tutorials/learn.nix"#);
    test_codegen(br#"("Your home directory is ${1} ${1}")"#);
    test_codegen(b"(true && false)");
    test_codegen(b"(true || false)");
    test_codegen(br#"(if 3 < 4 then "a" else "b")"#);
    test_codegen(br#"(4 + 6 + 12 - 2)"#);
    test_codegen(br#"(4 - 2.5)"#);
    test_codegen(br#"(7 / 2)"#);
    test_codegen(br#"(7 / 2.0)"#);
    test_codegen(br#""Strings literals are in double quotes.""#);
    test_codegen(
        br#""
       String literals can span
       multiple lines.
     ""#,
    );
    test_codegen(
        br#"''
       This is called an "indented string" literal.
       It intelligently strips leading whitespace.
     ''"#,
    );
    test_codegen(
        br#"''
       a
         b
     ''"#,
    );
    test_codegen(br#"("ab" + "cd")"#);
    test_codegen(br#"7/2"#);
    test_codegen(br#"(7 / 2)"#);
    test_codegen(
        br#"(let x = "a"; in
       x + x + x)"#,
    );
    test_codegen(br#"(n: n + 1)"#);
    test_codegen(br#"((n: n + 1) 5)"#);
    test_codegen(br#"(let succ = (n: n + 1); in succ 5)"#);
    test_codegen(br#"((x: y: x + "-" + y) "a" "b")"#);
    test_codegen(br#"([1 2 3] ++ [4 5])"#);
    test_codegen(br#"(concatLists [[1 2] [3 4] [5]])"#);
    test_codegen(br#"(head [1 2 3])"#);
    test_codegen(br#"(tail [1 2 3])"#);
    test_codegen(br#"(elemAt ["a" "b" "c" "d"] 2)"#);
    test_codegen(br#"(elem 2 [1 2 3])"#);
    test_codegen(br#"(elem 5 [1 2 3])"#);
    test_codegen(br#"(filter (n: n < 3) [1 2 3 4])"#);
    test_codegen(br#"{ foo = [1 2]; bar = "x"; }"#);
    test_codegen(br#"{ a = 1; b = 2; }.a"#);
    test_codegen(br#"({ a = 1; b = 2; } ? a)"#);
    test_codegen(br#"({ a = 1; b = 2; } ? c)"#);
    test_codegen(br#"({ a = 1; } // { b = 2; })"#);
    test_codegen(br#"({ a = 1; b = 2; } // { a = 3; c = 4; })"#);
    test_codegen(br#"(let a = 1; in { a = 2; b = a; }.b)"#);
    test_codegen(br#"(let a = 1; in rec { a = 2; b = a; }.b)"#);
    test_codegen(
        br#"{
           a.b = 1;
           a.c.d = 2;
           a.c.e = 3;
         }.a.c"#,
    );
    test_codegen(
        br#"{
           a = { b = 1; };
           a.c = 2;
         }"#,
    );
    test_codegen(
        br#"(with { a = 1; b = 2; };
           a + b)"#,
    );
    test_codegen(
        br#"(with { a = 1; b = 2; };
           (with { a = 5; };
             a + b))"#,
    );
    test_codegen(br#"(args: args.x + "-" + args.y) { x = "a"; y = "b"; }"#);
    test_codegen(br#"({x, y}: x + "-" + y) { x = "a"; y = "b"; }"#);
    test_codegen(br#"({x, y, ...}: x + "-" + y) { x = "a"; y = "b"; z = "c"; }"#);
    test_codegen(br#"(assert 1 < 2; 42)"#);
    test_codegen(br#"(tryEval 42)"#);

    test_codegen(b"{ a = 1; b = 10; }");
    test_codegen(b"let a = 5; b = 7; in a + b");
    test_codegen(b"(a: a + 1) 2");
    test_codegen(br#"["1" "true" "yes"]"#);
    test_codegen(b"1");
    test_codegen(b"1 + 1");
    test_codegen(b"if 1 == 1 then 1 + 1 else 2 + 2");
    test_codegen(b"a: a + 1");
}
