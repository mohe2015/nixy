use std::slice::Iter;


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
    pub iter: Iter<'a, u8>,
    line_start: bool,
}

impl<'a> NixLexer<'a> {

    pub fn new(iter: Iter<'a, u8>) -> Self {
        Self { 
            iter: iter,
            line_start: true,
        }
    }
}

impl<'a> Iterator for NixLexer<'a> {
    type Item = NixToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            //Some(b'i') => Some(NixToken { token_type: NixTokenType::If }),
            Some(b'{') => Some(NixToken { token_type: NixTokenType::CurlyOpen }),
            Some(b'#') if self.line_start => Some(NixToken { token_type: NixTokenType::SingleLineComment }),
            None => None,
            _ => todo!()
        }
    }
}