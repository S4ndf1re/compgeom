use crate::algorithm::sweep_line::context::SweepLineContext;
use crate::algorithm::sweep_line::events::{EventStructure, SweepLineEvent};
use crate::algorithm::sweep_line::line::Line;
use crate::algorithm::sweep_line::line_storage::SweepLineStateStructure;
use crate::objects::vertex::Vertex;
use num::Float;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::rc::Rc;

/// Compute the possible line intersection of line1 and line2. When intersection is found and was not previously computed, add intersection to queue
pub fn is_intersecting_trigger_event<T: Float + Ord + Copy + Debug>(
    intersecting_store: &mut HashSet<(usize, usize)>,
    queue: &mut EventStructure<T>,
    line1: &Line<T>,
    line2: &Line<T>,
) {
    if line1.id == 5 && line2.id == 4 || line1.id == 4 && line2.id == 5 {
        println!("Debug here");
    }
    if intersecting_store.get(&(line1.id, line2.id)).is_some() {
        return;
    }

    if let Some(vert) = line1.intersects(line2) {
        queue.add_event(
            vert.x(),
            SweepLineEvent::Intersection(line1.clone(), line2.clone(), vert),
        );

        intersecting_store.insert((line1.id, line2.id));
        intersecting_store.insert((line2.id, line1.id));
    }
}

pub fn sweep_line_intersections<T: Float + Ord + Debug, L: AsRef<[Line<T>]>>(
    lines: L,
) -> Vec<(Vertex<T>, Line<T>, Line<T>)> {
    let context = Rc::new(RefCell::new(SweepLineContext {
        is_intersection: false,
        line_id1: 0,
        line_id2: 0,
        x_pos: T::zero(),
        relative_order: HashMap::new(),
    }));
    let mut sss = SweepLineStateStructure::new();
    let mut queue = EventStructure::new();

    // Infuse context into line (x_pos) wrap line into swappable line node
    let lines = lines
        .as_ref()
        .iter()
        .map(|line| {
            let mut line = line.clone();
            line.infuse_context(context.clone());
            line
        })
        .collect::<Vec<_>>();

    lines.iter().for_each(|line| {
        queue.add_event(line.x1.x(), SweepLineEvent::LineStart(line.clone()));
        queue.add_event(line.x2.x(), SweepLineEvent::LineEnd(line.clone()));
    });

    let mut already_intersected: HashSet<(usize, usize)> = HashSet::new();
    let mut intersections: Vec<(Vertex<T>, Line<T>, Line<T>)> = Vec::new();

    while let Some(event) = queue.next_event() {
        let x = event.0;
        context.borrow_mut().x_pos = x;
        context.borrow_mut().is_intersection = false;

        match event.1 {
            SweepLineEvent::LineStart(line) => {
                println!("Start {:?}", line);
                if !line.is_vertical() {
                    sss.insert_line(line.clone());
                    if let Some(pred) = sss.pred(&line) {
                        is_intersecting_trigger_event(
                            &mut already_intersected,
                            &mut queue,
                            &pred.cell.borrow_mut(),
                            &line,
                        );
                    }

                    if let Some(succ) = sss.succ(&line) {
                        is_intersecting_trigger_event(
                            &mut already_intersected,
                            &mut queue,
                            &line,
                            &succ.cell.borrow(),
                        );
                    }
                } else {
                    for l in sss.iter() {
                        is_intersecting_trigger_event(
                            &mut already_intersected,
                            &mut queue,
                            &line,
                            &l.cell.borrow_mut(),
                        );
                    }
                }
            }
            SweepLineEvent::LineEnd(line) => {
                println!("End {:?}", line);
                if !line.is_vertical() {
                    let pred = sss.pred(&line);
                    let succ = sss.succ(&line);
                    sss.remove_line(&line);

                    if pred.is_some() && succ.is_some() {
                        is_intersecting_trigger_event(
                            &mut already_intersected,
                            &mut queue,
                            &pred.unwrap().cell.borrow(),
                            &succ.unwrap().cell.borrow(),
                        );
                    }
                }
            }
            SweepLineEvent::Intersection(line_a, line_b, vert) => {
                println!("Intersection {:?}, {:?}", line_a, line_b);
                context.borrow_mut().is_intersection = true;
                context.borrow_mut().line_id1 = line_a.id;
                context.borrow_mut().line_id2 = line_b.id;
                intersections.push((vert, line_a.clone(), line_b.clone()));

                if !line_a.is_vertical() && !line_b.is_vertical() {
                    // Only exchange, if a line segment is starting. In this case, the lines are already ordered correctly
                    if line_a.get_t_for_context() > T::epsilon()
                        && line_b.get_t_for_context() > T::epsilon()
                    {
                        sss.exchange(&context, &line_a, &line_b);
                    }
                    sss.print_in_order();
                    println!("Exchanged");

                    if let Some(pred) = sss.pred(&line_b) {
                        is_intersecting_trigger_event(
                            &mut already_intersected,
                            &mut queue,
                            &pred.cell.borrow(),
                            &line_b,
                        );
                    }

                    if let Some(succ) = sss.succ(&line_a) {
                        is_intersecting_trigger_event(
                            &mut already_intersected,
                            &mut queue,
                            &line_a,
                            &succ.cell.borrow(),
                        );
                    }

                    // if let Some(pred) = sss.pred(&line_a) {
                    //     is_intersecting_trigger_event(
                    //         &mut already_intersected,
                    //         &mut queue,
                    //         &pred.cell.borrow(),
                    //         &line_a,
                    //     );
                    // }
                    //
                    // if let Some(succ) = sss.succ(&line_b) {
                    //     is_intersecting_trigger_event(
                    //         &mut already_intersected,
                    //         &mut queue,
                    //         &line_b,
                    //         &succ.cell.borrow(),
                    //     );
                    // }
                } else {
                    println!("LIne is vertical");
                }
            }
        }
        sss.print_in_order();
    }

    intersections
}
