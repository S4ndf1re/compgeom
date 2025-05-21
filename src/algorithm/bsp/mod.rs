use std::fmt::Debug;

use num::Float;
use rand::Rng;

use crate::objects::{line::Line, polygon::Polygon, vertex::Vertex};


/// The point orientation, e.g. left or right of a line, the distance to the line, and the polygon containing the single point
pub  enum PointOrientation<T: Float + Copy + Debug> {
    Left(T, Polygon<T>),
    Right(T, Polygon<T>),
}


pub fn line_decider<T: Float + Copy + Debug, P: AsRef<[Vertex<T>]>>(line: &Line<T>, points: P) -> (T, Vec<PointOrientation<T>>) {

    let mut orientation = vec![];
    let mut max_dist = T::min_value();

    for p in points.as_ref() {
        let (_t, point) = p.point_on_line_with_min_distance_to_self_2d(line);
        let diff = point - *p;
        let dist = diff.magnitude();
        max_dist = max_dist.max(dist);

        if (line.normal * -T::one()).cosine(&diff) >= T::zero() {
            // is on left
            orientation.push(PointOrientation::Left(dist, Polygon::new(vec![*p])));
        } else {
            // is on right
            orientation.push(PointOrientation::Right(dist, Polygon::new(vec![*p])));
        }
    }

    (max_dist, orientation)
}



/// Generate a random range of n points in the given range.
pub fn generate_points<T: Float + Copy + Debug>(lower: f32, upper: f32, n: usize) -> Vec<Vertex<T>> {
    let mut output = vec![];
    let mut rng = rand::rng();


    for i in 0..n {
        let (x, y) = (rng.random_range(lower..=upper), rng.random_range(lower..=upper));

        let vertex = Vertex {
            id: i,
            position: [T::from(x).unwrap(), T::from(y).unwrap(), T::zero()],
            color: [0.0, 0.0, 0.0, 0.0],
        };

        output.push(vertex);
    }

    output
}
