use std::{
    iter::{Enumerate, Peekable},
    slice::Iter,
    vec,
};

// https://wduquette.github.io/parsing-strings-into-slices/
// https://github.com/NixOS/nix/blob/master/src/libexpr/lexer.l

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
    PathSegment(&'a [u8]),
    PathEnd,
    StringStart,
    String(&'a [u8]),
    StringEnd,
    IndentedStringStart,
    IndentedString(&'a [u8]),
    IndentedStringEnd,
    InterpolateStart,
    InterpolateEnd,
    CurlyOpen,
    CurlyClose,
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
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
    Assign,
    Semicolon,
    Colon,
    Select,
    Comma,
    AtSign,
}

#[derive(Debug)]
pub struct NixToken<'a> {
    pub token_type: NixTokenType<'a>,
    //pub location: SourceLocation,
}

enum NixLexerState {
    Default,
    String,
    Path,
}

pub struct NixLexer<'a> {
    pub data: &'a [u8],
    pub iter: Peekable<Enumerate<Iter<'a, u8>>>,
    state: Vec<NixLexerState>,
    line_start: bool,
}

impl<'a> NixLexer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            iter: data.iter().enumerate().peekable(),
            state: vec![NixLexerState::Default],
            line_start: true,
        }
    }
}

impl<'a> Iterator for NixLexer<'a> {
    type Item = NixToken<'a>;

