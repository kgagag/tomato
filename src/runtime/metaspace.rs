use std::collections::HashMap;

use crate::classfile::class::{Class, MethodInfo};

pub struct Metaspace {
    classes: [Class; 1024 * 128],
    method_area: HashMap<String, MethodInfo>    
}