use std::{iter::Peekable, path::Display, str::Chars};

use smol_str::SmolStr;
use thiserror::Error;

pub trait TokenSource {
    fn read_token(&mut self) -> Result<Token, Error>;
}

pub struct Token {
    payload: Payload,
    position: Position,
}

#[derive(Debug, Clone)]
pub enum Payload {
    Function(SmolStr),
    Variable(SmolStr),
    LitNumber(Number),
    Pipe,
    ParenL,
    ParenR,
    Newline,
    EOF,
}

#[derive(Debug, Clone, Copy)]
pub enum Number {
    Integer(u64),
    Float(f64),
}

#[derive(Debug, Clone, Copy, Error)]
#[error("Error tokenizing input at {at}: {kind:?}")]
pub struct Error {
    kind: ErrorKind,
    at: Position,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    InvalidLiteral,
    InvalidSymbol,
    /// \r without a \n after it
    InvalidCRLFSequence,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    line: usize,
    column: usize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.line, self.column)
    }
}

pub struct StringTokenizer<'s> {
    chars: Peekable<Chars<'s>>,
    slice: &'s str,
    position: Position,
}

impl<'s> StringTokenizer<'s> {
    #[must_use]
    pub fn new(content: &'s impl AsRef<str>) -> Self {
        let slice = content.as_ref();
        Self {
            chars: slice.chars().peekable(),
            slice,
            position: Default::default(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.chars.next();
        match next {
            Some('\n') => {
                self.position = Position {
                    line: self.position.line + 1,
                    column: 0,
                }
            }
            None => {}
            Some(_) => {
                self.position.column += 1;
            }
        }
        next
    }

    #[must_use]
    fn current(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    #[must_use]
    fn token(&self, payload: Payload) -> Token {
        Token {
            payload,
            position: self.position,
        }
    }

    #[must_use]
    fn error(&self, kind: ErrorKind) -> Error {
        Error {
            kind,
            at: self.position,
        }
    }
}

impl<'s> TokenSource for StringTokenizer<'s> {
    fn read_token(&mut self) -> Result<Token, Error> {
        let first = loop {
            let Some(next) = self.advance() else {
                return Ok(self.token(Payload::EOF));
            };
            match next {
                '\n' => return Ok(self.token(Payload::Newline)),
                '\r' => {
                    let Some('\n') = self.advance() else {
                        return Err(self.error(ErrorKind::InvalidCRLFSequence));
                    };
                    return Ok(self.token(Payload::Newline));
                }
                ';' => return Ok(self.token(Payload::Pipe)),
                '(' => return Ok(self.token(Payload::ParenL)),
                ')' => return Ok(self.token(Payload::ParenR)),
                other => {
                    if other.is_whitespace() {
                        continue;
                    } else {
                        break other;
                    }
                }
            }
        };

        if first == '$' {}

        todo!()
    }
}
