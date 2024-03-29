use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::{fmt, fs};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

/*
Child
 */
#[derive(Debug, Copy)]
pub struct Child {
    pub(crate) idx: usize,
    pub(crate) weight: u32,
}

impl Clone for Child {
    fn clone(&self) -> Self {
        *self
    }
}

impl Child {
    pub(crate) fn new(idx: usize) -> Child {
        Child {
            idx,
            weight: 0
        }
    }

    pub(crate) fn new_with_weight(idx: usize, weight: u32) -> Child {
        Child {
            idx,
            weight,
        }
    }

    pub(crate) fn get_clean_value(&self, weighted: bool) -> String {
        if weighted {
            format!("{} {}", self.idx, self.weight)
        } else {
            format!("{}", self.idx)
        }
    }
}

/*
Node
 */
#[derive(Debug)]
pub struct Node<T> {
    pub(crate) val: T,
    pub(crate) children: Vec<Child> // child and weight
}

impl<T> Node<T> {
    pub(crate) fn new(val: T) -> Node<T> {
        Node {
            val,
            children: vec![]
        }
    }

    pub(crate) fn new_with_children(val: T, children: Vec<Child>) -> Node<T> {
        Node {
            val,
            children
        }
    }

    pub(crate) fn add_child(&mut self, node: Child) {
        self.children.push(node);
    }

    pub(crate) fn add_children(&mut self, nodes: Vec<Child>) {
        for node in nodes {
            self.add_child(node);
        }
    }
}

impl<T> From<(T, Vec<usize>)> for Node<T> {
    fn from(value: (T, Vec<usize>)) -> Self {
        Node {
            val: value.0,
            children: value.1
                .iter()
                .map(|val| Child::new(val.clone()))
                .collect()
        }
    }
}

impl<T> From<(T, Vec<(usize, u32)>)> for Node<T> {
    fn from(value: (T, Vec<(usize, u32)>)) -> Self {
        Node {
            val: value.0,
            children: value.1
                .iter()
                .map(|val| Child::new_with_weight(val.0.clone(), val.1.clone()))
                .collect()
        }
    }
}

/*
Graph
 */
#[derive(Debug)]
pub struct Graph<T> {
    pub(crate) nodes: Vec<Node<T>>,
    pub(crate) weighted: bool
}

/*
New
 */
impl<T> Graph<T> {
    pub(crate) fn new() -> Graph<T> {
        Graph {
            nodes: vec![],
            weighted: false
        }
    }

    pub(crate) fn add_node(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    pub(crate) fn add_nodes(&mut self, nodes: Vec<Node<T>>) {
        for node in nodes {
            self.add_node(node);
        }
    }

    pub(crate) fn alloc_childs(&mut self, num: usize) {
        self.nodes.reserve(num);
    }
}

impl<T> Graph<T>
    where T: Default {
    pub(crate) fn new_with_size(size: usize) -> Graph<T> {
        let mut v = vec![];

        v.resize_with(size, || Node {
            val: T::default(),
            children: vec![]
        });

        Graph {
            nodes: v,
            weighted: false
        }
    }
}

/*
From implementations
 */
impl<T> From<Vec<(T, Vec<usize>)>> for Graph<T>
    where T: Default + Clone {
    fn from(value: Vec<(T, Vec<usize>)>) -> Self {
        let mut graph = Graph::<T>::new_with_size(value.len());

        for (idx, node) in value.iter().enumerate() {
            graph.nodes[idx] = Node::from(node.clone())
        }

        graph
    }
}

impl From<File> for Graph<String> {
    fn from(value: File) -> Self {
        let mut graph: Graph<String> = Graph::new();

        let mut reader = BufReader::new(value);

        // read the first line
        let mut number_of_nodes = String::new();
        reader.read_line(&mut number_of_nodes).unwrap();

        let number_of_nodes: u32 = number_of_nodes.trim_end().parse().unwrap();

        reader.lines().for_each(|line| {
            let line = line.unwrap();

            let mut split = line.split_whitespace();

            let idx: u32 = split.next().unwrap().parse().unwrap();
            let value: String = split.next().unwrap().parse().unwrap();

            let num_nodes: u32 = split.next().unwrap().parse().unwrap();

            let node_children: Vec<usize> = (0..num_nodes)
                .map(|i| split.next().unwrap().parse().unwrap())
                .collect();

            graph.add_node(Node::from((value, node_children)));
        });

        graph
    }
}


impl From<File> for Graph<char> {
    fn from(value: File) -> Self {
        let mut graph: Graph<char> = Graph::new();

        let mut reader = BufReader::new(value);

        // read the first line
        let mut number_of_nodes = String::new();
        reader.read_line(&mut number_of_nodes).unwrap();

        let number_of_nodes: u32 = number_of_nodes.trim_end().parse().unwrap();

        reader.lines().for_each(|line| {
            let line = line.unwrap();

            let mut split = line.split_whitespace();

            let idx: u32 = split.next().unwrap().parse().unwrap();
            let value: char = split.next().unwrap().parse().unwrap();

            let num_nodes: usize = split.next().unwrap().parse().unwrap();
            let children_count = split.clone().count();

            let weighted = children_count != num_nodes;

            if weighted {
                let node_children: Vec<(usize, u32)> = (0..num_nodes)
                    .map(|i| (
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap()
                    ))
                    .collect();

                graph.add_node(Node::from((value, node_children)));
            } else {
                let node_children: Vec<usize> = (0..num_nodes)
                    .map(|i| split.next().unwrap().parse().unwrap())
                    .collect();

                graph.add_node(Node::from((value, node_children)));
            }
        });

        graph
    }
}

impl<T> Display for Graph<T>
    where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut display_string = String::new();

