use num::Float;

use crate::objects::{
    color::{BLUE, Color, GREEN, MAGENTA, RED, YELLOW},
    vertex::Vertex,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum VertexType {
    Start,
    Split,
    End,
    Merge,
    Regular,
}

impl From<VertexType> for Color {
    fn from(value: VertexType) -> Self {
        match value {
            VertexType::Start => GREEN,
            VertexType::Split => BLUE,
            VertexType::End => RED,
            VertexType::Merge => MAGENTA,
            VertexType::Regular => YELLOW,
        }
    }
}

pub struct LinkedVertex<T: Float + Copy> {
    pub index: usize,
    pub vertex: Vertex<T>,
    pub vert_type: VertexType,
    pub prev: Option<*const LinkedVertex<T>>,
    pub next: Option<*const LinkedVertex<T>>,
    pub visited: bool,
}

impl<T: Float + Copy> LinkedVertex<T> {
    fn new(vertex: Vertex<T>, vert_type: VertexType, index: usize) -> *mut Self {
        Box::into_raw(Box::new(Self {
            index,
            vertex,
            vert_type,
            prev: None,
            next: None,
            visited: false,
        }))
    }

    pub fn from_verticies(verticies: &[Vertex<T>]) -> Vec<*mut Self> {
        assert!(verticies.len() >= 2);

        let first = Self::new(verticies[0], VertexType::Regular, verticies[0].id);
        let mut current = first;
        let mut result = vec![first];

        for i in 1..verticies.len() {
            let node = Self::new(verticies[i], VertexType::Regular, verticies[i].id);
            unsafe {
                (*node).prev = Some(current);
                (*current).next = Some(node);
            }
            current = node;
            result.push(node);
        }

        unsafe {
            (*first).prev = Some(current);
            (*current).next = Some(first);
        }

        result
    }

    pub fn connect(node1: *mut Self, node2: *mut Self) {
        unsafe {
            (*node1).next = Some(node2);
            (*node2).prev = Some(node1);
        }
    }

    pub fn insert_between(
        mut node: *mut Self,
        mut next: *mut Self,
        vert_type: VertexType,
        verticies: &mut Vec<*mut LinkedVertex<T>>,
    ) -> (*mut Self, *mut Self) {
        unsafe {
            let mut was_swapped = false;
            if vert_type == VertexType::Merge {
                let y1 = (*node).vertex.y();
                let y2 = (*next).vertex.y();
                if y1 < y2 {
                    std::mem::swap(&mut node, &mut next);
                    was_swapped = true;
                }
            } else if vert_type == VertexType::Split {
                let y1 = (*node).vertex.y();
                let y2 = (*next).vertex.y();
                if y1 > y2 {
                    std::mem::swap(&mut node, &mut next);
                    was_swapped = true;
                }
            }

            let id1 = (*node).index;
            let id2 = (*next).index;

            println!("Inserting edge between {id1} and {id2}");
            let mut new_sub_start = Self::new((*node).vertex, (*node).vert_type, verticies.len());
            verticies.push(new_sub_start);

            let mut new_sub_end = Self::new((*next).vertex, (*next).vert_type, verticies.len());
            verticies.push(new_sub_end);

            // Remove old connections and create subgraph
            Self::connect(new_sub_start, (*node).next.unwrap() as *mut _);

            Self::connect((*next).prev.unwrap() as *mut _, new_sub_end);

            // Connect subgraph back together
            Self::connect(new_sub_end, new_sub_start);

            // Connect old graph back together
            Self::connect(node, next);

            if was_swapped {
                std::mem::swap(&mut new_sub_start, &mut new_sub_end);
            }
            (new_sub_start, new_sub_end)
        }
    }
}

impl<T: Float + Copy> Eq for LinkedVertex<T> {}

impl<T: Float + Copy> PartialEq for LinkedVertex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.vertex == other.vertex
    }
}

impl<T: Float + Copy> Ord for LinkedVertex<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.vertex.cmp(&other.vertex)
    }
}

impl<T: Float + Copy> PartialOrd for LinkedVertex<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.vertex.cmp(&other.vertex))
    }
}
