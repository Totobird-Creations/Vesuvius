use crate::parse::node::{
    Range,
    TypeDescriptor,
    Block
};


pub(crate) struct Value {
    value : ValueType,
    range : Range
}

impl Value {

    pub(crate) fn new(value : ValueType, range : Range) -> Self {
        return Self {
            value,
            range
        };
    }

    pub(crate) fn value(self) -> ValueType {
        return self.value;
    }

    pub(crate) fn range(&self) -> &Range {
        return &self.range;
    }

}


#[allow(unused)]
pub(crate) enum ValueType {
    Failed,

    Void,

    ModuleAccess(Vec<String>),
    Function(String, Vec<(String, TypeDescriptor)>, Option<TypeDescriptor>, Block)

}
