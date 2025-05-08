use crate::algorithm::sweep_line::line::Line;
use crate::objects::vertex::Vertex;
use num::Float;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct LineNode<T>
where
    T: Float,
{
    pub cell: Rc<RefCell<Line<T>>>,
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

    pub fn intersects(&self, other: &Self) -> Option<Vertex<T>> {
        self.cell.borrow().intersects(other.cell.borrow().deref())
    }
}
