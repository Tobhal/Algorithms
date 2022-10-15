#[cfg(test)]
mod tests {
    use crate::array::avl_tree::Rotate;
    use crate::AVLTree;

    #[test]
    fn rotate_left() {
        let mut root = AVLTree {
            root: vec![Some('a'),
                       None, Some('b'),
                       None, None, None, Some('c')
            ],
            balanceFactor: vec![2,
                                0, 1,
                                0, 0, 0, 0
            ],
            nodes: 3,
            height: 3
        };

        root.rotate_left(2);

        assert_eq!(root.root, vec![Some('b'),
                                   Some('a'), Some('c'),
                                   None, None, None, None
        ])

    }

    #[test]
    fn rotate_right() {
        let mut root = AVLTree {
            root: vec![Some('c'),
                       Some('b'), None,
                       Some('a'), None, None, None
            ],
            balanceFactor: vec![2,
                                1, 0,
                                0, 0, 0, 0
            ],
            nodes: 3,
            height: 3
        };

        root.rotate_right(1);

        assert_eq!(root.root, vec![Some('b'),
                                   Some('a'), Some('c'),
                                   None, None, None, None
        ])
    }

    #[test]
    fn rotate_left_right() {
        let mut root = AVLTree {
            root: vec![Some('c'),
                       Some('a'), None,
                       None, Some('b'), None, None
            ],
            balanceFactor: vec![2,
                                1, 0,
                                0, 0, 0, 0
            ],
            nodes: 3,
            height: 3
        };

        root.rotate_left_right(1);

        assert_eq!(root.root, vec![Some('b'),
                                   Some('a'), Some('c'),
                                   None, None, None, None
        ])
    }

    #[test]
    fn rotate_right_left() {
        let mut root = AVLTree {
            root: vec![Some('c'),
                       None, Some('a'),
                       None, None, Some('b'), None
            ],
            balanceFactor: vec![2,
                                0, 1,
                                0, 0, 0, 0
            ],
            nodes: 3,
            height: 3
        };

        root.rotate_right_left(2);

        assert_eq!(root.root, vec![Some('b'),
                                   Some('a'), Some('c'),
                                   None, None, None, None
        ])
    }
}

