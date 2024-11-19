use smol_str::SmolStr;
use thiserror::Error;

use crate::token::{self, Keyword, Number, Payload, Position, Token, TokenSource};

#[derive(Debug, Default)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Default)]
pub struct Instruction {
    pub expressions: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub content: ExpressionContent,
    pub draw_result: bool,
    pub position: Position,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionContent {
    Literal(Literal),
    Variable(SmolStr),
    FunctionCall {
        name: SmolStr,
        args: Vec<Argument>,
    },
    Let {
        name: SmolStr,
        init: Option<Argument>,
    },
    Screen(Argument, Argument),
}

#[derive(Debug, PartialEq)]
pub enum Argument {
    Variable(SmolStr),
    Literal(Literal),
    Parenthesized(Box<ExpressionContent>),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(Number),
}

#[derive(Debug, Error)]
#[error("Error parsing file at {at}: {kind}")]
pub struct Error {
    at: Position,
    kind: ErrorKind,
}

impl Error {
    pub fn new(at: Position, kind: ErrorKind) -> Self {
        Self { at, kind }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum ErrorKind {
    #[error("Error parsing token: {0}")]
    InvalidToken(#[from] token::ErrorKind),
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Payload),
    #[error("Expected an expression")]
    ExpectedExpression,
    #[error("Expected closing delimiter")]
    UnclosedDelimiter,
    #[error("Expected identifier")]
    ExpectedIdentifier,
}

impl From<token::Error> for Error {
    fn from(value: token::Error) -> Self {
        Self {
            at: value.at,
            kind: value.kind.into(),
        }
    }
}

pub fn parse_file<S>(source: &mut S) -> Result<Program, Error>
where
    S: TokenSource,
{
    let mut program = Program::default();
    while let Some(instruction) = parse_instruction(source)? {
        program.instructions.push(instruction);
    }
    Ok(program)
}

fn parse_instruction<S>(source: &mut S) -> Result<Option<Instruction>, Error>
where
    S: TokenSource,
{
    let mut result = Instruction::default();

    loop {
        let position = source.position();
        let Some(content) = parse_expr(source)? else {
            break;
        };
        let Token {
            payload: join,
            position: end,
        } = source.read_token()?;
        let draw_result = match join {
            Payload::Pipe => false,
            Payload::Concat | Payload::Newline | Payload::Eof => true,
            other => return Err(Error::new(end, ErrorKind::UnexpectedToken(other))),
        };

        result.expressions.push(Expression {
            content,
            draw_result,
            position,
        });

        if let Payload::Newline | Payload::Eof = join {
            break;
        }
    }

    if result.expressions.is_empty() {
        if source.peek_token()?.payload == Payload::Eof {
            Ok(None)
        } else {
            // Empty line, parse the next one
            parse_instruction(source)
        }
    } else {
        Ok(Some(result))
    }
}

fn parse_expr<S>(source: &mut S) -> Result<Option<ExpressionContent>, Error>
where
    S: TokenSource,
{
    let Token { payload, position } = source.read_token()?;

    let content = match payload {
        Payload::LitNumber(number) => ExpressionContent::Literal(Literal::Number(number)),
        Payload::Variable(name) => ExpressionContent::Variable(name),
        Payload::Name(name) => {
            println!("name: {name}");
            let mut args = vec![];
            while let Some(arg) = parse_arg(source)? {
                args.push(arg);
            }
            ExpressionContent::FunctionCall { name, args }
        }
        Payload::Keyword(Keyword::Let) => {
            let Token { payload, position } = source.read_token()?;

            let Payload::Name(name) = payload else {
                return Err(Error::new(position, ErrorKind::ExpectedIdentifier));
            };

            let init = parse_arg(source)?;

            ExpressionContent::Let { name, init }
        }
        Payload::Keyword(Keyword::Screen) => {
            let x = parse_arg(source)
                .and_then(|x| x.ok_or(Error::new(position, ErrorKind::ExpectedExpression)))?;
            let y = parse_arg(source)
                .and_then(|y| y.ok_or(Error::new(position, ErrorKind::ExpectedExpression)))?;

            ExpressionContent::Screen(x, y)
        }
        Payload::Newline | Payload::Eof => return Ok(None),
        other => return Err(Error::new(position, ErrorKind::UnexpectedToken(other))),
    };

    Ok(Some(content))
}

fn parse_arg<S>(source: &mut S) -> Result<Option<Argument>, Error>
where
    S: TokenSource,
{
    let start = source.peek_token()?;
    let arg = match start.payload {
        Payload::Variable(name) => Argument::Variable(name),
        Payload::LitNumber(number) => Argument::Literal(Literal::Number(number)),
        Payload::ParenL => {
            source.read_token().expect(
                "Did not expect error reading token when peeking that same token worked fine",
            );
            let Some(expr) = parse_expr(source)? else {
                return Err(Error::new(start.position, ErrorKind::ExpectedExpression));
            };
            let next = source.peek_token()?;
            let Payload::ParenR = next.payload else {
                return Err(Error::new(start.position, ErrorKind::UnclosedDelimiter));
            };
            Argument::Parenthesized(Box::new(expr))
        }
        _ => return Ok(None),
    };
    source
        .read_token()
        .expect("Did not expect error reading token when peeking that same token worked fine");

    Ok(Some(arg))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::{Payload, StringTokenizer as StringTokenSource};

    #[test]
    fn test_parse_literal_number() {
        let input = "42";
        let mut source = StringTokenSource::new(&input);
        let result = parse_expr(&mut source).unwrap();
        assert_eq!(
            result,
            Some(ExpressionContent::Literal(Literal::Number(
                Number::Integer(42)
            )))
        );
    }

    #[test]
    fn test_parse_variable() {
        let input = "$x";
        let mut source = StringTokenSource::new(&input);
        let result = parse_expr(&mut source).unwrap();
        assert_eq!(result, Some(ExpressionContent::Variable(SmolStr::new("x"))));
    }

    #[test]
    fn test_parse_function_call() {
        let input = "foo 42 $x";
        let mut source = StringTokenSource::new(&input);
        let result = parse_expr(&mut source).unwrap();
        assert_eq!(
            result,
            Some(ExpressionContent::FunctionCall {
                name: SmolStr::new("foo"),
                args: vec![
                    Argument::Literal(Literal::Number(Number::Integer(42))),
                    Argument::Variable(SmolStr::new("x")),
                ],
            })
        );
    }

    #[test]
    fn test_parse_let_statement() {
        let input = "#let x 42";
        let mut source = StringTokenSource::new(&input);
        let result = parse_expr(&mut source).unwrap();
        assert_eq!(
            result,
            Some(ExpressionContent::Let {
                name: SmolStr::new("x"),
                init: Some(Argument::Literal(Literal::Number(Number::Integer(42)))),
            })
        );
    }

    #[test]
    fn test_parse_parenthesized_expression() {
        let input = "(42)";
        let mut source = StringTokenSource::new(&input);
        let result = parse_arg(&mut source).unwrap();
        assert_eq!(
            result,
            Some(Argument::Parenthesized(Box::new(
                ExpressionContent::Literal(Literal::Number(Number::Integer(42)))
            )))
        );
    }

    #[test]
    fn test_parse_instruction() {
        let input = "42 => #let x";
        let mut source = StringTokenSource::new(&input);
        let result = parse_instruction(&mut source).unwrap();
        assert!(result.is_some());
        let instruction = result.unwrap();
        assert_eq!(instruction.expressions.len(), 2);
        assert_eq!(
            instruction.expressions[0].content,
            ExpressionContent::Literal(Literal::Number(Number::Integer(42)))
        );
        assert!(!instruction.expressions[0].draw_result);
        assert_eq!(
            instruction.expressions[1].content,
            ExpressionContent::Let {
                name: "x".into(),
                init: None
            }
        );
    }

    #[test]
    fn test_parse_file() {
        let input = "42 ; print \nfoo 42 $x\n#let y 42";
        let mut source = StringTokenSource::new(&input);
        let result = parse_file(&mut source).unwrap();
        assert_eq!(result.instructions.len(), 3);
    }

    #[test]
    fn test_unexpected_token_error() {
        let input = "42 @";
        let mut source = StringTokenSource::new(&input);
        let result = parse_instruction(&mut source);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error.kind,
            ErrorKind::UnexpectedToken(Payload::Name(SmolStr::new("@")))
        );
    }

    #[test]
    fn test_expected_identifier_error() {
        let input = "#let 42";
        let mut source = StringTokenSource::new(&input);
        let result = parse_expr(&mut source);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind, ErrorKind::ExpectedIdentifier);
    }

    #[test]
    fn test_unclosed_delimiter_error() {
        let input = "(42";
        let mut source = StringTokenSource::new(&input);
        let result = parse_arg(&mut source);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind, ErrorKind::UnclosedDelimiter);
    }
}
