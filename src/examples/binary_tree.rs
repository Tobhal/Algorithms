use crate::array::binary_tree::BinaryTree;
use crate::utils::util::{BFS, Counting, Insert, OrderedTraversal};

fn main() {
    let mut root: BinaryTree<u8> = BinaryTree::new();

    root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

    /*
    Get info of the binary tree
     */
    let numNodes = root.num_nodes();
    let numLevels = root.num_levels();
    let numLeaves = root.num_leaves();
    let numTwoChildren = root.num_two_children();

    /*
    Traversing
     */
    let preOrder = root.pre_order_from(0);
    let inOrder = root.in_order_from(0);
    let postOrder = root.post_order_from(0);
    let bfs = root.bfs_from(0);

}