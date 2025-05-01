use crate::objects::polygon::Vertex;
use num::Float;
use std::cmp::Ordering;

/// Test if p->test->q is a right turn (true)
/// NOTE: this function only operates on 2d at the moment
pub fn is_right_turn<T: Float + Copy>(p: Vertex<T>, test: Vertex<T>, q: Vertex<T>) -> bool {
    let a_vec = q - test;
    let b_vec = test - p;

    let a = a_vec.position[0];
    let c = a_vec.position[1];

    let b = b_vec.position[0];
    let d = b_vec.position[1];

    let det = a * d - b * c;

    det >= T::zero()
}

/// Test if p->test->q is a right turn (true)
/// NOTE: this function only operates on 2d at the moment
pub fn is_left_turn<T: Float + Copy>(p: Vertex<T>, test: Vertex<T>, q: Vertex<T>) -> bool {
    let a_vec = q - test;
    let b_vec = test - p;

    let a = a_vec.position[0];
    let c = a_vec.position[1];

    let b = b_vec.position[0];
    let d = b_vec.position[1];

    let det = a * d - b * c;

    det <= T::zero()
}

/// Compute the convex hull using graham scan, sorting by angle (not x-coordinate)
pub fn graham_scan_by_angle<T: Float + Copy, P: AsRef<[Vertex<T>]>>(
    points: P,
) -> Option<Vec<Vertex<T>>> {
    let indices: Vec<usize> = points.as_ref().iter().enumerate().map(|(i, _)| i).collect();

    // Complex sorting function, to properly use the minimal y value. if multiple equal min_y values exists, use min x
    let min_y_point = *indices.iter().min_by(|idx_1, idx_2| {
        // Copy here, to avoid using unsafe function calls (UB), since position is packed and not properly aligned
        let v1 = points.as_ref()[**idx_1].position;
        let v2 = points.as_ref()[**idx_2].position;

        let ord_x = v1[0].partial_cmp(&v2[0]).unwrap();
        let ord_y = v1[1].partial_cmp(&v2[1]).unwrap();

        match ord_y {
            Ordering::Equal => ord_x,
            _ => ord_y,
        }
    })?;

    let x_axis = Vertex {
        position: [T::one(), T::zero(), T::zero()],
        color: [0.0, 0.0, 0.0],
    };

    // Sort by angle, ignoring min_y_point, since the is point always part of hull
    let mut angled = indices
        .iter()
        .filter_map(|v| {
            if *v == min_y_point {
                None
            } else {
                Some((
                    x_axis
                        .cosine(&(points.as_ref()[*v] - points.as_ref()[min_y_point]))
                        .acos(),
                    *v,
                ))
            }
        })
        .collect::<Vec<_>>();

    // Sort by angle, if multiple angles are the same, order by magnitude. Note, that larger is more important, meaning it should get sorted first.
    // This also means, that we have to invert the sort order for the magnitude
    angled.sort_by(|v1, v2| {
        let cosine = v1.0.partial_cmp(&v2.0).unwrap();

        if cosine == Ordering::Equal {
            points.as_ref()[v2.1]
                .magnitude()
                .partial_cmp(&points.as_ref()[v1.1].magnitude())
                .unwrap()
        } else {
            cosine
        }
    });

    let mut hull = Vec::new();
    hull.push(points.as_ref()[min_y_point]);
    hull.push(points.as_ref()[angled[0].1]);

    if angled.len() <= 2 {
        return Some(hull.into());
    }

    for i in 1..angled.len() {
        let next_vertex = points.as_ref()[angled[i].1];
        while hull.len() >= 2
            && !is_left_turn(hull[hull.len() - 2], hull[hull.len() - 1], next_vertex)
        {
            hull.pop();
        }
        hull.push(next_vertex);
    }

    Some(hull)
}

enum Iteration {
    Upper,
    Lower,
}

