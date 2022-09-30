use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::{fmt, fs};
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    pub(crate) fn new(idx: usize, weight: u32) -> Child {
        Child {
            idx,
            weight
        }
    }
}

#[derive(Debug)]
pub struct Node<T>
where T: Debug {
    pub(crate) val: T,
    pub(crate) children: Vec<Child> // child and weight
}

impl<T> Node<T>
where T: Clone + Display + Debug {
    pub(crate) fn new(val: T) -> Node<T> {
        Node {
            val,
            children: vec![]
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

#[derive(Debug)]
pub struct Graph<T>
where T: Debug {
    pub(crate) nodes: Vec<Node<T>>
}

impl<T> Graph<T>
where T: Copy + Debug {
    pub(crate) fn new() -> Graph<T> {
        Graph {
            nodes: vec![]
        }
    }

    pub(crate) fn new_with_size(size: usize, emptyNodeValue: T) -> Graph<T> {
        let mut v = vec![];

         v.resize_with(size, || Node {
             val: emptyNodeValue,
             children: vec![]
         });

        Graph {
            nodes: v
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

pub(crate) trait FileReader<T>
where T: Debug {
    fn read_file(filePath: &str) -> Graph<T>;
}

impl FileReader<char> for Graph<char> {
    fn read_file(filePath: &str) -> Graph<char> {
        // Read the file
        // Refactor to send a resoult and not just Graph?
        let file = File::open(filePath)
            .expect(&*format!("Did not fine the file at the given file path: ({filePath})"));

        let mut lines = BufReader::new(file)
            .lines();

        // Get the number of nodes described in the rest of the file, the first line
        let numNodes: usize = lines
            .next()
            .expect("Could not read the first line")
            .expect("Error while fetching line")
            .parse()
            .expect("could not parse string to int");

        // Create the graph and fill with the correct amount of elements
        let mut graph = Graph::<char>::new_with_size(numNodes, ' ');

        // For the rest of the lines in the file add the node to graph
        for line in lines {
            // split the line to a vector
            let splitLine: Vec<String> = line
                .expect("Did not find line")
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            // Get index of node to edit
            let currentNodeIndex = splitLine[0]
                .parse::<usize>()
                .expect("Cant parse the node index. First value of line");

            graph.nodes[currentNodeIndex].val = splitLine[1]
                .parse()
                .expect("Cant parse node value. Second value of line");

            // Add childs to correct node in the graph
            for i in 3..splitLine[2]
                .parse::<usize>()
                .expect("Cant parse number of child nodes. Third value of line") + 3 {
                graph.nodes[currentNodeIndex]
                    .add_child(Child::new(splitLine[i]
                                              .parse::<usize>()
                                              .expect(&*format!("Could not parse ({}) to usize", splitLine[i])), 0));
            }
        }

        graph
    }
}

impl<T> Display for Graph<T>
where T: Debug + Display + PartialOrd {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        let mut s = String::new();

        s += &*self.nodes.len().to_string();
        let indexWidth = s.len();

        let mut childWidth = 0;
        let mut valWitdth = 0;

        for node in self.nodes.iter() {
            childWidth = if node.children.len().to_string().len() > childWidth {
                node.children.len().to_string().len()
            } else {
                childWidth
            };

            valWitdth = if node.val.to_string().len() > valWitdth {
                node.val.to_string().len()
            } else {
                valWitdth
            }
        }

        s += "\n";

        for (i, node) in self.nodes.iter().enumerate() {
            s += &*format!("{:indexWidth$} {:childWidth$} {:valWidth$}    ",
                           i, node.val, node.children.len(),
                           indexWidth = indexWidth,
                           childWidth = childWidth,
                           valWidth = valWitdth);

            for child in node.children.iter() {
                s += &*format!("{:width$} ", child.idx, width = indexWidth);
            }
            s += "\n";
        }

        write!(f, "{s}")
    }
}















