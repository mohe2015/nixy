use std::{fs, io::Result};

use crate::lexer::NixLexer;

pub mod lexer;

fn main() -> Result<()> {
    println!("Hello, world!");

    // check whether this here is cache-wise better or if reading in chunks is better
    let file = fs::read("/etc/nixos/nixpkgs/flake.nix")?;

    let lexer = NixLexer::new(&file);

    for token in lexer {
        println!("{:?}", token);
    }

    Ok(())
}