pub fn graham_scan_by_x<T: Float + Copy, P: AsRef<[Vertex<T>]>>(
    points: P,
) -> Option<Vec<Vertex<T>>> {
    let indizes: Vec<usize> = points.as_ref().iter().enumerate().map(|(i, _)| i).collect();

    let min_x = *indizes.iter().min_by(|p1, p2| {
        let x1 = points.as_ref()[**p1].position[0];
        let x2 = points.as_ref()[**p2].position[0];

        x1.partial_cmp(&x2).unwrap()
    })?;

    let max_x = *indizes.iter().max_by(|p1, p2| {
        let x1 = points.as_ref()[**p1].position[0];
        let x2 = points.as_ref()[**p2].position[0];

        x1.partial_cmp(&x2).unwrap()
    })?;

    let diff = points.as_ref()[max_x] - points.as_ref()[min_x];
    let normal_upper_lower_separator = Vertex {
        position: [-diff.position[1], diff.position[0], T::zero()],
        color: [0.0, 0.0, 0.0],
    };
    let midpoint = points.as_ref()[min_x] * T::from(0.5).unwrap()
        + points.as_ref()[max_x] * T::from(0.5).unwrap();

    #[rustfmt::skip]
    let mut x_mapped: Vec<(T, usize)> = indizes.iter().filter_map(|idx| {
            if *idx == min_x { None } else { Some((points.as_ref()[*idx].position[0], *idx)) }
        }).collect();

    x_mapped.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Split up in upper and lower half
    #[rustfmt::skip]
    let upper = x_mapped.iter().filter(|idx| {
            normal_upper_lower_separator.cosine(&(points.as_ref()[idx.1] - midpoint)) >= T::zero()
        }).collect::<Vec<_>>();

    let tmp = (T::zero(), min_x);
    #[rustfmt::skip]
    let mut lower = x_mapped.iter().filter(|idx| {
            normal_upper_lower_separator.cosine(&(points.as_ref()[idx.1] - midpoint)) < T::zero()
        }).collect::<Vec<_>>();
    lower.reverse();
    lower.push(&tmp); // Use tmp to avoid borrow checker from rust, push first min_x to recheck final shape

    let mut hull = Vec::new();
    hull.push(points.as_ref()[min_x]);
    if !upper.is_empty() {
        hull.push(points.as_ref()[upper[0].1]);
    } else if !lower.is_empty() {
        hull.push(points.as_ref()[lower[0].1]);
    }

    if x_mapped.len() <= 2 {
        return Some(hull.into());
    }

    let mut slice = &upper[..];
    let mut iteration = Iteration::Upper;
    // The loop below will execute twice. Once with slice = upper, the second time with slice = lower. Avoid code duplication
    for _ in 0..2 {
        for i in 1..slice.len() {
            let next_vertex = points.as_ref()[slice[i].1];
            while hull.len() >= 2
                && !is_right_turn(hull[hull.len() - 2], hull[hull.len() - 1], next_vertex)
            {
                hull.pop();
            }
            hull.push(next_vertex);
        }
        match iteration {
            Iteration::Upper => {
                slice = &lower[..];
                // Also, add the next point, since we have to look at the bottom part
                if !slice.is_empty() {
                    hull.push(points.as_ref()[slice[0].1]);
                }

                iteration = Iteration::Lower;
            }
            _ => (),
        }
    }

    // Remove last point, since the last point == first point
    hull.pop();
    Some(hull)
}

pub fn graham_scan_vorlesungsfolie<T: Float + Copy, P: AsRef<[Vertex<T>]>>(
    points: P,
) -> Option<Vec<Vertex<T>>> {
    if points.as_ref().is_empty() {
        return None;
    }

    if points.as_ref().len() <= 2 {
        return Some(points.as_ref().to_vec());
    }

    let mut list = points.as_ref().to_vec();
    list.sort_by(|a, b| {
        let a_x = a.position[0];
        let b_x = b.position[0];
        a_x.partial_cmp(&b_x).unwrap()
    });

    let mut upper_hull = vec![];
    let mut lower_hull = vec![];

    upper_hull.push(list[0]);
    upper_hull.push(list[1]);

    lower_hull.push(list[list.len() - 1]);
    lower_hull.push(list[list.len() - 2]);

    for i in 2..list.len() {
        let next_vertex = list[i];
        while upper_hull.len() >= 2
            && !is_right_turn(
                upper_hull[upper_hull.len() - 2],
                upper_hull[upper_hull.len() - 1],
                next_vertex,
            )
        {
            upper_hull.pop();
        }
        upper_hull.push(next_vertex);
    }

    for i in (0..list.len() - 2).rev() {
        let next_vertex = list[i];
        while lower_hull.len() >= 2
            && !is_right_turn(
                lower_hull[lower_hull.len() - 2],
                lower_hull[lower_hull.len() - 1],
                next_vertex,
            )
        {
            lower_hull.pop();
        }
        lower_hull.push(next_vertex);
    }

    // The first and last point in the lower hull are also contained within the upper hull
    if lower_hull.len() >= 2 {
        lower_hull.remove(0);
        lower_hull.pop();
    }

    upper_hull.extend(lower_hull.iter());
    Some(upper_hull)
}
