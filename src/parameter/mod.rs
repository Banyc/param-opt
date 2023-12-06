use std::num::NonZeroUsize;

pub mod dataset;
pub mod table;

pub trait ParameterSpace {
    fn space_size(&self) -> NonZeroUsize;
}

pub trait Parameter: ParameterSpace {
    type Value;
    fn value(&self, index: usize) -> &Self::Value;
}
