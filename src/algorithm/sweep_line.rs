use crate::objects::polygon::Vertex;
use num::Float;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::Debug;

struct SweepLineContext<T> {
    x_pos: T,
}

struct Line<'c, T> {
    x1: Vertex<T>,
    x2: Vertex<T>,
    direction: Vertex<T>,
    context: &'c SweepLineContext<T>,
}

impl<'c, T> Line<'c, T>
where
    T: Float + Copy + Debug,
{
    pub fn new(x1: Vertex<T>, x2: Vertex<T>, context: &'c SweepLineContext<T>) -> Self {
        Self {
            x1,
            x2,
            direction: x2 - x1,
            context,
        }
    }

    /// For the context, that represents the current position of a line, compute the y-value if x is contained in the line itself
    /// If x is outside the line interval, return None
    pub fn get_y_for_context(&self) -> Option<T> {
        let x = self.context.x_pos;

        // x1_1 + t * r_1 = x
        let t = (x - self.x1.position[0]) / self.direction.position[0];

        // validate with t being in [0; 1]
        if t < T::zero() || t > T::one() {
            return None;
        }

        let p = self.f(t);
        Some(p.position[1])
    }

    pub fn f(&self, t: T) -> Vertex<T> {
        self.x1 * (T::one() - t) + self.x2 * t
    }
}

impl<'c, T> Eq for Line<'c, T> where T: Float {}

impl<'c, T> PartialEq<Self> for Line<'c, T>
where
    T: Float,
{
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<'c, T> PartialOrd<Self> for Line<'c, T>
where
    T: Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl<'c, T> Ord for Line<'c, T>
where
    T: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}
struct SweepLineStateStructure<'c, T> {
    container: BTreeSet<Line<'c, T>>,
}

impl<'c, T> SweepLineStateStructure<'c, T>
where
    T: Float + Copy + Debug,
{
    pub fn new() -> Self {
        Self {
            container: BTreeSet::new(),
        }
    }

    pub fn insert_line(&mut self, line: Line<'c, T>) {
        self.container.insert(line);
    }
}
