use std::{iter::Peekable, str::Chars};

use smol_str::{SmolStr, SmolStrBuilder};
use thiserror::Error;

pub trait TokenSource {
    fn read_token(&mut self) -> Result<Token, Error>;
    fn peek_token(&self) -> Result<Token, Error>;
    fn position(&self) -> Position;
}

pub struct Token {
    pub payload: Payload,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Payload {
    /// Unqualified name
    Name(SmolStr),
    /// Name prefixed with $ for variable access
    Variable(SmolStr),
    /// Number literal
    LitNumber(Number),
    /// "let"
    Let,
    /// =>
    Pipe,
    /// ;
    Concat,
    /// (
    ParenL,
    /// )
    ParenR,
    /// A newline.
    Newline,
    /// A bang (!) followed by a newline.
    VoidNewline,
    EOF,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Integer(u64),
    Float(f64),
}

#[derive(Debug, Clone, Copy, Error)]
#[error("Error tokenizing input at {at}: {kind:?}")]
pub struct Error {
    pub kind: ErrorKind,
    pub at: Position,
}

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum ErrorKind {
    #[error("Invalid CLRF sequence")]
    InvalidCRLFSequence,
    #[error("Expected a variable name")]
    EmptyVariableName,
    #[error("Expected a function name")]
    EmptyFunctionName,
    #[error("Invalid literal")]
    InvalidLiteral,
    #[error("An equals sign '=' must be followed by a '>' to make a pipe.")]
    InvalidPipe,
    #[error("Expected a newline after a '!' to make it a 'void' token.")]
    ExpectedNewlineAfterBang,
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

#[derive(Clone)]
pub struct StringTokenizer<'s> {
    chars: Peekable<Chars<'s>>,
    position: Position,
}

impl<'s> StringTokenizer<'s> {
    #[must_use]
    pub fn new(slice: &'s impl AsRef<str>) -> Self {
        Self {
            chars: slice.as_ref().chars().peekable(),
            position: Default::default(),
        }
    }

    /// Consume characters while the condition is true.
    /// Use this instead of `Iterator::take_while` because it
    /// doesn't consume the next character. By contrast,
    fn take_while(&mut self, mut condition: impl FnMut(&char) -> bool) -> SmolStr {
        let mut result = SmolStrBuilder::new();
        while let Some(next) = self.chars.peek().copied() {
            if condition(&next) {
                result.push(next);
                self.advance();
            } else {
                break;
            }
        }
        result.finish()
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.chars.next();

        match next {
            Some('\n') => {
                self.position.line += 1;
                self.position.column = 0;
            }
            Some(_) => {
                self.position.column += 1;
            }
            None => {}
        }

        next
    }

    fn parse_name(&mut self) -> Option<SmolStr> {
        let mut first = true;
        let name = self.take_while(|c| {
            let valid =
                !matches!(c, ';' | '(' | ')' | '\'' | '$' | '=' | '!') && !c.is_whitespace();
            if first {
                first = false;
                valid && !c.is_numeric()
            } else {
                valid
            }
        });

        if name.is_empty() {
            None
        } else {
            Some(name)
        }
    }

    fn parse_integer(&mut self) -> Option<SmolStr> {
        let digits = self.take_while(char::is_ascii_digit);

        if digits.is_empty() {
            None
        } else {
            Some(digits)
        }
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
            let Some(next) = self.current() else {
                return Ok(self.token(Payload::EOF));
            };
            let single = match next {
                '\n' => Payload::Newline,
                '\r' => {
                    self.advance();
                    let Some('\n') = self.advance() else {
                        return Err(self.error(ErrorKind::InvalidCRLFSequence));
                    };
                    Payload::Newline
                }
                '!' => {
                    self.advance();
                    let Some('\n') = self.advance() else {
                        return Err(self.error(ErrorKind::ExpectedNewlineAfterBang));
                    };
                    Payload::VoidNewline
                }
                ';' => Payload::Concat,
                '=' => {
                    self.advance();
                    let Some('>') = self.advance() else {
                        return Err(self.error(ErrorKind::InvalidPipe));
                    };
                    Payload::Pipe
                }
                '(' => Payload::ParenL,
                ')' => Payload::ParenR,

                other => {
                    if other.is_whitespace() {
                        self.advance();
                        continue;
                    } else {
                        break other;
                    }
                }
            };

            self.advance();
            return Ok(self.token(single));
        };

