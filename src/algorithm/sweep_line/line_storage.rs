use crate::algorithm::sweep_line::context::SweepLineContext;
use crate::algorithm::sweep_line::line::Line;
use crate::algorithm::sweep_line::line_node::LineNode;
use num::Float;
use std::cell::RefCell;
use std::collections::btree_set::Iter;
use std::collections::{BTreeSet, Bound};
use std::fmt::Debug;
use std::ops::Deref;

pub struct SweepLineStateStructure<T>
where
    T: Float + Debug,
{
    container: BTreeSet<LineNode<T>>,
}

impl<T> SweepLineStateStructure<T>
where
    T: Float + Ord + Debug,
{
    pub fn new() -> Self {
        Self {
            container: BTreeSet::new(),
        }
    }

    pub fn insert_line(&mut self, line: Line<T>) {
        self.container.insert(LineNode::new(line));
    }

    pub fn remove_line(&mut self, line: &Line<T>) {
        self.container.remove(&LineNode::new(line.clone()));
    }

    pub fn exchange(
        &mut self,
        context: &RefCell<SweepLineContext<T>>,
        line1: &Line<T>,
        line2: &Line<T>,
    ) {
        let line1_node = self.container.get(&LineNode::new(line1.clone())).unwrap();
        let line2_node = self.container.get(&LineNode::new(line2.clone())).unwrap();

        if line1_node.cell.borrow().id != line2_node.cell.borrow().id {
            line1_node.cell.swap(&line2_node.cell);
        }

        // swap compare order in context
        context.borrow_mut().exchange_order(&line1.id, &line2.id);
    }

    pub fn pred(&self, line: &Line<T>) -> Option<LineNode<T>> {
        let range = self.container.range((
            Bound::Unbounded,
            Bound::Excluded(&LineNode::new(line.clone())),
        ));

        range.last().map(|l| l.clone())
    }

    pub fn succ(&self, line: &Line<T>) -> Option<LineNode<T>> {
        let range = self.container.range((
            Bound::Excluded(&LineNode::new(line.clone())),
            Bound::Unbounded,
        ));

        range.rev().last().map(|n| n.clone())
    }

    pub fn print_in_order(&self) {
        for line in self.container.iter() {
            let line = line.cell.borrow().deref().clone();
            print!("{:?} , ", line);
        }
        println!();
    }

    pub fn iter(&self) -> Iter<LineNode<T>> {
        self.container.iter()
    }
}
