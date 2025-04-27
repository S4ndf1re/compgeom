use std::ops::{Add, Mul, Sub};

pub type Position = [f32; 3];
pub type Color = [f32; 3];

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Position,
    pub color: Color,
}

impl Vertex {
    pub fn magnitude(&self) -> f32 {
        (*self * *self).sqrt()
    }
    pub fn cosine(&self, other: &Vertex) -> f32 {
        (*self * *other) / (self.magnitude() * other.magnitude())
    }
}

impl Add for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Self) -> Self::Output {
        Vertex {
            position: [
                self.position[0] + rhs.position[0],
                self.position[1] + rhs.position[1],
                self.position[2] + rhs.position[2],
            ],
            color: self.color,
        }
    }
}

impl Sub for Vertex {
    type Output = Vertex;

    fn sub(self, rhs: Self) -> Self::Output {
        Vertex {
            position: [
                self.position[0] - rhs.position[0],
                self.position[1] - rhs.position[1],
                self.position[2] - rhs.position[2],
            ],
            color: self.color,
        }
    }
}

impl Mul for Vertex {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.position[0] * rhs.position[0]
            + self.position[1] * rhs.position[1]
            + self.position[2] * rhs.position[2]
    }
}

#[derive(Clone)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
}

impl Polygon {
    pub fn new(verticies: Vec<Vertex>) -> Self {
        Self {
            vertices: verticies,
        }
    }

    pub fn get_bounds(&self) -> (f32, f32, f32, f32) {
        let mut bounds = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);

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
    pub fn shift_to(&mut self, x: f32, y: f32) {
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
