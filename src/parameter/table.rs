use std::{collections::HashMap, num::NonZeroUsize, sync::Arc};

use thiserror::Error;

use super::{
    dataset::{ParameterDataset, ParameterValue},
    ParameterSpace,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterTable {
    named: HashMap<String, ParameterDataset>,
    vectored: Arc<[String]>,
}
impl ParameterTable {
    pub fn new(
        named: HashMap<String, ParameterDataset>,
        vectored: Arc<[String]>,
    ) -> Result<Self, ParameterTableError> {
        if named.len() != vectored.len() {
            return Err(ParameterTableError::InconsistentLength);
        }
        for key in vectored.as_ref() {
            if named.get(key).is_none() {
                return Err(ParameterTableError::KeyNotExists {
                    key: key.to_owned(),
                });
            }
        }
        Ok(Self { named, vectored })
    }

    pub fn new_unordered(parameters: HashMap<String, ParameterDataset>) -> Self {
        let vectored = parameters.keys().cloned().collect();
        Self {
            named: parameters,
            vectored,
        }
    }

    pub fn spaces(&self) -> impl Iterator<Item = NonZeroUsize> + '_ {
        self.vectored
            .iter()
            .map(|name| &self.named[name])
            .map(ParameterSpace::space_size)
    }

    pub fn values(
        &self,
        indices: impl Iterator<Item = usize>,
    ) -> impl Iterator<Item = (&String, ParameterValue)> {
        indices
            .enumerate()
            .map(|(i, index)| (&self.vectored[i], index))
            .map(|(name, index)| (name, self.named[name].value(index)))
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParameterTableError {
    #[error("`named` and `vectored` should be of the same size")]
    InconsistentLength,
    #[error("`{key}` does not exist in `named`")]
    KeyNotExists { key: String },
}
