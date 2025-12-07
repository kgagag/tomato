use std::collections::HashMap;

use crate::common::value::StackFrameValue;

#[derive(Debug, Clone)]
pub struct Object {
    pub id: u64,
    pub class: String,
    pub lock: u8,
    pub field: HashMap<String, StackFrameValue>,
}

impl Object {
    pub fn new(id: u64, class: String) -> Object {
        Object {
            id,
            class,
            lock: 0,
            field: HashMap::new(),
        }
    }
}
