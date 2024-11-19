use std::collections::HashMap;

use smol_str::SmolStr;
use thiserror::Error;

use crate::{
    ast::{Argument, ExpressionContent, Instruction, Literal, Program},
    output::{DrawBuffer, DrawCommand},
    stdlib::{self, Point, Scalar, Vector},
};

pub struct Runtime<Backend> {
    stack: Stack,
    variables: HashMap<SmolStr, Value>,
    functions: HashMap<SmolStr, Function>,
    draw: Backend,
}

impl<Backend> Default for Runtime<Backend>
where
    Backend: DrawBuffer + Default,
{
    fn default() -> Self {
        let mut runtime = Self {
            stack: Stack::default(),
            variables: HashMap::default(),
            functions: HashMap::default(),
            draw: Backend::default(),
        };

        stdlib::register(&mut runtime);

        runtime
    }
}

impl<Backend> Runtime<Backend> {
    pub fn define_fn(&mut self, name: &str, function: Function) {
        self.functions.insert(SmolStr::new(name), function);
    }
}

impl<Backend> Runtime<Backend>
where
    Backend: DrawBuffer,
{
    pub fn execute(&mut self, program: Program) -> Result<(), Error> {
        for instruction in program.instructions {
            self.execute_instruction(instruction)?;
        }
        Ok(())
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), Error> {
        for expression in instruction.expressions {
            let value = self.execute_expression(expression.content)?;
            self.stack.push(value);
            if !expression.draw_result {
                continue;
            }

            if let Some(cmd) = value.into() {
                self.draw.draw(cmd);
            }
        }

        self.stack.clear();

        Ok(())
    }

    fn execute_expression(&mut self, expression: ExpressionContent) -> Result<Value, Error> {
        match expression {
            ExpressionContent::Literal(literal) => {
                let value = match literal {
                    Literal::Number(number) => Value::Scalar(number.try_into()?),
                };
                Ok(value)
            }
            ExpressionContent::Variable(name) => self
                .variables
                .get(&name)
                .copied()
                .ok_or(Error::VariableNotFound(name)),
            ExpressionContent::FunctionCall { name, args } => {
                for arg in args {
                    let value = self.execute_argument(arg)?;
                    self.stack.push(value);
                }

                let function = self
                    .functions
                    .get(&name)
                    .ok_or(Error::FunctionNotFound(name))?;

                function(&mut self.stack)
            }
            ExpressionContent::Let { name, init } => {
                let value = if let Some(init) = init {
                    self.execute_argument(init)?
                } else {
                    self.stack.pop()?
                };
                self.variables.insert(name, value);
                Ok(value)
            }
            ExpressionContent::Screen(argument, argument1) => {
                let (Value::Scalar(x), Value::Scalar(y)) = (
                    self.execute_argument(argument)?,
                    self.execute_argument(argument1)?,
                ) else {
                    return Err(Error::InvalidArgument);
                };

                self.draw.draw(DrawCommand::Resize {
                    x: x.into(),
                    y: y.into(),
                });

                Ok(Value::Void)
            }
        }
    }

    fn execute_argument(&mut self, argument: Argument) -> Result<Value, Error> {
        match argument {
            Argument::Variable(name) => self
                .variables
                .get(&name)
                .copied()
                .ok_or(Error::VariableNotFound(name)),
            Argument::Literal(literal) => match literal {
                Literal::Number(number) => Ok(Value::Scalar(number.try_into()?)),
            },
            Argument::Parenthesized(content) => self.execute_expression(*content),
        }
    }

    pub fn finish(mut self) {
        self.draw.flush()
    }
}

#[derive(Default)]
pub struct Stack {
    stack: Vec<Value>,
}

impl Stack {
    pub fn push(&mut self, value: Value) {
        if let Value::Void = value {
            return;
        }
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn clear(&mut self) {
        self.stack.clear();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Void,
    Scalar(Scalar),
    Point(Point),
    Vector(Vector),
    Line(Point, Vector),
}

type Function = fn(&mut Stack) -> Result<Value, Error>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Fatal: stack underflow")]
    StackUnderflow,
    #[error("Invalid argument")]
    InvalidArgument,
    #[error("Variable {0} not in scope")]
    VariableNotFound(SmolStr),
    #[error("Function {0} not in scope")]
    FunctionNotFound(SmolStr),
    #[error("Invalid type for operation")]
    TypeError,
    #[error("Integer literal too large to fit in a 64-bit integer")]
    IntLiteralTooLarge,
    #[error("Too few arguments for this function call")]
    MissingArgument,
    #[error("Non-real result")]
    NonRealResult,
}
