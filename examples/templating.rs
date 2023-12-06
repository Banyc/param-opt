//! # Dependencies
//!
//! ```toml
//! anyhow = "1"
//! tera = "1"
//! toml = "0.8"
//! ```

use std::collections::HashMap;

use param_opt::{
    parameter::{dataset::ParameterDataset, table::ParameterTable},
    selector::grid_search::GridSearch,
};
use tera::{Context, Tera};

const TEMPLATE_NAME: &str = "template";
const TEMPLATE_CONTENT: &str = r#"
"foobar" = {{ foobar }}
"hi" = {{ hi }}
"#;
const PARAMETER_TABLE: &str = r#"
"foobar" = { type = "strings", values = ["foo", "bar"] }
"hi" = { type = "strings", values = ["hello", "world"] }
"#;

fn main() -> anyhow::Result<()> {
    let mut tera = Tera::default();
    tera.add_raw_template(TEMPLATE_NAME, TEMPLATE_CONTENT)?;

    let parameter_table: HashMap<String, ParameterDataset> = toml::from_str(PARAMETER_TABLE)?;
    let parameter_table = ParameterTable::new(parameter_table);

    let grid_search = GridSearch::new(parameter_table.spaces());
    for indices in grid_search {
        let mut context = Context::new();
        for (k, v) in parameter_table.values(indices.into_iter()) {
            context.insert(k, &v.to_string());
        }
        let rendered = tera.render(TEMPLATE_NAME, &context)?;
        println!("{rendered}");
    }

    Ok(())
}
