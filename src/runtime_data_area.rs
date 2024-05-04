use crate::array::array::Array;
use crate::class::{Class, MethodInfo};
use crate::class_loader::class_loader::load_class;
use crate::object::{self, Object};
use crate::param::param::DataType;
use crate::reference::reference::Reference;
use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
use lazy_static::lazy_static;
use log::info;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::sync::Mutex;
lazy_static! {
    // 创建一个包含UnsafeCell的Mutex，用于包装全局变量
    //类常量池
    pub static ref CLASS_DATA: Mutex<UnsafeCell<HashMap<String, Class>>> = Mutex::new(UnsafeCell::new(HashMap::new()));

    pub static ref CLASS_ID_DATA: Mutex<UnsafeCell<HashMap<usize,String>>> = Mutex::new(UnsafeCell::new(HashMap::new()));

    //方法数据，便于调用方法时快速查找
    pub static ref METHOD_DATA: Mutex<UnsafeCell<HashMap<String,MethodInfo>>> = Mutex::new(UnsafeCell::new(HashMap::new()));

    //字符串常量池
    pub static ref STR_POOL: Mutex<UnsafeCell<HashMap<String, u32>>> = Mutex::new(UnsafeCell::new(HashMap::new()));

    //对象存储
    pub static ref OBJECT_DATA: Mutex<UnsafeCell<HashMap<u32, Reference>>> = Mutex::new(UnsafeCell::new(HashMap::new()));

    //对象ID游标
    pub static ref OBJECT_ID: Mutex<UnsafeCell<u32>> = Mutex::new(UnsafeCell::new(0));

    //虚拟机栈
    pub static ref VM_STACKS: Mutex<UnsafeCell<HashMap<u32, Vec<StackFrame>>>> = Mutex::new(UnsafeCell::new(HashMap::new()));

}

pub fn get_constant_pool_str(str: &String) -> Option<&u32> {
    // 获取全局变量的Mutex锁
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<String, u32>>> =
        STR_POOL.lock().unwrap();
    unsafe {
        // 从 UnsafeCell 中获取 HashMap 的可变引用
        let map = &mut *data.get();
        // 释放Mutex锁
        drop(data);
        map.get(str)
    }
}

pub fn put_into_str_constant_pool(string: String, obj_id: u32) {
    // 获取全局变量的Mutex锁
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<String, u32>>> =
        STR_POOL.lock().unwrap();
    unsafe {
        // 从 UnsafeCell 中获取 HashMap 的可变引用
        let map = &mut *data.get();
        // 释放Mutex锁
        map.insert(string, obj_id);
        drop(data);
    }
}

pub fn class_exists(class_name: &String) -> bool {
    // 获取全局变量的Mutex锁
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<String, Class>>> =
        CLASS_DATA.lock().unwrap();
    unsafe {
        // 从 UnsafeCell 中获取 HashMap 的可变引用
        let map = &mut *data.get();
        // 释放Mutex锁
        drop(data);
        map.contains_key(class_name)
    }
}

pub fn create_object<'a>(class: usize) -> u32 {
    let data = OBJECT_DATA.lock().unwrap();
    let obj_id_data = OBJECT_ID.lock().unwrap();
    let obj;
    let obj_id: &mut u32;
    unsafe {
        let map = &mut *data.get();
        obj_id = &mut *obj_id_data.get();
        obj = Object::new(*obj_id + 1, class);
        map.insert(obj.id, Reference::Object(obj.clone()));
        drop(data);
        drop(obj_id_data);
        *obj_id += 1;
        obj.id
    }
}

pub fn create_array(len: u32, array_type: DataType) -> u32 {
    let data = OBJECT_DATA.lock().unwrap();
    let obj_id_data = OBJECT_ID.lock().unwrap();
    let array;
    let obj_id: &mut u32;
    let mut next_id: u32 = 0;
    unsafe {
        let map = &mut *data.get();
        obj_id = &mut *obj_id_data.get();
        next_id = *obj_id + 1;
        array = Array::new(next_id, len, array_type);
        map.insert(array.id, Reference::Array(array));
        drop(data);
        drop(obj_id_data);
        *obj_id += 1;
        next_id
    }
}

/**
 * @id java对象的id
 * 返回引用类型数据
 */
pub fn get_reference<'a>(id: &u32) -> &'a mut Reference {
    let data = OBJECT_DATA.lock().unwrap();
    unsafe {
        let map = &mut *data.get();
        return map.get_mut(id).unwrap();
    }
}

/**
 * @class_name java类名称
 */
