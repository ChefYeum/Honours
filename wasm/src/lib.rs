mod checker;

#[cfg(test)]
mod test_bar {
    use std::{
        fs::File,
        io::{self, Read},
    };

    use crate::checker::{
        checker::{check_ids, check_morph_count, check_source_target},
        graph::{CompositionTable, MorphID},
    };

    fn read_comp_table(path: &str) -> Result<CompositionTable, io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let table: CompositionTable = serde_json::from_str(&contents)?;
        Ok(table)
    }

    fn test_comp_table(filename: &str, expected_morph_count: usize, expected_ids: Vec<MorphID>) {
        let comp_table = read_comp_table(filename).unwrap();
        check_morph_count(&comp_table)
            .and_then(|(table, morph_count)| {
                assert_eq!(morph_count, expected_morph_count);
                Ok((table, morph_count))
            })
            .and_then(check_ids)
            .and_then(|(table, size, ids)| {
                assert_eq!(
                    ids,
                    expected_ids
                        .iter()
                        .map(|&id| id)
                        .collect::<Vec<MorphID>>()
                        .into_boxed_slice()
                );
                Ok((table, size, ids))
            })
            // Run check_source_target
            .and_then(|(table, size, ids)| {
                check_source_target((table, size, ids))
            })
            .unwrap();
    }

    #[test]
    fn test_c3() {
        test_comp_table(
            "test/examples/c3.json",
            9,
            vec![MorphID(0), MorphID(1), MorphID(2), MorphID(3)],
        );
    }

    #[test]
    fn test_c4() {
        test_comp_table(
            "test/examples/c4_s1.json",
            10,
            vec![MorphID(0), MorphID(1), MorphID(2), MorphID(3)],
        );
    }

    #[test]
    fn test_c5() {
        test_comp_table(
            "test/examples/c5_s1.json",
            11,
            vec![MorphID(0), MorphID(1), MorphID(2), MorphID(3)],
        );
    }

    #[test]
    fn test_dummy_check_ids() {
        // let comp_table: CompositionTable = CompositionTable::new(vec![vec![0, 0], vec![1, 1]]);
        let comp_table = CompositionTable::new(vec![vec![Some(MorphID(0))]]);
        let result = check_morph_count(&comp_table)
            .and_then(check_ids)
            // .and_then(check_source_target(_));
            ;

        assert_eq!(result.is_ok(), true);
    }
}
