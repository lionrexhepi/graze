use svg::Document;

use crate::stdlib::{Point, Scalar, Vector};

use super::{DrawCommand, Screen};

pub struct SVGOutput {
    dimensions: (f64, f64),
    svg: Document,
}

impl SVGOutput {
    /// TODO fork svg crate and make a mut api
    /// Or handroll this with an xml writer crate since we need neither in-memory nor streaming
    fn append_node<N>(&mut self, element: N)
    where
        N: Into<Box<dyn svg::Node>>,
    {
        let svg = std::mem::replace(&mut self.svg, Document::new());
        self.svg = svg.add(element);
    }

    /// TODO handle negative values
    fn fit_point(&mut self, fit: (f64, f64)) {
        self.dimensions.0 = self.dimensions.0.max(fit.0);
        self.dimensions.1 = self.dimensions.1.max(fit.1);
    }
}

impl Screen for SVGOutput {
    type Output = Document;

    fn reset(&mut self) {
        self.svg = Document::new().set("viewBox", (0, 0, self.dimensions.0, self.dimensions.1));
    }

    fn draw(&mut self, command: DrawCommand) {
        match command {
            DrawCommand::Line(Point { x: p1, y: p2 }, Vector { x: v1, y: v2 }) => {
                let (x1, y1) = (mm_to_px(p1), mm_to_px(p2));
                let (x2, y2) = (x1 + mm_to_px(v1), y1 + mm_to_px(v2));

                self.fit_point((x1, y1));
                self.fit_point((x2, y2));

                self.append_node(
                    svg::node::element::Line::new()
                        .set("x1", x1)
                        .set("y1", y1)
                        .set("x2", x2)
                        .set("y2", y2)
                        .set("stroke", "black")
                        .set("stroke-width", 1),
                );
            }
            DrawCommand::Circle(p, r) => {
                self.append_node(
                    svg::node::element::Circle::new()
                        .set("cx", mm_to_px(p.x))
                        .set("cy", mm_to_px(p.y))
                        .set("r", mm_to_px(r))
                        .set("fill", "none")
                        .set("stroke", "black")
                        .set("stroke-width", 1),
                );
            }
        }
    }

    fn finish(&mut self) -> Self::Output {
        self.svg.clone()
    }
}

const DPI: f64 = 96.0;
const MM_PER_INCH: f64 = 25.4;
const PX_PER_MM: f64 = DPI / MM_PER_INCH;

fn mm_to_px(scalar: Scalar) -> f64 {
    f64::from(scalar) * PX_PER_MM
}

#[cfg(test)]
mod test {
    use svg::Document;

    const OUT_DIR: &str = "target/test";

    fn clear_output() {
        for entry in std::fs::read_dir(OUT_DIR).unwrap() {
            let entry = entry.unwrap();
            std::fs::remove_file(entry.path()).unwrap();
        }
    }
}
