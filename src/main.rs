use std::panic;
use std::{fs, io::Result};
use walkdir::WalkDir;

use crate::lexer::NixLexer;

pub mod lexer;

// cargo run |& sort | uniq -c | sort -n

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut success = 0;
    let mut failure = 0;

    for entry in WalkDir::new("/etc/nixos/nixpkgs") {
        let entry = entry.unwrap();
        let f_name = entry.file_name().to_string_lossy();
        let path = entry.path();
        match panic::catch_unwind(|| {
            if f_name.ends_with(".nix") {
                //println!("{}", path.display());

                // check whether this here is cache-wise better or if reading in chunks is better
                let file = fs::read(path).unwrap();

                let lexer = NixLexer::new(&file);

                for token in lexer {
                    //println!("{:?}", token.token_type);
                }
            };
        }) {
            Ok(_) => success += 1,
            Err(_) => {
                failure += 1;
                println!("{}", path.display());
            }
        }
    }

    // 51886/51886
    println!("{}/{}", success, success + failure);

    Ok(())
}
