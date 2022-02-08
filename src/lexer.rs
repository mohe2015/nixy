use core::fmt;
use std::{
    fmt::Display,
    iter::{Enumerate, Peekable},
    slice::Iter,
    vec,
};

// https://wduquette.github.io/parsing-strings-into-slices/
// https://github.com/NixOS/nix/blob/master/src/libexpr/lexer.l
// https://learnxinyminutes.com/docs/nix/
// https://nixos.org/manual/nix/stable/expressions/language-values.html

// cant convert let to lambda because it can refer to itself?
// their order does not matter? (maybe we could ignore that)
/*
 (let y = x + "b";
       x = "a"; in
    y + "c")
*/
// our lambdas always have 1 parameter (currying)?
/*
This kind of string literal intelligently strips indentation from the start of each line. To be precise, it strips from each line a number of spaces equal to the minimal indentation of the string as a whole (disregarding the indentation of empty lines). For instance, the first and second line are indented two spaces, while the third line is indented four spaces. Thus, two spaces are stripped from each line, so the resulting string is

Note that the whitespace and newline following the opening '' is ignored if there is no non-whitespace text on the initial line.
*/

// another idiotic syntax instead of "${bar}"
// { foo = 123; }.${bar} or 456

// and another one
// { ${if foo then "bar" else null} = true; }

/*
A set that has a __functor attribute whose value is callable (i.e. is itself a function or a set with a __functor attribute whose value is callable) can be applied as if it were a function, with the set itself passed in first , e.g.,

let add = { __functor = self: x: x + self.x; };
    inc = add // { x = 1; };
in inc 1
*/

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
    Whitespace(&'a [u8]),
    SingleLineComment(&'a [u8]),
    MultiLineComment(&'a [u8]),
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
    ExclamationMark,
    Addition,
}

impl<'a> fmt::Debug for NixTokenType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(arg0) => f
                .debug_tuple("Identifier")
                .field(&std::str::from_utf8(arg0).unwrap().to_owned())
                .finish(),
            Self::Integer(arg0) => f.debug_tuple("Integer").field(arg0).finish(),
            Self::PathStart => write!(f, "PathStart"),
            Self::PathSegment(arg0) => f
                .debug_tuple("PathSegment")
                .field(&std::str::from_utf8(arg0).unwrap().to_owned())
                .finish(),
            Self::PathEnd => write!(f, "PathEnd"),
            Self::StringStart => write!(f, "StringStart"),
            Self::String(arg0) => f
                .debug_tuple("String")
                .field(&std::str::from_utf8(arg0).unwrap().to_owned())
                .finish(),
            Self::StringEnd => write!(f, "StringEnd"),
            Self::IndentedStringStart => write!(f, "IndentedStringStart"),
            Self::IndentedString(arg0) => f
                .debug_tuple("IndentedString")
                .field(&std::str::from_utf8(arg0).unwrap().to_owned())
                .finish(),
            Self::IndentedStringEnd => write!(f, "IndentedStringEnd"),
            Self::InterpolateStart => write!(f, "InterpolateStart"),
            Self::InterpolateEnd => write!(f, "InterpolateEnd"),
            Self::CurlyOpen => write!(f, "CurlyOpen"),
            Self::CurlyClose => write!(f, "CurlyClose"),
            Self::ParenOpen => write!(f, "ParenOpen"),
            Self::ParenClose => write!(f, "ParenClose"),
            Self::Addition => write!(f, "Addition"),
            Self::BracketOpen => write!(f, "BracketOpen"),
            Self::BracketClose => write!(f, "BracketClose"),
            Self::Whitespace(arg0) => f
                .debug_tuple("Whitespace")
                .field(&std::str::from_utf8(arg0).unwrap().to_owned())
                .finish(),
            Self::SingleLineComment(arg0) => f
                .debug_tuple("SingleLineComment")
                .field(&std::str::from_utf8(arg0).unwrap().to_owned())
                .finish(),
            Self::MultiLineComment(arg0) => f
                .debug_tuple("MultiLineComment")
                .field(&std::str::from_utf8(arg0).unwrap().to_owned())
                .finish(),
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
            Self::ExclamationMark => write!(f, "ExclamationMark"),
        }
    }
}

#[derive(Debug)]
pub struct NixToken<'a> {
    pub token_type: NixTokenType<'a>,
    //pub location: SourceLocation,
}

