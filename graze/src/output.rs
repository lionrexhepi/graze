use crate::{
    runtime::Value,
    stdlib::{Point, Scalar, Vector},
};

pub enum DrawCommand {
    Line(Point, Vector),
    Circle(Point, Scalar),
    Resize { x: Scalar, y: Scalar },
}

impl From<Value> for Option<DrawCommand> {
    fn from(value: Value) -> Self {
        match value {
            Value::Line(p, v) => Some(DrawCommand::Line(p, v)),

            _ => None,
        }
    }
}

pub trait DrawBuffer {
    fn reset(&mut self);

    fn draw(&mut self, command: DrawCommand);

    fn flush(&mut self);
}
