#![feature(unboxed_closures)]

mod algorithm;
mod basic_rendering;
mod objects;

use crate::algorithm::graham_scan::graham_scan::graham_scan_vorlesungsfolie;
use crate::algorithm::sweep_line::context::IntersectionMode;
use crate::algorithm::sweep_line::sweep_line::sweep_line_intersections;
use crate::basic_rendering::renderer::DrawMode;
use crate::objects::polygon::Polygon;
use crate::objects::vertex::Vertex;
use basic_rendering::app::App;
use basic_rendering::util::window_attributes;
use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;
use num::Float;
use objects::obj_file_manager::ObjFileManager;
use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};
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
    let (mut quad, _) = load_polygon_with_hull::<OrderedFloat<f32>>("assets/quad.obj");
    quad.set_color([1.0, 0.0, 0.0, 1.0]);

    let (mut star, _) = load_polygon_with_hull::<OrderedFloat<f32>>("assets/star2.obj");
    star.set_color([0.0, 1.0, 0.0, 1.0]);

    let mut lines = vec![];
    let mut offset = 0;
    for (idx, poly) in [&quad, &star].iter().enumerate() {
        let tmp = poly.to_lines(offset, idx);
        lines.extend(tmp.1);
        offset = tmp.0;
    }

    let mut intersections = sweep_line_intersections(&lines, IntersectionMode::PolygonDifference)
        .into_iter()
        .map(|v| v.0)
        .collect::<Vec<_>>();
    let mut intersected_set = HashSet::<(OrderedFloat<f32>, OrderedFloat<f32>)>::new();
    for i in &intersections {
        intersected_set.insert((i.x(), i.y()));
    }

    println!("Found {} intersections", intersections.len());
    // Add all other lines that belong to the output
    intersections.extend(star.unit(&quad, &mut intersected_set));
    let mut poly_intersections = Polygon::new(
        intersected_set
            .into_iter()
            .map(|p| Vertex {
                position: [p.0, p.1, OrderedFloat(0.0)],
                color: [0.0, 0.0, 0.0, 0.0],
            })
            .collect(),
    );
    poly_intersections.set_color([1.0, 1.0, 1.0, 1.0]);

    let mut poly_fill =
        Polygon::new(graham_scan_vorlesungsfolie(poly_intersections.vertices.clone()).unwrap());
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
            (gl::TRIANGLE_FAN, poly_fill),
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
