
use lazy_static::lazy_static;
use log::{info, warn};
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};

use crate::classfile::class::{Class, MethodInfo};
use crate::classloader;
use crate::common::array::array::Array;
use crate::common::object::Object;
use crate::common::param::DataType;
use crate::common::reference::Reference;
use crate::common::stack_frame::StackFrame;
use crate::common::value::StackFrameValue;
lazy_static! {
    // 创建一个包含UnsafeCell的Mutex，用于包装全局变量
    //类常量池
    pub static ref CLASS_DATA: Mutex<UnsafeCell<HashMap<String, Class>>> = Mutex::new(UnsafeCell::new(HashMap::new()));

    pub static ref CLASS_ID_DATA: Mutex<UnsafeCell<HashMap<usize,String>>> = Mutex::new(UnsafeCell::new(HashMap::new()));

    //方法数据，便于调用方法时快速查找
    pub static ref METHOD_DATA: Mutex<UnsafeCell<HashMap<String,MethodInfo>>> = Mutex::new(UnsafeCell::new(HashMap::new()));

    //字符串常量池
    pub static ref STR_CONSTANT_POOL: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());

    // 类对象常量池，是否可以跟字符串常量池合并？
    pub static ref CLASS_CONSTANT_POOL: Mutex<UnsafeCell<HashMap<String, u64>>> = Mutex::new(UnsafeCell::new(HashMap::new()));

    //对象存储
    pub static ref OBJECT_DATA: Mutex<UnsafeCell<HashMap<u64, Reference>>> = Mutex::new(UnsafeCell::new(HashMap::new()));

    static ref GLOBAL_COUNTER: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    //虚拟机栈
    pub static ref VM_STACKS: Mutex<UnsafeCell<HashMap<u32, Vec<StackFrame>>>> = Mutex::new(UnsafeCell::new(HashMap::new()));

    //TCP 连接
    pub static ref TCP_CONNECT: Mutex<UnsafeCell<HashMap<u64, TcpStream>>> = Mutex::new(UnsafeCell::new(HashMap::new()));


}

// 增加计数器的函数
fn increment_counter() -> u64 {
    // 使用 lock() 获取 Mutex 的锁，确保线程安全
    let mut counter = GLOBAL_COUNTER.lock().unwrap();
    *counter += 1;
    *counter
}

pub fn get_constant_pool_str(str: &String) -> Option<u64> {
    // 获取全局变量的Mutex锁
    let data: std::sync::MutexGuard<'_, HashMap<String, u64>> = STR_CONSTANT_POOL.lock().unwrap();
    //data.get(str).copied()
    let id: Option<&u64> = data.get(str);
    if id.is_some() {
        let reference = get_reference(id.unwrap());
        if reference.is_some() {
            return id.copied();
        }
    }
    None
}

pub fn put_into_class_constant_pool(string: String, obj_id: u64) {
    // 获取全局变量的Mutex锁
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<String, u64>>> =
        CLASS_CONSTANT_POOL.lock().unwrap();
    unsafe {
        // 从 UnsafeCell 中获取 HashMap 的可变引用
        let map = &mut *data.get();
        // 释放Mutex锁
        map.insert(string, obj_id);
        drop(data);
    }
}

pub fn get_constant_pool_class(str: &String) -> Option<&u64> {
    // 获取全局变量的Mutex锁
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<String, u64>>> =
        CLASS_CONSTANT_POOL.lock().unwrap();
    unsafe {
        // 从 UnsafeCell 中获取 HashMap 的可变引用
        let map = &mut *data.get();
        // 释放Mutex锁
        drop(data);
        let id = map.get(str);
        if id.is_some() {
            let reference = get_reference(id.unwrap());
            if reference.is_some() {
                return id;
            }
        }

        None
    }
}

pub fn put_into_str_constant_pool(string: String, obj_id: u64) {
    // 获取全局变量的Mutex锁
    let mut data: std::sync::MutexGuard<'_, HashMap<String, u64>> =
        STR_CONSTANT_POOL.lock().unwrap();
    // 从 UnsafeCell 中获取 HashMap 的可变引用
    //let  map =*data;
    // 释放Mutex锁
    data.insert(string, obj_id);
    drop(data);
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

pub fn create_object<'a>(class: usize) -> u64 {
    let data = OBJECT_DATA.lock().unwrap();
    let obj;
    unsafe {
        let map = &mut *data.get();
        let id = increment_counter();
        obj = Object::new(id, class);
        let id = obj.id;
        map.insert(obj.id, Reference::Object(obj));
        drop(data);
        id
    }
}

pub fn create_array(len: u32, array_type: DataType) -> u64 {
    let data = OBJECT_DATA.lock().unwrap();
    let array;
    unsafe {
        let map = &mut *data.get();
        let id = increment_counter();
        array = Array::new(id, len,array_type);
        map.insert(array.id, Reference::Array(array));
        drop(data);
        id
        
    }
}

/**
 * @id java对象的id
 * 返回引用类型数据
 */
pub fn get_reference<'a>(id: &u64) -> Option<&'a mut Reference> {
    let data = OBJECT_DATA.lock().unwrap();
    unsafe {
        let map = &mut *data.get();
        return map.get_mut(id);
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
           // classloader::class_loader::load_class(class_name);
        }
        let a = map.get_mut(class_name).unwrap();
        a
    }
}

pub fn init_class_id<'a>(class: &mut Class) -> &'a mut Class {
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<String, Class>>> =
        CLASS_DATA.lock().unwrap();
    unsafe {
        let map = &mut *data.get();
        let class_name = class.class_name.clone();
        if !map.contains_key(&class.class_name) {
            class.id = map.len() + 1_usize;
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

pub fn get_method_from_pool(
    class_name: &String,
    method_name: &String,
    descriptor: &String,
) -> Option<MethodInfo> {
    let data = METHOD_DATA.lock().unwrap();
    unsafe {
        let key = format!("{}{}{}{}{}", class_name, ".", method_name, ".", descriptor);
        let map = &mut *data.get();
        drop(data);
        return map.get(&key).cloned();
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

pub fn put_tcp(id:&u64,tcp_stream:TcpStream) {
    // 获取全局变量的Mutex锁
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u64, TcpStream>>> = TCP_CONNECT.lock().unwrap();
    unsafe {
        let map: &mut HashMap<u64, TcpStream> = &mut *data.get();
        map.insert(*id, tcp_stream);
    }
}

pub fn get_tcp <'a>(id:&u64) -> &'a TcpStream{
    // 获取全局变量的Mutex锁
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u64, TcpStream>>> = TCP_CONNECT.lock().unwrap();
    unsafe {
        let map: &mut HashMap<u64, TcpStream> = &mut *data.get();
        return map.get(id).unwrap()
    }
}



pub fn close_tcp(id:&u64) {
    // 获取全局变量的Mutex锁
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u64, TcpStream>>> = TCP_CONNECT.lock().unwrap();
    unsafe {
        let map: &mut HashMap<u64, TcpStream> = &mut *data.get();
         let _res = map.get(id).unwrap().shutdown(Shutdown::Both);
         //warn!("{:?}",map.capacity());
         //map.remove(id);
    }
}





