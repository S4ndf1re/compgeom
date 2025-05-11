use crate::algorithm::sweep_line::line::Line;
use num::{cast, Float};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};

pub type Position<T> = [T; 3];
pub type Color = [f32; 4];

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
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
                cast(self.x()).unwrap(),
                cast(self.y()).unwrap(),
                cast(self.z()).unwrap(),
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
        line: &Line<T>,
    ) -> (T, Vertex<T>) {
        let r = line.direction;

        let s1 = self.x();
        let s2 = self.y();

        let r1 = r.x();
        let r2 = r.y();

        let x1_1 = line.original_x1.x();
        let x1_2 = line.original_x1.y();

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
        (t, line.original_x1 + r * t)
    }

    pub fn x(&self) -> T {
        self.position[0]
    }

    pub fn y(&self) -> T {
        self.position[1]
    }

    pub fn z(&self) -> T {
        self.position[2]
    }

    pub fn set_x(&mut self, x: T) {
        self.position[0] = x;
    }

    pub fn set_y(&mut self, y: T) {
        self.position[1] = y;
    }

    pub fn set_z(&mut self, z: T) {
        self.position[2] = z;
    }

    pub fn normal(&self) -> Self {
        Vertex {
            position: [-self.y(), self.x(), T::zero()],
            color: [0.0, 0.0, 0.0, 0.0],
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
            position: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
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
            position: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
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
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
}

impl<T> Mul<T> for Vertex<T>
where
    T: Float + Copy,
{
    type Output = Vertex<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Vertex {
            position: [self.x() * rhs, self.y() * rhs, self.z() * rhs],
            color: self.color,
        }
    }
}

impl<T> Eq for Vertex<T> where T: Float {}

impl<T> Ord for Vertex<T>
where
    T: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> From<(Vertex<T>, Vertex<T>)> for Line<T>
where
    T: Copy + Float + Debug,
{
    fn from(value: (Vertex<T>, Vertex<T>)) -> Self {
        Line::new(0, 0, value.0, value.1)
    }
}