    // TODO keywords import let in
    fn next(&mut self) -> Option<Self::Item> {
        match self.state.last() {
            Some(NixLexerState::Default) => match self.iter.next() {
                Some((_offset, b'{')) => {
                    self.state.push(NixLexerState::Default);
                    Some(NixToken {
                        token_type: NixTokenType::CurlyOpen,
                    })
                }
                Some((_offset, b'}')) => {
                    self.state.pop();
                    Some(NixToken {
                        token_type: NixTokenType::CurlyClose,
                    })
                }
                Some((_offset, b'(')) => Some(NixToken {
                    token_type: NixTokenType::ParenOpen,
                }),
                Some((_offset, b')')) => Some(NixToken {
                    token_type: NixTokenType::ParenClose,
                }),
                Some((_offset, b'[')) => Some(NixToken {
                    token_type: NixTokenType::BracketOpen,
                }),
                Some((_offset, b']')) => Some(NixToken {
                    token_type: NixTokenType::BracketClose,
                }),
                Some((_offset, b':')) => Some(NixToken {
                    token_type: NixTokenType::Colon,
                }),
                Some((_offset, b'=')) => Some(NixToken {
                    token_type: NixTokenType::Assign,
                }),
                Some((_offset, b';')) => Some(NixToken {
                    token_type: NixTokenType::Semicolon,
                }),
                Some((_offset, b',')) => Some(NixToken {
                    token_type: NixTokenType::Comma,
                }),
                Some((_offset, b'@')) => Some(NixToken {
                    token_type: NixTokenType::AtSign,
                }),
                Some((_offset, b'/')) => match self.iter.next() {
                    Some((_, b'/')) => Some(NixToken {
                        token_type: NixTokenType::Update,
                    }),
                    _ => todo!(),
                },
                Some((_offset, b'+')) => match self.iter.next() {
                    Some((_, b'+')) => Some(NixToken {
                        token_type: NixTokenType::Concatenate,
                    }),
                    _ => todo!(),
                },
                Some((_offset, b'.')) => {
                    // ./ for path
                    // ... for ellipsis
                    // or select

                    match self.iter.peek() {
                        Some((_, b'/')) => {
                            self.state.push(NixLexerState::Path);
                            self.next()
                        }
                        Some((_, b'.')) => {
                            self.iter.next();
                            match self.iter.next() {
                                Some((_, b'.')) => Some(NixToken {
                                    token_type: NixTokenType::Ellipsis,
                                }),
                                _ => todo!(),
                            }
                        }
                        _ => Some(NixToken {
                            token_type: NixTokenType::Select,
                        }),
                    }
                }
                Some((_offset, b'"')) => {
                    self.state.push(NixLexerState::String);
                    self.next()
                }
                Some((offset, b'#')) if self.line_start => {
                    let end = self.iter.find(|(_, c)| **c == b'\n');
                    let comment = &self.data[offset..=end.map(|v| v.0).unwrap_or(usize::MAX)];
                    println!("{:?}", std::str::from_utf8(comment));
                    Some(NixToken {
                        token_type: NixTokenType::SingleLineComment,
                    })
                }
                Some((offset, b' '))
                | Some((offset, b'\t'))
                | Some((offset, b'\r'))
                | Some((offset, b'\n')) => {
                    while let Some((_, b' ')) | Some((_, b'\t')) | Some((_, b'\r'))
                    | Some((_, b'\n')) = self.iter.peek()
                    {
                        self.iter.next();
                    }
                    let whitespace = &self.data[offset..self.iter.peek().unwrap().0];
                    println!("{:?}", std::str::from_utf8(whitespace));
                    Some(NixToken {
                        token_type: NixTokenType::Whitespace,
                    })
                }
                // this can be literally anything (path, ..)
                Some((offset, b'a'..=b'z'))
                | Some((offset, b'A'..=b'Z'))
                | Some((offset, b'_')) => {
                    loop {
                        match self.iter.peek() {
                            Some((_, b'a'..=b'z'))
                            | Some((_, b'A'..=b'Z'))
                            | Some((_, b'0'..=b'9'))
                            | Some((_, b'_'))
                            | Some((_, b'\''))
                            | Some((_, b'-')) => {
                                self.iter.next();
                            }
                            _ => break,
                        }
                    }
                    let identifier = &self.data[offset..self.iter.peek().unwrap().0];
                    println!("{:?}", std::str::from_utf8(identifier));
                    Some(NixToken {
                        token_type: NixTokenType::Identifier(identifier),
                    })
                }
                None => None,
                _ => todo!(),
            },
            Some(NixLexerState::String) => {
                let start = self.iter.peek().unwrap().0; // TODO FIXME throw proper parse error (maybe error token)

                if self.data[start..].starts_with(b"${") {
                    self.iter.next();
                    self.iter.next();

                    self.state.push(NixLexerState::Default);
                    return Some(NixToken {
                        token_type: NixTokenType::InterpolateStart,
                    });
                }

                loop {
                    let current = self.iter.peek().unwrap().0;
                    if self.data[current] == b'"' {
                        let (offset, _) = self.iter.next().unwrap(); // TODO FIXME
                        self.state.pop();

                        let string = &self.data[start..offset];
                        println!("{:?}", std::str::from_utf8(string));
                        break Some(NixToken {
                            token_type: NixTokenType::String(string),
                        });
                    }
                    if self.data[current..].starts_with(b"${") {
                        let (offset, _) = self.iter.next().unwrap(); // TODO FIXME
                        self.iter.next().unwrap();

                        self.state.push(NixLexerState::Default);

                        let string = &self.data[start..current];
                        println!("{:?}", std::str::from_utf8(string));
                        break Some(NixToken {
                            token_type: NixTokenType::String(string),
                        });
                    }
                    self.iter.next();
                }
            }
            Some(NixLexerState::Path) => {
                let start = self.iter.peek().unwrap().0; // TODO FIXME throw proper parse error (maybe error token)

                // $ read
                // peek for {
                // if it is one? we need to revert to before it?
                // I think we need to do slice magic to peekahead of this or somehow get a two-ahead peeker.
                // rust should probably have a constant peekahead iterator

                loop {
                    match self.iter.peek() {
                        Some((_, b'a'..=b'z'))
                        | Some((_, b'A'..=b'Z'))
                        | Some((_, b'0'..=b'9'))
                        | Some((_, b'.'))
                        | Some((_, b'_'))
                        | Some((_, b'-'))
                        | Some((_, b'+'))
                        | Some((_, b'/')) => {
                            self.iter.next();
                        }
                        Some((offset, _)) => {
                            self.state.pop();

                            let path = &self.data[start - 1..*offset];
                            println!("{:?}", std::str::from_utf8(path));
                            break Some(NixToken {
                                token_type: NixTokenType::PathSegment(path),
                            });
                        }
                        _ => todo!(),
                    }
                }
            }
            None => todo!(),
        }
    }
}
