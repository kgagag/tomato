use crate::class::MethodInfo;
use crate::class::*;
use crate::param::param::DataType;
use crate::runtime_data_area::get_or_load_class;
use crate::u8c::u8s_to_u16;
use crate::u8c::*;
use crate::value::value::number_to_u32tuple;
use crate::value::*;
use crate::value::value::StackFrameValue;
use log::*;
use std::collections::HashMap;
use std::mem;
use std::cell::UnsafeCell;
use std::sync::Mutex;
use crate::runtime_data_area::VM_STACKS;
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
}

impl StackFrame {
    pub fn new(
        class: usize,
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        code_attr: CodeAttribute,
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
        };
        for _i in 0..stake_frame.max_locals as usize {
            stake_frame.local.push(StackFrameValue::Byte(0));
        }
        //info!("{:?}",stake_frame);
        return stake_frame;
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
            StackFrameValue::Long(data) => data as i64,
            StackFrameValue::Short(data) => data as i64,
            _ => {
                panic!("wrong value type");
            }
        }
    }

    pub fn popf64(&mut self) -> f64 {
        let value = self.op_stack.pop().unwrap();
        match value {
            StackFrameValue::Double(data) => data as f64,
            StackFrameValue::Float(data) => data as f64,
            _ => {
                panic!("wrong value type");
            }
        }
    }

    pub fn pop_reference(&mut self) -> u32 {
        let value = self.op_stack.pop().unwrap();
        match value {
            StackFrameValue::Reference(data) => data,
            _ => {
                panic!("wrong value type");
            }
        }
    }
}

pub fn init_stack_frame(
    frame: &mut StackFrame,
    method_info: &MethodInfo,
    start: usize,
) -> StackFrame {
    let mut new_stack_frame: StackFrame = create_stack_frame(&method_info).unwrap();
    new_stack_frame.vm_stack_id = frame.vm_stack_id;
    let mut i: usize = start;
    if method_info.param.len() > 0 {
        for j in 0..method_info.param.len() {
            let v = frame.op_stack.pop().unwrap();
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
                    element_type,
                    depth,
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
                    let u32tuple = number_to_u32tuple(&v);
                    new_stack_frame.local[i] =
                        StackFrameValue::U32(u32tuple.0);
                    new_stack_frame.local[i + 1] =
                        StackFrameValue::U32(u32tuple.1);
                    i += 2;
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
                   // info!("{:?}", v);
                    let u32tuple = number_to_u32tuple(&v);
                    new_stack_frame.local[i] =
                        StackFrameValue::U32(u32tuple.0);
                    new_stack_frame.local[i + 1] =
                        StackFrameValue::U32(u32tuple.1);
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
    return new_stack_frame;
}


pub fn create_stack_frame(method_info: &MethodInfo) -> Option<StackFrame> {
    let class = get_or_load_class(&method_info.class_name);
    for attr in &method_info.attributes {
        match attr {
            AttributeInfo::Code(code_attr) => {
                return Some(StackFrame::new(
                    class.id,
                    code_attr.max_stack,
                    code_attr.max_locals,
                    code_attr.code.clone(),
                    code_attr.clone(),
                ));
            }
        }
    }
    return None;
}

pub fn create_stack_frame_with_class(method_info: &MethodInfo,class:&Class) -> Option<StackFrame> {
    for attr in &method_info.attributes {
        match attr {
            AttributeInfo::Code(code_attr) => {
                return Some(StackFrame::new(
                    class.id,
                    code_attr.max_stack,
                    code_attr.max_locals,
                    code_attr.code.clone(),
                    code_attr.clone(),
                ));
            }
        }
    }
    return None;
}

pub fn push_stack_frame(mut stack_frame: StackFrame) -> u32 {
    let data: std::sync::MutexGuard<'_, UnsafeCell<HashMap<u32, Vec<StackFrame>>>> =
        VM_STACKS.lock().unwrap();
        let mut vm_stack_id :u32= stack_frame.vm_stack_id;
    unsafe {
        let map: &mut HashMap<u32, Vec<StackFrame>> = &mut *data.get();
        if stack_frame.vm_stack_id == 0 {
            for i in 0x1..0xFFFFFFFF as u32 {
                if !map.contains_key(&i) {
                    stack_frame.vm_stack_id = i;
                    vm_stack_id = i;
                    let mut stack_frames: Vec<StackFrame> = Vec::new();
                    stack_frames.push(stack_frame);
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
    return vm_stack_id;
}


