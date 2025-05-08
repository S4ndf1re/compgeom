use crate::algorithm::sweep_line::line_node::LineNode;
use num::Float;
use std::collections::{BTreeSet, Bound};

pub struct SweepLineStateStructure<T>
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
