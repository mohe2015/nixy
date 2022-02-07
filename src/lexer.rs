use std::{slice::Iter, iter::{Peekable, Enumerate}};

// https://wduquette.github.io/parsing-strings-into-slices/

#[derive(Debug)]
pub struct SourcePosition {
    pub line: u16,
    pub column: u16,
}

#[derive(Debug)]
pub struct SourceLocation {
    pub start_location: SourcePosition,
    pub end_location: SourcePosition,
}

#[derive(Debug)]
pub enum NixTokenType<'a> {
    Identifier(&'a [u8]),
    Integer(i64),
    // Float,
    PathStart,
    PathSegment,
    PathEnd,
    StringStart,
    String,
    StringEnd,
    IndentedStringStart,
    IndentedString,
    IndentedStringEnd,
    InterpolateStart,
    InterpolateEnd,
    CurlyOpen,
    CurlyClose,
    Whitespace,
    SingleLineComment,
    MultiLineComment,
    // Uri,
    If,
    Then,
    Else,
    Assert,
    With,
    Let,
    In,
    Rec,
    Inherit,
    Or,
    Ellipsis,
    Equals,
    NotEquals,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Implies,
    Update,
    Concatenate,
}

#[derive(Debug)]
pub struct NixToken<'a> {
    pub token_type: NixTokenType<'a>,
    //pub location: SourceLocation,
}

pub struct NixLexer<'a>{
    pub data: &'a [u8],
    pub iter: Peekable<Enumerate<Iter<'a, u8>>>,
    line_start: bool,
}

impl<'a> NixLexer<'a> {

    pub fn new(data: &'a [u8]) -> Self {
        Self { 
            data,
            iter: data.into_iter().enumerate().peekable(),
            line_start: true,
        }
    }
}

impl<'a> Iterator for NixLexer<'a> {
    type Item = NixToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((offset, b'{')) => {
                Some(NixToken { token_type: NixTokenType::CurlyOpen })
            },
            Some((offset, b'#')) if self.line_start => {
                let end = self.iter.find(|(_, c)| **c == b'\n');
                let comment = &self.data[offset..=end.map(|v| v.0).unwrap_or(usize::MAX)];
                println!("{:?}", std::str::from_utf8(comment));
                Some(NixToken { token_type: NixTokenType::SingleLineComment })
            },
            Some((offset, b' ')) | Some((offset, b'\t')) | Some((offset, b'\r')) | Some((offset, b'\n')) => {
                loop {
                    match self.iter.peek() {
                        Some((_, b' ')) | Some((_, b'\t')) | Some((_, b'\r')) | Some((_, b'\n')) => {
                            self.iter.next();
                        }
                        _ => break
                    }
                }
                let whitespace = &self.data[offset..self.iter.peek().unwrap().0];
                println!("{:?}", std::str::from_utf8(whitespace));
                Some(NixToken { token_type: NixTokenType::Whitespace })
            },
            Some((_, b'A'..=b'Z')) => {
            
                Some(NixToken { token_type: NixTokenType::Identifier(self.data) })
            },
            None => None,
            _ => todo!()
        }
    }
}