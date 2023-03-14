mod checker;

#[cfg(test)]
mod test_bar {
    use std::{
        fs::File,
        io::{self, Read},
    };

    use crate::checker::{
        checker::{check_ids, check_size},
        graph::CompositionTable,
    };

    fn read_comp_table() -> Result<CompositionTable, io::Error> {
        let mut file = File::open("examples.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let table: CompositionTable = serde_json::from_str(&contents)?;
        Ok(table)
    }

    #[test]
    fn test_read_examples() {
        let comp_table = read_comp_table().unwrap();
        let result = check_size(&comp_table)
            .and_then(check_ids)
            // .and_then(check_source_target(_));
            ;

        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_dummy_check_ids() {
        // let comp_table: CompositionTable = CompositionTable::new(vec![vec![0, 0], vec![1, 1]]);
        let comp_table = CompositionTable::new(vec![vec![Some(0)]]);
        let result = check_size(&comp_table)
            .and_then(check_ids)
            // .and_then(check_source_target(_));
            ;

        assert_eq!(result.is_ok(), true);
    }
}
