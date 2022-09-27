use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Node<T>
where T: Clone, T: Display, T: Debug {
    pub(crate) val: T,
    pub(crate) children: Vec<(*const Node<T>, u32)> // child and weight
}

impl<T> Node<T>
where T: Clone, T: Display, T: Debug {
    pub(crate) fn add_child(&mut self, node: *const Node<T>, weight: u32) {
        self.children.push((node, weight));
    }

    pub(crate) fn add_children(&mut self, nodes: Vec<*const Node<T>>, weights: Vec<u32>) {
        for i in 0..nodes.len() {
            self.add_child(nodes[i], weights[i]);
        }
    }
}

#[derive(Debug)]
pub struct Graph<T>
where T: PartialOrd, T: Copy, T: Display, T: Debug {
    pub(crate) data: Vec<Node<T>>
}