        if first == '$' {
            self.advance();
            self.parse_name()
                .map(Payload::Variable)
                .map(|var| self.token(var))
                .ok_or_else(|| self.error(ErrorKind::EmptyVariableName))
        } else if first.is_ascii_digit() {
            let lit = self
                .parse_integer()
                .expect("At least 1 digit is confirmed available");

            let Ok(value) = lit.parse::<u64>() else {
                return Err(self.error(ErrorKind::InvalidLiteral));
            };

            Ok(self.token(Payload::LitNumber(Number::Integer(value))))
        } else {
            self.parse_name()
                .map(|func| {
                    let payload = if func == "let" {
                        Payload::Let
                    } else {
                        Payload::Name(func)
                    };
                    self.token(payload)
                })
                .ok_or_else(|| self.error(ErrorKind::EmptyFunctionName))
        }
    }

    fn peek_token(&self) -> Result<Token, Error> {
        let mut copy = self.clone();
        copy.read_token()
    }

    fn position(&self) -> Position {
        self.position
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_payload {
        ($tokenizer:ident equals $payload:expr) => {
            assert_eq!($tokenizer.read_token().unwrap().payload, $payload)
        };
        ($tokenizer:ident matches $payload:pat) => {
            assert!(matches!($tokenizer.read_token().unwrap().payload, $payload))
        };
    }

    #[test]
    fn test_correct_source_positions() {
        let input = "func1\nfunc2";
        let mut tokenizer = StringTokenizer::new(&input);

        let token1 = tokenizer.read_token().unwrap();
        assert_eq!(token1.position, Position { line: 0, column: 5 });

        let token2 = tokenizer.read_token().unwrap();
        assert_eq!(token2.position, Position { line: 1, column: 0 });

        let token3 = tokenizer.read_token().unwrap();
        assert_eq!(token3.position, Position { line: 1, column: 5 });
    }

    #[test]
    fn test_correct_identification_of_tokens() {
        let input = "func1 $var1 123 ; ( ) =>";
        let mut tokenizer = StringTokenizer::new(&input);

        assert_payload!(tokenizer matches Payload::Name(_));
        assert_payload!(tokenizer matches Payload::Variable(_));
        assert_payload!(tokenizer matches Payload::LitNumber(_));
        assert_payload!(tokenizer matches Payload::Concat);
        assert_payload!(tokenizer matches Payload::ParenL);
        assert_payload!(tokenizer matches Payload::ParenR);
        assert_payload!(tokenizer matches Payload::Pipe);
        assert_payload!(tokenizer matches Payload::EOF);
    }

    #[test]
    fn test_edge_cases_for_number_literals() {
        let input = "0 12345678901234567890";
        let mut tokenizer = StringTokenizer::new(&input);

        assert_payload!(tokenizer equals Payload::LitNumber(Number::Integer(0)));
        assert_payload!(tokenizer equals Payload::LitNumber(Number::Integer(12345678901234567890)));
    }

    #[test]
    fn test_newlines() {
        let input = "func1\r\n   $var1\n  123";
        let mut tokenizer = StringTokenizer::new(&input);

        assert_payload!(tokenizer equals Payload::Name("func1".into()));
        assert_payload!(tokenizer equals Payload::Newline);
        assert_payload!(tokenizer equals Payload::Variable("var1".into()));
        assert_payload!(tokenizer equals Payload::Newline);
        assert_payload!(tokenizer equals Payload::LitNumber(Number::Integer(123)));
    }
}