        // Add number of nodes
        display_string += &*self.nodes.len().to_string();

        let index_width = display_string.len();

        let mut num_child_width = 0;
        let mut child_width = 0;
        let mut val_width = 0;

        for node in self.nodes.iter() {
            num_child_width = if node.children.len().to_string().len() > num_child_width {
                node.children.len().to_string().len()
            } else {
                num_child_width
            };

            val_width = if node.val.to_string().len() > val_width {
                node.val.to_string().len()
            } else {
                val_width
            };

            if self.weighted {
                for child in node.children.iter() {
                    child_width = if child.weight.to_string().len() > child_width {
                        child.weight.to_string().len()
                    } else {
                        child_width
                    };
                }
            }
        }

        if self.weighted {
            child_width += 3;
        }

        val_width += 1;

        // Add header
        display_string += "\n";
        display_string += format!("{:index_width$}  {:val_width$}{:num_child_width$}   {:width$}\n", "n", "v", "nc", "c", width = index_width + child_width).as_str();


        for (i, node) in self.nodes.iter().enumerate() {
            display_string += &*format!("{:indexWidth$} {:valWidth$} {:numChildWidth$}    ",
                                        i, node.val, node.children.len(),
                                        indexWidth = index_width,
                                        valWidth = val_width,
                                        numChildWidth = num_child_width);

            for child in node.children.iter() {
                display_string += &*format!("{:width$} ", child.get_clean_value(self.weighted), width = index_width + child_width);
            }

            display_string += "\n";
        }

        write!(f, "{display_string}")
    }
}

/*
Traversal
 */
impl<T> Graph<T> {
    pub(crate) fn dfs(&self, from_index: usize) -> Vec<&Node<T>> {
        let mut parent: VecDeque<usize> = VecDeque::new();
        let mut out: Vec<&Node<T>> = vec![&self.nodes[from_index]];
        let mut visited: Vec<bool> = vec![false; self.nodes.len()];

        parent.reserve(self.nodes.len());
        out.reserve(self.nodes.len());
        visited[from_index] = true;

        let mut current_node = from_index;

        'outer: loop {
            // loop through all children to the current node
            for c in self.nodes[current_node].children.iter() {
                // check if not the child is already visited
                if !visited[c.idx] {
                    // register as visited
                    visited[c.idx] = true;

                    // add parent to parent stack
                    parent.push_back(current_node);

                    // add child node as current node
                    current_node = c.idx;

                    // add child node to output
                    out.push(&self.nodes[current_node]);

                    // continue loop
                    continue 'outer;
                }
            }

            // there are no childs, move back to parent, or break out of loop
            match parent.pop_back() {
                Some(v) => current_node = v,
                None => return out
            }
        }
    }

    pub(crate) fn bfs(&self, from_index: usize) -> Vec<&Node<T>> {
        let mut childs: VecDeque<usize> = VecDeque::new();
        let mut out: Vec<&Node<T>> = vec![];
        let mut visited: Vec<bool> = vec![false; self.nodes.len()];

        childs.reserve(self.nodes.len());
        out.reserve(self.nodes.len());

        let mut current_node: usize = from_index;

        loop {
            if !visited[current_node] {
                self.nodes[current_node].children.iter()
                    .filter(|c| !visited[c.idx])
                    .for_each(|c| childs.push_front(c.idx));

                visited[current_node] = true;
                out.push(&self.nodes[current_node]);
            }

            match childs.pop_back() {
                Some(v) => current_node = v,
                None => return out
            }
        }
    }
}

/*
Get neighborhood
 */
impl<T> Graph<T> {
    pub(crate) fn get_neighborhood_matrix(&self) -> Vec<Vec<bool>> {
        let mut neighborhood = vec![vec![false; self.nodes.len()]; self.nodes.len()];

        for i in 0..neighborhood.len() {
            neighborhood[i][i] = true;
        }

        self.nodes
            .iter()
            .enumerate()
            .for_each(|(idx, node)| node.children
                    .iter()
                    .for_each(|child| neighborhood[idx][child.idx] = true)
            );

        neighborhood
    }
}

/*
Reachability
 */
impl<T> Graph<T> {
    pub(crate) fn warshall(self) -> Vec<Vec<bool>> {
        let neighborhood_size = self.nodes.len();
        let mut neighborhood = self.get_neighborhood_matrix();

        (0..neighborhood_size).for_each(|k| {
            (0..neighborhood_size).for_each(|i| {
                (0..neighborhood_size).for_each(|j| {
                    neighborhood[i][j] = neighborhood[i][j] || (neighborhood[i][k] && neighborhood[k][j]);
                });
            });
        });

        neighborhood
    }
}






