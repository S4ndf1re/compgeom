use std::{
    collections::VecDeque,
    fmt::Debug,
    time::{SystemTime, SystemTimeError},
};

use num::Float;
use winit::{
    event::KeyEvent,
    keyboard::{Key, KeyCode, NamedKey, PhysicalKey},
};

use crate::{
    basic_rendering::live_renderable::{LiveRenderable, ZDepth},
    objects::{aabb_rect::AaBbRect, polygon::Polygon, vertex::Vertex},
};

#[derive(Clone, Debug)]
struct PartitionList<T>
where
    T: Copy + Debug,
{
    y: Vec<Vertex<T>>,
    x: Vec<Vertex<T>>,
}

impl<T> PartitionList<T>
where
    T: Float + Copy + Debug,
{
    pub fn new() -> Self {
        Self {
            y: vec![],
            x: vec![],
        }
    }

    pub fn add_y(&mut self, vert: Vertex<T>) {
        self.y.push(vert);
    }

    pub fn add_x(&mut self, vert: Vertex<T>) {
        self.x.push(vert);
    }

    pub fn find_y(&self, vert: &Vertex<T>) -> Result<usize, usize> {
        self.y
            .binary_search_by(|probe| probe.y().partial_cmp(&vert.y()).unwrap())
    }

    pub fn find_x(&self, vert: &Vertex<T>) -> Result<usize, usize> {
        self.x
            .binary_search_by(|probe| probe.x().partial_cmp(&vert.x()).unwrap())
    }

    pub fn sort(&mut self) {
        self.x.sort_by(|a, b| a.x().partial_cmp(&b.x()).unwrap());
        self.y.sort_by(|a, b| a.y().partial_cmp(&b.y()).unwrap());
    }

    pub fn clear(&mut self) {
        self.y.clear();
        self.x.clear();
    }

    pub fn x(&self) -> &Vec<Vertex<T>> {
        &self.x
    }

    pub fn y(&self) -> &Vec<Vertex<T>> {
        &self.y
    }

    pub fn is_empty(&self) -> bool {
        self.y.is_empty() && self.x.is_empty()
    }

    /// Partition the lists by either y or x, and return the pivot vertex
    fn partition_by(&self, by_y: bool) -> (Vertex<T>, PartitionList<T>, PartitionList<T>) {
        let mut list = &self.y;
        let mut alt_list = &self.x;

        if !by_y {
            list = &self.x;
            alt_list = &self.y;
        }

        // Last point is beeing partitioned
        if list.len() == 1 {
            let p = list.first().unwrap();
            return (*p, PartitionList::new(), PartitionList::new());
        } else if list.is_empty() {
            unimplemented!()
        }

        #[allow(clippy::unnecessary_cast)]
        let midpoint = (0 as usize).midpoint(list.len());
        let median = list[midpoint];

        // both lists are sorted, meaning binary search works
        let mut partition_left = PartitionList::<T>::new();
        let mut partition_right = PartitionList::<T>::new();

        for p in list[0..midpoint].iter() {
            if by_y {
                partition_left.add_y(*p);
            } else {
                partition_left.add_x(*p);
            }
        }

        for p in list[(midpoint + 1)..].iter() {
            if by_y {
                partition_right.add_y(*p);
            } else {
                partition_right.add_x(*p);
            }
        }

        for p in alt_list.iter() {
            if by_y {
                if let Ok(_idx) = partition_left.find_y(p) {
                    partition_left.add_x(*p);
                }
                if let Ok(_idx) = partition_right.find_y(p) {
                    partition_right.add_x(*p);
                }
            } else {
                if let Ok(_idx) = partition_left.find_x(p) {
                    partition_left.add_y(*p);
                }
                if let Ok(_idx) = partition_right.find_x(p) {
                    partition_right.add_y(*p);
                }
            }
        }

        (median, partition_left, partition_right)
    }
}

#[derive(Clone)]
pub struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: Option<T>,
    is_y: bool,
    selected: bool,
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
            left: None,
            right: None,
            value: None,
            is_y: false,
            selected: false,
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
    fn to_renderable(self) -> Vec<(ZDepth, gl::types::GLenum, Polygon<T>)> {
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

        let mut polygon = Polygon::new(vec![start, end]);
        let mut depth = 1;
        polygon.set_color([1.0, 1.0, 1.0, 1.0]);
        if self.node.selected {
            polygon.set_color([1.0, 1.0, 0.0, 1.0]);
            depth = 3;
        }
        objects.push((depth, gl::LINES, polygon));

        let mut polygon = Polygon::new(vec![self.node.value.unwrap()]);
        polygon.set_color([1.0, 0.0, 0.0, 1.0]);
        if self.node.selected {
            polygon.set_color([1.0, 0.0, 1.0, 1.0]);
        }
        objects.push((4, gl::POINTS, polygon));

        let polygons = self.rect.to_renderable();
        let (mut depth, mode, mut polygon) = polygons.first().unwrap().clone();
        polygon.set_color([0.3, 0.3, 0.3, 1.0]);
        if self.node.selected {
            depth = 2;
            polygon.set_color([0.0, 1.0, 0.0, 1.0]);
        }
        objects.push((depth, mode, polygon));

        objects
    }
}

pub enum Step {
    Left,
    Right,
    Parent,
}

#[derive(Clone)]
pub struct KdTree<T>
where
    T: Copy,
{
    root: Box<Node<Vertex<T>>>,
    current_selected: Vec<*const Node<Vertex<T>>>,
    last_pressed: SystemTime,
}

