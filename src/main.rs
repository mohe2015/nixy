use std::{fs, io::Result};

use crate::lexer::NixLexer;

pub mod lexer;

fn main() -> Result<()> {
    println!("Hello, world!");

    let file = fs::read("/etc/nixos/nixpkgs/flake.nix")?;

    let lexer = NixLexer::new(&file);

    for token in lexer {
        println!("{:?}", token);
    }

    Ok(())
}
