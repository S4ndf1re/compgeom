use crate::basic_rendering::renderer::DrawMode;
use crate::gl::Gl;
use crate::objects::polygon::Polygon;
use glutin::config::Config;
use glutin::context::{ContextApi, ContextAttributesBuilder, NotCurrentContext, Version};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::prelude::GlConfig;
use num::Float;
use std::ffi::CStr;
use std::fmt::Debug;
use winit::raw_window_handle::HasWindowHandle;
use winit::window::{Window, WindowAttributes};

pub fn get_gl_string(
    gl: &crate::gl::Gl,
    variant: crate::gl::types::GLenum,
) -> Option<&'static CStr> {
    unsafe {
        let s = gl.GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}

// Find the config with the maximum number of samples, so our triangle will be
// smooth.
pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}

pub fn window_attributes() -> WindowAttributes {
    Window::default_attributes()
        .with_transparent(true)
        .with_title("Glutin triangle gradient example (press Escape to exit)")
}

pub fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

    // The context creation part.
    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    // There are also some old devices that support neither modern OpenGL nor GLES.
    // To support these we can try and create a 2.1 context.
    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(Some(Version::new(2, 1))))
        .build(raw_window_handle);

    let gl_display = gl_config.display();

    unsafe {
        gl_display
            .create_context(gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(gl_config, &fallback_context_attributes)
                    .unwrap_or_else(|_| {
                        gl_display
                            .create_context(gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    })
            })
    }
}

pub unsafe fn create_shader(
    gl: &Gl,
    shader: gl::types::GLenum,
    source: &[u8],
) -> gl::types::GLuint {
    unsafe {
        let shader = gl.CreateShader(shader);
        gl.ShaderSource(
            shader,
            1,
            [source.as_ptr().cast()].as_ptr(),
            std::ptr::null(),
        );
        gl.CompileShader(shader);
        shader
    }
}

pub fn fit_all_polygons<T: Float + Copy + Debug>(
    draw_mode: DrawMode,
    polygons: &mut [(gl::types::GLenum, Polygon<T>)],
) -> (T, T, T, T) {
    match draw_mode {
        DrawMode::FitSideBySide => {
            let mut bounds: (T, T, T, T) = (T::zero(), T::zero(), T::zero(), T::zero());
            let mut last_x = T::zero();

            for p in polygons {
                p.1.shift_to(last_x, T::zero());
                let p_bounds = p.1.get_bounds();
                bounds.1 = p_bounds.1;
                bounds.3 = bounds.3.max(p_bounds.3);
                last_x = bounds.1;
            }

            bounds
        }
        DrawMode::Overlap => {
            let mut bounds: (T, T, T, T) = (T::zero(), T::zero(), T::zero(), T::zero());

            for p in polygons {
                let p_bounds = p.1.get_bounds();
                bounds.0 = bounds.0.min(p_bounds.0);
                bounds.1 = bounds.1.max(p_bounds.1);
                bounds.2 = bounds.2.min(p_bounds.2);
                bounds.3 = bounds.3.max(p_bounds.3);
            }

            bounds
        }
    }
}
