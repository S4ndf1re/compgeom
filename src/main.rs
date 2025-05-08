#![feature(unboxed_closures)]

mod algorithm;
mod basic_rendering;
mod objects;

use crate::algorithm::graham_scan::graham_scan::graham_scan_vorlesungsfolie;
use crate::algorithm::sweep_line::line::Line;
use crate::algorithm::sweep_line::sweep_line::sweep_line_intersections;
use crate::basic_rendering::renderer::DrawMode;
use crate::objects::polygon::Polygon;
use basic_rendering::app::App;
use basic_rendering::util::window_attributes;
use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;
use num::Float;
use objects::obj_file_manager::ObjFileManager;
use ordered_float::OrderedFloat;
use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

fn load_polygon_with_hull<T: Float + Copy + FromStr<Err: Debug> + Debug>(
    path: &str,
) -> (Polygon<T>, Polygon<T>) {
    let obj_file_manager = ObjFileManager::<T>::new(path);
    let polygon = obj_file_manager.get_polygon(0);
    let hull = graham_scan_vorlesungsfolie(&polygon.vertices)
        .expect("A hull must be present, since there are more than 1 point");
    let polygon_hull = Polygon::new(hull);

    (polygon, polygon_hull)
}
fn main() -> Result<(), Box<dyn Error>> {
    // Load f32 or f64 Points (type info provided by generic, must be any num::Float)
    let (mut quad, mut quad_hull) =
        load_polygon_with_hull::<OrderedFloat<f32>>("assets/test/quad.obj");
    quad.set_color([1.0, 0.0, 0.0, 1.0]);

    let (mut star, mut star_hull) =
        load_polygon_with_hull::<OrderedFloat<f32>>("assets/test/star2.obj");
    star.set_color([0.0, 1.0, 0.0, 1.0]);

    let mut lines = vec![];
    let mut offset = 0;
    for (idx, poly) in [&quad, &star].iter().enumerate() {
        let tmp = poly.to_lines(offset, idx);
        lines.extend(tmp.1);
        offset = tmp.0;
    }

    let mut intersections = sweep_line_intersections(&lines)
        .into_iter()
        .map(|v| v.0)
        .collect::<Vec<_>>();
    // intersections.extend(star.unit(&quad));
    println!("Found {} intersections", intersections.len());
    let mut poly_intersections = Polygon::new(intersections.clone());
    poly_intersections.set_color([1.0, 1.0, 1.0, 1.0]);

    let mut poly_fill = Polygon::new(graham_scan_vorlesungsfolie(intersections).unwrap());
    // poly_fill.push_back(poly_fill.get_center());
    // poly_fill.insert_front(poly_fill.get_center());
    poly_fill.set_color([1.0, 1.0, 1.0, 0.3]);

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
            // (gl::TRIANGLE_FAN, poly_fill),
            (gl::LINE_LOOP, star.clone()),
            (gl::LINE_LOOP, quad.clone()),
            (gl::POINTS, quad),
            (gl::POINTS, star),
            (gl::POINTS, poly_intersections),
        ],
        DrawMode::Overlap,
    );
    event_loop.run_app(&mut app)?;

    app.exit_state
}
