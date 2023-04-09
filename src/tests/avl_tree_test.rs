#[cfg(test)]
mod tests {
    use crate::array::avl_tree::{Direction, Rotate};
    use crate::AVLTree;
    use crate::utils::util::{Contains, Insert, Util};

    /*
    Insert
     */
    #[test]
    fn insert() {
        let mut root: AVLTree<char> = AVLTree::new_from_vec(vec!['a', 'b', 'c', 'd']);

        assert_eq!(root.root, vec![
            Some('b'),
            Some('a'), Some('c'),
            None, None, None, Some('d')
        ]);
        assert_eq!(root.balanceFactor, vec![
            3,
            1, 2,
            0, 0, 0, 1
        ]);

        root.insert('e');

        assert_eq!(root.root, vec![
            Some('b'),
            Some('a'), Some('d'),
            None, None, Some('c'), Some('e')
        ]);
        assert_eq!(root.balanceFactor, vec![
            3,
            1, 2,
            0, 0, 1, 1
        ]);
    }

    /*
    Iterative insert tests
     */
    #[test]
    fn insert_iterative_easy() {
        let mut root: AVLTree<char> = AVLTree::new();

        root.insert('a');
        assert_eq!(root.root, vec![Some('a')]);

        root.insert('b');
        assert_eq!(root.root, vec![Some('a'), None, Some('b')]);

        root.insert('c');
        assert_eq!(root.root, vec![Some('b'), Some('a'), Some('c')]);
    }

    #[test]
    fn insert_iterative_easy_inverse() {
        let mut root: AVLTree<char> = AVLTree::new();

        root.insert('c');
        assert_eq!(root.root, vec![Some('c')]);

        root.insert('b');
        assert_eq!(root.root, vec![Some('c'), Some('b'), None]);

        root.insert('a');
        assert_eq!(root.root, vec![Some('b'), Some('a'), Some('c')]);
    }

    #[test]
    fn insert_iterative_medium() {
        let mut root: AVLTree<char> = AVLTree::new();

        root.insert('a');
        assert_eq!(root.root, vec![Some('a')]);

        root.insert('b');
        assert_eq!(root.root, vec![Some('a'), None, Some('b')]);

        root.insert('c');
        assert_eq!(root.root, vec![Some('b'), Some('a'), Some('c')]);

        root.insert('d');
        assert_eq!(root.root, vec![Some('b'), Some('a'), Some('c'), None, None, None, Some('d')]);

        root.insert('e');
        assert_eq!(root.root, vec![Some('b'), Some('a'), Some('d'), None, None, Some('c'), Some('e')]);

        root.insert('f');
        assert_eq!(root.root, vec![Some('d'), Some('b'), Some('e'), Some('a'), Some('c'), None, Some('f')]);

        root.insert('g');
        assert_eq!(root.root, vec![Some('d'), Some('b'), Some('f'), Some('a'), Some('c'), Some('e'), Some('g')]);
    }

    #[test]
    fn insert_iterative_medium_inverse() {
        let mut root: AVLTree<char> = AVLTree::new();

        root.insert('g');
        assert_eq!(root.root, vec![Some('g')]);

        root.insert('f');
        assert_eq!(root.root, vec![Some('g'), Some('f'), None]);

        root.insert('e');
        assert_eq!(root.root, vec![Some('f'), Some('e'), Some('g')]);

        root.insert('d');
        assert_eq!(root.root, vec![Some('f'), Some('e'), Some('g'), Some('d'), None, None, None]);

        root.insert('c');
        assert_eq!(root.root, vec![Some('f'), Some('d'), Some('g'), Some('c'), Some('e'), None, None]);

        root.insert('b');
        assert_eq!(root.root, vec![Some('d'), Some('c'), Some('f'), Some('b'), None, Some('e'), Some('g')]);

        root.insert('a');
        assert_eq!(root.root, vec![Some('d'), Some('b'), Some('f'), Some('a'), Some('c'), Some('e'), Some('g')]);
    }

