use crate::objects::line::Line;
use num::{Float, cast};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, Mul, Neg, Sub};

use super::color::Color;

pub type Position<T> = [T; 3];

#[repr(C, packed)]
#[derive(Clone, Copy, PartialOrd, PartialEq)]
pub struct Vertex<T> {
    pub position: Position<T>,
    pub color: Color,
    pub id: usize,
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
            id: self.id,
        }
    }

    pub fn point_on_line_with_min_distance_to_self_2d(&self, line: &Line<T>) -> (T, Vertex<T>) {
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
        (t, line.original_x1 + r * t)
    }

    pub fn distance_to_line_hessen(&self, line: &Line<T>) -> T {
        *self * line.hessen_normal - line.hessen_d
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
        let t = self.point_on_line_with_min_distance_to_self_2d(line).0;

        let t = t.clamp(T::zero(), T::one());
        (t, line.original_x1 + r * t)
    }

    pub fn normal(&self) -> Self {
        Vertex {
            position: [-self.y(), self.x(), T::zero()],
            color: [0.0, 0.0, 0.0, 0.0],
            id: self.id,
        }
    }
}

impl<T> Vertex<T>
where
    T: Copy,
{
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
            id: self.id,
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
            id: self.id,
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
            id: self.id,
        }
    }
}

impl<T> Neg for Vertex<T>
where
    T: Float + Copy,
{
    type Output = Vertex<T>;
    fn neg(self) -> Self::Output {
        self * -T::one()
    }
}

impl<T> Eq for Vertex<T> where T: Float {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl<T> Ord for Vertex<T>
where
    T: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> From<(T, T)> for Vertex<T>
where
    T: Float + Copy,
{
    fn from(value: (T, T)) -> Self {
        Self {
            id: 0,
            position: [value.0, value.1, T::zero()],
            color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

impl<T> From<(T, T, T)> for Vertex<T>
where
    T: Float + Copy,
{
    fn from(value: (T, T, T)) -> Self {
        Self {
            id: 0,
            position: [value.0, value.1, value.2],
            color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

impl<T> Debug for Vertex<T>
where
    T: Debug + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.id;
        write!(f, "(id: {}, {:?}, {:?})", id, self.x(), self.y())
    }
}
