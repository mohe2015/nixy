
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
    pub location: SourceLocation,
}


pub struct NixLexer<'a>(pub &'a [u8]);

impl<'a> Iterator for NixLexer<'a> {
    type Item = NixToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {

        
        todo!()
    }
}