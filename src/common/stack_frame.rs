
use log::info;
use std::cell::UnsafeCell;
use std::collections::HashMap;

use crate::{classfile::class::{AttributeInfo, Class, CodeAttribute, ConstantPoolInfo, MethodInfo}, runtime::runtime_data_area::{VM_STACKS, get_method_from_pool, get_or_load_class}, utils::u8c::u8s_to_u16};

use super::{param::DataType, value::{number_to_u32tuple, StackFrameValue}};
/**
 * 栈桢
 */
#[derive(Debug, Clone)]
pub struct StackFrame {
    // //指令
    // pub code: Vec<u8>,
    //程序计数器
    pub pc: usize,
    // //局部变量表
    pub local: Vec<StackFrameValue>,
    // //操作数栈
    pub op_stack: Vec<StackFrameValue>,
    // //类
    pub class: usize,

    pub max_stack: u16,

    pub max_locals: u16,

    pub code: Vec<u8>,

    pub code_attr: CodeAttribute,

    //所属虚拟机栈id
    pub vm_stack_id: u32,

    pub method_name:String,

    pub descriptor:String,

    pub class_name:String
}

impl StackFrame {
    pub fn new(
        class: usize,
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        code_attr: CodeAttribute,
        method_name:String,
        descriptor:String,
        class_name:String
    ) -> StackFrame {
        let mut stake_frame = StackFrame {
            pc: 0,
            class,
            local: Vec::new(),
            op_stack: Vec::new(),
            max_stack,
            max_locals,
            code,
            code_attr,
            vm_stack_id: 0,
            method_name,
            descriptor,
            class_name
        };
        for _i in 0..stake_frame.max_locals as usize {
            stake_frame.local.push(StackFrameValue::Byte(0));
        }
        //info!("{:?}",stake_frame);
        stake_frame
    }

    pub fn store(&mut self, index: usize, stack_value: StackFrameValue) {
        self.local[index] = stack_value;
    }

    pub fn loadu32(&mut self, index: usize) -> u32 {
        let value = self.local.get(index).unwrap();
        match value {
            StackFrameValue::Int(data) => *data as u32,
            StackFrameValue::Byte(data) => *data as u32,
            StackFrameValue::Char(data) => *data as u32,
            StackFrameValue::Double(data) => *data as u32,
            StackFrameValue::Float(data) => *data as u32,
            StackFrameValue::Long(data) => *data as u32,
            StackFrameValue::Short(data) => *data as u32,
            StackFrameValue::CHARACTER(data) => *data as u32,
            StackFrameValue::Boolean(data) => *data as u32,
            _ => {
                panic!("wrong value type");
            }
        }
    }

    pub fn popi64(&mut self) -> i64 {
        let value = self.op_stack.pop().unwrap();
        match value {
            StackFrameValue::Int(data) => data as i64,
            StackFrameValue::Byte(data) => data as i64,
            StackFrameValue::Char(data) => data as i64,
            StackFrameValue::Long(data) => data ,
            StackFrameValue::Short(data) => data as i64,
            StackFrameValue::U32(data) => data as i64,
            StackFrameValue::CHARACTER(data) => data as i64,
            StackFrameValue::Boolean(data) => data as i64,
            StackFrameValue::Double(data) => data as i64,
            StackFrameValue::Float(data) => data as i64,
            _ => {
                panic!("wrong value type");
            }
        }
    }

    pub fn popf64(&mut self) -> f64 {
        let value = self.op_stack.pop().unwrap();
        match value {
            StackFrameValue::Double(data) => data ,
            StackFrameValue::Float(data) => data as f64,
            _ => {
                panic!("wrong value type");
            }
        }
    }

    pub fn pop_reference(&mut self) -> u64 {
        let value = self.op_stack.pop().unwrap();
        match value {
            StackFrameValue::Reference(data) => data,
            _ => {
                panic!("wrong value type");
            }
        }
    }


    pub fn get_method_for_invoke(&self) -> Option<MethodInfo> {
        let this_class = get_or_load_class(&self.class_name).clone();
        // 使用 match 代替 if let 以减少嵌套，并处理 unwrap 导致的潜在 panic
        let (class_index, name_and_type_index) = match this_class
            .constant_pool
            .get(&u8s_to_u16(&self.code[(self.pc + 1)..(self.pc + 3)]))
        {
            Some(ConstantPoolInfo::Methodref(class_index, name_and_type_index)) => {
                (class_index, name_and_type_index)
            }
            _ => return None,
        };

        // 通过链式调用减少嵌套
        let target_class_name = this_class
            .constant_pool
            .get(class_index)
            .and_then(|cp_info| match cp_info {
                ConstantPoolInfo::Class(name_index) => this_class.constant_pool.get(name_index),
                _ => None,
            })
            .and_then(|name_info| match name_info {
                ConstantPoolInfo::Utf8(target_class_name) => Some(target_class_name),
                _ => None,
            });

        let target_class = match target_class_name {
            Some(class_name_target) => get_or_load_class(class_name_target),
            None => return None,
        };

        // 继续减少嵌套并简化逻辑
        let (method_name, descriptor) = match this_class.constant_pool.get(name_and_type_index) {
            Some(ConstantPoolInfo::NameAndType(name_index, descriptor_index)) => {
                let method_name = match this_class.constant_pool.get(name_index) {
                    Some(ConstantPoolInfo::Utf8(name)) => name,
                    _ => return None,
                };
                let descriptor = match this_class.constant_pool.get(descriptor_index) {
                    Some(ConstantPoolInfo::Utf8(desc)) => desc,
                    _ => return None,
                };
                (method_name, descriptor)
            }
            _ => return None,
        };

        let mut method = get_method_from_pool(
        & target_class.class_name,
        & method_name,
        & descriptor,
        );
        let mut curr_class = target_class;
        while method.is_none() {
            let super_class = get_or_load_class(&curr_class.super_class_name);
            method = get_method_from_pool(
                &super_class.class_name,
            & method_name,
                &descriptor,
            );
            curr_class = super_class;
        }
        method
    }

}

