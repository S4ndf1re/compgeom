use crate::objects::vertex::Vertex;
use num::Float;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use crate::objects::line::Line;

#[derive(Ord, PartialOrd, PartialEq, Eq)]
pub enum SweepLineEvent<T>
where
    T: Float + Debug,
{
    LineStart(Line<T>),
    Intersection(Line<T>, Line<T>, Vertex<T>),
    LineEnd(Line<T>),
}

pub struct EventStructure<T>
where
    T: Float + Debug,
{
    // Construct min heap
    queue: BinaryHeap<Reverse<(T, SweepLineEvent<T>)>>,
}

impl<T> EventStructure<T>
where
    T: Float + Ord + Copy + Debug,
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
