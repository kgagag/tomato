use std::collections::HashMap;

use crate::common::value::StackFrameValue;

#[derive(Debug, Clone)]
pub struct Object {
    pub id: u32,
    pub class: usize,
    pub lock: u8,
    pub field: HashMap<String, StackFrameValue>,
}

impl Object {
    pub fn new(id: u32, class: usize) -> Object {
        Object {
            id,
            class,
            lock: 0,
            field: HashMap::new(),
        }
    }
}