    #[test]
    fn insert_iterative_hard() {
        let mut root: AVLTree<char> = AVLTree::new();

        root.insert('a');
        assert_eq!(root.root, vec![Some('a')]);

        root.insert('b');
        assert_eq!(root.root, vec![Some('a'), None, Some('b')]);

        root.insert('c');
        assert_eq!(root.root, vec![Some('b'), Some('a'), Some('c')]);

        root.insert('d');
        assert_eq!(root.root, vec![Some('b'), Some('a'), Some('c'), None, None, None, Some('d')]);

        root.insert('e');
        assert_eq!(root.root, vec![Some('b'), Some('a'), Some('d'), None, None, Some('c'), Some('e')]);

        root.insert('f');
        assert_eq!(root.root, vec![Some('d'), Some('b'), Some('e'), Some('a'), Some('c'), None, Some('f')]);

        root.insert('g');
        assert_eq!(root.root, vec![Some('d'), Some('b'), Some('f'), Some('a'), Some('c'), Some('e'), Some('g')]);

        root.insert('h');
        assert_eq!(root.root, vec![
            Some('d'),
            Some('b'), Some('f'),
            Some('a'), Some('c'), Some('e'), Some('g'),
            None, None, None, None, None, None, None, Some('h'),
        ]);

        root.insert('i');
        assert_eq!(root.root, vec![
            Some('d'),
            Some('b'), Some('f'),
            Some('a'), Some('c'), Some('e'), Some('h'),
            None, None, None, None, None, None, Some('g'), Some('i'),
        ]);

        root.insert('j');
        assert_eq!(root.root, vec![
            Some('d'),
            Some('b'), Some('h'),
            Some('a'), Some('c'), Some('f'), Some('i'),
            None, None, None, None, Some('e'), Some('g'), None, Some('j'),
        ]);

        root.insert('k');
        assert_eq!(root.root, vec![
            Some('d'),
            Some('b'), Some('h'),
            Some('a'), Some('c'), Some('f'), Some('j'),
            None, None, None, None, Some('e'), Some('g'), Some('i'), Some('k'),
        ]);

        // Note: Big change to tree, so there might be errors here.
        // Note: 'i' from second to last position is not moved. So there needs to be added a loop somewhere.
        root.insert('l');
        assert_eq!(root.root, vec![
            Some('h'),
            Some('d'), Some('j'),
            Some('b'), Some('f'), Some('i'), Some('k'),
            Some('a'), Some('c'), Some('e'), Some('g'), None, None, None, Some('l'),
        ]);

        root.insert('m');
        assert_eq!(root.root, vec![
            Some('h'),
            Some('d'), Some('j'),
            Some('b'), Some('f'), Some('i'), Some('l'),
            Some('a'), Some('c'), Some('e'), Some('g'), None, None, Some('k'), Some('m'),
        ]);

        root.insert('n');
        assert_eq!(root.root, vec![
            Some('h'),
            Some('d'), Some('l'),
            Some('b'), Some('f'), Some('j'), Some('m'),
            Some('a'), Some('c'), Some('e'), Some('g'), Some('i'), Some('k'), None, Some('n'),
        ]);

        root.insert('o');
        assert_eq!(root.root, vec![
            Some('h'),
            Some('d'), Some('l'),
            Some('b'), Some('f'), Some('j'), Some('n'),
            Some('a'), Some('c'), Some('e'), Some('g'), Some('i'), Some('k'), Some('m'), Some('o'),
        ]);
    }

    /*
    Insert complex
     */
    #[test]
    fn insert_complex() {
        // TODO: Fix
        /*
                       d
               b               h
           a       c       f       j
         _   _   _   _   e   g   i   k
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        -------------------------------
        d b h a c f j _ _ _ _ e g i k
        -------------------------------
        insert l
        -------------------------------
                       h
               d               j
           b       f       i       k
         a   c   e   g   _   _   _   l
        _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
        -------------------------------
        h d j b f i k a c e g _ _ _ l
         */
        let mut root = AVLTree {
            root: vec![
                Some('d'),
                Some('b'), Some('h'),
                Some('a'), Some('c'), Some('f'), Some('j'),
                None, None, None, None, Some('e'), Some('g'), Some('i'), Some('k'),
            ],
            balanceFactor: vec![
                4,
                2, 3,
                1, 1, 2, 2,
                0, 0, 0, 0, 1, 1, 1, 1
            ],
            nodes: 11,
            height: 4
        };

        root.println();

        root.insert('l');

        root.println();

        assert_eq!(root.root, vec![
            Some('h'),
            Some('d'), Some('j'),
            Some('b'), Some('f'), Some('i'), Some('k'),
            Some('a'), Some('c'), Some('e'), Some('g'), None, None, None, Some('l')
        ]);
        assert_eq!(root.balanceFactor, vec![
            4,
            3, 3,
            2, 2, 1, 2,
            1, 1, 1, 1, 0, 0, 0, 1
        ]);
    }

    /*
    Move nodes
     */
    #[test]
    fn move_nodes_left() {
        let mut root = AVLTree {
            root: vec![Some('b'), Some('a'), Some('c'), None, None, None, None],
            balanceFactor: vec![2, 1, 1, 0, 0, 0, 0],
            nodes: 3,
            height: 2
        };

        root.move_nodes(0, 1);

        assert_eq!(root.root, vec![
            None,
            Some('b'), None,
            Some('a'), Some('c'), None, None
        ]);
    }

