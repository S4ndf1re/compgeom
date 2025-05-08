use crate::algorithm::sweep_line::context::SweepLineContext;
use crate::objects::vertex::Vertex;
use num::Float;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

type LineContext<T> = Rc<RefCell<SweepLineContext<T>>>;

#[derive(Clone)]
pub struct Line<T: Copy> {
    pub id: usize,
    pub polygon_id: usize,
    pub x1: Vertex<T>,
    pub x2: Vertex<T>,
    pub direction: Vertex<T>,
    pub context: Option<LineContext<T>>,
}

impl<T> Line<T>
where
    T: Float,
{
    pub fn new(id: usize, polygon_id: usize, x1: Vertex<T>, x2: Vertex<T>) -> Self {
        Self {
            id,
            polygon_id,
            x1,
            x2,
            direction: x2 - x1,
            context: None,
        }
    }

    pub fn infuse_context(&mut self, context: LineContext<T>) {
        self.context = Some(context);
    }

    /// For the context, that represents the current position of a line, compute the y-value if x is contained in the line itself
    /// If x is outside the line interval, return None
    pub fn get_y_for_context(&self) -> T {
        let p = self.f(self.get_t_for_context());
        p.y()
    }

    pub fn get_t_for_context(&self) -> T {
        let x = self.context.clone().unwrap().borrow().x_pos;

        // x1_1 + t * r_1 = x
        (x - self.x1.x()) / self.direction.x()
    }

    pub fn f(&self, t: T) -> Vertex<T> {
        self.x1 * (T::one() - t) + self.x2 * t
    }

    /// Test for intersection of two line segments
    pub fn intersects(&self, other: &Self) -> Option<Vertex<T>> {
        if self.polygon_id == other.polygon_id {
            return None;
        }

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
        self.id == other.id
    }
}

impl<T> PartialOrd<Self> for Line<T>
where
    T: Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<T> Ord for Line<T>
where
    T: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.id == other.id {
            return Ordering::Equal;
        }

        if let Some(order) = self
            .context
            .clone()
            .unwrap()
            .borrow()
            .get_order(&self.id, &other.id)
        {
            if order != Ordering::Equal {
                return order;
            }
        }

        let y_self = self.get_y_for_context();
        let y_other = other.get_y_for_context();

        let ordering = y_self.partial_cmp(&y_other).unwrap();
        // In case both x and y are equal, determine the general direction. If this is also equal, the lines are collinear, and the length is important
        let ordering = if ordering == Ordering::Equal {
            let dir1 = self.x2 - self.x1;
            let dir2 = other.x2 - other.x1;

            let det = dir1.x() * dir2.y() - dir1.y() * dir2.x();
            if det > T::zero() {
                Ordering::Less
            } else if det < T::zero() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        } else {
            ordering
        };

        self.context
            .clone()
            .unwrap()
            .borrow_mut()
            .set_order(self.id, other.id, ordering);

        ordering
    }
}
