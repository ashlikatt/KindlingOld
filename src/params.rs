use crate::values::{ParameterList, Tag, Value};

pub struct ParamBuilder {
    params: Vec<Value>,
    tags: Vec<Tag>
}
impl ParamBuilder {
    pub fn new() -> Self {
        Self {
            params: Vec::with_capacity(27),
            tags: Vec::with_capacity(27),
        }
    }
    pub fn param(mut self, v: Value) -> Self {
        self.params.push(v);
        self
    }
    pub fn tag(mut self, v: Tag) -> Self {
        self.tags.push(v);
        self
    }
    pub fn complete(self) -> Option<ParameterList> {
        if self.params.len() + self.tags.len() > 27 {
            return None;
        }
        let mut out = [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None];
        for (index, val) in self.params.into_iter().enumerate() {
            out[index] = Some(val);
        }
        for (index, tag) in self.tags.into_iter().rev().enumerate() {
            out[out.len()-index-1] = Some(Value::Tag(tag));
        }
        Some(out)
    }
    pub fn complete_unchecked(self) -> ParameterList {
        self.complete().unwrap()
    }
}