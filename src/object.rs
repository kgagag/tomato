use std::collections::HashMap;

use crate::value::value::StackFrameValue;
#[derive(Debug, Clone)]
pub struct Object {
    pub id: u64,
    pub class: usize,
    pub lock: u8,
    pub field: HashMap<String, StackFrameValue>,
}

impl Object {
    pub fn new(id: u64, class: usize) -> Object {
        Object {
            id,
            class,
            lock: 0,
            field: HashMap::new(),
        }
    }
}
