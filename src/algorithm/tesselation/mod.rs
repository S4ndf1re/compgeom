pub mod linked_node;

use std::{
    cell::RefCell,
    collections::{BTreeMap, BinaryHeap},
    fmt::Debug,
    rc::Rc,
};

use linked_node::{LinkedVertex, VertexType};
use num::Float;

use crate::{algorithm::util::is_left_turn, objects::vertex::Vertex};

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

fn helper<T: Float + Copy + Debug>(
    key: &usize,
    tree: &BTreeMap<usize, usize>,
    verticies: &[*mut LinkedVertex<T>],
) -> usize {
    if let Some(value) = tree.get(key) {
        *value
    } else {
        unsafe {
            if (*verticies[*key]).vertex.y() > (*(*verticies[*key]).next.unwrap()).vertex.y() {
                (*verticies[*key]).vertex.id
            } else {
                (*(*verticies[*key]).next.unwrap()).vertex.id
            }
        }
    }
}

/// Partition a list of verticies into a list of indicies. Each sublist contains a single
/// y-monotone polygon
pub fn partition_to_y_monotone<T: Float + Copy + Debug + Ord>(
    verticies: &[Vertex<T>],
) -> Vec<Vec<Vertex<T>>> {
    let mut edges = Vec::with_capacity(verticies.len());
    let mut helper_tree = BTreeMap::new();

    // assure, that the numbering starts at first idx
    let mut verticies = verticies.to_owned();
    let (max_idx, _) = verticies
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.y().cmp(&b.1.y()))
        .unwrap();
    verticies.rotate_left(max_idx);

    let n = verticies.len();
    #[allow(clippy::needless_range_loop)]
    for i in 0..verticies.len() {
        verticies[i].id = i;
    }

    let mut verticies = LinkedVertex::from_verticies(&verticies);
    for i in 0..n {
        edges.push((i, verticies[i], verticies[(i + 1) % n]));
    }

    classify_verticies(&verticies);

    let mut heap = BinaryHeap::new();
    for vertex in verticies.iter() {
        unsafe {
            heap.push(((**vertex).vertex.y(), *vertex));
        }
    }

    while !heap.is_empty() {
        unsafe {
            let (_, v) = heap.pop().unwrap();
            let id = (*v).vertex.id;
            println!(
                "Iterating over vertex with id: {id} of type: {:?}",
                (*v).vert_type
            );

            println!("Tree before: {helper_tree:?}");

            match (*v).vert_type {
                VertexType::Start => {
                    helper_tree.insert(id, id);
                }
                VertexType::End => {
                    if (*verticies[helper(&(id - 1), &helper_tree, &verticies)]).vert_type
                        == VertexType::Merge
                    {
                        let (temp1, temp2) =
                            LinkedVertex::insert_between(v, verticies[helper_tree[&(id - 1)]]);
                        verticies.push(temp1);
                        verticies.push(temp2);
                    }
                    helper_tree.remove(&(id - 1));
                }
                VertexType::Split => {
                    let lesser_edge_range = helper_tree.range(0..id);
                    if let Some((e_j, e_j_h)) = lesser_edge_range.last() {
                        let (temp1, temp2) = LinkedVertex::insert_between(v, verticies[*e_j_h]);
                        verticies.push(temp1);
                        verticies.push(temp2);

                        helper_tree.insert(*e_j, id);
                        helper_tree.insert(id, id);
                    }
                }
                VertexType::Merge => {
                    if (*verticies[helper(&(id - 1), &helper_tree, &verticies)]).vert_type
                        == VertexType::Merge
                    {
                        LinkedVertex::insert_between(
                            v,
                            verticies[helper(&(id - 1), &helper_tree, &verticies)],
                        );
                    }

                    helper_tree.remove(&(id - 1));
                    let lesser_edge_range = helper_tree.range(0..id);
                    if let Some((e_j, e_j_h)) = lesser_edge_range.last() {
                        println!("Found {e_j}");
                        if (*verticies[*e_j_h]).vert_type == VertexType::Merge {
                            let (temp1, temp2) = LinkedVertex::insert_between(verticies[*e_j_h], v);
                            verticies.push(temp1);
                            verticies.push(temp2);
                        }
                        helper_tree.insert(*e_j, id);
                    }
                }
                VertexType::Regular => {
                    if is_inner_right(v) {
                        if (*verticies[helper(&(id - 1), &helper_tree, &verticies)]).vert_type
                            == VertexType::Merge
                        {
                            let (temp1, temp2) = LinkedVertex::insert_between(
                                v,
                                verticies[helper(&(id - 1), &helper_tree, &verticies)],
                            );
                            verticies.push(temp1);
                            verticies.push(temp2);
                        }
                        helper_tree.remove(&(id - 1));
                        helper_tree.insert(id, id);
                    } else {
                        let lesser_edge_range = helper_tree.range(0..id);
                        if let Some((e_j, e_j_h)) = lesser_edge_range.last() {
                            if (*verticies[*e_j_h]).vert_type == VertexType::Merge {
                                let (temp1, temp2) =
                                    LinkedVertex::insert_between(verticies[*e_j_h], v);
                                verticies.push(temp1);
                                verticies.push(temp2);
                            }
                            helper_tree.insert(*e_j, id);
                        }
                    }
                }
            }
            println!("Tree after: {helper_tree:?}");
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

pub fn triangulate_y_monotone_polygon<T: Float + Copy + Debug + Ord>(
    mut polygon: Vec<Vertex<T>>,
) -> Vec<Vec<Vertex<T>>> {
    polygon.sort_by_key(|a| a.y());
    let n = polygon.len();

    let mut stack = Vec::with_capacity(n);
    stack.push(polygon[0]);
    stack.push(polygon[1]);

    for i in 2..n - 1 {}

    todo!()
}
