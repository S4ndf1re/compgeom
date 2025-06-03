use crate::algorithm::graham_scan::graham_scan::graham_scan_vorlesungsfolie;
use crate::objects::line::Line;
use crate::objects::vertex::Vertex;
use num::Float;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

use super::color::Color;

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
            id: 0,
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
    pub fn unit(&self, other: &Self) -> Option<Vec<Vertex<T>>>
    where
        T: Float,
    {
        let mut all_points = self.vertices.clone().into_iter().collect::<Vec<_>>();
        all_points.extend(other.vertices.iter().map(|v| v.clone()));
        let all_points = all_points
            .into_iter()
            .enumerate()
            .map(|(id, mut v)| {
                v.id = id;
                v
            })
            .collect::<Vec<_>>();

        let hull = graham_scan_vorlesungsfolie(&all_points)?;
        let hull_set: HashSet<usize> = HashSet::from_iter(hull.into_iter().map(|v| v.id));

        let mut inner_points = vec![];

        for p in all_points {
            let id = p.id;
            if !hull_set.contains(&id) {
                inner_points.push(p);
            }
        }

        Some(inner_points)
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

impl<T> From<Line<T>> for Polygon<T>
where
    T: Float + Copy + Debug,
{
    fn from(value: Line<T>) -> Self {
        Self::new(vec![value.x1, value.x2])
    }
}
