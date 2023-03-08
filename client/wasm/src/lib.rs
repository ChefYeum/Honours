mod core;

#[cfg(test)]
mod test_bar {
    use crate::core::graph::DiMultGraph;


    #[test]
    fn test_new() {
        let g = DiMultGraph {
            node_count: 0,
            links: vec![],
        };
        assert_eq!(g, DiMultGraph {
            node_count: 0,
            links: vec![],
        });
    }

    // #[test]
    // fn test_check_identity() {
    // ...
    // }
}
