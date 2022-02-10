use itertools::multipeek;
use std::{fs, io::Result};
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;
use walkdir::WalkDir;

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
        .with_max_level(Level::WARN)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let mut success = 0;
    let mut failure = 0;

    for entry in WalkDir::new("/etc/nixos/nixpkgs") {
        let entry = entry.unwrap();
        let f_name = entry.file_name().to_string_lossy();
        let path = entry.path();
        match std::panic::catch_unwind(|| {
        if f_name.ends_with(".nix") {
            println!("{}", path.display());

            // check whether this here is cache-wise better or if reading in chunks is better
            let file = fs::read(path).unwrap();

            let lexer = NixLexer::new(&file).filter(|t| match t.token_type {
                NixTokenType::Whitespace(_)
                | NixTokenType::SingleLineComment(_)
                | NixTokenType::MultiLineComment(_) => false,
                _ => true,
            });

            for token in lexer.clone() {
                //println!("{:?}", token.token_type);
            }

            parse(&mut multipeek(lexer));
        };
        }) {
            Ok(_) => success += 1,
            Err(_) => {
                failure += 1;
                //panic!("{}", path.display());
            }
        }
    }

    // 27530/51963
    println!("{}/{}", success, success + failure);

    Ok(())
}
