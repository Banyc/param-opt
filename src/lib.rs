pub mod parameter;
pub mod selector;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        parameter::{
            dataset::{strings::Strings, ParameterDataset},
            table::ParameterTable,
        },
        selector::grid_search::GridSearch,
    };

    #[test]
    fn test_param_on_grid() {
        let mut datasets = HashMap::new();
        let dataset = ParameterDataset::Strings(
            Strings::new(
                ["foo", "bar"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            )
            .unwrap(),
        );
        datasets.insert(String::from("hello"), dataset);
        let dataset = ParameterDataset::Strings(
            Strings::new(["1", "2"].into_iter().map(ToString::to_string).collect()).unwrap(),
        );
        datasets.insert(String::from("world"), dataset);
        let table = ParameterTable::new(datasets);

        let grid_search = GridSearch::new(table.spaces());
        for indices in grid_search {
            for (k, v) in table.values(indices.into_iter()) {
                print!("({k}, {v}), ");
            }
            println!();
        }

        let mut grid_search = GridSearch::new(table.spaces());

        let indices = grid_search.next().unwrap();
        assert_eq!(indices, [0, 0]);

        let indices = grid_search.next().unwrap();
        assert_eq!(indices, [0, 1]);

        let indices = grid_search.next().unwrap();
        assert_eq!(indices, [1, 0]);

        let indices = grid_search.next().unwrap();
        assert_eq!(indices, [1, 1]);

        assert!(grid_search.next().is_none());
    }
}