#[derive(PartialEq)]
enum NixLexerState {
    Default,
    String,
    IndentedString,
    Path,
    SearchPath,
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
                Some((_offset, b'!')) => Some(NixToken {
                    token_type: NixTokenType::ExclamationMark,
                }),
                Some((offset, b'/')) => match self.iter.next() {
                    Some((_, b'/')) => Some(NixToken {
                        token_type: NixTokenType::Update,
                    }),
                    Some((_, b'*')) => {
                        loop {
                            let current = self.iter.next().unwrap().0;

                            if self.data[current..].starts_with(b"*/") {
                                self.iter.next();
                                let string = &self.data[offset..current];
                                //println!("{:?}", std::str::from_utf8(string));
                                break Some(NixToken {
                                    token_type: NixTokenType::MultiLineComment(string),
                                });
                            }
                        }
                    }
                    _ => {
                        panic!("{}", std::str::from_utf8(&self.data[offset..]).unwrap());
                    }
                },
                Some((_offset, b'|')) => match self.iter.next() {
                    Some((_, b'|')) => Some(NixToken {
                        token_type: NixTokenType::Or,
                    }),
                    _ => todo!(),
                },
                Some((_offset, b'-')) => match self.iter.next() {
                    Some((_, b'>')) => Some(NixToken {
                        token_type: NixTokenType::Implies,
                    }),
                    _ => todo!(),
                },
                Some((_offset, b'&')) => match self.iter.next() {
                    Some((_, b'&')) => Some(NixToken {
                        token_type: NixTokenType::Or,
                    }),
                    _ => todo!(),
                },
                Some((_offset, b'$')) => match self.iter.next() {
                    Some((_, b'{')) => {
                        self.state.push(NixLexerState::Default);
                        Some(NixToken {
                            token_type: NixTokenType::InterpolateStart,
                        })
                    }
                    _ => todo!(),
                },
                Some((_offset, b'+')) => match self.iter.peek() {
                    Some((_, b'+')) => {
                        self.iter.next();
                        Some(NixToken {
                            token_type: NixTokenType::Concatenate,
                        })
                    }
                    _ => Some(NixToken {
                        token_type: NixTokenType::Addition,
                    }),
                },
                Some((_offset, b'\'')) => match self.iter.next() {
                    Some((_, b'\'')) => {
                        self.state.push(NixLexerState::IndentedString);
                        Some(NixToken {
                            token_type: NixTokenType::IndentedStringStart,
                        })
                    }
                    _ => todo!(),
                },
                Some((offset, b'<')) => {
                    match self.iter.peek() {
                        Some((_, b'a'..=b'z'))
                        | Some((_, b'A'..=b'Z'))
                        | Some((_, b'0'..=b'9'))
                        | Some((_, b'.'))
                        | Some((_, b'_'))
                        | Some((_, b'-'))
                        | Some((_, b'+'))
                        | Some((_, b'/')) => {
                            self.state.push(NixLexerState::SearchPath);
                            // TODO FIXME SearchPath
                            Some(NixToken {
                                token_type: NixTokenType::PathStart,
                            })
                        }
                        _ => todo!(),
                    }
                }
                Some((_offset, b'.')) => {
                    // ./ for path
                    // ../ for path
                    // ... for ellipsis
                    // or select

                    // TODO FIXME absolute path

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
                                Some((_, b'/')) => {
                                    self.state.push(NixLexerState::Path);
                                    Some(NixToken {
                                        token_type: NixTokenType::PathStart,
                                    })
                                }
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
                        token_type: NixTokenType::SingleLineComment(comment),
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
                        token_type: NixTokenType::Whitespace(whitespace),
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
            state @ (Some(NixLexerState::String) | Some(NixLexerState::IndentedString)) => {
                let start = self.iter.peek().unwrap().0; // TODO FIXME throw proper parse error (maybe error token)

                if state == Some(&NixLexerState::String) && self.data[start] == b'"' {
                    self.iter.next();

                    self.state.pop();
                    return Some(NixToken {
                        token_type: NixTokenType::StringEnd,
                    });
                }

                // TODO FIXME escaped '''
                if state == Some(&NixLexerState::IndentedString)
                    && self.data[start..].starts_with(b"''")
                {
                    self.iter.next();
                    self.iter.next();

                    self.state.pop();
                    return Some(NixToken {
                        token_type: NixTokenType::IndentedStringEnd,
                    });
                }

                // TODO FIXME ''${
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
                    if state == Some(&NixLexerState::String) && self.data[current] == b'"' {
                        let string = &self.data[start..current];
                        //println!("{:?}", std::str::from_utf8(string));
                        break Some(NixToken {
                            token_type: NixTokenType::String(string),
                        });
                    }
                    if self.data[current..].starts_with(b"${") {
                        let string = &self.data[start..current];
                        //println!("{:?}", std::str::from_utf8(string));
                        break Some(NixToken {
                            token_type: NixTokenType::String(string),
                        });
                    }
                    if state == Some(&NixLexerState::IndentedString)
                        && self.data[current..].starts_with(b"''")
                    {
                        let string = &self.data[start..current];
                        //println!("{:?}", std::str::from_utf8(string));
                        break Some(NixToken {
                            token_type: NixTokenType::IndentedString(string),
                        });
                    }
                    self.iter.next();
                }
            }
            state @ (Some(NixLexerState::Path) | Some(NixLexerState::SearchPath)) => {
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
                        Some((offset, b'>')) if state == Some(&NixLexerState::SearchPath) => {
                            if start == *offset {
                                self.iter.next();
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
