use std::panic;
use std::{fs, io::Result};
use walkdir::WalkDir;
use tracing::{info, Level};
use tracing_subscriber::{FmtSubscriber, fmt::format::FmtSpan};

use crate::{
    lexer::{NixLexer, NixTokenType},
    parser::parse,
};

pub mod lexer;
pub mod parser;

// cargo run |& sort | uniq -c | sort -n

fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
    .with_span_events(FmtSpan::ACTIVE)
    .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::trace!("Hello, world!");

    let mut success = 0;
    let mut failure = 0;

    for entry in WalkDir::new("/etc/nixos/nixpkgs") {
        let entry = entry.unwrap();
        let f_name = entry.file_name().to_string_lossy();
        let path = entry.path();
        match panic::catch_unwind(|| {
            if f_name.ends_with(".nix") {
                println!("{}", path.display());

                // check whether this here is cache-wise better or if reading in chunks is better
                let file = fs::read(path).unwrap();

                let mut lexer = NixLexer::new(&file).filter(|t| match t.token_type {
                    NixTokenType::Whitespace(_)
                    | NixTokenType::SingleLineComment(_)
                    | NixTokenType::MultiLineComment(_) => false,
                    _ => true,
                });

                for token in lexer.clone() {
                    println!("{:?}", token.token_type);
                }

                println!("parsing");

                parse(&mut lexer);
            };
        }) {
            Ok(_) => success += 1,
            Err(_) => {
                failure += 1;
                panic!("{}", path.display());
            }
        }
    }

    // 51886/51886
    println!("{}/{}", success, success + failure);

    Ok(())
}
