use crate::objects::polygon::{Polygon, Position, Vertex};
use gl::types::GLsizei;
use glutin::display::GlDisplay;
use num::Float;
use std::ffi::CString;
use std::fmt::Debug;

pub struct Renderable {
    gl: crate::gl::Gl,
    pub program: gl::types::GLuint,
    pub vao: gl::types::GLuint,
    pub vbo: gl::types::GLuint,
    pub mode: gl::types::GLenum,
    pub count: usize,
}

impl Renderable {
    pub fn new<T: Float + Debug + Copy + 'static, D: GlDisplay>(
        gl_display: &D,
        program: gl::types::GLuint,
        polygon: &Polygon<T>,
        mode: gl::types::GLenum,
    ) -> Self {
        unsafe {
            let gl = crate::gl::Gl::load_with(|symbol| {
                let symbol = CString::new(symbol).unwrap();
                gl_display.get_proc_address(symbol.as_c_str()).cast()
            });

            gl.UseProgram(program);

            let mut vao = std::mem::zeroed();
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            let buffer: Vec<Vertex<f32>> = polygon.vertices.iter().map(|v| v.to_other()).collect();
            let mut vbo = std::mem::zeroed();
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (buffer.len() * std::mem::size_of::<Vertex<f32>>()) as gl::types::GLsizeiptr,
                buffer.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let pos_attrib = gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                std::mem::size_of::<Vertex<f32>>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);

            let col_attrib = gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
            gl.VertexAttribPointer(
                col_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                std::mem::size_of::<Vertex<f32>>() as gl::types::GLsizei,
                std::mem::size_of::<Position<f32>>() as *const _,
            );
            gl.EnableVertexAttribArray(col_attrib as gl::types::GLuint);

            Self {
                program,
                vao,
                vbo,
                gl,
                mode,
                count: polygon.vertices.len(),
            }
        }
    }

    pub fn render(&self) {
        unsafe {
            self.gl.UseProgram(self.program);

            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.gl.DrawArrays(self.mode, 0, self.count as GLsizei);
        }
    }
}

impl Drop for Renderable {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteVertexArrays(1, &self.vao);
            self.gl.DeleteBuffers(1, &self.vbo);
        }
    }
}
