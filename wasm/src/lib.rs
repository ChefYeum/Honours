mod core;

#[cfg(test)]
mod test_bar {
    use crate::core::{
        checker::check_endomorphism,
        errors::EndoCheckFail,
        graph::{DiMultGraph, Link},
    };

    #[test]
    fn test_new() {
        let g = DiMultGraph {
            node_count: 0,
            links: vec![],
        };
        assert_eq!(
            g,
            DiMultGraph {
                node_count: 0,
                links: vec![],
            }
        );
    }
}

mod test_check_identity {
    use crate::core::{
        checker::check_endomorphism,
        errors::EndoCheckFail,
        graph::{DiMultGraph, Link},
    };

    #[test]
    fn no_identity() {
        // Check for no identity
        let g = DiMultGraph {
            node_count: 2,
            links: vec![Link {
                source: 1,
                target: 2,
            }],
        };

        match check_endomorphism(g) {
            Err(EndoCheckFail::NoEndo(n)) => assert_eq!(n, 1),
            _ => assert!(false),
        }
    }

    #[test]
    fn multiple_identities() {
        use crate::core::{
            checker::check_endomorphism,
            errors::EndoCheckFail,
            graph::{DiMultGraph, Link},
        };
        // Check for multiple identities
        let g = DiMultGraph {
            node_count: 2,
            links: vec![
                Link {
                    source: 1,
                    target: 1,
                },
                Link {
                    source: 2,
                    target: 2,
                },
                Link {
                    source: 2,
                    target: 2,
                },
            ],
        };

        match check_endomorphism(g) {
            Err(EndoCheckFail::ManyEndo(n)) => assert_eq!(n, 2),
            _ => assert!(false),
        }
    }
}
