use core::panic;
use std::{collections::HashMap, iter::FlatMap, sync::Arc};

use log::info;

use crate::{
    classfile::class::{self, Class, MethodInfo},
    common::{
        error::{Exception, Throwable},
        param::DataType,
        value::StackFrameValue,
    },
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

    /**
     * 持查找父类字段
     */
    pub fn get_field_tupple(
        &mut self,
        class_name: &String,
        field_name: &String,
    ) -> Result<(u16, u32, DataType, u16, String), Throwable> {
        //info!("get_field_tupple:{},{}", class_name, field_name);
        let class_id = self.class_map.get(class_name).unwrap();
        {
            let class = &self.classes[*class_id];
            let field = class.field_info.get(field_name);
            if field.is_some() {
                let field = field.unwrap();
                return Ok((
                    field.field_index,
                    field.offset,
                    field.data_type.clone(),
                    field.access_flag,
                    field.class_name.clone(),
                ));
            }
        }

        let mut class = &self.classes[*class_id];
        let mut field = class.field_info.get(field_name);
        while field.is_none() {
            class = &self.classes[class.super_class_id];
            if class.super_class_name.is_empty() {
                break;
            }
            field = class.field_info.get(field_name);
            if field.is_some() {
                let field = field.unwrap();
                return Ok((
                    field.field_index,
                    field.offset,
                    field.data_type.clone(),
                    field.access_flag,
                    field.class_name.clone(),
                ));
            }
        }
        return Err(Throwable::Exception(
            crate::common::error::Exception::FieldNotFound {
                class_name: class_name.clone(),
                field_name: field_name.clone(),
            },
        ));
    }

    pub fn get_method_from_pool(
        &mut self,
        class_name: &String,
        method_name: &String,
        descriptor: &String,
    ) -> Option<&MethodInfo> {
        let key = format!("{}{}{}{}{}", class_name, ".", method_name, ".", descriptor);
        return self.method_area.get(&key);
    }

    /*
     * 获取方法
     */
    pub fn get_method_from_root(
        &mut self,
        class_name: &String,
        method_name: &String,
        descriptor: &String,
    ) -> (Option<&MethodInfo>, &Class) {
        let key = format!("{}{}{}{}{}", class_name, ".", method_name, ".", descriptor);
        let mut m: Option<&MethodInfo> = self.method_area.get(&key);
        if m.is_some() {
            // info!("####{:?}#####",class_name);
            return (m, &self.classes[*self.class_map.get(class_name).unwrap()]);
        }
        let mut curr_class_name = class_name.clone();
        while m.is_none() {
            let class_id = self.class_map.get(&curr_class_name).unwrap();
            let class = &self.classes[*class_id];
            if class.super_class_name.is_empty() {
                break;
            }
            let sub_key = format!(
                "{}{}{}{}{}",
                class.super_class_name, ".", method_name, ".", descriptor
            );
            m = self.method_area.get(&sub_key);
            if m.is_some() {
                let supper_class =
                    &self.classes[*self.class_map.get(&class.super_class_name).unwrap()];
                return (m, supper_class);
            }
            if class.super_class_name.is_empty() {
                break;
            }
            curr_class_name = class.super_class_name.clone();
        }
        panic!("method not found");
    }

    /**
     * 获取字段
     */
    pub fn get_field_value_from_root(
        &mut self,
        class_name: &String,
        field_name: &String,
    ) -> Result<StackFrameValue, Throwable> {
        let class_id = self.class_map.get(class_name).unwrap();
        let mut class = &self.classes[*class_id];
        if class.field_info.contains_key(field_name) {
            return Ok(class.field_info.get(field_name).unwrap().value);
        }
        while !class.field_info.contains_key(field_name) {
            class = &self.classes[class.super_class_id];
            if class.field_info.contains_key(field_name) {
                return Ok(class.field_info.get(field_name).unwrap().value);
            }
            if class.super_class_name.is_empty() {
                break;
            }
        }
        return Err(Throwable::Exception(
            crate::common::error::Exception::FieldNotFound {
                class_name: class_name.clone(),
                field_name: field_name.clone(),
            },
        ));
    }

    pub fn put_field_value_to_root(
        &mut self,
        class_name: &String,
        field_name: &String,
        value: StackFrameValue,
    ) -> Result<(), Throwable> {
        {
            let class_id = self.class_map.get(class_name).unwrap();
            let class = &mut self.classes[*class_id];
            if class.field_info.contains_key(field_name) {
                class.field_info.get_mut(field_name).unwrap().value = value;
                return Ok(());
            }
        }
        let class_id = self.class_map.get(class_name).unwrap();
        let mut class = &self.classes[*class_id];
        let mut class_id = None;
        while !class.field_info.contains_key(field_name) {
            class = &self.classes[class.super_class_id];
            if class.field_info.contains_key(field_name) {
                class_id = Some(class.id.clone());
            }
            if class.super_class_name.is_empty() {
                break;
            }
        }
        if class_id.is_some() {
            self.classes[class_id.unwrap()]
                .field_info
                .get_mut(field_name)
                .unwrap()
                .value = value;
            return Ok(());
        }
        return Err(Throwable::Exception(
            crate::common::error::Exception::FieldNotFound {
                class_name: class_name.clone(),
                field_name: field_name.clone(),
            },
        ));
    }
}
