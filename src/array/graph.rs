use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::{fmt, fs};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Child
 */
#[derive(Debug, Copy)]
pub struct Child {
    idx: usize,
    weight: u32
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
            weight
        }
    }

    pub(crate) fn getCleanValue(&self, weighted: bool) -> String {
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
pub struct Node<T>
where T: Debug {
    pub(crate) val: T,
    pub(crate) children: Vec<Child> // child and weight
}

impl<T> Node<T>
where T: Debug {
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

/*
Graph
 */
#[derive(Debug)]
pub struct Graph<T>
where T: Debug {
    pub(crate) nodes: Vec<Node<T>>,
    pub(crate) weighted: bool
}

impl<T> Graph<T>
where T: Debug {
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
where T: Debug + Clone {
    pub(crate) fn new_with_size(size: usize, emptyNodeValue: T) -> Graph<T> {
        let mut v = vec![];

        v.resize_with(size, || Node {
            val: emptyNodeValue.clone(),
            children: vec![]
        });

        Graph {
            nodes: v,
            weighted: false
        }
    }
}

impl<T> Display for Graph<T>
where T: Debug + Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        s += &*self.nodes.len().to_string();
        let indexWidth = s.len();

        let mut numChildWidth = 0;
        let mut childWidth = 0;
        let mut valWidth = 0;

        for node in self.nodes.iter() {
            numChildWidth = if node.children.len().to_string().len() > numChildWidth {
                node.children.len().to_string().len()
            } else {
                numChildWidth
            };

            valWidth = if node.val.to_string().len() > valWidth {
                node.val.to_string().len()
            } else {
                valWidth
            };

            if self.weighted {
                for child in node.children.iter() {
                    childWidth = if child.weight.to_string().len() > childWidth {
                        child.weight.to_string().len()
                    } else {
                        childWidth
                    };
                }
            }
        }

        if self.weighted {
            childWidth += 3;
        }

        valWidth += 1;

        s += "\n";

        for (i, node) in self.nodes.iter().enumerate() {
            s += &*format!("{:indexWidth$} {:valWidth$} {:numChildWidth$}    ",
                           i, node.val, node.children.len(),
                           indexWidth = indexWidth,
                           valWidth = valWidth,
                           numChildWidth = numChildWidth);

            for child in node.children.iter() {
                s += &*format!("{:width$} ", child.getCleanValue(self.weighted), width = indexWidth + childWidth);
            }
            s += "\n";
        }

        write!(f, "{s}")
    }
}

impl<T> Graph<T>
where T: Debug {
    pub(crate) fn dfs(&self, fromIndex: usize) -> Vec<&Node<T>> {
        let mut parent: VecDeque<usize> = VecDeque::new();
        let mut visited: Vec<bool> = vec![false; self.nodes.len()];
        let mut out: Vec<&Node<T>> = vec![];

        out.push(&self.nodes[fromIndex]);
        visited[fromIndex] = true;

        let mut currentNode = fromIndex;
        let mut i = 0;

        'outer: loop {
            // loop through all children to the current node
            for c in self.nodes[currentNode].children.iter() {
                // check if not the child is already visited
                if !visited[c.idx] {
                    // register as visited
                    visited[c.idx] = true;

                    // add parent to parent stack
                    parent.push_back(currentNode);

                    // add child node as current node
                    currentNode = c.idx;

                    // add child node to output
                    out.push(&self.nodes[currentNode]);

                    // start loop again
                    continue 'outer;
                }


            }

            if parent.is_empty() {
                break;
            }

            // there are no childs, move back to parent
            currentNode = parent.pop_back().unwrap();

            if i > 100 {
                break;
            }
            i += 1;
        }

        out
    }
}










