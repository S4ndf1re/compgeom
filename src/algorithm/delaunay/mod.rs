use std::{fmt::Debug, time::SystemTime};

use circle::Circle;
use half_edge::{HalfEdgeDs, HeEdge};
use num::Float;
use rand::seq::IndexedRandom;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::{basic_rendering::live_renderable::LiveRenderable, objects::polygon::Polygon};

pub mod circle;
pub mod half_edge;

pub struct Delauny<T> {
    half_edge_ds: HalfEdgeDs<T>,
    circles: Vec<Circle<T>>,
    last_pressed: SystemTime,
}

impl<T> Delauny<T>
where
    T: Float + Copy + Debug + Default,
{
    pub fn new(polygons: &[Polygon<T>]) -> Self {
        let mut this = Self {
            half_edge_ds: HalfEdgeDs::from_polygons(polygons),
            circles: Vec::new(),
            last_pressed: SystemTime::now(),
        };

        this.populate_circles();
        this.update_circles();

        this
    }

    fn populate_circles(&mut self) {
        for face in self.half_edge_ds.query_all_faces() {
            let verticies = unsafe { (*face).query_verticies() };
            assert!(verticies.len() == 3);
            let new_circle: Circle<T> = (verticies[0], verticies[1], verticies[2], face).into();

            self.circles.push(new_circle);
        }
    }

    fn update_circles(&mut self) {
        for circle in &mut self.circles {
            circle.set_delauny_ok();

            for p in self.half_edge_ds.query_all_verticies() {
                if circle.is_in_circle(p) {
                    circle.set_delauny_not_ok();
                }
            }
        }
    }
}

impl<T> LiveRenderable<T> for Delauny<T>
where
    T: Float + Copy + Debug + Default,
{
    fn to_renderable(
        &self,
    ) -> Vec<(
        crate::basic_rendering::live_renderable::ZDepth,
        gl::types::GLenum,
        Polygon<T>,
    )> {
        let mut renderable = Vec::new();

        renderable.extend(self.half_edge_ds.to_renderable());

        renderable.extend(self.circles.iter().flat_map(|c| c.to_renderable()));

        renderable
    }

    fn user_input(&mut self, event: Option<winit::event::KeyEvent>) {
        if event.is_none() {
            return;
        }

        let current_time = SystemTime::now();

        let elapsed = current_time.duration_since(self.last_pressed);
        if elapsed.is_err() || elapsed.unwrap().as_millis() < 200 {
            return;
        }

        if let Some(event) = event
            && event.physical_key == PhysicalKey::Code(KeyCode::Space)
        {
            let mut rng = rand::rng();
            let edges = self.half_edge_ds.query_all_edges();
            let mut choice = edges.choose(&mut rng).unwrap();

            while !HeEdge::is_flippable(*choice) {
                choice = edges.choose(&mut rng).unwrap();
            }

            HeEdge::flip_edge(*choice);

            // self.half_edge_ds.sanity_check();
            self.update_circles();
            self.last_pressed = current_time;
        }
    }
}
