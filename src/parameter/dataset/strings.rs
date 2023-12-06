use std::num::NonZeroUsize;

use serde::Deserialize;

use super::{Parameter, ParameterSpace};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Strings {
    values: Vec<String>,
}

impl Strings {
    /// Return [None] if `values` are empty
    pub fn new(values: Vec<String>) -> Option<Self> {
        if values.is_empty() {
            return None;
        }
        Some(Self { values })
    }
}

impl ParameterSpace for Strings {
    fn space_size(&self) -> std::num::NonZeroUsize {
        NonZeroUsize::new(self.values.len()).expect("No values")
    }
}

impl Parameter for Strings {
    type Value = String;

    fn value(&self, index: usize) -> &Self::Value {
        &self.values[index]
    }
}
