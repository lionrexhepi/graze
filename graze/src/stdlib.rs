mod point;
mod scalar;
mod vector;

pub use point::*;
pub use scalar::*;
pub use vector::*;

use crate::runtime::{Error, Stack, Value};

#[macro_export]
macro_rules! reverse_pop {
     ($stack:ident => $arg:ident) => {
         let $arg = $stack.pop()?;
     };
     ($stack:ident => $arg:ident, $($args:ident),*) => {
         reverse_pop!($stack => $($args),*);
         reverse_pop!($stack => $arg);
     };
 }

pub fn add(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => a, b);
    match (a, b) {
        (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a + b)),
        (Value::Vector(a), Value::Vector(b)) => Ok(Value::Vector(a + b)),
        (Value::Vector(vec), Value::Point(pnt)) | (Value::Point(pnt), Value::Vector(vec)) => {
            Ok(Value::Point(pnt + vec))
        }
        _ => Err(Error::InvalidType),
    }
}

pub fn sub(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => a, b);
    match (a, b) {
        (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a - b)),
        (Value::Vector(a), Value::Vector(b)) => Ok(Value::Vector(a - b)),
        (Value::Point(a), Value::Point(b)) => Ok(Value::Vector(a - b)),
        (Value::Point(pnt), Value::Vector(vec)) => Ok(Value::Point(pnt - vec)),
        _ => Err(Error::InvalidType),
    }
}

pub fn mul(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => a, b);
    match (a, b) {
        (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a * b)),
        (Value::Vector(vec), Value::Scalar(r)) | (Value::Scalar(r), Value::Vector(vec)) => {
            Ok(Value::Vector(vec * r))
        }
        _ => Err(Error::InvalidType),
    }
}

pub fn div(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => a, b);
    match (a, b) {
        (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a / b)),
        (Value::Vector(vec), Value::Scalar(r)) => Ok(Value::Vector(vec / r)),
        _ => Err(Error::InvalidType),
    }
}
