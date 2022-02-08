use std::{fs, io::Result};
use walkdir::WalkDir;

use crate::lexer::NixLexer;

pub mod lexer;

fn main() -> Result<()> {
    println!("Hello, world!");

    for entry in WalkDir::new("/etc/nixos/nixpkgs") {
        let entry = entry.unwrap();
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".nix") {
            println!("{}", entry.path().display());

            // check whether this here is cache-wise better or if reading in chunks is better
            let file = fs::read(entry.path())?;

            let lexer = NixLexer::new(&file);

            for token in lexer {
                println!("{:?}", token.token_type);
            }
        }
    }

    Ok(())
}
