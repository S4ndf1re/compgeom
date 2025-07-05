use std::{collections::HashMap, fmt::Debug, marker::PhantomData, thread::current};

use num::Float;

use crate::{
    algorithm::tesselation::inner_angle,
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

impl<T> HeFace<T>
where
    T: Float + Copy + Debug,
{
    pub fn query_verticies(&self) -> Vec<*mut HeVertex<T>> {
        let mut verticies = Vec::new();

        unsafe {
            let start_edge = self.edge.unwrap();
            let mut current = start_edge;

            loop {
                verticies.push((*current).vertex);
                current = (*current).next.unwrap();
                if current == start_edge {
                    break;
                }
            }
        }

        verticies
    }

    pub fn contains_vertex(&self, vertex: *mut HeVertex<T>) -> bool {
        let verticies = self.query_verticies();

        verticies.contains(&vertex)
    }
}

pub struct HeEdge<T> {
    pub vertex: *mut HeVertex<T>,
    pub pair: Option<*mut HeEdge<T>>,
    pub face: Option<*mut HeFace<T>>,
    pub next: Option<*mut HeEdge<T>>,
    pub prev: Option<*mut HeEdge<T>>,
    _data: PhantomData<T>,
}

impl<T> HeEdge<T>
where
    T: Debug + Float + Copy,
{
    pub fn is_flippable(edge: *mut HeEdge<T>) -> bool {
        unsafe {
            if (*edge).pair.is_none() {
                return false;
            }

            let twin = (*edge).pair.unwrap();

            let e1 = (*edge).next.unwrap();
            let e2 = (*edge).prev.unwrap();

            let e3 = (*twin).next.unwrap();
            let e4 = (*twin).prev.unwrap();

            let verticies = [
                (*(*e1).vertex).vertex,
                (*(*e2).vertex).vertex,
                (*(*edge).vertex).vertex,
                (*(*e4).vertex).vertex,
            ];

            let mut angles = Vec::with_capacity(4);
            let n = verticies.len();
            for i in 0..n {
                let a = verticies[i];
                let b = verticies[(i + 1) % n];
                let c = verticies[(i + 2) % n];

                angles.push(inner_angle(&a, &b, &c));
            }

            angles
                .iter()
                .all(|a| *a < T::from(std::f64::consts::PI).unwrap())
                && (*e1).face.unwrap() != (*e4).face.unwrap()
                && (*e2).face.unwrap() != (*e3).face.unwrap()
        }
    }

    pub fn flip_edge(edge: *mut HeEdge<T>) {
        if !Self::is_flippable(edge) {
            return;
        }

        unsafe {
            let e4 = (*edge).next.unwrap();
            let e5 = (*edge).prev.unwrap();

            let twin = (*edge).pair.unwrap();
            let e0 = (*twin).next.unwrap();
            let e1 = (*twin).prev.unwrap();

            // ensure that no vertex or face references e or twin. This is inherently safe, since
            // the edge for both the vertex and the face are arbitrary
            for half_edge in [e0, e1, e4, e5] {
                (*(*half_edge).vertex).edge = Some(half_edge);
            }
            (*(*e1).face.unwrap()).edge = Some(e1);
            (*(*e5).face.unwrap()).edge = Some(e5);

            // Update the faces and edge and twin. This will result in an inconsistent state. Make
            // sure, the last step after is also executed
            (*edge).next = Some(e5);
            (*edge).prev = Some(e0);
            (*edge).vertex = (*e1).vertex;
            (*edge).face = (*e5).face;
            // do the same for the twin, in opposite order
            (*twin).next = Some(e1);
            (*twin).prev = Some(e4);
            (*twin).vertex = (*e5).vertex;
            (*twin).face = (*e1).face;

            // bring back consistency
            (*e0).next = Some(edge);
            (*e1).next = Some(e4);
            (*e4).next = Some(twin);
            (*e5).next = Some(e0);

            (*e0).prev = Some(e5);
            (*e1).prev = Some(twin);
            (*e4).prev = Some(e1);
            (*e5).prev = Some(edge);
        }
    }
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

        let mut half_edge_tracked: HashMap<(usize, usize), *mut HeEdge<T>> = HashMap::new();
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
                    prev: None,
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
                    let n = tmp_edges.len();
                    (*tmp_edges[i]).next = Some(tmp_edges[(i + 1) % n]);
                    (*tmp_edges[(i + 1) % n]).prev = Some(tmp_edges[i]);

                    half_edge_tracked.insert(
                        (
                            (*(*tmp_edges[i]).vertex).vertex.id,
                            (*(*tmp_edges[(i + 1) % n]).vertex).vertex.id,
                        ),
                        tmp_edges[i],
                    );
                }
            }

            edges.extend(tmp_edges);
            faces.push(face);
        }

        println!("Half Edge Store: {half_edge_tracked:?}");
        // set all twins. This is the second phase
        for ((from, to), edge) in &half_edge_tracked {
            // Find the reverse edge going from to->from
            if let Some(twin) = half_edge_tracked.get(&(*to, *from)) {
                println!(
                    "Setting halfedge of {edge:?} to {twin:?}, ({from}->{to}), ({to}->{from})",
                );
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

    pub fn query_all_faces(&self) -> Vec<*mut HeFace<T>> {
        self.faces.clone()
    }

    pub fn query_all_verticies(&self) -> Vec<*mut HeVertex<T>> {
        self.verticies
            .iter()
            .filter(|v| !v.is_null())
            .copied()
            .collect()
    }

    pub fn query_all_edges(&self) -> Vec<*mut HeEdge<T>> {
        self.edges.clone()
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

                let edges = (**face).query_verticies();
                assert!(edges.len() == 3);

                for i in 0..edges.len() {
                    let current = edges[i];
                    let next = edges[(i + 1) % edges.len()];

                    let start = (*current).vertex;
                    let end = (*next).vertex;
                    let diff = (*next).vertex - (*current).vertex;
                    let normalized_diff = diff / diff.magnitude();

                    let normal = diff.normal() / diff.normal().magnitude();
                    let start = start + normal * T::from(0.01).unwrap();
                    let end = end + normal * T::from(0.01).unwrap();

                    let end_arrow = start + diff * T::from(0.7).unwrap();
                    let start_arrow = end_arrow - normalized_diff * T::from(0.1).unwrap();
                    let start_arrow_upper = start_arrow + normal * T::from(0.04).unwrap();

                    drawable.push((
                        1,
                        gl::LINES,
                        Polygon::new(vec![start_arrow_upper, end_arrow]),
                    ));
                    drawable.last_mut().unwrap().2.set_color(color);

                    drawable.push((1, gl::LINES, Polygon::new(vec![start, end])));
                    drawable.last_mut().unwrap().2.set_color(color);
                }
            }
        }

        drawable
    }
}
