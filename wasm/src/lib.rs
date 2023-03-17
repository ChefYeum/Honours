mod checker;

#[cfg(test)]
mod test_bar {
    use std::{
        fs::File,
        io::{self, Read},
    };

    use crate::checker::{
        checker::{check_ids, check_morph_count},
        graph::{CompositionTable, MorphID},
    };

    fn read_comp_table(path: &str) -> Result<CompositionTable, io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let table: CompositionTable = serde_json::from_str(&contents)?;
        Ok(table)
    }

    #[test]
    fn test_c1_s1() {
        let comp_table = read_comp_table("test/examples/c1_s1.json").unwrap();
        check_morph_count(&comp_table)
            .and_then(|(table, morph_count)| {
                assert_eq!(morph_count, 8);
                Ok((table, morph_count))
            })
            .and_then(check_ids)
            .and_then(|(table, size, ids)| {
                assert_eq!(ids, vec![MorphID(0), MorphID(1), MorphID(2)]);
                Ok((table, size, ids))
            })
            .unwrap();
    }

    #[test]
    fn test_c3() {
        let comp_table = read_comp_table("test/examples/c3.json").unwrap();
        check_morph_count(&comp_table)
            .and_then(|(table, morph_count)| {
                assert_eq!(morph_count, 10);
                Ok((table, morph_count))
            })
            .and_then(check_ids)
            .and_then(|(table, size, ids)| {
                assert_eq!(ids, vec![MorphID(0), MorphID(1), MorphID(2), MorphID(3)]);
                Ok((table, size, ids))
            })
            .unwrap();
    }

        assert_eq!(result.is_ok(), true);
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
