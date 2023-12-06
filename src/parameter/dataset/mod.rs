use std::{fmt, num::NonZeroUsize};

use self::strings::Strings;

use super::{Parameter, ParameterSpace};

pub mod strings;

pub enum ParameterDataset {
    Strings(Strings),
}
impl ParameterSpace for ParameterDataset {
    fn space_size(&self) -> NonZeroUsize {
        match self {
            ParameterDataset::Strings(x) => ParameterSpace::space_size(x),
        }
    }
}
impl ParameterDataset {
    pub fn value(&self, index: usize) -> ParameterValue {
        match self {
            ParameterDataset::Strings(x) => ParameterValue::String(Parameter::value(x, index)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterValue<'a> {
    String(&'a str),
    Int(i64),
    Float(f64),
}
impl<'a> ParameterValue<'a> {
    pub fn as_string(&'a self) -> Option<&'a str> {
        match self {
            ParameterValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            ParameterValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            ParameterValue::Float(x) => Some(*x),
            _ => None,
        }
    }
}
impl fmt::Display for ParameterValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParameterValue::String(s) => f.write_str(s),
            ParameterValue::Int(i) => f.write_str(&i.to_string()),
            ParameterValue::Float(x) => f.write_str(&x.to_string()),
        }
    }
}
