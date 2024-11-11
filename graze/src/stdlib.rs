mod point;
mod scalar;
mod vector;

pub use point::Point;
pub use scalar::Scalar;
pub use vector::Vector;

use crate::runtime::{Error, Runtime, Stack, Value};

#[macro_export]
macro_rules! reverse_pop {
     ($stack:ident => $arg:ident) => {
         let $arg = $stack.pop().map_err(|_| Error::MissingArgument)?;
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
        _ => Err(Error::TypeError),
    }
}

pub fn sub(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => a, b);
    match (a, b) {
        (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a - b)),
        (Value::Vector(a), Value::Vector(b)) => Ok(Value::Vector(a - b)),
        (Value::Point(a), Value::Point(b)) => Ok(Value::Vector(a - b)),
        (Value::Point(pnt), Value::Vector(vec)) => Ok(Value::Point(pnt - vec)),
        _ => Err(Error::TypeError),
    }
}

pub fn mul(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => a, b);
    match (a, b) {
        (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a * b)),
        (Value::Vector(vec), Value::Scalar(r)) | (Value::Scalar(r), Value::Vector(vec)) => {
            Ok(Value::Vector(vec * r))
        }
        _ => Err(Error::TypeError),
    }
}

pub fn div(stack: &mut Stack) -> Result<Value, Error> {
    reverse_pop!(stack => a, b);
    match (a, b) {
        (Value::Scalar(a), Value::Scalar(b)) => Ok(Value::Scalar(a / b)),
        (Value::Vector(vec), Value::Scalar(r)) => Ok(Value::Vector(vec / r)),
        _ => Err(Error::TypeError),
    }
}

pub fn register_stdlib(runtime: &mut Runtime) {
    runtime.register("add", add);
    runtime.register("sub", sub);
    runtime.register("mul", mul);
    runtime.register("div", div);

    vector::register_stdlib(runtime);
    point::register_stdlib(runtime);
    scalar::register_stdlib(runtime);
}
