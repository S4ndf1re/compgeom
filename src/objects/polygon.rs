use crate::objects::vertex::{Color, Vertex};
use num::Float;
use std::fmt::{Debug, Display, Formatter};

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
                bounds.0.min(v.position[0]),
                bounds.1.max(v.position[0]),
                bounds.2.min(v.position[1]),
                bounds.3.max(v.position[1]),
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
            v.position[0] = v.position[0] + x_diff;
            v.position[1] = v.position[1] + y_diff;
        }
    }

    pub fn set_color(&mut self, color: Color) {
        for v in &mut self.vertices {
            v.color = color;
        }
    }
}

impl<T> Display for Polygon<T>
where
    T: Float + Copy + Display + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Polygon( verticies: \n")?;
        Ok(for v in &self.vertices {
            let x = v.position[0];
            let y = v.position[1];
            let z = v.position[1];
            write!(f, "\tV({:.}, {:.}, {:.})", x, y, z)?
        })
    }
}
