use crate::algorithm::sweep_line::context::SweepLineContext;
use crate::objects::vertex::Vertex;
use num::Float;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Clone)]
pub struct Line<T: Copy> {
    pub id: usize,
    pub x1: Vertex<T>,
    pub x2: Vertex<T>,
    pub direction: Vertex<T>,
    pub context: Rc<RefCell<SweepLineContext<T>>>,
}

impl<T> Line<T>
where
    T: Float,
{
    pub fn new(
        id: usize,
        x1: Vertex<T>,
        x2: Vertex<T>,
        context: Rc<RefCell<SweepLineContext<T>>>,
    ) -> Self {
        Self {
            id,
            x1,
            x2,
            direction: x2 - x1,
            context,
        }
    }

    /// For the context, that represents the current position of a line, compute the y-value if x is contained in the line itself
    /// If x is outside the line interval, return None
    pub fn get_y_for_context(&self) -> Option<T> {
        let x = self.context.borrow().x_pos;

        // x1_1 + t * r_1 = x
        let t = (x - self.x1.position[0]) / self.direction.position[0];

        // validate with t being in [0; 1]
        if t < T::zero() - T::epsilon() || t > T::one() + T::epsilon() {
            return None;
        }

        let p = self.f(t);
        Some(p.position[1])
    }

    pub fn f(&self, t: T) -> Vertex<T> {
        self.x1 * (T::one() - t) + self.x2 * t
    }

    /// Test for intersection of two line segements
    pub fn intersects(&self, other: &Self) -> Option<Vertex<T>> {
        let s1 = self.x2 - self.x1;
        let s2 = other.x2 - other.x1;

        let denom = -s2.x() * s1.y() + s1.x() * s2.y();
        if denom.abs() <= T::epsilon() {
            return None; // Lines are parallel
        }

        let s = (-s1.y() * (self.x1.x() - other.x1.x()) + s1.x() * (self.x1.y() - other.x1.y()))
            / denom;
        let t =
            (s2.x() * (self.x1.y() - other.x1.y()) - s2.y() * (self.x1.x() - other.x1.x())) / denom;

        if s >= T::zero() - T::epsilon()
            && s <= T::one() + T::epsilon()
            && t >= T::zero() - T::epsilon()
            && t <= T::one() + T::epsilon()
        {
            Some(self.f(t))
        } else {
            None
        }
    }
}

impl<T> Eq for Line<T> where T: Float {}

impl<T> PartialEq<Self> for Line<T>
where
    T: Float,
{
    fn eq(&self, other: &Self) -> bool {
        // let y = self.get_y_for_context();
        // if y.is_none() {
        //     return false;
        // }
        //
        // let y_other = other.get_y_for_context();
        // if y_other.is_none() {
        //     return false;
        // }
        //
        // y.unwrap() == y_other.unwrap()
        self.id == other.id
    }
}

impl<T> PartialOrd<Self> for Line<T>
where
    T: Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let y_self = self.get_y_for_context();
        let y_other = other.get_y_for_context();

        if y_self.is_none() {
            return None;
        }
        if y_other.is_none() {
            return None;
        }

        y_self.unwrap().partial_cmp(&y_other.unwrap())
    }
}

impl<T> Ord for Line<T>
where
    T: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let y_self = self.get_y_for_context();
        let y_other = other.get_y_for_context();

        if y_self.is_none() {
            return Ordering::Equal;
        }
        if y_other.is_none() {
            return Ordering::Equal;
        }

        y_self.unwrap().partial_cmp(&y_other.unwrap()).unwrap()
    }
}
