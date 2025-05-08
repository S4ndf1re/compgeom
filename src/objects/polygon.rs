use crate::algorithm::sweep_line::line::Line;
use crate::objects::vertex::{Color, Vertex};
use num::Float;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct Polygon<T>
where
    T: Debug + Copy,
{
    pub vertices: Vec<Vertex<T>>,
}

impl<T> Polygon<T>
where
    T: Float + Clone + Debug + Copy,
{
    pub fn new(verticies: Vec<Vertex<T>>) -> Self {
        Self {
            vertices: verticies,
        }
    }

    pub fn get_bounds(&self) -> (T, T, T, T) {
        let mut bounds = (
            T::max_value(),
            T::min_value(),
            T::max_value(),
            T::min_value(),
        );

        for v in &self.vertices {
            bounds = (
                bounds.0.min(v.x()),
                bounds.1.max(v.x()),
                bounds.2.min(v.y()),
                bounds.3.max(v.y()),
            );
        }

        bounds
    }

    /// Shift polygon so that x_0, y_0 are set to x, y
    pub fn shift_to(&mut self, x: T, y: T) {
        let bounds = self.get_bounds();

        let x_diff = x - bounds.0;
        let y_diff = y - bounds.2;

        for v in &mut self.vertices {
            v.set_x(v.x() + x_diff);
            v.set_y(v.y() + y_diff);
        }
    }

    pub fn set_color(&mut self, color: Color) {
        for v in &mut self.vertices {
            v.color = color;
        }
    }

    pub fn to_lines(&self, offset: usize, polygon_id: usize) -> (usize, Vec<Line<T>>) {
        let mut lines = vec![];
        for i in 0..self.vertices.len() {
            let x1 = self.vertices[i];
            let x2 = self.vertices[(i + 1) % self.vertices.len()];
            lines.push(Line::new(i + offset, polygon_id, x1, x2))
        }
        (lines.last().unwrap().id + 1, lines)
    }

    pub fn get_center(&self) -> Vertex<T> {
        let mut sum = Vertex {
            position: [T::zero(), T::zero(), T::zero()],
            color: [0.0, 0.0, 0.0, 0.0],
        };

        for p in self.vertices.iter() {
            sum = sum + *p * (T::one() / T::from(self.vertices.len()).unwrap());
        }
        sum
    }

    pub fn insert_front(&mut self, vert: Vertex<T>) {
        self.vertices.insert(0, vert);
    }

    pub fn push_back(&mut self, vertex: Vertex<T>) {
        self.vertices.push(vertex);
    }

    pub fn get_idx(&self, idx: usize) -> Option<Vertex<T>> {
        self.vertices.get(idx).map(|o| o.clone())
    }
}

impl<T> Polygon<T>
where
    T: Float + Ord + Debug + Hash,
{
    pub fn is_point_inside(&self, point: &Vertex<T>) -> bool {
        let (_, lines) = self.to_lines(0, 0);

        lines.iter().all(|l| l.point_on_normal_side(point))
    }

    pub fn unit(&self, other: &Self) -> Vec<Vertex<T>> {
        let mut result = vec![];

        for p in other.vertices.iter() {
            if self.is_point_inside(p) {
                result.push(*p);
            }
        }

        for p in self.vertices.iter() {
            if other.is_point_inside(p) {
                result.push(*p);
            }
        }

        result
    }
}

impl<T> Display for Polygon<T>
where
    T: Float + Copy + Display + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Polygon( verticies: \n")?;
        Ok(for v in &self.vertices {
            let x = v.x();
            let y = v.y();
            let z = v.z();
            write!(f, "\tV({:.}, {:.}, {:.})", x, y, z)?
        })
    }
}
