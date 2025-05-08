use crate::algorithm::sweep_line::context::SweepLineContext;
use crate::algorithm::sweep_line::events::{EventStructure, SweepLineEvent};
use crate::algorithm::sweep_line::line::Line;
use crate::algorithm::sweep_line::line_node::LineNode;
use crate::algorithm::sweep_line::line_storage::SweepLineStateStructure;
use num::Float;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::rc::Rc;

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

    if let Some(vert) = line1.intersects(line2) {
        queue.add_event(
            vert.x(),
            SweepLineEvent::Intersection(line1.clone(), line2.clone(), vert),
        );
    }
}

pub fn sweep_line_intersections<T: Float + Ord, L: AsRef<[Line<T>]>>(
    lines: L,
) -> Vec<(T, T, Line<T>, Line<T>)> {
    let context = Rc::new(RefCell::new(SweepLineContext { x_pos: T::zero() }));
    let mut sss = SweepLineStateStructure::new();
    let mut queue = EventStructure::new();

    let lines = lines
        .as_ref()
        .iter()
        .enumerate()
        .map(|line| LineNode::new(line.1.clone()))
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
            SweepLineEvent::Intersection(line_a, line_b, vert) => {
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
