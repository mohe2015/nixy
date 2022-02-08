use core::fmt;
use std::{
    iter::{Enumerate, Peekable},
    slice::Iter,
    vec, fmt::Display,
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
    QuestionMark,
}

impl<'a> fmt::Debug for NixTokenType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(arg0) => f.debug_tuple("Identifier").field(&std::str::from_utf8(arg0).unwrap().to_owned()).finish(),
            Self::Integer(arg0) => f.debug_tuple("Integer").field(arg0).finish(),
            Self::PathStart => write!(f, "PathStart"),
            Self::PathSegment(arg0) => f.debug_tuple("PathSegment").field(&std::str::from_utf8(arg0).unwrap().to_owned()).finish(),
            Self::PathEnd => write!(f, "PathEnd"),
            Self::StringStart => write!(f, "StringStart"),
            Self::String(arg0) => f.debug_tuple("String").field(&std::str::from_utf8(arg0).unwrap().to_owned()).finish(),
            Self::StringEnd => write!(f, "StringEnd"),
            Self::IndentedStringStart => write!(f, "IndentedStringStart"),
            Self::IndentedString(arg0) => f.debug_tuple("IndentedString").field(&std::str::from_utf8(arg0).unwrap().to_owned()).finish(),
            Self::IndentedStringEnd => write!(f, "IndentedStringEnd"),
            Self::InterpolateStart => write!(f, "InterpolateStart"),
            Self::InterpolateEnd => write!(f, "InterpolateEnd"),
            Self::CurlyOpen => write!(f, "CurlyOpen"),
            Self::CurlyClose => write!(f, "CurlyClose"),
            Self::ParenOpen => write!(f, "ParenOpen"),
            Self::ParenClose => write!(f, "ParenClose"),
            Self::BracketOpen => write!(f, "BracketOpen"),
            Self::BracketClose => write!(f, "BracketClose"),
            Self::Whitespace => write!(f, "Whitespace"),
            Self::SingleLineComment => write!(f, "SingleLineComment"),
            Self::MultiLineComment => write!(f, "MultiLineComment"),
            Self::If => write!(f, "If"),
            Self::Then => write!(f, "Then"),
            Self::Else => write!(f, "Else"),
            Self::Assert => write!(f, "Assert"),
            Self::With => write!(f, "With"),
            Self::Let => write!(f, "Let"),
            Self::In => write!(f, "In"),
            Self::Rec => write!(f, "Rec"),
            Self::Inherit => write!(f, "Inherit"),
            Self::Or => write!(f, "Or"),
            Self::Ellipsis => write!(f, "Ellipsis"),
            Self::Equals => write!(f, "Equals"),
            Self::NotEquals => write!(f, "NotEquals"),
            Self::LessThanOrEqual => write!(f, "LessThanOrEqual"),
            Self::GreaterThanOrEqual => write!(f, "GreaterThanOrEqual"),
            Self::And => write!(f, "And"),
            Self::Implies => write!(f, "Implies"),
            Self::Update => write!(f, "Update"),
            Self::Concatenate => write!(f, "Concatenate"),
            Self::Assign => write!(f, "Assign"),
            Self::Semicolon => write!(f, "Semicolon"),
            Self::Colon => write!(f, "Colon"),
            Self::Select => write!(f, "Select"),
            Self::Comma => write!(f, "Comma"),
            Self::AtSign => write!(f, "AtSign"),
            Self::QuestionMark => write!(f, "QuestionMark"),
        }
    }
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
                Some((_offset, b'?')) => Some(NixToken {
                    token_type: NixTokenType::QuestionMark,
                }),
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
                            Some(NixToken {
                                token_type: NixTokenType::PathStart,
                            })
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
                    Some(NixToken {
                        token_type: NixTokenType::StringStart,
                    })
                }
                Some((offset, b'#')) if self.line_start => {
                    let end = self.iter.find(|(_, c)| **c == b'\n');
                    let comment = &self.data[offset..=end.map(|v| v.0).unwrap_or(usize::MAX)];
                    //println!("{:?}", std::str::from_utf8(comment));
                    Some(NixToken {
                        token_type: NixTokenType::SingleLineComment,
                    })
                }
                Some((offset, b' '))
                | Some((offset, b'\t'))
                | Some((offset, b'\r'))
                | Some((offset, b'\n')) => {
                    let mut end = offset;
                    while let Some((_, b' ')) | Some((_, b'\t')) | Some((_, b'\r'))
                    | Some((_, b'\n')) = self.iter.peek()
                    {
                        end = self.iter.next().unwrap().0;
                    }
                    let whitespace = &self.data[offset..end];
                    //println!("{:?}", std::str::from_utf8(whitespace));
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
                    //println!("{:?}", std::str::from_utf8(identifier));
                    Some(NixToken {
                        token_type: NixTokenType::Identifier(identifier),
                    })
                }
                Some((offset, b'0'..=b'9')) => {
                    loop {
                        match self.iter.peek() {
                            Some((_, b'0'..=b'9')) => {
                                self.iter.next();
                            }
                            _ => break,
                        }
                    }
                    let integer =
                        std::str::from_utf8(&self.data[offset..self.iter.peek().unwrap().0])
                            .unwrap();
                    //println!("{:?}", integer);
                    Some(NixToken {
                        token_type: NixTokenType::Integer(integer.parse().unwrap()),
                    })
                }
                None => None,
                _ => todo!(),
            },
            Some(NixLexerState::String) => {
                let start = self.iter.peek().unwrap().0; // TODO FIXME throw proper parse error (maybe error token)

                if self.data[start] == b'"' {
                    self.iter.next();

                    self.state.pop();
                    return Some(NixToken {
                        token_type: NixTokenType::StringEnd,
                    });
                }

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
                        let string = &self.data[start..current];
                        //println!("{:?}", std::str::from_utf8(string));
                        break Some(NixToken {
                            token_type: NixTokenType::String(string),
                        });
                    }
                    if self.data[current..].starts_with(b"${") {
                        self.iter.next().unwrap();
                        self.iter.next().unwrap();

                        self.state.push(NixLexerState::Default);

                        let string = &self.data[start..current];
                        //println!("{:?}", std::str::from_utf8(string));
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
                            if start == *offset {
                                self.state.pop();
                                break Some(NixToken {
                                    token_type: NixTokenType::PathEnd,
                                });
                            } else {
                                let path = &self.data[start - 1..*offset];
                                //println!("{:?}", std::str::from_utf8(path));
                                break Some(NixToken {
                                    token_type: NixTokenType::PathSegment(path),
                                });
                            }
                        }
                        _ => todo!(),
                    }
                }
            }
            None => todo!(),
        }
    }
}
