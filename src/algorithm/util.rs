use std::fmt::Debug;

use num::Float;

use crate::objects::vertex::Vertex;

/// Test if p->test->q is a right turn (true)
/// NOTE: this function only operates on 2d at the moment
pub fn is_right_turn<T: Float + Copy + Debug>(p: Vertex<T>, test: Vertex<T>, q: Vertex<T>) -> bool {
    let direction_vec_line = q - p;
    let line_normal = direction_vec_line.normal();

    let (_, p_min_dist) =
        test.point_on_line_with_min_distance_to_self_clamped_0_1_2d(&(p, q).into());
    let directional_test = test - p_min_dist;
    let dist = directional_test.magnitude();

    let cosine = line_normal.cosine(&directional_test);

    cosine >= T::zero() || dist < T::from(0.000001).unwrap()
}

/// Test if p->test->q is a right turn (true)
/// NOTE: this function only operates on 2d at the moment
pub fn is_left_turn<T: Float + Copy + Debug>(p: Vertex<T>, test: Vertex<T>, q: Vertex<T>) -> bool {
    let direction_vec_line = q - p;
    let line_normal = direction_vec_line.normal();

    let (_, p_min_dist) =
        test.point_on_line_with_min_distance_to_self_clamped_0_1_2d(&(p, q).into());
    let directional_test = test - p_min_dist;
    let dist = directional_test.magnitude();

    let cosine = line_normal.cosine(&directional_test);

    cosine <= T::zero() || dist < T::from(0.000001).unwrap()
}
