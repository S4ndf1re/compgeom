use std::fmt::Debug;

use num::Float;

use crate::{
    basic_rendering::live_renderable::{LiveRenderable, ZDepth},
    objects::{
        aabb_rect::AaBbRect,
        color::{GLOBAL_COLOR_GENERATOR, GREEN, MAGENTA, RED, WHITE, YELLOW},
        polygon::Polygon,
        vertex::Vertex,
    },
};

#[derive(Clone)]
pub struct Node<T> {
    pub id: usize,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub value: Option<T>,
    pub is_y: bool,
    pub selected: bool,
    pub range_query_visited: bool,
    pub range_query_result: bool,
}

pub fn convert_to_visual_nodes<'n, T: Float + Copy>(
    root: &'n Node<Vertex<T>>,
    rect: AaBbRect<T>,
    list: &mut Vec<VisualisedNode<'n, T>>,
) {
    if root.value.is_none() {
        return;
    }

    list.push(VisualisedNode::new(root, rect.clone()));
    let (left, right) = if root.is_y {
        rect.split_by(root.value.unwrap().y(), !root.is_y)
    } else {
        rect.split_by(root.value.unwrap().x(), !root.is_y)
    };

    if let Some(left_child) = &root.left
        && let Some(left_box) = left
    {
        convert_to_visual_nodes(left_child.as_ref(), left_box, list);
    }

    if let Some(right_child) = &root.right
        && let Some(right_box) = right
    {
        convert_to_visual_nodes(right_child.as_ref(), right_box, list);
    }
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self {
            id: 0,
            left: None,
            right: None,
            value: None,
            is_y: false,
            selected: false,
            range_query_visited: false,
            range_query_result: false,
        }
    }
}

#[derive(Clone)]
pub struct VisualisedNode<'n, T> {
    node: &'n Node<Vertex<T>>,
    rect: AaBbRect<T>,
}

impl<'n, T> VisualisedNode<'n, T>
where
    T: Copy + Float,
{
    pub fn new(node: &'n Node<Vertex<T>>, rect: AaBbRect<T>) -> Self {
        Self { node, rect }
    }
}

impl<'n, T> LiveRenderable<T> for VisualisedNode<'n, T>
where
    T: Copy + Debug + Float,
{
    fn to_renderable(&self) -> Vec<(ZDepth, gl::types::GLenum, Polygon<T>)> {
        if self.node.value.is_none() {
            return vec![];
        }

        let mut objects = Vec::with_capacity(2);

        let (start, end) = if self.node.is_y {
            let start: Vertex<_> = (self.rect.x, self.node.value.unwrap().y()).into();
            let end: Vertex<_> = (self.rect.x + self.rect.w, self.node.value.unwrap().y()).into();
            (start, end)
        } else {
            let start: Vertex<_> = (self.node.value.unwrap().x(), self.rect.y).into();
            let end: Vertex<_> = (self.node.value.unwrap().x(), self.rect.y + self.rect.h).into();
            (start, end)
        };

        if self.node.left.is_some() || self.node.right.is_some() {
            let mut polygon = Polygon::new(vec![start, end]);
            let mut depth = 1;
            polygon.set_color(WHITE);
            if self.node.selected {
                polygon.set_color(YELLOW);
                depth = 3;
            }
            objects.push((depth, gl::LINES, polygon));
        }

        let mut polygon = Polygon::new(vec![self.node.value.unwrap()]);
        polygon.set_color(RED);
        if self.node.selected || self.node.range_query_result {
            polygon.set_color(MAGENTA);
        }
        objects.push((4, gl::POINTS, polygon));

        let polygons = self.rect.to_renderable();
        let (mut depth, mode, mut polygon) = polygons.first().unwrap().clone();
        polygon.set_color([0.3, 0.3, 0.3, 1.0]);
        if self.node.selected {
            depth = 2;
            polygon.set_color(GREEN);
        }
        objects.push((depth, mode, polygon.clone()));

        let mut polygon = polygon.clone();
        if self.node.range_query_visited {
            polygon.set_color(GLOBAL_COLOR_GENERATOR.next_pastell_color(1.0));
            objects.push((0, gl::TRIANGLE_FAN, polygon));
        }

        objects
    }
}