pub fn get_or_load_class<'a>(class_name: &String) -> &'a mut Class {
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<String, Class>>> =
        CLASS_DATA.lock().unwrap();
    unsafe {
        let map = &mut *data.get();
        drop(data);
        if !map.contains_key(class_name) {
            load_class(class_name);
        }
        return map.get_mut(class_name).unwrap();
    }
}

pub fn init_class_id<'a>(class: &mut Class) -> &'a mut Class {
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<String, Class>>> =
        CLASS_DATA.lock().unwrap();
    unsafe {
        let map = &mut *data.get();
        let class_name = class.class_name.clone();
        if !map.contains_key(&class.class_name) {
            class.id = map.len() + 1 as usize;
            map.insert(class_name.clone(), class.clone());
            add_id_class(map.len(), class_name.clone());
        }
        drop(data);
        map.get_mut(&class_name).unwrap()
    }
}

pub fn add_id_class(class_id: usize, class_name: String) {
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<usize, String>>> =
        CLASS_ID_DATA.lock().unwrap();
    unsafe {
        let map = &mut *data.get();
        map.insert(class_id, class_name);
    }
    drop(data);
}

pub fn get_class_name(class_id: &usize) -> String {
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<usize, String>>> =
        CLASS_ID_DATA.lock().unwrap();
    unsafe {
        let map = &mut *data.get();
        drop(data);
        map.get(class_id).unwrap().clone()
    }
}

pub fn add_method(method_info: MethodInfo) {
    let data = METHOD_DATA.lock().unwrap();
    unsafe {
        let key = format!(
            "{}{}{}{}{}",
            method_info.class_name, ".", method_info.method_name, ".", method_info.descriptor
        );
        let map = &mut *data.get();
        map.insert(key, method_info);
    }
    drop(data);
}

pub fn get_method_from_pool<'a>(
    class_name: String,
    method_name: String,
    descriptor: String,
) -> Option<&'a MethodInfo> {
    let data = METHOD_DATA.lock().unwrap();
    unsafe {
        let key = format!("{}{}{}{}{}", class_name, ".", method_name, ".", descriptor);
        let map = &mut *data.get();
        drop(data);
        return map.get(&key);
    }
}

pub fn pop_stack_frame(vm_stack_id: u32) {
    let data = VM_STACKS.lock().unwrap();
    unsafe {
        let map = &mut *data.get();
        let frames = map.get_mut(&vm_stack_id).unwrap();
        frames.pop();
        // 如果过这个虚拟机栈已经被被清空就删除这个虚拟机栈
        if frames.is_empty() {
            map.remove(&vm_stack_id);
        }
    }
    drop(data);
}

pub fn push_frame_data(vm_stack_id: u32, value: StackFrameValue) {
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> =
        VM_STACKS.lock().unwrap();
    unsafe {
        let map: &mut HashMap<u32, Vec<StackFrame>> = &mut *data.get();
        //println!("before push_frame_data：{:?}",&map);
        let l = map.get_mut(&vm_stack_id).unwrap();
        let len = l.len();
        l.get_mut(len - 1).unwrap().op_stack.push(value);
        //println!("after push_frame_data：{:?}",&map);
    }
    drop(data);
}

pub fn create_string_object(str: &String) -> u32 {
   let char_array_id =  {
        let chars: Vec<char> = str.chars().collect();
        let char_array_id: u32 = create_array(chars.len() as u32, DataType::Char);
        let char_array_reference = get_reference(&char_array_id);
        match char_array_reference {
            Reference::Array(array) => {
                for i in 0..chars.len() {
                    array.data[i] = StackFrameValue::CHARACTER(*chars.get(i).unwrap());
                }
            }
            _ => panic!(),
        }
        char_array_id
    };
    let class_name = String::from("java/lang/String");
    let class: &mut Class = get_or_load_class(&class_name);
    let obj_id: u32 = create_object(class.id);
    let reference = get_reference(&obj_id);
    let object: &mut Object = match reference {
        Reference::Object(obj) => obj,
        _ => panic!(),
    };

    object.field.insert(
        String::from("value"),
        StackFrameValue::Reference(char_array_id),
    );
    put_into_str_constant_pool(str.clone(), obj_id);
    obj_id
}

pub fn create_class_object(class_name: &String) -> u32 {
    let class = get_or_load_class(class_name);
    let class0 = get_or_load_class(&String::from("java/lang/Class"));
    let obj_id = create_object(class0.id as usize);
    let id = create_string_object(&class.class_name);
    let referencre: &mut Reference = get_reference(&obj_id);
    match referencre {
        Reference::Object(object) => {
            object
                .field
                .insert(String::from("name"), StackFrameValue::Reference(id));
        }
        _ => panic!(),
    }
    obj_id
}
