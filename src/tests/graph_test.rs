#[cfg(test)]
mod tests {
    use std::fs::File;
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
    fn neighborhood() {
        let graph = gen_graph();
        let neighborhood = graph.get_neighborhood_matrix();
        let correct_neighborhood = vec![
            vec![true, true, false, false, false, false, false],
            vec![true, true, true, true, false, false, false],
            vec![false, true, true, true, true, false, false],
            vec![false, true, true, true, false, false, false],
            vec![false, false, true, false, true, false, true],
            vec![false, false, false, false, false, true, true],
            vec![false, false, false, false, true, true, true]
        ];

        assert_eq!(neighborhood, correct_neighborhood)
    }

    #[test]
    fn warshall() {
        let graph = Graph::from(vec![
            (1, vec![5]),
            (2, vec![2, 5]),
            (3, vec![1, 3, 5]),
            (4, vec![0, 4]),
            (5, vec![5, 3]),
            (6, vec![0, 3])
        ]);
        let aprp = graph.warshall();
        let solution_aprp = vec![
            vec![true, false, false, true, true, true],
            vec![true, true, true, true, true, true],
            vec![true, true, true, true, true, true],
            vec![true, false, false, true, true, true],
            vec![true, false, false, true, true, true],
            vec![true, false, false, true, true, true]
        ];

        assert_eq!(aprp, solution_aprp)
    }

    #[test]
    fn unweighted_from_file() {
        let file = File::open("/Users/tobiashallingstad/Prog/Languages/Rust/Algoritmes/src/files/graf_13.txt").unwrap();
        let graph: Graph<char> = Graph::from(file);

        let correct_graph = vec![
            ('A', vec![1, 5, 6]),
            ('B', vec![]),
            ('C', vec![0]),
            ('D', vec![5]),
            ('E', vec![3]),
            ('F', vec![4]),
            ('G', vec![2, 4, 9]),
            ('H', vec![6, 8]),
            ('I', vec![7]),
            ('J', vec![10, 11, 12]),
            ('K', vec![]),
            ('L', vec![6, 12]),
            ('M', vec![11]),
        ];

        graph.nodes.iter().enumerate().for_each(|(idx, node)| {
            let children: Vec<usize> = node.children.iter().map(|child| child.idx).collect();
            let correct_values = &correct_graph[idx];

            assert_eq!(node.val, correct_values.0);
            assert_eq!(children, correct_values.1);
        });
    }

    #[test]
    fn weighed_from_file() {
        let file = File::open("/Users/tobiashallingstad/Prog/Languages/Rust/Algoritmes/src/files/vgraf_13.txt").unwrap();
        let graph: Graph<char> = Graph::from(file);

        let correct_graph = vec![
            ('A', vec![(1, 1), (5, 2), (6, 4)]),
            ('B', vec![]),
            ('C', vec![(0, 1)]),
            ('D', vec![(5, 1)]),
            ('E', vec![(3, 2)]),
            ('F', vec![(4, 2)]),
            ('G', vec![(2, 1), (4, 1), (9, 1)]),
            ('H', vec![(6, 3), (8, 2)]),
            ('I', vec![(7, 2)]),
            ('J', vec![(10, 1), (11, 3), (12, 2)]),
            ('K', vec![]),
            ('L', vec![(6, 5), (12, 1)]),
            ('M', vec![(11, 1)]),
        ];

        graph.nodes.iter().enumerate().for_each(|(idx, node)| {
            let children: Vec<(usize, u32)> = node.children.iter()
                .map(|child| (child.idx, child.weight))
                .collect();

            let correct_values = &correct_graph[idx];

            assert_eq!(node.val, correct_values.0);
            assert_eq!(children, correct_values.1);
        });
    }
}