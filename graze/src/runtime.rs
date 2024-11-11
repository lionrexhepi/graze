use std::collections::HashMap;

use smol_str::SmolStr;
use thiserror::Error;

use crate::{
    ast::{Argument, ExpressionContent, Instruction, Literal, Program},
    stdlib::{self, Point, Scalar, Vector},
};

pub struct Runtime {
    stack: Stack,
    variables: HashMap<SmolStr, Value>,
    functions: HashMap<SmolStr, Function>,
    draw: Vec<DrawCommand>,
}

impl Default for Runtime {
    fn default() -> Self {
        let mut runtime = Self {
            stack: Stack::default(),
            variables: HashMap::default(),
            functions: HashMap::default(),
            draw: Vec::default(),
        };

        stdlib::register(&mut runtime);

        runtime
    }
}

impl Runtime {
    pub fn define_fn(&mut self, name: &str, function: Function) {
        self.functions.insert(SmolStr::new(name), function);
    }

    pub fn execute(&mut self, program: Program) -> Result<(), Error> {
        for instruction in program.instructions {
            self.execute_instruction(instruction)?;
        }
        Ok(())
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), Error> {
        for expression in instruction.expressions {
            let value = self.execute_expression(expression.content)?;
            if expression.draw_result {
                self.draw.push(DrawCommand::new(value))
            }
            self.stack.push(value);
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

    pub fn finish(self) -> Vec<DrawCommand> {
        self.draw
    }
}

#[derive(Default)]
pub struct Stack {
    stack: Vec<Value>,
}

impl Stack {
    fn push(&mut self, value: Value) {
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

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Void,
    Scalar(Scalar),
    Point(Point),
    Vector(Vector),
    Line(Point, Vector),
}

type Function = fn(&mut Stack) -> Result<Value, Error>;

pub enum DrawCommand {
    Line(Point, Vector),
    Circle(Point, f64),
}

impl DrawCommand {
    fn new(value: Value) -> Self {
        match value {
            Value::Line(point, vector) => DrawCommand::Line(point, vector),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Error)]
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
