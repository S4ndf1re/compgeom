use crate::objects::polygon::Vertex;
use num::Float;
use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeSet, BinaryHeap, HashSet};
use std::fmt::Debug;
use std::ops::Bound;
use std::rc::Rc;

struct SweepLineContext<T> {
    x_pos: T,
}

#[derive(Clone)]
struct Line<T: Copy> {
    id: usize,
    x1: Vertex<T>,
    x2: Vertex<T>,
    direction: Vertex<T>,
    context: Rc<RefCell<SweepLineContext<T>>>,
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

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone)]
struct LineNode<T>
where
    T: Float,
{
    cell: Rc<RefCell<Line<T>>>,
}

impl<T> LineNode<T>
where
    T: Float,
{
    pub fn new(line: Line<T>) -> Self {
        Self {
            cell: Rc::new(RefCell::new(line)),
        }
    }
    pub fn swap(&self, other: &Self) {
        self.cell.swap(other.cell.as_ref());
    }
}

struct SweepLineStateStructure<T>
where
    T: Float,
{
    container: BTreeSet<LineNode<T>>,
}

impl<T> SweepLineStateStructure<T>
where
    T: Float + Ord,
{
    pub fn new() -> Self {
        Self {
            container: BTreeSet::new(),
        }
    }

    pub fn insert_line(&mut self, line: LineNode<T>) {
        self.container.insert(line);
    }

    pub fn remove_line(&mut self, node: &LineNode<T>) {
        self.container.remove(node);
    }

    pub fn exchange(&mut self, line1: &LineNode<T>, line2: &LineNode<T>) {
        let line1 = self.container.get(line1);
        let line2 = self.container.get(line2);

        if line1.is_some() && line2.is_some() {
            line1.unwrap().swap(line2.unwrap());
        }
    }

    pub fn pred(&self, line: &LineNode<T>) -> Option<LineNode<T>> {
        let range = self
            .container
            .range((Bound::Unbounded, Bound::Excluded(line)));

        range.last().map(|n| n.clone())
    }

    pub fn succ(&self, line: &LineNode<T>) -> Option<LineNode<T>> {
        let range = self
            .container
            .range((Bound::Excluded(line), Bound::Unbounded));

        range.rev().last().map(|n| n.clone())
    }
}

#[derive(Ord, PartialOrd, PartialEq, Eq)]
enum SweepLineEvent<T>
where
    T: Float,
{
    LineStart(LineNode<T>),
    LineEnd(LineNode<T>),
    Intersection(LineNode<T>, LineNode<T>),
}

struct EventStructure<T>
where
    T: Float,
{
    // Construct min heap
    queue: BinaryHeap<Reverse<(T, SweepLineEvent<T>)>>,
}

impl<T> EventStructure<T>
where
    T: Float + Ord + Copy,
{
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
        }
    }

    pub fn add_event(&mut self, x: T, event: SweepLineEvent<T>) {
        self.queue.push(Reverse((x, event)))
    }

    pub fn next_event(&mut self) -> Option<(T, SweepLineEvent<T>)> {
        self.queue.pop().map(|e| e.0)
    }
}

/// Compute the possible line intersection of line1 and line2.
pub fn is_intersecting<T: Float>(line1: &LineNode<T>, line2: &LineNode<T>) -> Option<(T, T)> {
    todo!()
}

/// Compute the possible line intersection of line1 and line2. When intersection is found and was not previously computed, add intersection to queue
pub fn is_intersecting_trigger_event<T: Float + Ord + Copy>(
    intersecting_store: &mut HashSet<(usize, usize)>,
    queue: &mut EventStructure<T>,
    line1: &LineNode<T>,
    line2: &LineNode<T>,
) {
    if intersecting_store
        .get(&(line1.cell.borrow().id, line2.cell.borrow().id))
        .is_some()
    {
        return;
    }

    if let Some((x, y)) = is_intersecting(line1, line2) {
        queue.add_event(
            x,
            SweepLineEvent::Intersection(line1.clone(), line2.clone()),
        );
    }
}

pub fn sweep_line_intersections<T: Float + Ord, L: AsRef<[(Vertex<T>, Vertex<T>)]>>(
    lines: L,
) -> Vec<(T, T, Line<T>, Line<T>)> {
    let context = Rc::new(RefCell::new(SweepLineContext { x_pos: T::zero() }));
    let mut sss = SweepLineStateStructure::new();
    let mut queue = EventStructure::new();

    let lines = lines
        .as_ref()
        .iter()
        .enumerate()
        .map(|(id, tuple)| Line::new(id, tuple.0, tuple.1, context.clone()))
        .map(|line| LineNode::new(line))
        .collect::<Vec<_>>();

    lines.iter().for_each(|line| {
        queue.add_event(
            line.cell.borrow().x1.position[0],
            SweepLineEvent::LineStart(line.clone()),
        );
        queue.add_event(
            line.cell.borrow().x2.position[0],
            SweepLineEvent::LineEnd(line.clone()),
        );
    });

    let mut already_intersected: HashSet<(usize, usize)> = HashSet::new();
    let mut intersections: Vec<(T, T, Line<T>, Line<T>)> = Vec::new();

    while let Some(event) = queue.next_event() {
        let x = event.0;
        context.borrow_mut().x_pos = x;

        match event.1 {
            SweepLineEvent::LineStart(line) => {
                sss.insert_line(line.clone());
                if let Some(pred) = sss.pred(&line) {
                    is_intersecting_trigger_event(
                        &mut already_intersected,
                        &mut queue,
                        &line,
                        &pred,
                    );
                }

                if let Some(succ) = sss.succ(&line) {
                    is_intersecting_trigger_event(
                        &mut already_intersected,
                        &mut queue,
                        &line,
                        &succ,
                    );
                }
            }
            SweepLineEvent::LineEnd(line) => {
                let pred = sss.pred(&line);
                let succ = sss.succ(&line);
                sss.remove_line(&line);

                if pred.is_some() && succ.is_some() {
                    is_intersecting_trigger_event(
                        &mut already_intersected,
                        &mut queue,
                        &pred.unwrap(),
                        &succ.unwrap(),
                    );
                }
            }
            SweepLineEvent::Intersection(line_a, line_b) => {
                // Context will always contain the current (intersection) x value
                let y = line_a
                    .cell
                    .borrow()
                    .get_y_for_context()
                    .expect("since an intersection was reported, an intersection must exist");

                intersections.push((
                    context.borrow().x_pos,
                    y,
                    line_a.cell.borrow().clone(),
                    line_b.cell.borrow().clone(),
                ));
                sss.exchange(&line_a, &line_b);

                // Pesudocode swaps comparison order. I think it will not really matter, since a and b are at the same location, currently
                if let Some(pred) = sss.pred(&line_a) {
                    is_intersecting_trigger_event(
                        &mut already_intersected,
                        &mut queue,
                        &line_b,
                        &pred,
                    );
                }

                if let Some(succ) = sss.succ(&line_b) {
                    is_intersecting_trigger_event(
                        &mut already_intersected,
                        &mut queue,
                        &line_a,
                        &succ,
                    );
                }
            }
        }
    }

    intersections
}
