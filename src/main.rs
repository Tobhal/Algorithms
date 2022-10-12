#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use crate::utils::util::{OrderedTraversal, BFS};
use crate::array::graph::{Graph, Node, Child};
use crate::utils::readFile::FileReader;

mod utils;
mod tests;
mod array;

fn main() {
    let mut node0 = Node::new("kake");
    let mut node1 = Node::new("eple");
    let mut node2 = Node::new("potet");

    let node3 = Node::new_with_children("poteter", vec![
        Child::new_with_weight(0, 1),
        Child::new_with_weight(1, 2),
        Child::new_with_weight(2, 3)
    ]);

    node0.add_children(vec![Child::new_with_weight(0, 1), Child::new_with_weight(1, 2)]);
    node1.add_children(vec![Child::new_with_weight(0, 3), Child::new_with_weight(2, 4)]);
    node2.add_children(vec![Child::new_with_weight(1, 4), Child::new_with_weight(0, 2)]);

    let mut graph1 = Graph::new();
    graph1.add_nodes(vec![node0, node1, node2, node3]);

    graph1.weighted = true;

    let graph2: Graph<String> = Graph::read_file("files/TestGraphDataV1.txt", true);

    // println!("{graph1}");
    // println!("{graph2}");

    /*
    for i in 0..graph2.nodes.len() {
        print!("{i}: ");
        graph2.dfs(i).iter().for_each(|e| print!("{:?} ", e.val));
        println!();
    }
     */

    graph2.bfs(0).iter().for_each(|e| print!("{:?} ", e.val));
    // println!("{:#?}", graph2.bfs(0));
}
