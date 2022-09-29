use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Node<T>
where T: Clone + Display + Debug {
    pub(crate) val: T,
    pub(crate) children: Vec<(usize, u32)> // child and weight
}

impl<T> Node<T>
where T: Clone + Display + Debug {
    pub(crate) fn new(value: T) -> Node<T> {
        Node {
            val: value,
            children: vec![]
        }
    }

    pub(crate) fn add_child(&mut self, node: (usize, u32)) {
        self.children.push(node);
    }

    pub(crate) fn add_children(&mut self, nodes: Vec<(usize, u32)>) {
        for node in nodes {
            self.add_child(node);
        }
    }
}

#[derive(Debug)]
pub struct Graph<T>
where T: PartialOrd + Copy + Display + Debug {
    pub(crate) nodes: Vec<Node<T>>
}

impl<T> Graph<T>
where T: PartialOrd + Copy + Display + Debug {
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

impl Graph<char> {
    pub(crate) fn read_file(filePath: &str) -> Graph<char> {
        // Read the file
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
                graph.nodes[currentNodeIndex].add_child((i, 0));
            }
        }

        graph
    }
}


















