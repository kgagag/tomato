use std::collections::HashMap;

use crate::value::value::StackFrameValue;
#[derive(Debug, Clone)]
pub struct Object {
    pub id: u32,
    pub class: usize,
    pub lock: u8,
    pub field: HashMap<u16, StackFrameValue>,
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
