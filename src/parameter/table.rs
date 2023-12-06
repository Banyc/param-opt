use std::{collections::HashMap, num::NonZeroUsize, sync::Arc};

use super::{
    dataset::{ParameterDataset, ParameterValue},
    ParameterSpace,
};

pub struct ParameterTable {
    named: HashMap<String, ParameterDataset>,
    vectored: Arc<[String]>,
}
impl ParameterTable {
    pub fn new(parameters: HashMap<String, ParameterDataset>) -> Self {
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
