use std::{cell::RefCell, cmp::Ordering, collections::BTreeMap, fmt::Debug, ops::Bound, rc::Rc};

use num::Float;

use crate::objects::line::Line;

use super::linked_node::LinkedVertex;

#[derive(Debug)]
pub struct SweepLineContext<T> {
    y_value: T,
}

#[derive(Debug)]
pub struct HelperTreeKey<T: Copy + Float> {
    line: Line<T>,
    context: Rc<RefCell<SweepLineContext<T>>>,
}

impl<T: Copy + Float> HelperTreeKey<T> {
    fn new(line: Line<T>, context: Rc<RefCell<SweepLineContext<T>>>) -> Self {
        Self { line, context }
    }
}

impl<T: Copy + Debug + Float + Ord> Eq for HelperTreeKey<T> {}
impl<T: Copy + Debug + Float + Ord> PartialEq for HelperTreeKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T: Copy + Debug + Float + Ord> PartialOrd for HelperTreeKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: Copy + Debug + Float + Ord> Ord for HelperTreeKey<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let x1 = self.line.get_x_for_y(self.context.borrow().y_value);
        let x2 = other.line.get_x_for_y(self.context.borrow().y_value);

        x1.cmp(&x2)
    }
}

#[derive(Debug)]
pub struct Helper<T: Copy + Debug + Float> {
    // a map from edge to helper
    tree: BTreeMap<usize, usize>,
    // a map from x to edge
    edges: BTreeMap<HelperTreeKey<T>, usize>,
    context: Rc<RefCell<SweepLineContext<T>>>,
}

impl<T> Helper<T>
where
    T: Float + Copy + Debug + Ord,
{
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
            edges: BTreeMap::new(),
            context: Rc::new(RefCell::new(SweepLineContext { y_value: T::zero() })),
        }
    }
    pub fn insert_helper(&mut self, id: usize, helper: usize) {
        self.tree.insert(id, helper);
    }

    pub fn insert_helper_and_edge(
        &mut self,
        id: usize,
        helper: usize,
        verticies: &[*mut LinkedVertex<T>],
    ) {
        self.tree.insert(id, helper);

        if let Some(vert) = verticies.get(id) {
            unsafe {
                self.edges.insert(
                    HelperTreeKey::new(
                        Line::new(0, 0, (**vert).vertex, (*(**vert).next.unwrap()).vertex),
                        self.context.clone(),
                    ),
                    id,
                );
            }
        }
    }

    pub fn remove_edge(&mut self, id: usize, verticies: &[*mut LinkedVertex<T>]) {
        self.tree.remove(&id);

        if let Some(vert) = verticies.get(id) {
            unsafe {
                self.edges.remove(&HelperTreeKey::new(
                    Line::new(0, 0, (**vert).vertex, (*(**vert).next.unwrap()).vertex),
                    self.context.clone(),
                ));
            }
        }
    }

    pub fn helper(&self, id: usize) -> usize {
        self.tree[&id]
    }

    /// Return a tuple containig (edge, helper(edge)), that is closest (in x direction) to id
    pub fn range_query(&self, id: usize, verticies: &[*mut LinkedVertex<T>]) -> (usize, usize) {
        if let Some(vert) = verticies.get(id) {
            unsafe {
                let key = HelperTreeKey::new(
                    Line::new(0, 0, (**vert).vertex, (*(**vert).next.unwrap()).vertex),
                    self.context.clone(),
                );

                let max = self
                    .edges
                    .range((Bound::Unbounded, Bound::Excluded(key)))
                    .last()
                    .unwrap();
                return (*max.1, self.helper(*max.1));
            }
        }

        (0, 0)
    }

    pub fn set_y(&self, y: T) {
        self.context.borrow_mut().y_value = y;
    }
}
