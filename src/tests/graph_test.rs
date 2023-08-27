#[cfg(test)]
mod tests {
    use crate::array::graph::{Child, Graph, Node};

    #[test]
    fn basic_insertion() {
        /*
        5

        n v nc  c
        0 a 1   1
        1 b 2   2 3
        2 c 1   3
        3 d 1   4
        4 e 1   0
         */
        let nodes = vec![
            ('a', vec![1]),
            ('b', vec![2, 3]),
            ('c', vec![3]),
            ('d', vec![4]),
            ('e', vec![0])
        ];

        let graph: Graph<char> = Graph::from(nodes.clone());

        assert_eq!(graph.nodes.len(), 5);
        assert_eq!(graph.weighted, false);

        for (i, node) in nodes.iter().enumerate() {
            assert_eq!(node.0, graph.nodes[i].val);

            for (l, child) in node.1.clone().iter().enumerate() {
                assert_eq!(child.clone(), graph.nodes[i].children[l].idx)
            }
        }
    }

    fn gen_graph() -> Graph<u8> {
        let nodes = vec![
            (0, vec![1]),
            (1, vec![0, 2, 3]),
            (2, vec![1, 3, 4]),
            (3, vec![1, 2]),
            (4, vec![2, 6]),
            (5, vec![6]),
            (6, vec![4, 5])
        ];

        Graph::from(nodes)
    }

    #[test]
    fn bfs() {
        let graph = gen_graph();
        let path: Vec<u8> = graph.bfs(2)
            .iter()
            .map(|n| n.val)
            .collect();

        assert_eq!(path, vec![2, 1, 3, 4, 0, 6, 5])
    }

    #[test]
    fn dfs() {
        let graph = gen_graph();
        let path: Vec<u8> = graph.dfs(2)
            .iter()
            .map(|n| n.val)
            .collect();

        assert_eq!(path, vec![2, 1, 0, 3, 4, 6, 5])
    }

    #[test]
    fn ssrp() {
        let graph = Graph {
            nodes: vec![
                Node {
                    val: 0,
                    children: vec![
                        Child::new(5)
                    ],
                },
                Node {
                    val: 1,
                    children: vec![
                        Child::new(2),
                        Child::new(5)
                    ],
                },
                Node {
                    val: 2,
                    children: vec![
                        Child::new(1),
                        Child::new(3),
                        Child::new(5),
                    ],
                },
                Node {
                    val: 3,
                    children: vec![
                        Child::new(0),
                        Child::new(4)
                    ],
                },
                Node {
                    val: 4,
                    children: vec![
                        Child::new(3),
                        Child::new(5),
                    ],
                },
                Node {
                    val: 5,
                    children: vec![
                        Child::new(0),
                        Child::new(3)
                    ],
                },
            ],
            weighted: false,
        };



    }
}