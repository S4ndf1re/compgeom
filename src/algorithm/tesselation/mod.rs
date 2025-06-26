mod helper;
pub mod linked_node;

use std::{collections::BinaryHeap, fmt::Debug};

use helper::Helper;
use linked_node::{LinkedVertex, VertexType};
use num::Float;

use crate::objects::{polygon::Polygon, vertex::Vertex};

use super::util::is_right_turn;

/// Return true if a < b
fn compare_points<T: Float + Copy>(a: &Vertex<T>, b: &Vertex<T>) -> bool {
    a.y() < b.y() || a.y() == b.y() && a.x() > b.x()
}

pub fn inner_angle<T: Float + Copy + Debug>(u: &Vertex<T>, v: &Vertex<T>, w: &Vertex<T>) -> T {
    let leg1 = *v - *u;
    let leg2 = *v - *w;

    let mut angle = leg1.cosine(&leg2).acos();

    // Since the polygon where u, v and w are part of is ccw (by definition), all smaller angles (<
    // 180°)
    // must be left turns. Every larger than 180° angles must therefore be right turns
    if is_right_turn(*u, *v, *w) {
        angle = T::from(2.0 * std::f64::consts::PI).unwrap() - angle;
    }

    angle
}

// Classify verticies in place
pub fn classify_verticies_non_pointer<T: Float + Copy + Debug>(
    verticies: &[Vertex<T>],
) -> Vec<(Vertex<T>, VertexType)> {
    let n = verticies.len() as i32;
    let mut result = Vec::new();

    for i in 0..n {
        let u = verticies[((i - 1 + n) % n) as usize];
        let v = verticies[i as usize];
        let w = verticies[((i + 1) % n) as usize];

        if compare_points(&u, &v)
            && compare_points(&w, &v)
            && inner_angle(&u, &v, &w) < T::from(std::f64::consts::PI).unwrap()
        {
            result.push((v, VertexType::Start));
        } else if compare_points(&u, &v)
            && compare_points(&w, &v)
            && inner_angle(&u, &v, &w) > T::from(std::f64::consts::PI).unwrap()
        {
            result.push((v, VertexType::Split));
        } else if compare_points(&v, &u)
            && compare_points(&v, &w)
            && inner_angle(&u, &v, &w) < T::from(std::f64::consts::PI).unwrap()
        {
            result.push((v, VertexType::End));
        } else if compare_points(&v, &u)
            && compare_points(&v, &w)
            && inner_angle(&u, &v, &w) > T::from(std::f64::consts::PI).unwrap()
        {
            result.push((v, VertexType::Merge));
        } else {
            result.push((v, VertexType::Regular));
        }
    }
    result
}

// Classify verticies in place
pub fn classify_verticies<T: Float + Copy + Debug>(verticies: &[*mut LinkedVertex<T>]) {
    unsafe {
        let n = verticies.len();

        for i in 0..n {
            let u = (*(*verticies[i]).prev.unwrap()).vertex;
            let v = (*verticies[i]).vertex;
            let w = (*(*verticies[i]).next.unwrap()).vertex;

            if compare_points(&u, &v)
                && compare_points(&w, &v)
                && inner_angle(&u, &v, &w) < T::from(std::f64::consts::PI).unwrap()
            {
                (*verticies[i]).vert_type = VertexType::Start;
            } else if compare_points(&u, &v)
                && compare_points(&w, &v)
                && inner_angle(&u, &v, &w) > T::from(std::f64::consts::PI).unwrap()
            {
                (*verticies[i]).vert_type = VertexType::Split;
            } else if compare_points(&v, &u)
                && compare_points(&v, &w)
                && inner_angle(&u, &v, &w) < T::from(std::f64::consts::PI).unwrap()
            {
                (*verticies[i]).vert_type = VertexType::End;
            } else if compare_points(&v, &u)
                && compare_points(&v, &w)
                && inner_angle(&u, &v, &w) > T::from(std::f64::consts::PI).unwrap()
            {
                (*verticies[i]).vert_type = VertexType::Merge;
            } else {
                (*verticies[i]).vert_type = VertexType::Regular;
            }

            println!("{u:?}, {v:?}, {w:?}");
            println!("{:?}", inner_angle(&u, &v, &w));
            println!("{:?}", (*verticies[i]).vert_type);
            println!();
            println!();
        }
    }
}

fn is_inner_right<T: Float + Copy + Debug + Ord>(vertex: *mut LinkedVertex<T>) -> bool {
    // TODO: Figure out if this is actually the correct way to check if is right or left
    unsafe { (*(*vertex).prev.unwrap()).vertex.y() > (*vertex).vertex.y() }
}

