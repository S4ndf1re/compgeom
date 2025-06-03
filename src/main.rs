#![feature(box_as_ptr)]

mod algorithm;

mod basic_rendering;
mod objects;

use crate::algorithm::graham_scan::graham_scan::graham_scan_vorlesungsfolie;
use crate::algorithm::sweep_line::context::IntersectionMode;
use crate::basic_rendering::renderer::DrawMode;
use crate::objects::polygon::Polygon;
use algorithm::bsp::{PointOrientation, generate_points, line_decider};
use algorithm::kd_tree::range_query::RangeQuery;
use algorithm::kd_tree::tree::KdTree;
use algorithm::sweep_line::sweep_line_algo::sweep_line_intersections;
use basic_rendering::app::App;
use basic_rendering::util::window_attributes;
use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;
use num::Float;
use objects::line::Line;
use objects::obj_file_manager::ObjFileManager;
use ordered_float::OrderedFloat;
use std::cell::RefCell;
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

pub fn sweep_line_task() -> Result<(), Box<dyn Error>> {
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

    println!("Found {} intersections", intersections.len());
    // Add all other lines that belong to the output
    intersections.extend(star.unit(&quad).unwrap());
    let mut poly_intersections = Polygon::new(intersections);
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

    let draw_polygons = RefCell::new(vec![
        (0, gl::TRIANGLE_FAN, poly_fill),
        (0, gl::LINE_LOOP, star.clone()),
        (0, gl::LINE_LOOP, quad.clone()),
        (1, gl::POINTS, quad),
        (1, gl::POINTS, star),
        (1, gl::POINTS, poly_intersections),
    ]);
    let mut app = App::new(template, display_builder, &draw_polygons, DrawMode::Overlap);
    event_loop.run_app(&mut app)?;

    app.exit_state
}

fn bsp_task() -> Result<(), Box<dyn Error>> {
    let points = generate_points(-10.0, 10.0, 20);
    let line = Line::<f32>::new(0, 0, (-2.0, 1.0).into(), (1.0, 3.0).into());

    let mut line_polygon: Polygon<_> = line.clone().into();
    line_polygon.set_color([1.0, 1.0, 1.0, 1.0]);

    const VISIBILITY_FACTOR: f32 = 2.0;
    let (mut max_dist, decided_points) = line_decider(&line, &points);
    max_dist += 0.2;
    let mut draw_polygons = vec![(0, gl::LINES, line_polygon)];

    decided_points
        .into_iter()
        .for_each(|decided| match decided {
            PointOrientation::Left(dist, mut p) => {
                p.set_color([
                    (dist + VISIBILITY_FACTOR) / (max_dist + VISIBILITY_FACTOR),
                    0.0,
                    0.0,
                    1.0,
                ]);
                draw_polygons.push((1, gl::POINTS, p));
            }
            PointOrientation::Right(dist, mut p) => {
                p.set_color([
                    0.0,
                    (dist + VISIBILITY_FACTOR) / (max_dist + VISIBILITY_FACTOR),
                    0.0,
                    1.0,
                ]);
                draw_polygons.push((1, gl::POINTS, p));
            }
        });

    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    // make sure, that on macos, transparency is enabled. Linux and Windows may allow for multiple configurations to be loaded, however, macos only provieds one configuration
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(cfg!(cgl_backend));

    // Create a display
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes()));

    let draw_polygons = RefCell::new(draw_polygons);
    let mut app = App::new(template, display_builder, &draw_polygons, DrawMode::Overlap);
    event_loop.run_app(&mut app)?;

    app.exit_state
}

fn kd_tree_task() -> Result<(), Box<dyn Error>> {
    let (points, _) = load_polygon_with_hull::<OrderedFloat<f32>>("assets/circularPoints20.obj");

    let mut tree = KdTree::build(&points.vertices);
    let result = tree.range_query(
        (
            (OrderedFloat(5.0), OrderedFloat(9.0)),
            (OrderedFloat(0.0), OrderedFloat(12.0)),
        )
            .into(),
    );
    println!("Result: {result:?}");

    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    // make sure, that on macos, transparency is enabled. Linux and Windows may allow for multiple configurations to be loaded, however, macos only provieds one configuration
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(cfg!(cgl_backend));

    // Create a display
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes()));

    let tree = RefCell::new(tree);
    let mut app = App::new(template, display_builder, &tree, DrawMode::Overlap);
    event_loop.run_app(&mut app)?;

    app.exit_state
}

fn main() -> Result<(), Box<dyn Error>> {
    // sweep_line_task()
    // bsp_task()
    kd_tree_task()
    // unimplemented!()
}
