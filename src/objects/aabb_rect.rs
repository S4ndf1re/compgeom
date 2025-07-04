use std::fmt::Debug;

use num::Float;

use crate::basic_rendering::live_renderable::{LiveRenderable, ZDepth};

use super::{polygon::Polygon, vertex::Vertex};

/// Represent and axis aligned bounding box
#[derive(Clone)]
pub struct AaBbRect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T> AaBbRect<T>
where
    T: Copy + Float,
{
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Self { x, y, w, h }
    }

    /// Split rect by either x or y axis.
    /// The first value of the tuple is the left (x) or lower (y) rect. the other is the
    /// corresponding opposite (right (x), upper (y)). This is assuming, that y is lower than y+h
    pub fn split_by(&self, value: T, x_axis: bool) -> (Option<AaBbRect<T>>, Option<AaBbRect<T>>) {
        if x_axis {
            if value == self.x {
                return (None, Some(self.clone()));
            } else if value == self.x + self.w {
                return (Some(self.clone()), None);
            }

            let left_half = AaBbRect::new(self.x, self.y, value - self.x, self.h);
            let right_half = AaBbRect::new(value, self.y, (self.x + self.w) - value, self.h);

            (Some(left_half), Some(right_half))
        } else {
            if value == self.y {
                return (None, Some(self.clone()));
            } else if value == self.y + self.h {
                return (Some(self.clone()), None);
            }

            let lower_half = AaBbRect::new(self.x, self.y, self.w, value - self.y);
            let upper_half = AaBbRect::new(self.x, value, self.w, (self.y + self.h) - value);

            (Some(lower_half), Some(upper_half))
        }
    }

    pub fn merge_point_into_self(&mut self, point: Vertex<T>) {
        if point.x() < self.x {
            self.w = self.x + self.w - point.x();
            self.x = point.x();
        }

        if point.x() > self.x + self.w {
            self.w = point.x() - self.x;
        }

        if point.y() < self.y {
            self.h = self.y + self.h - point.y();
            self.y = point.y();
        }

        if point.y() > self.y + self.h {
            self.h = point.y() - self.y;
        }
    }
}

impl<T> From<(T, T, T, T)> for AaBbRect<T>
where
    T: Copy + Float,
{
    fn from((x, y, w, h): (T, T, T, T)) -> Self {
        Self::new(x, y, w, h)
    }
}

impl<T> LiveRenderable<T> for AaBbRect<T>
where
    T: Float + Copy + Debug,
{
    fn to_renderable(&self) -> Vec<(ZDepth, gl::types::GLenum, Polygon<T>)> {
        let mut polygon = Polygon::new(vec![
            (self.x, self.y).into(),
            (self.x + self.w, self.y).into(),
            (self.x + self.w, self.y + self.h).into(),
            (self.x, self.y + self.h).into(),
        ]);
        polygon.set_color([1.0, 1.0, 1.0, 1.0]);
        vec![(0, gl::LINE_LOOP, polygon)]
    }
}
