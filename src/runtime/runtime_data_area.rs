use lazy_static::lazy_static;
use log::{info, warn};
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
    // 类常量池
    pub static ref CLASS_DATA: Mutex<HashMap<String, Class>> = Mutex::new(HashMap::new());

    pub static ref CLASS_ID_DATA: Mutex<HashMap<usize,String>> = Mutex::new(HashMap::new());

    // 方法数据，便于调用方法时快速查找
    pub static ref METHOD_DATA: Mutex<HashMap<String,MethodInfo>> = Mutex::new(HashMap::new());

    // 字符串常量池
    pub static ref STR_CONSTANT_POOL: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());

    // 类对象常量池
    pub static ref CLASS_CONSTANT_POOL: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());

    // 对象存储
    pub static ref OBJECT_DATA: Mutex<HashMap<u64, Reference>> = Mutex::new(HashMap::new());

    static ref GLOBAL_COUNTER: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    // TCP 连接
    pub static ref TCP_CONNECT: Mutex<HashMap<u64, TcpStream>> = Mutex::new(HashMap::new());
}

// 增加计数器的函数
fn increment_counter() -> u64 {
    let mut counter = GLOBAL_COUNTER.lock().unwrap();
    *counter += 1;
    *counter
}

pub fn get_constant_pool_str(str: &String) -> Option<u64> {
    let data = STR_CONSTANT_POOL.lock().unwrap();
    if let Some(id) = data.get(str) {
        // 注意：这里可能存在死锁风险，如果get_reference也获取了OBJECT_DATA锁
        if get_reference(id).is_some() {
            return Some(*id);
        }
    }
    None
}

pub fn put_into_class_constant_pool(string: String, obj_id: u64) {
    let mut data = CLASS_CONSTANT_POOL.lock().unwrap();
    data.insert(string, obj_id);
}

pub fn get_constant_pool_class(str: &String) -> Option<u64> {
    let data = CLASS_CONSTANT_POOL.lock().unwrap();
    if let Some(id) = data.get(str) {
        // 注意：这里可能存在死锁风险
        if get_reference(id).is_some() {
            return Some(*id);
        }
    }
    None
}

pub fn put_into_str_constant_pool(string: String, obj_id: u64) {
    let mut data = STR_CONSTANT_POOL.lock().unwrap();
    data.insert(string, obj_id);
}

pub fn class_exists(class_name: &String) -> bool {
    let data = CLASS_DATA.lock().unwrap();
    data.contains_key(class_name)
}

pub fn create_object(class: usize) -> u64 {
    let mut data = OBJECT_DATA.lock().unwrap();
    let id = increment_counter();
    let obj = Object::new(id, class);
    let id = obj.id;
    data.insert(obj.id, Reference::Object(obj));
    id
}

pub fn create_array(len: u32, array_type: DataType) -> u64 {
    let mut data = OBJECT_DATA.lock().unwrap();
    let id = increment_counter();
    let array = Array::new(id, len, array_type);
    data.insert(array.id, Reference::Array(array));
    id
}

/**
 * @id java对象的id
 * 返回引用类型数据
 */
pub fn get_reference(id: &u64) -> Option<Reference> {
    let data = OBJECT_DATA.lock().unwrap();
    data.get(id).cloned()
}

/**
 * @class_name java类名称
 */
pub fn get_or_load_class(class_name: &String) -> Class {
    {
        let data = CLASS_DATA.lock().unwrap();
        if data.contains_key(class_name) {
            return data.get(class_name).unwrap().clone();
        }
    } // Release the lock before loading class
    
    classloader::class_loader::load_class(class_name);
    
    let data = CLASS_DATA.lock().unwrap();
    data.get(class_name).unwrap().clone()
}

pub fn init_class_id(class: &mut Class) -> Class {
    let mut data = CLASS_DATA.lock().unwrap();
    let class_name = class.class_name.clone();
    if !data.contains_key(&class.class_name) {
        class.id = data.len() + 1_usize;
        data.insert(class_name.clone(), class.clone());
        drop(data); // Release lock before calling add_id_class
        add_id_class(class.id, class_name.clone());
        data = CLASS_DATA.lock().unwrap(); // Reacquire lock
    }
    data.get(&class_name).unwrap().clone()
}

pub fn add_id_class(class_id: usize, class_name: String) {
    let mut data = CLASS_ID_DATA.lock().unwrap();
    data.insert(class_id, class_name);
}

pub fn get_class_name(class_id: &usize) -> String {
    let data = CLASS_ID_DATA.lock().unwrap();
    data.get(class_id).unwrap().clone()
}

pub fn add_method(method_info: MethodInfo) {
    let mut data = METHOD_DATA.lock().unwrap();
    let key = format!(
        "{}.{}.{}",
        method_info.class_name, method_info.method_name, method_info.descriptor
    );
    data.insert(key, method_info);
}

pub fn get_method_from_pool(
    class_name: &String,
    method_name: &String,
    descriptor: &String,
) -> Option<MethodInfo> {
    let data = METHOD_DATA.lock().unwrap();
    let key = format!("{}.{}.{}", class_name, method_name, descriptor);
    data.get(&key).cloned()
}


pub fn put_tcp(id: &u64, tcp_stream: TcpStream) {
    let mut data = TCP_CONNECT.lock().unwrap();
    data.insert(*id, tcp_stream);
}

pub fn close_tcp(id: &u64) {
    let mut data = TCP_CONNECT.lock().unwrap();
    if let Some(stream) = data.get(id) {
        let _res = stream.shutdown(Shutdown::Both);
        warn!("{:?}", data.capacity());
        // If you want to remove the connection after closing:
        // data.remove(id);
    }
}