/// Partition a list of verticies into a list of indicies. Each sublist contains a single
/// y-monotone polygon
pub fn partition_to_y_monotone<T: Float + Copy + Debug + Ord>(
    verticies: &[Vertex<T>],
) -> Vec<Vec<Vertex<T>>> {
    let mut helper = Helper::new();

    // assure, that the numbering starts at first idx
    let mut verticies = verticies.to_owned();
    let (max_idx, _) = verticies
        .iter()
        .enumerate()
        .max_by_key(|a| (a.1.y(), -a.1.x()))
        .unwrap();
    verticies.rotate_left(max_idx);

    #[allow(clippy::needless_range_loop)]
    for i in 0..verticies.len() {
        verticies[i].id = i;
    }
    println!("Verticies: {verticies:?}");

    let mut verticies = LinkedVertex::from_verticies(&verticies);

    classify_verticies(&verticies);

    let mut heap = BinaryHeap::new();
    for vertex in verticies.iter() {
        unsafe {
            heap.push((((**vertex).vertex.y(), -(**vertex).vertex.x()), *vertex));
        }
    }

    while !heap.is_empty() {
        unsafe {
            let (_, v) = heap.pop().unwrap();
            helper.set_y((*v).vertex.y());
            let id = (*v).index;
            println!(
                "Iterating over vertex with id: {id} of type: {:?}",
                (*v).vert_type
            );
            println!("State before: {helper:?}");

            match (*v).vert_type {
                VertexType::Start => {
                    helper.insert_helper_and_edge(id, id, &verticies);
                }
                VertexType::End => {
                    let prev = (*(*v).prev.unwrap()).index;
                    if (*verticies[helper.helper(prev)]).vert_type == VertexType::Merge {
                        // edges.push((
                        //     (*v).vertex.id,
                        //     (*verticies[helper.helper(id - 1)]).vertex.id,
                        // ));
                        LinkedVertex::insert_between(
                            v,
                            verticies[helper.helper(prev)],
                            VertexType::Merge,
                            &mut verticies,
                        );
                    }

                    helper.remove_edge(prev, &verticies);
                }
                VertexType::Split => {
                    let (e_j, e_j_h) = helper.range_query(id, &verticies);
                    println!("Found edge {e_j} with helper {e_j_h}");

                    // edges.push(((*v).vertex.id, (*verticies[e_j_h]).vertex.id));
                    let (temp1, _) = LinkedVertex::insert_between(
                        v,
                        verticies[e_j_h],
                        VertexType::Split,
                        &mut verticies,
                    );

                    helper.insert_helper(e_j, id);
                    helper.insert_helper_and_edge((*temp1).index, (*temp1).index, &verticies);
                }
                VertexType::Merge => {
                    let prev = (*(*v).prev.unwrap()).index;
                    if (*verticies[helper.helper(prev)]).vert_type == VertexType::Merge {
                        // edges.push((
                        //     (*v).vertex.id,
                        //     (*verticies[helper.helper(id - 1)]).vertex.id,
                        // ));
                        let (v2, _) = LinkedVertex::insert_between(
                            v,
                            verticies[helper.helper(prev)],
                            VertexType::Merge,
                            &mut verticies,
                        );
                    }

                    helper.remove_edge(prev, &verticies);
                    let (e_j, e_j_h) = helper.range_query(id, &verticies);
                    println!("Found edge {e_j} with helper {e_j_h}");
                    if (*verticies[e_j_h]).vert_type == VertexType::Merge {
                        // edges.push(((*v).vertex.id, (*verticies[e_j_h]).vertex.id));
                        LinkedVertex::insert_between(
                            v,
                            verticies[e_j_h],
                            VertexType::Split,
                            &mut verticies,
                        );
                    }

                    helper.insert_helper(e_j, id);
                }
                VertexType::Regular => {
                    if is_inner_right(v) {
                        let prev = (*(*v).prev.unwrap()).index;
                        println!("Prev idx : {prev}");
                        if (*verticies[helper.helper(prev)]).vert_type == VertexType::Merge {
                            // edges.push((
                            //     (*v).vertex.id,
                            //     (*verticies[helper.helper(id - 1)]).vertex.id,
                            // ));
                            LinkedVertex::insert_between(
                                v,
                                verticies[helper.helper(prev)],
                                VertexType::Merge,
                                &mut verticies,
                            );
                        }
                        helper.remove_edge(prev, &verticies);
                        helper.insert_helper_and_edge(id, id, &verticies);
                    } else {
                        let (e_j, e_j_h) = helper.range_query(id, &verticies);
                        println!("Found edge {e_j} with helper {e_j_h}");
                        if (*verticies[e_j_h]).vert_type == VertexType::Merge {
                            // edges.push(((*v).vertex.id, (*verticies[e_j_h]).vertex.id));
                            LinkedVertex::insert_between(
                                v,
                                verticies[e_j_h],
                                VertexType::Split,
                                &mut verticies,
                            );
                        }
                        helper.insert_helper(e_j, id);
                    }
                }
            }
            println!("State after: {helper:?}");
        }
    }

    // Collect linked list into subpolygons
    let mut result = Vec::new();
    for v in verticies.iter() {
        let v = *v;
        unsafe {
            if (*v).visited {
                continue;
            }

            let start = v;
            let mut contained_verticies = vec![(*start).vertex];
            (*start).visited = true;
            let mut current = (*start).next;

            while let Some(next_vertex) = current
                && !(*next_vertex).visited
            {
                let next_vertex = next_vertex as *mut LinkedVertex<T>;
                contained_verticies.push((*next_vertex).vertex);
                (*next_vertex).visited = true;
                current = (*next_vertex).next;
            }

            result.push(contained_verticies);
        }
    }

    // clean up memory, since the algorithm used leaked raw pointers
    for v in verticies {
        unsafe {
            drop(Box::from_raw(v));
        }
    }

    result
}

