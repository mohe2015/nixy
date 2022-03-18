use std::io::Write;
use crate::{codegen_lowmem::ASTJavaTranspiler, ast::AST};


impl<'a, W: Write> ASTJavaTranspiler<'a, W> {

    fn codegen_expr(&mut self, expr: AST<'a>) {
        match expr {
            AST::Identifier(id) => write!(self.writer, "{}", id).unwrap(),
            ast => panic!("{:?}", ast)
        }
    }

    fn codegen(&mut self, expr: AST<'a>) {
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


fn test_codegen<'a>(expr: AST<'a>) {
    let mut data = Vec::new();
    let mut transpiler = ASTJavaTranspiler { writer: &mut data };

    transpiler.codegen(expr);

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
    test_codegen(AST::Identifier("hi"));
}