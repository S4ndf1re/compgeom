use crate::objects::polygon::Vertex;
use std::cmp::Ordering;
use std::collections::VecDeque;

/// Test if p->test->q is a right turn (true)
/// NOTE: this function only operates on 2d at the moment
pub fn is_right_turn(p: Vertex, test: Vertex, q: Vertex) -> bool {
    let a_vec = q - test;
    let b_vec = test - p;

    let a = a_vec.position[0];
    let c = a_vec.position[1];

    let b = b_vec.position[0];
    let d = b_vec.position[1];

    // Account for f32 errors using EPSILON (smallest f32 representable number without error)
    a * d - b * c >= -f32::EPSILON
}

/// Test if p->test->q is a right turn (true)
/// NOTE: this function only operates on 2d at the moment
pub fn is_left_turn(p: Vertex, test: Vertex, q: Vertex) -> bool {
    let a_vec = q - test;
    let b_vec = test - p;

    let a = a_vec.position[0];
    let c = a_vec.position[1];

    let b = b_vec.position[0];
    let d = b_vec.position[1];

    // Account for f32 errors using EPSILON (smallest f32 representable number without error)
    a * d - b * c <= f32::EPSILON
}

/// Compute the convex hull using graham scan, sorting by angle (not x-coordinate)
pub fn graham_scan_by_angle<T: AsRef<[Vertex]>>(points: T) -> Option<Vec<Vertex>> {
    // Complex sorting function, to properly use the minimal y value. if multiple equal min_y values exists, use min x
    let min_y_point = points.as_ref().iter().min_by(|v1, v2| {
        // Copy here, to avoid using unsafe function calls (UB), since position is packed and not properly aligned
        let v1_x: f32 = v1.position[0];
        let v2_x: f32 = v2.position[0];

        let v1_y: f32 = v1.position[1];
        let v2_y: f32 = v2.position[1];
        let ord_x = v1_x.total_cmp(&v2_x);
        let ord_y = v1_y.total_cmp(&v2_y);

        match ord_y {
            Ordering::Equal => ord_x,
            _ => ord_y,
        }
    });

    if min_y_point.is_none() {
        return None;
    }
    let min_y_point = min_y_point.unwrap().clone();

    // Sort by angle
    let mut angled = points
        .as_ref()
        .iter()
        .map(|v| (min_y_point.cosine(v), *v))
        .collect::<Vec<_>>();
    angled.sort_by(|v1, v2| v1.0.total_cmp(&v2.0));

    let mut hull = VecDeque::new();
    hull.push_back(angled[0].1);
    hull.push_back(angled[1].1);

    if angled.len() <= 2 {
        return Some(hull.into());
    }

    for i in 2..angled.len() {
        let next_vertex = angled[i].1;
        loop {
            // Exit condition if hull is not found
            if hull.is_empty() {
                return None;
            }

            if is_left_turn(hull[hull.len() - 2], hull[hull.len() - 1], next_vertex) {
                hull.push_back(next_vertex);
                break;
            } else {
                hull.pop_back();
            }
        }
    }

    Some(hull.into())
}
