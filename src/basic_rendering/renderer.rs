use crate::basic_rendering::renderable::Renderable;
use crate::basic_rendering::util::{create_shader, fit_all_polygons, get_gl_string};
use crate::objects::polygon::Polygon;
use glutin::display::GlDisplay;
use std::ffi::CString;
use std::ops::Deref;

pub struct Renderer {
    program: gl::types::GLuint,
    gl: crate::gl::Gl,
    renderable: Vec<Renderable>,
}

impl Renderer {
    pub fn new<D: GlDisplay, P: AsRef<[(gl::types::GLenum, Polygon)]>>(
        gl_display: &D,
        polygons: P,
    ) -> Self {
        unsafe {
            let mut polygons: Vec<(gl::types::GLenum, Polygon)> = polygons.as_ref().to_vec();
            let mut bounds = fit_all_polygons(&mut polygons);
            bounds.0 = bounds.0 - 0.1;
            bounds.1 = bounds.1 + 0.1;
            bounds.2 = bounds.2 - 0.1;
            bounds.3 = bounds.3 + 0.1;

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
            gl.FrontFace(gl::CCW);
            gl.CullFace(gl::FRONT);

            let ortho_projection = gl.GetUniformLocation(program, b"ortho\0".as_ptr() as *const _);
            assert!(ortho_projection >= 0);

            gl.DeleteShader(vertex_shader);
            gl.DeleteShader(fragment_shader);

            let (left, right, bottom, top) = bounds;

            let (near, far) = (-1.0, 1.0); // Set -1 and 1 for Ortho2D equivalent

            #[rustfmt::skip]
            let matrix:[f32; 16] =
                [2.0 / (right - left),      0.0,                    0.0,                -(right + left) / (right - left),
                 0.0,                       2.0 / (top - bottom),   0.0,                -(top + bottom) / (top - bottom),
                 0.0,                       0.0,                    -2.0/(far - near),  -(far + near) / (far - near),
                 0.0,                       0.0,                    0.0,                1.0];

            // NOTE(Jan): This has to get transposed, since opengl uses column first storage, instead of row first
            gl.UniformMatrix4fv(ortho_projection, 1, 1, matrix.as_ptr());

            Self {
                program,
                gl,
                renderable: polygons
                    .iter()
                    .map(|p| Renderable::new(gl_display, program, &p.1, p.0))
                    .collect(),
            }
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.gl.ClearColor(0.0, 0.0, 0.0, 1.0);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }
        for renderable in &self.renderable {
            renderable.render();
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }
}

impl Deref for Renderer {
    type Target = crate::gl::Gl;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl Drop for Renderer {
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
attribute vec3 color;
varying vec3 vColor;

void main() {
    gl_Position = ortho * vec4(position, 1.0);
    gl_PointSize = 10.0;
    vColor = color;
}
\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

varying vec3 vColor;

void main() {
    gl_FragColor = vec4(vColor, 1.0);
}
\0";
