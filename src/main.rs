mod app;
mod appstate;
mod obj_file_manager;
mod polygon;
mod renderable;
mod renderer;
mod util;

use std::error::Error;

use crate::app::App;
use crate::obj_file_manager::ObjFileManager;
use crate::util::window_attributes;
use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

fn main() -> Result<(), Box<dyn Error>> {
    let obj_file_manager = ObjFileManager::new("star.obj");

    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    // make sure, that on macos, transparency is enabled. Linux and Windows may allow for multiple configurations to be loaded, however, macos only provieds one configuration
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(cfg!(cgl_backend));

    // Create a display
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes()));

    let mut app = App::new(template, display_builder, obj_file_manager);
    event_loop.run_app(&mut app)?;

    app.exit_state
}
