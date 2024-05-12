use std::collections::{HashSet, VecDeque};

use log::{info, warn};

use crate::{
    array::{self, array::Array},
    object::{self, Object},
    param::param::DataType,
    reference::{self, reference::Reference},
    runtime_data_area::*,
    value::{self, value::StackFrameValue},
};

/**
 * 一、如何判断一个对象是否进行回收 ?
 *  采用使用可达性分析法，分析一个对象是否还能被引用,如果不可能再次被引用则回收
 * 二、哪些对象不能回收 ?
 *  1、还处于虚拟机栈中的对象不予回收
 *  2、刚创建的对象不予回收（为了确保这一条GC时将需要终止所有线程的执行，并确保G
 * C时所有指令集函数已经结束）
 * 三、何时进行GC ?
 *  1、由于只是一个简单的示例GC方法，我暂定当内存中对象数量超过指定数量时进行GC，这样做起来比较简单。
 */
pub fn gc() {
    // 获取所有虚拟机栈
    let vm_stacks = VM_STACKS.lock().unwrap();
    let object_data = OBJECT_DATA.lock().unwrap();
    unsafe {
        let vm_stacks_map = &mut *vm_stacks.get();
        let reference_data = &mut *object_data.get();
        drop(object_data);
        if reference_data.len() > 10000 {
            warn!("gc before size:{}",reference_data.len());
            let mut exp: HashSet<u64> = HashSet::new();
            for (_key, value) in vm_stacks_map.iter() {
                for frame in value.iter() {
                    for sfv in frame.op_stack.iter() {
                        match sfv {
                            StackFrameValue::Reference(ref_id) => {
                                exp.extend(&get_ref_id(*ref_id));
                            }
                            _ => continue,
                        }
                    }
                }
            }
            let mut prepare_del_vec: Vec<u64> = Vec::new();
            for (key, _value) in reference_data.iter() {
                if !exp.contains(key) {
                    prepare_del_vec.push(*key);
                }
            }
            for id in prepare_del_vec.iter() {
                reference_data.remove(id);
            }
            warn!("gc after size:{}",reference_data.len());
        }
    }
}

fn get_ref_id(obj_id: u64) -> HashSet<u64> {
    let mut ans: HashSet<u64> = HashSet::new();
    let mut queue: VecDeque<u64> = VecDeque::new();
    queue.push_back(obj_id);
    while !queue.is_empty() {
        let id = queue.pop_front().unwrap();
        ans.insert(id);
        let reference = get_reference(&id);
        match reference {
            Reference::Object(object) => {
                for (key, value) in object.field.iter() {
                    match value {
                        StackFrameValue::Reference(id) => {
                            queue.push_back(*id);
                        }
                        _ => continue,
                    }
                }
            }
            Reference::Array(array) => {
                let data = array.data.clone();
                for v in data.into_iter() {
                    match v {
                        StackFrameValue::Reference(id) => {
                            queue.push_back(id);
                        }
                        _ => continue,
                    }
                }
            }
        }
    }
    ans
}
