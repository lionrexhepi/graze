mod basic;
mod point;
mod scalar;
mod vector;

pub use point::Point;
pub use scalar::Scalar;
pub use vector::Vector;

use crate::runtime::Runtime;

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

pub fn register(runtime: &mut Runtime) {
    basic::register(runtime);
    vector::register(runtime);
    point::register(runtime);
    scalar::register(runtime);
}
