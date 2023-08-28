use crate::array::binary_tree::BinaryTree;
use crate::utils::util::{
    BFS, Counting, Insert, OrderedTraversal
};

fn main() {
    let mut root: BinaryTree<u8> = BinaryTree::new();

    root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

    /*
    Get info of the binary tree
     */
    let num_nodes = root.num_nodes();
    let num_levels = root.num_levels();
    let num_leaves = root.num_leaves();
    let num_two_children = root.num_two_children();

    /*
    Traversing
     */
    let pre_order = root.pre_order_from(0);
    let in_order = root.in_order_from(0);
    let post_order = root.post_order_from(0);
    let bfs = root.bfs_from(0);

}