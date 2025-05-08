use std::cmp::Ordering;
use std::collections::HashMap;

pub struct SweepLineContext<T> {
    pub is_intersection: bool,
    pub line_id1: usize,
    pub line_id2: usize,
    pub x_pos: T,
    pub relative_order: HashMap<usize, HashMap<usize, Ordering>>,
}

impl<T> SweepLineContext<T> {
    pub fn get_order(&self, id1: &usize, id2: &usize) -> Option<Ordering> {
        Some(self.relative_order.get(&id1)?.get(id2)?.clone())
    }

    pub fn set_order(&mut self, id1: usize, id2: usize, order: Ordering) {
        let entry1 = self.relative_order.entry(id1).or_insert(HashMap::new());
        *entry1.entry(id2).or_insert(order) = order;

        let entry2 = self.relative_order.entry(id2).or_insert(HashMap::new());
        *entry2.entry(id1).or_insert(order) = order.reverse();
    }

    pub fn exchange_order(&mut self, id1: &usize, id2: &usize) {
        let order1 = self.get_order(id1, id2);
        let order2 = self.get_order(id2, id1);

        if order1.is_some() && order2.is_some() {
            *self
                .relative_order
                .get_mut(id1)
                .unwrap()
                .get_mut(id2)
                .unwrap() = order2.unwrap();

            *self
                .relative_order
                .get_mut(id2)
                .unwrap()
                .get_mut(id1)
                .unwrap() = order1.unwrap();
        }
    }
}