    #[test]
    fn move_nodes_right() {
        let mut root = AVLTree {
            root: vec![Some('b'), Some('a'), Some('c'), None, None, None, None],
            balanceFactor: vec![2, 1, 1, 0, 0, 0, 0],
            nodes: 3,
            height: 2
        };

        root.move_nodes(0, 2);

        assert_eq!(root.root, vec![
            None,
            None, Some('b'),
            None, None, Some('a'), Some('c')
        ]);
    }

    #[test]
    fn move_nodes_left_complex() {
        let mut root = AVLTree {
            root: vec![Some('d'), Some('b'), Some('f'), Some('a'), Some('c'), Some('e'), Some('g')],
            balanceFactor: vec![3, 2, 2, 1, 1, 1, 1],
            nodes: 7,
            height: 2
        };

        root.move_nodes(0, 1);

        assert_eq!(root.root, vec![
            None,
            Some('d'), None,
            Some('b'), Some('f'), None, None,
            Some('a'), Some('c'), Some('e'), Some('g'), None, None, None, None
        ]);
    }

    #[test]
    fn move_nodes_right_complex() {
        let mut root = AVLTree {
            root: vec![Some('d'), Some('b'), Some('f'), Some('a'), Some('c'), Some('e'), Some('g')],
            balanceFactor: vec![3, 2, 2, 1, 1, 1, 1],
            nodes: 7,
            height: 2
        };

        root.move_nodes(0, 2);

        assert_eq!(root.root, vec![
            None,
            None, Some('d'),
            None, None, Some('b'), Some('f'),
            None, None, None, None, Some('a'), Some('c'), Some('e'), Some('g')
        ]);
    }

    /*
    Rotate
     */
    #[test]
    fn rotate_left_single_val() {
        let mut root: AVLTree<char> = AVLTree {
            root: vec![Some('a')],
            balanceFactor: vec![3],
            nodes: 1,
            height: 0,
        };

        root.increase_levels(1);

        root.rotate(0, Direction::LEFT);

        assert_eq!(root.root, vec![None, Some('a'), None]);
    }

    #[test]
    fn rotate_right_single_val() {
        let mut root: AVLTree<char> = AVLTree {
            root: vec![Some('a')],
            balanceFactor: vec![3],
            nodes: 1,
            height: 0,
        };

        root.increase_levels(1);

        root.rotate(0, Direction::RIGHT);

        assert_eq!(root.root, vec![None, None, Some('a')]);
    }

    #[test]
    fn rotate_left() {
        let mut root = AVLTree {
            root: vec![
                Some('a'),
                None, Some('b'),
                None, None, None, Some('c')
            ],
            balanceFactor: vec![
                2,
                0, 1,
                0, 0, 0, 0
            ],
            nodes: 3,
            height: 3
        };

        root.rotate(root.find('a').unwrap(), Direction::LEFT);

        assert_eq!(root.root, vec![
            Some('b'),
            Some('a'), Some('c'),
            None, None, None, None
        ])
    }

    #[test]
    fn rotate_right() {
        let mut root = AVLTree {
            root: vec![
                Some('c'),
                Some('b'), None,
                Some('a'), None, None, None
            ],
            balanceFactor: vec![
                2,
                1, 0,
                0, 0, 0, 0
            ],
            nodes: 3,
            height: 3
        };

        root.rotate(root.find('c').unwrap(), Direction::RIGHT);

        assert_eq!(root.root, vec![
            Some('b'),
            Some('a'), Some('c'),
            None, None, None, None
        ])
    }

    #[test]
    fn rotate_left_right() {
        let mut root = AVLTree {
            root: vec![
                Some('c'),
                Some('a'), None,
                None, Some('b'), None, None
            ],
            balanceFactor: vec![
                3,
                2, 0,
                0, 1, 0, 0
            ],
            nodes: 3,
            height: 3
        };

        root.rotate(root.find('a').unwrap(), Direction::LEFT);
        root.rotate(root.find('c').unwrap(), Direction::RIGHT);

        assert_eq!(root.root, vec![
            Some('b'),
            Some('a'), Some('c'),
            None, None, None, None
        ])
    }

    #[test]
    fn rotate_right_left() {
        let mut root = AVLTree {
            root: vec![
                Some('a'),
                None, Some('c'),
                None, None, Some('b'), None
            ],
            balanceFactor: vec![
                3,
                0, 2,
                0, 0, 1, 0
            ],
            nodes: 3,
            height: 3
        };

        root.rotate(root.find('c').unwrap(), Direction::RIGHT);
        root.rotate(root.find('a').unwrap(), Direction::LEFT);

        assert_eq!(root.root, vec![
            Some('b'),
            Some('a'), Some('c'),
            None, None, None, None
        ])
    }
}

