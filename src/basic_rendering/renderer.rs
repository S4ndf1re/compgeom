use crate::basic_rendering::live_renderable::ZDepth;
use crate::basic_rendering::renderable::Renderable;
use crate::basic_rendering::util::{create_shader, fit_all_polygons, get_gl_string};
use crate::objects::polygon::Polygon;
use glutin::display::GlDisplay;
use num::Float;
use std::cell::RefCell;
use std::ffi::CString;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

use super::live_renderable::LiveRenderable;

#[derive(Clone, Copy)]
pub enum DrawMode {
    FitSideBySide,
    Overlap,
}

pub struct Renderer<'l, T, L>
where
    T: Debug + Copy,
    L: LiveRenderable<T>,
{
    program: gl::types::GLuint,
    gl: crate::gl::Gl,
    renderable: &'l RefCell<L>,
    draw_mode: DrawMode,
    _data: PhantomData<T>,
}

impl<'l, T, L> Renderer<'l, T, L>
where
    T: Float + Debug + Copy + 'static,
    L: LiveRenderable<T>,
{
    pub fn new<D: GlDisplay>(
        gl_display: &D,
        draw_mode: DrawMode,
        polygons: &'l RefCell<L>,
    ) -> Self {
        unsafe {
            let gl = crate::gl::Gl::load_with(|symbol| {
                let symbol = CString::new(symbol).unwrap();
                gl_display.get_proc_address(symbol.as_c_str()).cast()
            });

            if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
                println!("Running on {}", renderer.to_string_lossy());
            }
            if let Some(version) = get_gl_string(&gl, gl::VERSION) {
                println!("OpenGL Version {}", version.to_string_lossy());
            }

            if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
                println!("Shaders version on {}", shaders_version.to_string_lossy());
            }

            let vertex_shader = create_shader(&gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
            let fragment_shader = create_shader(&gl, gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

            let program = gl.CreateProgram();

            gl.AttachShader(program, vertex_shader);
            gl.AttachShader(program, fragment_shader);

            gl.LinkProgram(program);

            gl.UseProgram(program);

            gl.Enable(gl::PROGRAM_POINT_SIZE);
            gl.Enable(gl::CULL_FACE);
            gl.FrontFace(gl::CW);
            gl.CullFace(gl::FRONT);

            gl.DeleteShader(vertex_shader);
            gl.DeleteShader(fragment_shader);

            Self {
                program,
                gl,
                draw_mode,
                renderable: polygons,
                _data: PhantomData {},
            }
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.gl.ClearColor(0.0, 0.0, 0.0, 1.0);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            let mut polygons: Vec<(ZDepth, gl::types::GLenum, Polygon<T>)> =
                self.renderable.borrow().clone().to_renderable();

            // Sort that smaller z-depth value will get rendered first
            polygons.sort_by(|a, b| a.0.cmp(&b.0));

            let mut bounds = fit_all_polygons(self.draw_mode, &mut polygons);
            bounds.0 = bounds.0 - T::one();
            bounds.1 = bounds.1 + T::one();
            bounds.2 = bounds.2 - T::one();
            bounds.3 = bounds.3 + T::one();

            let ortho_projection = self
                .gl
                .GetUniformLocation(self.program, b"ortho\0".as_ptr() as *const _);
            assert!(ortho_projection >= 0);

            let (left, right, bottom, top) = (
                bounds.0.to_f32().unwrap(),
                bounds.1.to_f32().unwrap(),
                bounds.2.to_f32().unwrap(),
                bounds.3.to_f32().unwrap(),
            );

            let (near, far) = (-1.0, 1.0); // Set -1 and 1 for Ortho2D equivalent

            #[rustfmt::skip]
            let matrix:[f32; 16] =
                [2.0 / (right - left),      0.0,                    0.0,                -(right + left) / (right - left),
                 0.0,                       2.0 / (top - bottom),   0.0,                -(top + bottom) / (top - bottom),
                 0.0,                       0.0,                    -2.0/(far - near),  -(far + near) / (far - near),
                 0.0,                       0.0,                    0.0,                1.0];

            // NOTE(Jan): This has to get transposed, since opengl uses column first storage, instead of row first
            self.gl
                .UniformMatrix4fv(ortho_projection, 1, 1, matrix.as_ptr());

            let renderables = polygons
                .iter()
                .map(|p| Renderable::new(self.gl.clone(), self.program, &p.2, p.1));
            for renderable in renderables {
                renderable.render();
            }
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }
}

impl<'l, T, L> Deref for Renderer<'l, T, L>
where
    T: Copy + Debug,
    L: LiveRenderable<T>,
{
    type Target = crate::gl::Gl;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl<'l, T, L> Drop for Renderer<'l, T, L>
where
    T: Copy + Debug,
    L: LiveRenderable<T>,
{
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.program);
        }
    }
}

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

uniform mat4 ortho;
attribute vec3 position;
attribute vec4 color;
varying vec4 vColor;

void main() {
    gl_Position = ortho * vec4(position, 1.0);
    gl_PointSize = 10.0;
    vColor = color;
}
\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

varying vec4 vColor;

void main() {
    gl_FragColor = vColor;
}
\0";
