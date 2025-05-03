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

    /// On a line defined by l(t) = (1-t)*x1 + t*x2 = x1 + t * (x2-x1), find a t and the corresponding point,
    /// so that the distance between self and the line is minimal.
    /// normally this would mean, that the cosine between the normal of the line and the vector from the line to the point is 1
    /// However, keep in mind that the value is clamped between 0 and 1.
    pub fn point_on_line_with_min_distance_to_self_clamped_0_1_2d(
        &self,
        x1: Vertex<T>,
        x2: Vertex<T>,
    ) -> (T, Vertex<T>) {
        let r = x2 - x1;
        let normal = Vertex {
            position: [-r.position[1], r.position[0], T::zero()],
            color: [0.0, 0.0, 0.0],
        };

        let s1 = self.position[0];
        let s2 = self.position[1];

        let r1 = r.position[0];
        let r2 = r.position[1];

        let x1_1 = x1.position[0];
        let x1_2 = x1.position[1];

        // normal . (self - l(t)) == 1
        // r = x2-x1
        // l(t) = x1 + t * r
        // r . (self - (x1 + t*r)) == 0
        // (r1, r2) . ((s1,s2) - ((x1, x2) + t * (r1, r2)) == 0
        // (r1, r2) . (s1 - (x1+t*r1), s2 - (x2+t*r2)) == 0
        // r1 * (s1-x1-t*r1) + r2 * (s2-x2-t*r2) == 0
        // r1*s1 - r1*x2 - r1*t*r1 + r2*s2 - r2*x2 - r2*t*r2 == 0
        // -r1*t*r1-r2*t*r2 = - r1*s1 + r1*x2 - r2*s2 + r2*x2
        // t * (-r1*r1 - r2*r2) == - r1*s1 + r1*x2 - r2*s2 + r2*x2
        // t == (- r1*s1 + r1*x1 - r2*s2 + r2*x2) / (-r1*r1 - r2*r2)

        let t = (T::one() - r1 * s1 + r1 * x1_1 - r2 * s2 + r2 * x1_2) / (-r1 * r1 - r2 * r2);
        let t = t.clamp(T::zero(), T::one());
        (t, x1 + r * t)
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
