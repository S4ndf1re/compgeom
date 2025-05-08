use crate::algorithm::sweep_line::context::SweepLineContext;
use crate::objects::vertex::Vertex;
use num::Float;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

type LineContext<T> = Rc<RefCell<SweepLineContext<T>>>;

#[derive(Clone)]
pub struct Line<T: Copy> {
    pub id: usize,
    pub polygon_id: usize,
    pub x1: Vertex<T>,
    pub x2: Vertex<T>,
    pub original_x1: Vertex<T>,
    pub original_x2: Vertex<T>,
    pub direction: Vertex<T>,
    pub normal: Vertex<T>,
    pub context: Option<LineContext<T>>,
}

fn greater_than<T: Float>(a: T, b: T, eps: T) -> bool {
    (a - b) > ((if a.abs() < b.abs() { b.abs() } else { a.abs() }) * eps)
}
fn less_than<T: Float>(a: T, b: T, eps: T) -> bool {
    (b - a) > ((if a.abs() < b.abs() { b.abs() } else { a.abs() }) * eps)
}

impl<T> Line<T>
where
    T: Float + Debug,
{
    pub fn new(id: usize, polygon_id: usize, mut x1: Vertex<T>, mut x2: Vertex<T>) -> Self {
        let mut x1_corrected = x1;
        let mut x2_corrected = x2;
        let normal = (x2 - x1).normal();
        if x1_corrected.x() > x2_corrected.x() {
            std::mem::swap(&mut x1_corrected, &mut x2_corrected);
        }

        Self {
            id,
            polygon_id,
            x1: x1_corrected,
            x2: x2_corrected,
            original_x1: x1,
            original_x2: x2,
            normal,
            direction: x2_corrected - x1_corrected,
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
        let t = (x - self.x1.x()) / self.direction.x();
        t
    }

    pub fn f(&self, t: T) -> Vertex<T> {
        self.x1 * (T::one() - t) + self.x2 * t
    }

    pub fn point_on_normal_side(&self, point: &Vertex<T>) -> bool {
        let (_, p_min_dist) = point.point_on_line_with_min_distance_to_self_clamped_0_1_2d(self);
        let directional_test = *point - p_min_dist;
        let dist = directional_test.magnitude();

        let cosine = self.normal.cosine(&directional_test);

        cosine <= T::zero() || dist < T::from(0.1).unwrap()
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

        let mut s = (-s1.y() * (self.x1.x() - other.x1.x())
            + s1.x() * (self.x1.y() - other.x1.y()))
            / denom;
        let mut t =
            (s2.x() * (self.x1.y() - other.x1.y()) - s2.y() * (self.x1.x() - other.x1.x())) / denom;

        if (T::one() - s).abs() <= T::epsilon() {
            s = T::one();
        }

        if (T::one() - t).abs() <= T::epsilon() {
            t = T::one();
        }

        if s >= T::zero() && s <= T::one() && t >= T::zero() && t <= T::one() {
            Some(self.f(t))
        } else {
            None
        }
    }

    pub fn is_vertical(&self) -> bool {
        self.direction.x() < T::epsilon() && self.direction.y().abs() > T::epsilon()
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
    T: Float + Debug,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let local_context = self.context.clone().unwrap();
        if self.id == other.id {
            return Ordering::Equal;
        }

        if let Some(order) = local_context.borrow().get_order(&self.id, &other.id) {
            if order != Ordering::Equal {
                return order;
            }
        }

        let y_self = self.get_y_for_context();
        let y_other = other.get_y_for_context();

        let ordering = if less_than(y_self, y_other, T::from(0.0000001).unwrap()) {
            Ordering::Less
        } else if greater_than(y_self, y_other, T::from(0.0000001).unwrap()) {
            Ordering::Greater
        } else {
            Ordering::Equal
        };
        // In case both x and y are equal, determine the general direction. If this is also equal, the lines are collinear, and the length is important
        let mut ordering = if ordering == Ordering::Equal {
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

        {
            // On intersections, the order will swap. So if we are currently looking at an intersection, swap order preemptively
            let context_inner = local_context.borrow();
            if context_inner.is_intersection
                && (context_inner.line_id1 == self.id && context_inner.line_id2 == other.id
                    || context_inner.line_id1 == other.id && context_inner.line_id2 == self.id)
            {
                ordering = ordering.reverse();
            }
        }

        local_context
            .borrow_mut()
            .set_order(self.id, other.id, ordering);

        ordering
    }
}

impl<T> Debug for Line<T>
where
    T: Debug + Float,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
