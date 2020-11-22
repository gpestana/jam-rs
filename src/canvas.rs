#![allow(dead_code)]
use svg::node::element::path::Parameters;

#[derive(Copy, Clone, Debug)]
pub struct Point(pub f32, pub f32);

impl From<Point> for Parameters {
    fn from(point: Point) -> Parameters {
        Parameters::from(vec![point.0, point.1])
    }
}
