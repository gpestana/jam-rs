#![allow(dead_code)]
extern crate rand;

use crate::canvas::Point;

use rand::Rng;
use svg::node::element::path::Data;
use rand::distributions::{Distribution, Uniform};

pub struct Triangle {
    vertices: [Point; 3],
}

impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point) -> Self {
        Triangle{
            vertices: [v0, v1, v2],
        }
    }

    pub fn random(
        origin_boundaries: (u64, u64), 
        size_boundaries: (u64, u64)
    ) -> Self {

        let mut rng = rand::thread_rng();
        let die = Uniform::from(size_boundaries.0..size_boundaries.1);

        let origin = Point(
            rng.gen_range(0, origin_boundaries.0) as f32,
            rng.gen_range(0, origin_boundaries.1) as f32,
        );

        let throw = die.sample(&mut rng) as f32;
        let v1 = Point(throw * -1.0, throw);
        let v2 = Point(throw * -1.0, throw * -1.0);

        Triangle::new(
            origin,
            v1,
            v2,
        )
    }

    pub fn rotate_random(self) -> Self {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(-1.0..1.0);
        let rotate = die.sample(&mut rng) as f32;
        let rotateb = die.sample(&mut rng) as f32;
        Triangle::new(
            Point(self.vertices[0].0, self.vertices[0].1),
            Point(self.vertices[1].0 * rotate, self.vertices[1].1 * rotate),
            Point(self.vertices[2].0 * rotateb, self.vertices[2].1 * rotateb),
        )
    }

    pub fn draw(&self) -> Data {
        Data::new()
            .move_to(self.vertices[0])
            .line_by(self.vertices[1])
            .line_by(self.vertices[2])
            .close()
    }

}
