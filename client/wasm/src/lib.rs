mod foo;
mod bar;


#[cfg(test)]
mod test_foo{
    // use crate::check_identity;

    use crate::foo::Foo;

    #[test]
    fn foo_works() {
        assert_eq!(Foo::new(2, vec![1, 2, 3]), Foo::new(2, vec![1, 2, 3]));
    }
}

#[cfg(test)]
mod test_bar {
    // import all the things from the parent module
    use crate::bar::*;

    #[test]
    fn test_new() {
        let g = DiMultGraph::new();
        assert_eq!(g, DiMultGraph::new());
    }

    // #[test]
    // fn test_add_node() {
    //     let mut g = DiMultGraph::new();
    //     g.add_node();
    //     assert_eq!(g.get_node_count(), 1);
    // }

    // #[test]
    // fn test_check_identity() {
    //     let g = DiMultGraph 
    //         nodes: vec![Node { id: 1 }, Node { id: 2 }],
    //         links: vec![Link { source: 1, target: 1 }, Link { source: 2, target: 2 }],
    //     };
    //     assert_eq!(check_identity(g), true);
    // }
}