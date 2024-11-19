use crate::{runtime::Value, stdlib::Scalar};

pub enum DrawCommand {
    Line { from: (Mm, Mm), to: (Mm, Mm) },
    Circle { at: (Mm, Mm), radius: Mm },
    Resize { x: Mm, y: Mm },
}

impl From<Value> for Option<DrawCommand> {
    fn from(value: Value) -> Self {
        match value {
            Value::Line(p, v) => {
                let from = (p.x.into(), p.y.into());
                let to = ((p.x + v.x).into(), (p.y + v.y).into());
                Some(DrawCommand::Line { from, to })
            }

            _ => None,
        }
    }
}

pub trait DrawBuffer {
    fn reset(&mut self);

    fn draw(&mut self, command: DrawCommand);

    fn flush(&mut self);
}

pub struct Mm(pub f64);

impl From<Scalar> for Mm {
    fn from(value: Scalar) -> Self {
        Self(value.into())
    }
}
