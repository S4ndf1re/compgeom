use std::fmt::Debug;

use num::Float;

use crate::{
    basic_rendering::live_renderable::LiveRenderable,
    objects::{
        color::{GREEN, RED},
        line::Line,
        polygon::Polygon,
        vertex::Vertex,
    },
};

use super::half_edge::{HeFace, HeVertex};

pub const CIRCLE_SEGMENTS: usize = 100;

pub struct Circle<T> {
    pub face: *mut HeFace<T>,
    pub center: Vertex<T>,
    pub radius: T,
    pub is_delauny_ok: bool,
}

impl<T> Circle<T>
where
    T: Float + Copy + Debug,
{
    pub fn new(center: Vertex<T>, radius: T, face: *mut HeFace<T>) -> Self {
        Self {
            center,
            radius,
            is_delauny_ok: true,
            face,
        }
    }

    pub fn set_delauny_not_ok(&mut self) {
        self.is_delauny_ok = false;
    }

    pub fn set_delauny_ok(&mut self) {
        self.is_delauny_ok = true;
    }

    pub fn is_in_circle(&self, vert: *mut HeVertex<T>) -> bool {
        unsafe {
            if (*self.face).contains_vertex(vert) {
                return false;
            }

            let dist = self.center - (*vert).vertex;
            dist.magnitude() <= self.radius
        }
    }
}

impl<T> From<(Vertex<T>, Vertex<T>, Vertex<T>, *mut HeFace<T>)> for Circle<T>
where
    T: Float + Copy + Debug,
{
    fn from((a, b, c, face): (Vertex<T>, Vertex<T>, Vertex<T>, *mut HeFace<T>)) -> Self {
        let line_ca = Line::from_normal_between_points(a, c);
        let line_ba = Line::from_normal_between_points(a, b);

        let circle_center = line_ca
            .intersects_no_context(&line_ba)
            .expect("Failed to create circle between three points (impossible)");

        let radius = (a - circle_center).magnitude();
        println!("Circle for {a:?}, {b:?}, {c:?} = center({circle_center:?}), radius({radius:?})");

        Self::new(circle_center, radius, face)
    }
}

impl<T>
    From<(
        *mut HeVertex<T>,
        *mut HeVertex<T>,
        *mut HeVertex<T>,
        *mut HeFace<T>,
    )> for Circle<T>
where
    T: Float + Copy + Debug,
{
    fn from(
        (a, b, c, face): (
            *mut HeVertex<T>,
            *mut HeVertex<T>,
            *mut HeVertex<T>,
            *mut HeFace<T>,
        ),
    ) -> Self {
        unsafe { ((*a).vertex, (*b).vertex, (*c).vertex, face).into() }
    }
}

impl<T> LiveRenderable<T> for Circle<T>
where
    T: Copy + Float + Debug,
{
    fn to_renderable(
        &self,
    ) -> Vec<(
        crate::basic_rendering::live_renderable::ZDepth,
        gl::types::GLenum,
        crate::objects::polygon::Polygon<T>,
    )> {
        let mut color = if self.is_delauny_ok { GREEN } else { RED };
        color[3] = 0.3;

        let step = T::from(std::f64::consts::PI * 2.0).unwrap() / T::from(CIRCLE_SEGMENTS).unwrap();

        let mut line_loop = Vec::new();

        for i in 0..CIRCLE_SEGMENTS {
            let (sin, cos) = (step * T::from(i).unwrap()).sin_cos();
            let x = cos * self.radius;
            let y = sin * self.radius;

            line_loop.push(Vertex::from((x, y)) + self.center);
        }

        let mut polygon = Polygon::new(line_loop);
        polygon.disable_include_in_fitting();
        polygon.set_color(color);
        vec![(0, gl::LINE_LOOP, polygon)]
    }
}
