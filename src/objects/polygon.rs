use num::cast::cast;
use num::Float;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul, Sub};

pub type Position<T: Float> = [T; 3];
pub type Color = [f32; 3];

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex<T> {
    pub position: Position<T>,
    pub color: Color,
}

impl<T> Vertex<T>
where
    T: Float + Copy,
{
    pub fn magnitude(&self) -> T {
        let magnitude = (*self * *self).sqrt();
        if magnitude < T::epsilon() {
            T::from(1.0).unwrap()
        } else {
            magnitude
        }
    }
    pub fn cosine(&self, other: &Vertex<T>) -> T {
        (*self * *other) / (self.magnitude() * other.magnitude())
    }

    pub fn to_other<O: Float + Copy + Debug>(&self) -> Vertex<O> {
        Vertex {
            position: [
                cast(self.position[0]).unwrap(),
                cast(self.position[1]).unwrap(),
                cast(self.position[2]).unwrap(),
            ],
            color: self.color,
        }
    }
}

impl<T> Add for Vertex<T>
where
    T: Float + Copy,
{
    type Output = Vertex<T>;

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

impl<T> Sub for Vertex<T>
where
    T: Float + Copy,
{
    type Output = Vertex<T>;

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

impl<T> Mul for Vertex<T>
where
    T: Float + Copy,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.position[0] * rhs.position[0]
            + self.position[1] * rhs.position[1]
            + self.position[2] * rhs.position[2]
    }
}

impl<T> Mul<T> for Vertex<T>
where
    T: Float + Copy,
{
    type Output = Vertex<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Vertex {
            position: [
                self.position[0] * rhs,
                self.position[1] * rhs,
                self.position[2] * rhs,
            ],
            color: self.color,
        }
    }
}

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
