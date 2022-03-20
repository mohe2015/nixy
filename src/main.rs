use itertools::multipeek;
use std::{fs, io::Result, marker::PhantomData};
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;
use walkdir::WalkDir;

use crate::{
    ast::ASTBuilder,
    lexer::{NixLexer, NixTokenType},
    parser::Parser,
};

pub mod ast;
pub mod codegen;
pub mod codegen_lowmem;
pub mod lexer;
pub mod parser;
pub mod visitor;

// cargo run --release |& sort | uniq -c | sort -n

fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ACTIVE)
        .with_max_level(Level::ERROR)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let success = 0;
    let failure = 0;

    for entry in WalkDir::new("/etc/nixos/nixpkgs") {
        let entry = entry.unwrap();
        let f_name = entry.file_name().to_string_lossy();
        let path = entry.path();
        //match std::panic::catch_unwind(|| {
        //if !path.to_string_lossy().contains("nixpkgs/doc/default.nix") { return; }

        if f_name.ends_with(".nix") {
            //println!("{}", path.display());

            // ./target/release/nixy | sort -n

            // check whether this here is cache-wise better or if reading in chunks is better
            // in chunks should be better, haskell is 11MB

            // TODO FIXME read block by block
            let file = fs::read(path).unwrap();
            println!("{} {}", file.len(), path.display());

            let lexer = NixLexer::new(&file).filter(|t| {
                !matches!(
                    t.token_type,
                    NixTokenType::Whitespace(_)
                        | NixTokenType::SingleLineComment(_)
                        | NixTokenType::MultiLineComment(_)
                )
            });

            //success += lexer.count();

            //for token in lexer.clone() {
            //println!("{:?}", token.token_type);
            //}

            let mut parser = Parser {
                lexer: multipeek(lexer),
                visitor: ASTBuilder,
                phantom: PhantomData,
            };

            println!("{:#?}", parser.parse());
            break;
        };
        /* }) {
            Ok(_) => success += 1,
            Err(_) => {
                failure += 1;
                println!("{}", path.display());
            }
        }*/
    }

    // 51975/51975
    println!("{}/{}", success, success + failure);

    Ok(())
}
