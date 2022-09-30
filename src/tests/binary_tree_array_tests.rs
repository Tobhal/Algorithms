#[cfg(test)]
mod tests {
    use crate::array::binary_tree::BinaryTree;
    use crate::{BFS, OrderedTraversal};
    use crate::utils::util::{Contains, Counting, Insert, Remove};

    #[test]
    fn num_nodes() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.num_nodes(), 9);
    }

    #[test]
    fn num_leaves() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.num_leaves(), 4);
    }


    #[test]
    fn num_two_children() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.num_two_children(), 3);
    }

    #[test]
    fn num_levels() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.num_levels(), 4);
    }

    #[test]
    fn insert() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.root, vec![Some(8), Some(3), Some(10), Some(1), Some(6), None, Some(14), None, None, Some(4), Some(7), None, None, Some(13), None])
    }

    #[test]
    fn remove() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        let mut correct: Vec<u8> = Vec::new();

        // Test removing element not in root
        root.remove(1);
        assert_eq!(root.num_nodes(), 0);

        // Basic delete functionality
        for i in 10..=20 {
            root.insert(i);
            correct.push(i);
        }

        for i in 0..=9 {
            root.insert(i);
        }
        for i in (0..=9).rev() {
            correct.insert(0, i);
        }

        for i in 0..10 {
            root.remove(i);
            correct.remove(0);
            assert_eq!(root.in_order(), correct);
        }

        for i in (10..=20).rev() {
            root.remove(i);
            if let Some(pos) = correct.iter().position(|x| *x == i) {
                correct.remove(pos);
            }
            assert_eq!(root.in_order(), correct);
        }

        // Edge cases
        // Deleting leaf
        root.insert_vec(vec![10, 8, 9, 6, 7]);
        root.remove(6);
        assert_eq!(root.in_order(), vec![7, 8, 9, 10]);

        // Deleting internal node
        root.insert(6);
        root.remove(7);
        assert_eq!(root.in_order(), vec![6, 8, 9, 10]);

        // Deleting an internal node and pushing new node up
        for i in root.pre_order() {root.remove(i)};

        root.insert_vec(vec![10, 8, 9, 7]);
        root.remove(8);
        assert_eq!(root.in_order(), vec![7, 9, 10]);

        // Deleting an internal node and progressing down left subtree to rightmost
        for i in root.pre_order() {root.remove(i)};

        root.insert_vec(vec![10, 8, 9, 6, 7]);
        root.remove(8);
        assert_eq!(root.in_order(), vec![6, 7, 9, 10]);
    }

    #[test]
    fn contains() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.contains(6), true);
        assert_eq!(root.contains(2), false);
        assert_eq!(root.contains(8), true);
        assert_eq!(root.contains(13), true);
        assert_eq!(root.contains(15), false);
    }

    #[test]
    fn pre_order() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.pre_order(), vec![8, 3, 1, 6, 4, 7, 10, 14, 13]);


    }

    #[test]
    fn pre_order_from() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.pre_order_from(0), vec![8, 3, 1, 6, 4, 7, 10, 14, 13]);
        assert_eq!(root.pre_order_from(1), vec![3, 1, 6, 4, 7]);
        assert_eq!(root.pre_order_from(2), vec![10, 14, 13]);
        assert_eq!(root.pre_order_from(3), vec![1]);
        assert_eq!(root.pre_order_from(4), vec![6, 4, 7]);
        assert_eq!(root.pre_order_from(5), vec![]);
        assert_eq!(root.pre_order_from(6), vec![14, 13]);
        assert_eq!(root.pre_order_from(7), vec![]);
        assert_eq!(root.pre_order_from(8), vec![]);
        assert_eq!(root.pre_order_from(9), vec![4]);
        assert_eq!(root.pre_order_from(10), vec![7]);
        assert_eq!(root.pre_order_from(11), vec![]);
        assert_eq!(root.pre_order_from(12), vec![]);
        assert_eq!(root.pre_order_from(13), vec![13]);
        assert_eq!(root.pre_order_from(14), vec![]);
    }

    #[test]
    fn in_order() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.in_order(), vec![1, 3, 4, 6, 7, 8, 10, 13, 14]);
    }

    #[test]
    fn in_order_from() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.in_order_from(0), vec![1, 3, 4, 6, 7, 8, 10, 13, 14]);
        assert_eq!(root.in_order_from(1), vec![1, 3, 4, 6, 7]);
        assert_eq!(root.in_order_from(2), vec![10, 13, 14]);
        assert_eq!(root.in_order_from(3), vec![1]);
        assert_eq!(root.in_order_from(4), vec![4, 6, 7]);
        assert_eq!(root.in_order_from(5), vec![]);
        assert_eq!(root.in_order_from(6), vec![13, 14]);
        assert_eq!(root.in_order_from(7), vec![]);
        assert_eq!(root.in_order_from(8), vec![]);
        assert_eq!(root.in_order_from(9), vec![4]);
        assert_eq!(root.in_order_from(10), vec![7]);
        assert_eq!(root.in_order_from(11), vec![]);
        assert_eq!(root.in_order_from(12), vec![]);
        assert_eq!(root.in_order_from(13), vec![13]);
        assert_eq!(root.in_order_from(14), vec![]);
    }

    #[test]
    fn post_order() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.post_order(), vec![1, 4, 7, 6, 3, 13, 14, 10, 8]);
    }

    #[test]
    fn post_order_from() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.post_order_from(0), vec![1, 4, 7, 6, 3, 13, 14, 10, 8]);
        assert_eq!(root.post_order_from(1), vec![1, 4, 7, 6, 3]);
        assert_eq!(root.post_order_from(2), vec![13, 14, 10]);
        assert_eq!(root.post_order_from(3), vec![1]);
        assert_eq!(root.post_order_from(4), vec![4, 7, 6]);
        assert_eq!(root.post_order_from(5), vec![]);
        assert_eq!(root.post_order_from(6), vec![13, 14]);
        assert_eq!(root.post_order_from(7), vec![]);
        assert_eq!(root.post_order_from(8), vec![]);
        assert_eq!(root.post_order_from(9), vec![4]);
        assert_eq!(root.post_order_from(10), vec![7]);
        assert_eq!(root.post_order_from(11), vec![]);
        assert_eq!(root.post_order_from(12), vec![]);
        assert_eq!(root.post_order_from(13), vec![13]);
        assert_eq!(root.post_order_from(14), vec![]);
    }

    #[test]
    fn bfs() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.bfs(), vec![8, 3, 10, 1, 6, 14, 4, 7, 13]);
    }

    #[test]
    fn bfs_from() {
        let mut root: BinaryTree<u8> = BinaryTree::new();
        root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

        assert_eq!(root.bfs_from(0), vec![8, 3, 10, 1, 6, 14, 4, 7, 13]);
        assert_eq!(root.bfs_from(1), vec![3, 1, 6, 4, 7]);
        assert_eq!(root.bfs_from(2), vec![10, 14, 13]);
        assert_eq!(root.bfs_from(3), vec![1]);
        assert_eq!(root.bfs_from(4), vec![6, 4, 7]);
        assert_eq!(root.bfs_from(5), vec![]);
        assert_eq!(root.bfs_from(6), vec![14, 13]);
        assert_eq!(root.bfs_from(7), vec![]);
        assert_eq!(root.bfs_from(8), vec![]);
        assert_eq!(root.bfs_from(9), vec![4]);
        assert_eq!(root.bfs_from(10), vec![7]);
        assert_eq!(root.bfs_from(11), vec![]);
        assert_eq!(root.bfs_from(12), vec![]);
        assert_eq!(root.bfs_from(13), vec![13]);
        assert_eq!(root.bfs_from(14), vec![]);
    }
}