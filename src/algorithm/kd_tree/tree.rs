use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    time::SystemTime,
};

use num::Float;
use winit::{
    event::KeyEvent,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::{
    basic_rendering::live_renderable::{LiveRenderable, ZDepth},
    objects::{aabb_rect::AaBbRect, color::GLOBAL_COLOR_GENERATOR, vertex::Vertex},
};

use super::{
    node::{Node, convert_to_visual_nodes},
    range_query::RangeQuery,
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
    last_query: Option<RangeQuery<T>>,
}

impl<T> KdTree<T>
where
    T: Float + Copy + Debug + Display,
{
    pub fn build(points: &[Vertex<T>]) -> Self {
        let mut this = Self {
            root: Box::new(Node {
                id: 1,
                ..Default::default()
            }),
            current_selected: vec![],
            last_pressed: SystemTime::now(),
            last_query: None,
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

            let mut node_left: Node<Vertex<T>> = Node {
                id: 2 * node.id,
                ..Default::default()
            };
            let mut node_right: Node<Vertex<T>> = Node {
                id: 2 * node.id + 1,
                ..Default::default()
            };

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

    fn range_query_rec(node: &mut Node<Vertex<T>>, query: &RangeQuery<T>) -> Vec<Vertex<T>> {
        println!("visited node {}", node.id);
        if node.value.is_none() {
            return vec![];
        }

        let ((l, r), coord) = if node.is_y {
            (query.y_range.into(), node.value.unwrap().y())
        } else {
            (query.x_range.into(), node.value.unwrap().x())
        };

        let mut output = vec![];

        node.range_query_result = false;
        if query.is_contained((&node.value.unwrap().x(), &node.value.unwrap().y())) {
            println!("found point {:?}", node.value.unwrap());
            node.range_query_result = true;
            output.push(node.value.unwrap());
        }

        // Reset children to not visited
        if node.left.is_some() {
            node.left.as_mut().unwrap().range_query_visited = false;
        }

        if node.right.is_some() {
            node.right.as_mut().unwrap().range_query_visited = false;
        }

        if l < coord && node.left.is_some() {
            // Node was visisted
            println!("visiting left node {}", node.left.as_ref().unwrap().id);
            node.left.as_mut().unwrap().range_query_visited = true;
            output.extend(Self::range_query_rec(node.left.as_mut().unwrap(), query));
        }

        if coord < r && node.right.is_some() {
            println!("visiting right node {}", node.right.as_ref().unwrap().id);
            node.right.as_mut().unwrap().range_query_visited = true;
            output.extend(Self::range_query_rec(node.right.as_mut().unwrap(), query));
        }

        output
    }

    pub fn range_query(&mut self, query: RangeQuery<T>) -> Vec<Vertex<T>> {
        self.last_query = Some(query);

        if self.root.value.is_none() {
            return vec![];
        }

        Self::range_query_rec(&mut self.root, &query)
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
    T: Copy + Debug + Float + Display,
{
    fn to_renderable(
        &self,
    ) -> Vec<(
        ZDepth,
        gl::types::GLenum,
        crate::objects::polygon::Polygon<T>,
    )> {
        GLOBAL_COLOR_GENERATOR.reset();
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

        if let Some(query) = self.last_query {
            objects.extend(query.to_renderable());
        }

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
