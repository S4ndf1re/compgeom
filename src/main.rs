mod algorithm;
mod basic_rendering;
mod objects;

use std::error::Error;

use crate::algorithm::graham_scan::{
    graham_scan_by_angle, graham_scan_by_x, graham_scan_vorlesungsfolie,
};
use crate::objects::polygon::Polygon;
use basic_rendering::app::App;
use basic_rendering::util::window_attributes;
use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;
use objects::obj_file_manager::ObjFileManager;

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

fn main() -> Result<(), Box<dyn Error>> {
    let obj_file_manager = ObjFileManager::<f32>::new("grahamScanDifficult.obj");
    let polygon = obj_file_manager.get_polygon(0);
    println!("Polygon: {}", polygon);

    let hull = graham_scan_vorlesungsfolie(&polygon.vertices)
        .expect("A hull must be present, since there are more than 1 point");
    let polygon_hull = Polygon::new(hull);
    println!("Hull: {}", polygon_hull);

    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    // make sure, that on macos, transparency is enabled. Linux and Windows may allow for multiple configurations to be loaded, however, macos only provieds one configuration
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(cfg!(cgl_backend));

    // Create a display
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes()));

    let mut app = App::new(
        template,
        display_builder,
        vec![
            (gl::POINTS, obj_file_manager.get_polygon(0)),
            (gl::LINE_LOOP, polygon_hull),
        ],
    );
    event_loop.run_app(&mut app)?;

    app.exit_state
}
