use std::fmt::Debug;

use winit::event::KeyEvent;

use crate::objects::polygon::Polygon;

pub type ZDepth = usize;

pub trait LiveRenderable<T>: Clone
where
    T: Copy + Debug,
{
    fn to_renderable(self) -> Vec<(ZDepth, gl::types::GLenum, Polygon<T>)>;
    fn user_input(&mut self, _event: Option<KeyEvent>) {
        // Do nothing on default
    }
    fn get_current_title(&self) -> String {
        String::from("Title")
    }
}

impl<T> LiveRenderable<T> for Vec<(ZDepth, gl::types::GLenum, Polygon<T>)>
where
    T: Copy + Debug,
{
    fn to_renderable(self) -> Vec<(ZDepth, gl::types::GLenum, Polygon<T>)> {
        self
    }
}

impl<T> LiveRenderable<T> for Vec<Polygon<T>>
where
    T: Copy + Debug,
{
    fn to_renderable(self) -> Vec<(ZDepth, gl::types::GLenum, Polygon<T>)> {
        self.into_iter().map(|v| (0, gl::POINTS, v)).collect()
    }
}
