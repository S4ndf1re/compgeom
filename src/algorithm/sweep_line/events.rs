use crate::algorithm::sweep_line::line_node::LineNode;
use crate::objects::vertex::Vertex;
use num::Float;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Ord, PartialOrd, PartialEq, Eq)]
pub enum SweepLineEvent<T>
where
    T: Float,
{
    LineStart(LineNode<T>),
    LineEnd(LineNode<T>),
    Intersection(LineNode<T>, LineNode<T>, Vertex<T>),
}

pub struct EventStructure<T>
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
