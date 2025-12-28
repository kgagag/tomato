use std::{collections::HashMap, iter::FlatMap, sync::Arc};

use log::info;

use crate::{
    classfile::class::{Class, MethodInfo}
};

#[derive(Debug)]
pub struct Metaspace {
    pub classes: Vec<Class>,
    pub class_map: HashMap<String, usize>,
    pub method_area: HashMap<String, MethodInfo>,
}

impl Metaspace {
    //创建元空间
    pub fn create() -> Metaspace {
        Metaspace {
            classes: vec![],
            class_map: HashMap::new(),
            method_area: HashMap::new(),
        }
    }

    pub fn get_method_from_pool(&self,
    class_name: &String,
    method_name: &String,
    descriptor: &String,
    ) -> Option<&MethodInfo> {
        let key = format!("{}{}{}{}{}", class_name, ".", method_name, ".", descriptor);
        info!("get_method_from_pool key:{}", key);
        for (key, value) in &self.method_area {
            println!("{}", key);
        }


        return self.method_area.get(&key);
    }
}

