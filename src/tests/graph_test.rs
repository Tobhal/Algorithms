#[cfg(test)]
mod tests {
    use crate::{Graph, Node};

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
        let values = vec!['a','b','c','d','e'];
        let neighbours = vec![
            vec![1],
            vec![2,3],
            vec![3],
            vec![4],
            vec![0]
        ];

        let mut graph: Graph<char> = Graph::new_with_size(5, ' ');

        for i in 0..5 {
            graph.add_node(
                Node::new(values[i])
            )
        }

    }
}