// fn combine_u32_to_f64(high: u32, low: u32) -> f64 {
//     let bits = ((high as u64) << 32) | (low as u64); // 将两个u32组合成一个64位整数
//     f64::from_bits(bits) // 将64位整数转换回f64
// }

pub fn init_stack_frame(
    frame: &mut StackFrame,
    method_info: &MethodInfo,
    start: usize,
) -> StackFrame {
    let mut new_stack_frame: StackFrame = create_stack_frame(method_info).unwrap();
    new_stack_frame.vm_stack_id = frame.vm_stack_id;
    let mut i: usize = start;
    let mut param: Vec<StackFrameValue> = Vec::new();
    for _j in 0..method_info.param.len() {
        param.push(frame.op_stack.pop().unwrap());
    }

    //param.reverse();
    if !method_info.param.is_empty()  {
        for j in 0..method_info.param.len() {
            let v = param.pop().unwrap();
            let param: &DataType = method_info.param.get(j).unwrap();
            match param {
                DataType::Byte => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Char => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Array {
                    element_type: _,
                    depth: _,
                } => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Boolean => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Double => {
                    // info!("{:?}", v);
                    let u32tuple: (u32, u32) = number_to_u32tuple(&v);
                    new_stack_frame.local[i] = StackFrameValue::U32(u32tuple.0);
                    new_stack_frame.local[i + 1] = StackFrameValue::U32(u32tuple.1);
                    i += 2;
                    // let a = combine_u32_to_f64(u32tuple.1,u32tuple.0);
                    // info!("{:?}",a);
                    // info!("{:?}",a);
                }
                DataType::Float => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Int => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Long => {
                    let u32tuple = number_to_u32tuple(&v);
                    new_stack_frame.local[i] = StackFrameValue::U32(u32tuple.0);
                    new_stack_frame.local[i + 1] = StackFrameValue::U32(u32tuple.1);
                    i += 2;
                }
                DataType::Reference(_string) => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                DataType::Short => {
                    new_stack_frame.local[i] = v;
                    i += 1;
                }
                _ => panic!(),
            }
        }
    }
    new_stack_frame
}

pub fn create_stack_frame(method_info: &MethodInfo) -> Option<StackFrame> {
    let class = get_or_load_class(&method_info.class_name);
    for attr in &method_info.attributes {
        if let AttributeInfo::Code(code_attr) = attr {
            return Some(StackFrame::new(
                class.id,
                code_attr.max_stack,
                code_attr.max_locals,
                code_attr.code.clone(),
                code_attr.clone(),
                method_info.method_name.clone(),
                method_info.descriptor.clone(),
                class.class_name.clone()
            ));
        }
    }
    None
}

pub fn create_stack_frame_with_class(
    method_info: &MethodInfo,
    class: &Class,
) -> Option<StackFrame> {
    for attr in &method_info.attributes {
        if let AttributeInfo::Code(code_attr) = attr {
            return Some(StackFrame::new(
                class.id,
                code_attr.max_stack,
                code_attr.max_locals,
                code_attr.code.clone(),
                code_attr.clone(),
                method_info.method_name.clone(),
                method_info.descriptor.clone(),
                class.class_name.clone()
            ));
        }
    }
    None
}

pub fn push_stack_frame(mut stack_frame: StackFrame) -> u32 {
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> =
        VM_STACKS.lock().unwrap();
    let mut vm_stack_id: u32 = stack_frame.vm_stack_id;
    unsafe {
        let map: &mut HashMap<u32, Vec<StackFrame>> = &mut *data.get();
        if stack_frame.vm_stack_id == 0 {
            for i in 0x1..0xFFFFFFFF_u32 {
                if !map.contains_key(&i) {
                    stack_frame.vm_stack_id = i;
                    vm_stack_id = i;
                    let stack_frames: Vec<StackFrame> = vec![stack_frame];
                    map.insert(i, stack_frames);
                    break;
                }
            }
        } else {
            let frames = map.get_mut(&stack_frame.vm_stack_id).unwrap();
            //info!("before:{:?}", frames);
            frames.push(stack_frame);
            //info!("after:{:?}", frames);
        }
        drop(data);
    }
    vm_stack_id
}
