use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use num::Float;

use crate::{
    basic_rendering::live_renderable::{LiveRenderable, ZDepth},
    objects::{
        color::{Color, GLOBAL_COLOR_GENERATOR},
        polygon::Polygon,
        vertex::Vertex,
    },
};

pub struct HeVertex<T> {
    pub vertex: Vertex<T>,
    pub edge: Option<*mut HeEdge<T>>,
}

impl<T> Clone for HeVertex<T>
where
    T: Clone + Copy,
{
    fn clone(&self) -> Self {
        Self {
            vertex: self.vertex,
            edge: self.edge,
        }
    }
}

impl<T> Default for HeVertex<T>
where
    T: Default + Clone + Copy,
{
    fn default() -> Self {
        Self {
            vertex: Vertex::default(),
            edge: None,
        }
    }
}

pub struct HeFace<T> {
    pub edge: Option<*mut HeEdge<T>>,
    pub color: Color,
    _data: PhantomData<T>,
}

pub struct HeEdge<T> {
    pub vertex: *mut HeVertex<T>,
    pub pair: Option<*mut HeEdge<T>>,
    pub face: Option<*mut HeFace<T>>,
    pub next: Option<*mut HeEdge<T>>,
    _data: PhantomData<T>,
}

pub struct HalfEdgeDs<T> {
    verticies: Vec<*mut HeVertex<T>>,
    faces: Vec<*mut HeFace<T>>,
    edges: Vec<*mut HeEdge<T>>,
}

impl<T> HalfEdgeDs<T>
where
    T: Debug + Copy + Default + Clone,
{
    /// compute the half edge datastructure in two phases. first, construct all verticies and edges
    /// based on the polygons proviced. make sure that the verticies within all polygons are properly connected by vertex.id.
    /// Then, iterate over all created edges, and link the twin edges. this is done by storing the twin edges at creation in a hashmap.
    /// If a twin edge for an edge does not exist yet, the twin edge must be an "oustide" edge,
    /// meaning it has no face assigned. Create the edge non the less, but without a face.
    pub fn from_polygons(polygons: &[Polygon<T>]) -> Self {
        let max_vertex_id = polygons
            .iter()
            .flat_map(|p| &p.vertices)
            .map(|v| v.id)
            .max()
            .unwrap();

        let mut verticies = vec![std::ptr::null_mut(); max_vertex_id + 10];

        for vertex in polygons.iter().flat_map(|p| &p.vertices) {
            verticies[vertex.id] = Box::into_raw(Box::new(HeVertex {
                vertex: *vertex,
                edge: None,
            }));
        }

        let mut half_edge_tracked: HashMap<(*mut HeVertex<T>, *mut HeVertex<T>), *mut HeEdge<T>> =
            HashMap::new();
        let mut edges = Vec::new();

        let mut faces = Vec::new();

        for polygon in polygons.iter() {
            let face = Box::into_raw(Box::new(HeFace {
                edge: None,
                color: GLOBAL_COLOR_GENERATOR.next_color(1.0),
                _data: PhantomData,
            }));

            let mut tmp_edges = Vec::new();
            for vert in &polygon.vertices {
                let edge = Box::into_raw(Box::new(HeEdge {
                    vertex: verticies[vert.id],
                    pair: None,
                    next: None,
                    face: Some(face),
                    _data: PhantomData,
                }));

                unsafe {
                    (*face).edge = Some(edge);
                    (*verticies[vert.id]).edge = Some(edge);
                }

                tmp_edges.push(edge);
            }

            for i in 0..tmp_edges.len() {
                unsafe {
                    // Set the next vertex to the next in list (wrapping)
                    (*tmp_edges[i]).next = Some(tmp_edges[(i + 1) % tmp_edges.len()]);

                    half_edge_tracked.insert(
                        (
                            (*tmp_edges[i]).vertex,
                            (*tmp_edges[(i + 1) % tmp_edges.len()]).vertex,
                        ),
                        tmp_edges[i],
                    );
                }
            }

            edges.extend(tmp_edges);
            faces.push(face);
        }

        // set all twins. This is the second phase
        for ((from, to), edge) in &half_edge_tracked {
            if let Some(twin) = half_edge_tracked.get(&(*to, *from)) {
                unsafe {
                    (**edge).pair = Some(*twin);
                }
            }
        }

        Self {
            verticies,
            faces,
            edges,
        }
    }
}

impl<T> Drop for HalfEdgeDs<T> {
    fn drop(&mut self) {
        for vert in &self.verticies {
            if !vert.is_null() {
                unsafe {
                    drop(Box::from_raw(*vert));
                }
            }
        }
        self.verticies.clear();

        for edge in &self.edges {
            unsafe {
                drop(Box::from_raw(*edge));
            }
        }
        self.edges.clear();

        for face in &self.faces {
            unsafe {
                drop(Box::from_raw(*face));
            }
        }
        self.faces.clear();
    }
}

impl<T> LiveRenderable<T> for HalfEdgeDs<T>
where
    T: Float + Copy + Debug + Clone,
{
    fn to_renderable(&self) -> Vec<(ZDepth, gl::types::GLenum, Polygon<T>)> {
        let mut drawable = Vec::new();

        for face in &self.faces {
            unsafe {
                let color = (**face).color;

                let start_edge = (**face).edge.unwrap();
                let mut current = start_edge;
                loop {
                    let next = (*current).next.unwrap();

                    let start = (*(*current).vertex).vertex;
                    let end = (*(*next).vertex).vertex;
                    let diff = (*(*next).vertex).vertex - (*(*current).vertex).vertex;
                    let normalized_diff = diff / diff.magnitude();

                    let normal = diff.normal() / diff.normal().magnitude();
                    let start = start + normal * T::from(0.01).unwrap();
                    let end = end + normal * T::from(0.01).unwrap();

                    let end_arrow = start + diff * T::from(0.7).unwrap();
                    let start_arrow = end_arrow - normalized_diff * T::from(0.1).unwrap();
                    let start_arrow_upper = start_arrow + normal * T::from(0.04).unwrap();
                    let start_arrow_lower = start_arrow - normal * T::from(0.04).unwrap();

                    drawable.push((
                        0,
                        gl::LINES,
                        Polygon::new(vec![start_arrow_upper, end_arrow]),
                    ));
                    drawable.last_mut().unwrap().2.set_color(color);

                    drawable.push((
                        0,
                        gl::LINES,
                        Polygon::new(vec![start_arrow_lower, end_arrow]),
                    ));
                    drawable.last_mut().unwrap().2.set_color(color);

                    drawable.push((0, gl::LINES, Polygon::new(vec![start, end])));
                    drawable.last_mut().unwrap().2.set_color(color);

                    current = next;
                    if current == start_edge {
                        break;
                    }
                }
            }
        }

        drawable
    }
}
