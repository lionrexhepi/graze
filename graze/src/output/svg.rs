use std::{cell::LazyCell, mem::take};

use xml_dom::level2::{
    convert::as_element_mut, get_implementation, Document, DocumentType, Element, Node, RefNode,
};

use crate::stdlib::Scalar;

use super::{DrawBuffer, DrawCommand};

pub struct SvgOutput {
    document: RefNode,
    element: RefNode,
}

impl Default for SvgOutput {
    fn default() -> Self {
        let document = make_svg_doc();
        let element = document
            .document_element()
            .expect("Expected Document to have a Document element");
        Self { document, element }
    }
}

impl DrawBuffer for SvgOutput {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn draw(&mut self, command: DrawCommand) {
        macro_rules! scalar_px_attr {
            ($on:expr, $name:expr,$scalar:expr) => {
                $on.set_attribute($name, &mm_to_px_str($scalar))
                    .expect("Expected to be able to set attribute");
            };
        }
        match command {
            DrawCommand::Line(p, v) => {
                let p2 = p + v;

                let mut line = self
                    .document
                    .create_element("line")
                    .expect("Expected to be able to create a line element");
                scalar_px_attr!(line, "x1", p.x);
                scalar_px_attr!(line, "y1", p.y);
                scalar_px_attr!(line, "x2", p2.x);
                scalar_px_attr!(line, "y2", p2.y);
                self.element
                    .append_child(line)
                    .expect("Expected to be able to append child");
            }

            DrawCommand::Circle(p, r) => {
                todo!()
            }

            DrawCommand::Resize { x, y } => {
                scalar_px_attr!(self.element, "width", x);
                scalar_px_attr!(self.element, "height", y);
            }
        }
    }

    fn flush(&mut self) {
        println!("{}", self.document);
    }
}

fn make_svg_doc() -> RefNode {
    let impl_ = get_implementation();

    impl_
        .create_document(None, Some("svg"), None)
        .expect("Expected to be able to create an SVG document")
}

fn mm_to_px(mm: Scalar) -> f64 {
    const DPI: f64 = 96.0;
    const MM_PER_INCH: f64 = 25.4;
    (f64::from(mm) * DPI) / MM_PER_INCH
}

fn mm_to_px_str(mm: Scalar) -> String {
    format!("{}", mm_to_px(mm))
}