pub fn is_on_left<T: Float + Copy + Debug>(
    a: *const LinkedVertex<T>,
    b: *const LinkedVertex<T>,
) -> bool {
    unsafe { std::ptr::addr_eq((*a).next.unwrap(), b) }
}

pub fn is_on_right<T: Float + Copy + Debug>(
    a: *const LinkedVertex<T>,
    b: *const LinkedVertex<T>,
) -> bool {
    unsafe { std::ptr::addr_eq((*a).prev.unwrap(), b) }
}

pub fn is_same_side<T: Float + Copy + Debug>(
    a: *const LinkedVertex<T>,
    b: *const LinkedVertex<T>,
) -> bool {
    is_on_left(a, b) || is_on_right(a, b)
}

pub fn triangulate_y_monotone_polygon<T: Float + Copy + Debug + Ord>(
    polygon: &[Vertex<T>],
) -> Vec<Vec<Vertex<T>>> {
    let mut linked_nodes = LinkedVertex::from_verticies(polygon);

    unsafe {
        linked_nodes.sort_by_key(|a| (**a).vertex.y());
    }
    let n = polygon.len();

    let mut stack = Vec::with_capacity(n);
    stack.push(linked_nodes[0]);
    stack.push(linked_nodes[1]);

    let mut triangles = Vec::new();

    for i in 2..n - 1 {
        let v = linked_nodes[i];
        if is_same_side(*stack.last().unwrap(), v) {
            let mut last_popped = stack.pop().unwrap();
            let is_on_left = is_on_left(last_popped, v);
            while let Some(popped) = stack.last() {
                unsafe {
                    if (is_on_left
                        && inner_angle(&(**popped).vertex, &(*last_popped).vertex, &(*v).vertex)
                            < T::from(std::f64::consts::PI).unwrap())
                        || (!is_on_left
                            && inner_angle(
                                &(*v).vertex,
                                &(*last_popped).vertex,
                                &(**popped).vertex,
                            ) < T::from(std::f64::consts::PI).unwrap())
                    {
                        let mut poly = Polygon::new(vec![
                            (*v).vertex,
                            (*last_popped).vertex,
                            (**popped).vertex,
                        ]);
                        poly.ensure_ccw();

                        last_popped = stack.pop().unwrap();
                        triangles.push(poly.vertices);
                    } else {
                        break;
                    }
                }
            }
            stack.push(last_popped);
            stack.push(v);
        } else {
            while let Some(value) = stack.pop() {
                unsafe {
                    if stack.last().is_some() {
                        let mut poly = Polygon::new(vec![
                            (*v).vertex,
                            (*value).vertex,
                            (**stack.last().unwrap()).vertex,
                        ]);
                        poly.ensure_ccw();
                        triangles.push(poly.vertices);
                    }
                }
            }
            stack.push(linked_nodes[((i as i32) - 1) as usize]);
            stack.push(linked_nodes[i]);
        }
    }

    let last = linked_nodes.last().unwrap();

    // add final triangle
    let mut helper = stack[0];
    for vert in stack[1..((stack.len() as i32) - 1) as usize].iter() {
        unsafe {
            let mut poly = Polygon::new(vec![(**last).vertex, (*helper).vertex, (**vert).vertex]);
            poly.ensure_ccw();

            triangles.push(poly.vertices);
            helper = *vert;
        }
    }
    // connect last triangle
    unsafe {
        let mut poly = Polygon::new(vec![
            (**last).vertex,
            (*helper).vertex,
            (**stack.last().unwrap()).vertex,
        ]);
        poly.ensure_ccw();
        triangles.push(poly.vertices);
    }

    // Clean up memory
    for n in linked_nodes {
        unsafe {
            drop(Box::from_raw(n));
        }
    }

    triangles
}
