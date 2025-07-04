use std::fmt::{Debug, Display};

use num::Float;

use crate::{
    basic_rendering::live_renderable::LiveRenderable,
    objects::{color::MAGENTA, polygon::Polygon, vertex::Vertex},
};

#[derive(Clone, Copy)]
pub struct DimRange<T>(T, T);

impl<T: Float> DimRange<T> {
    pub fn is_contained(&self, value: &T) -> bool {
        self.0 <= *value && *value <= self.1
    }
}

impl<T> From<DimRange<T>> for (T, T) {
    fn from(value: DimRange<T>) -> Self {
        (value.0, value.1)
    }
}

impl<T> From<(T, T)> for DimRange<T> {
    fn from(value: (T, T)) -> Self {
        DimRange(value.0, value.1)
    }
}

#[derive(Clone, Copy)]
pub struct RangeQuery<T> {
    pub x_range: DimRange<T>,
    pub y_range: DimRange<T>,
}

impl<T> From<((T, T), (T, T))> for RangeQuery<T> {
    fn from((x, y): ((T, T), (T, T))) -> Self {
        Self {
            x_range: x.into(),
            y_range: y.into(),
        }
    }
}

impl<T: Float + Display> RangeQuery<T> {
    pub fn is_contained(&self, value: (&T, &T)) -> bool {
        self.x_range.is_contained(value.0) && self.y_range.is_contained(value.1)
    }
}

impl<T> LiveRenderable<T> for RangeQuery<T>
where
    T: Debug + Copy + Float + Display,
{
    fn to_renderable(
        &self,
    ) -> Vec<(
        crate::basic_rendering::live_renderable::ZDepth,
        gl::types::GLenum,
        crate::objects::polygon::Polygon<T>,
    )> {
        let points: Vec<Vertex<T>> = vec![
            (self.x_range.0, self.y_range.0).into(),
            (self.x_range.0, self.y_range.1).into(),
            (self.x_range.1, self.y_range.1).into(),
            (self.x_range.1, self.y_range.0).into(),
        ];

        let mut polygon = Polygon::new(points);
        polygon.set_color(MAGENTA);

        vec![(10, gl::LINE_LOOP, polygon)]
    }
}
