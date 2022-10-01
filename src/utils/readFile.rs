use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::{Child, Graph};

pub(crate) trait FileReader<T>
    where T: Debug {
    fn read_file(filePath: &str, weighted: bool) -> Graph<T>;
}

impl FileReader<char> for Graph<char> {
    fn read_file(filePath: &str, weighted: bool) -> Graph<char> {
        // Read the file
        // Refactor to send a result and not just Graph?
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

        graph.weighted = weighted;

        // For the rest of the lines in the file add the node to graph
        let multiply = if weighted {
            graph.weighted = true;
            2
        } else {
            1
        };

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

            // Set the correct value of the current node
            graph.nodes[currentNodeIndex].val = splitLine[1]
                .parse()
                .expect("Cant parse node value. Second value of line");

            let childs = splitLine[2]
                .parse::<usize>()
                .expect("Cant parse number of child nodes. Third value of line");

            // Add childs to correct node in the graph
            for i in 0..childs {
                let weight = if weighted {
                    splitLine[3+(i * multiply)+1].parse::<u32>().unwrap()
                } else {
                    0
                };

                graph.nodes[currentNodeIndex]
                    .add_child(Child::new_with_weight(splitLine[3+(i * multiply)]
                                              .parse::<usize>()
                                              .expect(&*format!("Could not parse ({}) to usize", splitLine[3+(i * multiply)])), weight));
            }
        }


        graph
    }
}

impl FileReader<String> for Graph<String> {
    fn read_file(filePath: &str, weighted: bool) -> Graph<String> {
        // Read the file
        // Refactor to send a result and not just Graph?
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
        let mut graph = Graph::<String>::new_with_size(numNodes, String::new());

        let multiply = if weighted {
            graph.weighted = true;
            2
        } else {
            1
        };

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

            // Set the correct value of the current node
            graph.nodes[currentNodeIndex].val = splitLine[1]
                .parse()
                .expect("Cant parse node value. Second value of line");

            let childs = splitLine[2]
                .parse::<usize>()
                .expect("Cant parse number of child nodes. Third value of line");

            // Add childs to correct node in the graph
            for i in 0..childs {
                let weight = if weighted {
                    splitLine[3+(i * multiply)+1].parse::<u32>().unwrap()
                } else {
                    0
                };

                graph.nodes[currentNodeIndex]
                    .add_child(Child::new_with_weight(splitLine[3+(i * multiply)]
                                              .parse::<usize>()
                                              .expect(&*format!("Could not parse ({}) to usize", splitLine[3+(i * multiply)])), weight));
            }
        }

        graph
    }
}