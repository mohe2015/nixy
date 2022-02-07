

#[derive(Debug)]
pub struct NixToken {}

pub struct NixLexer(pub String);

impl Iterator for NixLexer {
    type Item = NixToken;

    fn next(&mut self) -> Option<Self::Item> {

        
        todo!()
    }
}