impl<T> KdTree<T>
where
    T: Float + Copy + Debug,
{
    pub fn build(points: &[Vertex<T>]) -> Self {
        let mut this = Self {
            root: Box::default(),
            current_selected: vec![],
            last_pressed: SystemTime::now(),
        };

        this.current_selected.push(Box::as_ptr(&this.root));
        this.root.selected = true;

        let mut list = PartitionList::new();
        for (i, p) in points.iter().enumerate() {
            let mut p = *p;
            p.id = i;
            list.add_x(p);
            list.add_y(p);
        }

        list.sort();

        Self::construct_tree(list.clone(), &mut this.root, true);

        this
    }

    fn construct_tree(partition: PartitionList<T>, node: &mut Node<Vertex<T>>, by_y: bool) {
        if !partition.is_empty() {
            let (median, left, right) = partition.partition_by(by_y);
            node.value = Some(median);
            node.is_y = by_y;

            let mut node_left: Node<Vertex<T>> = Default::default();
            let mut node_right: Node<Vertex<T>> = Default::default();

            Self::construct_tree(left, &mut node_left, !by_y);
            Self::construct_tree(right, &mut node_right, !by_y);

            if node_left.value.is_some() {
                node.left = Some(Box::new(node_left));
            }

            if node_right.value.is_some() {
                node.right = Some(Box::new(node_right));
            }
        }
    }

    pub fn step(&mut self, step: Step) {
        // This is inherently safe, since there is no referenced outside access to nodes. Hence we
        // are contained within the step function that is already checked by the borrow checker.
        unsafe {
            let currently_selected =
                self.current_selected[self.current_selected.len() - 1] as *mut Node<Vertex<T>>;
            (*currently_selected).selected = false;
        }

        match step {
            Step::Left => unsafe {
                if (*self.current_selected[self.current_selected.len() - 1])
                    .left
                    .is_some()
                {
                    self.current_selected.push(Box::as_ptr(
                        (*self.current_selected[self.current_selected.len() - 1])
                            .left
                            .as_ref()
                            .unwrap(),
                    ));
                }
            },
            Step::Right => unsafe {
                if (*self.current_selected[self.current_selected.len() - 1])
                    .right
                    .is_some()
                {
                    self.current_selected.push(Box::as_ptr(
                        (*self.current_selected[self.current_selected.len() - 1])
                            .right
                            .as_ref()
                            .unwrap(),
                    ));
                }
            },
            Step::Parent => {
                if self.current_selected.len() > 1 {
                    self.current_selected.pop();
                }
            }
        };

        // This is inherently safe, since there is no referenced outside access to nodes. Hence we
        // are contained within the step function that is already checked by the borrow checker.
        unsafe {
            let currently_selected =
                self.current_selected[self.current_selected.len() - 1] as *mut Node<Vertex<T>>;
            (*currently_selected).selected = true;
        }
    }

    /// This function will return a clone to the currently selected node
    pub fn get_current_selected(&self) -> Node<Vertex<T>> {
        unsafe { (**self.current_selected.last().unwrap()).clone() }
    }
}

impl<T> LiveRenderable<T> for KdTree<T>
where
    T: Copy + Debug + Float,
{
    fn to_renderable(
        self,
    ) -> Vec<(
        ZDepth,
        gl::types::GLenum,
        crate::objects::polygon::Polygon<T>,
    )> {
        let mut objects = vec![];

        let mut nodes_to_visit = VecDeque::new();
        let mut original_bounding_box = AaBbRect::new(T::zero(), T::zero(), T::zero(), T::zero());
        nodes_to_visit.push_back(self.root.as_ref());

        while !nodes_to_visit.is_empty() {
            if let Some(next) = nodes_to_visit.pop_front() {
                if let Some(value) = next.value {
                    original_bounding_box.merge_point_into_self(value);
                }

                if let Some(left) = &next.left {
                    nodes_to_visit.push_back(left);
                }

                if let Some(right) = &next.right {
                    nodes_to_visit.push_back(right);
                }
            }
        }

        let mut visual_nodes = vec![];
        convert_to_visual_nodes(
            self.root.as_ref(),
            original_bounding_box.clone(),
            &mut visual_nodes,
        );

        for vnode in visual_nodes {
            objects.extend(vnode.to_renderable());
        }

        let mut bounding_rect_render = original_bounding_box.to_renderable();
        bounding_rect_render
            .first_mut()
            .unwrap()
            .2
            .set_color([0.3, 0.3, 0.3, 1.0]);

        objects.extend(bounding_rect_render);

        objects
    }

    fn user_input(&mut self, event: Option<KeyEvent>) {
        if event.is_none() {
            return;
        }

        let current_time = SystemTime::now();

        let elapsed = current_time.duration_since(self.last_pressed);
        if elapsed.is_err() || elapsed.unwrap().as_millis() < 300 {
            return;
        }

        match event.unwrap() {
            KeyEvent {
                physical_key: PhysicalKey::Code(KeyCode::KeyR),
                ..
            } => {
                self.step(Step::Right);
                self.last_pressed = current_time;
            }
            KeyEvent {
                physical_key: PhysicalKey::Code(KeyCode::KeyL),
                ..
            } => {
                self.step(Step::Left);
                self.last_pressed = current_time;
            }
            KeyEvent {
                physical_key: PhysicalKey::Code(KeyCode::KeyP),
                ..
            } => {
                self.step(Step::Parent);
                self.last_pressed = current_time;
            }
            _ => (),
        }
    }

    fn get_current_title(&self) -> String {
        if let Some(current_selected) = self.current_selected.last() {
            // This is safe, since there is no outside access to self.root and all sub-nodes.
            // Since &self is check by the borrow checker, self.root and all sub-nodes are also
            // checked
            unsafe {
                if let Some(value) = (**current_selected).value {
                    format!("Currently Selected: {value:?}")
                } else {
                    String::from("No current selection")
                }
            }
        } else {
            String::from("No current selection")
        }
    }
}
