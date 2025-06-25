use std::{collections::BTreeMap, fmt::Debug, ops::Bound};

use num::Float;

use crate::objects::line::Line;

use super::linked_node::LinkedVertex;

#[derive(Debug)]
pub struct Helper<T: Copy + Debug + Float> {
    // a map from edge to helper
    tree: BTreeMap<usize, usize>,
    // a map from x to edge
    edges: Vec<(Line<T>, usize)>,
}

impl<T> Helper<T>
where
    T: Float + Copy + Debug + Ord,
{
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
            edges: Vec::new(),
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
                self.edges.push((
                    Line::new(0, 0, (**vert).vertex, (*(**vert).next.unwrap()).vertex),
                    id,
                ));
            }
        }
    }

    pub fn remove_edge(&mut self, id: usize, verticies: &[*mut LinkedVertex<T>]) {
        self.tree.remove(&id);

        if let Some(vert) = verticies.get(id) {
            unsafe {
                if let Some(idx) = self
                    .edges
                    .iter()
                    .position(|v| v.0.get_x_for_y((**vert).vertex.y()) == (**vert).vertex.x())
                {
                    self.edges.remove(idx);
                }
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
                if let Some(max) = self
                    .edges
                    .iter()
                    .filter(|v| v.0.get_x_for_y((**vert).vertex.y()) < (**vert).vertex.x())
                    .max_by_key(|v| v.0.get_x_for_y((**vert).vertex.y()))
                {
                    return (max.1, self.helper(max.1));
                }
            }
        }

        (0, 0)
    }
}
