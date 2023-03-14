mod checker;

#[cfg(test)]
mod test_bar {

    // Example test

    use crate::checker::{
        checker::{check_ids, check_size},
        graph::CompositionTable,
    };

    #[test]
    fn test_dummy_check_ids() {
        // let comp_table: CompositionTable = CompositionTable::new(vec![vec![0, 0], vec![1, 1]]);
        let comp_table = CompositionTable::new(vec![vec![Some(0)]]);
        let result = check_size(&comp_table).and_then(check_ids);

        assert_eq!(result.is_ok(), true);
    }
}
