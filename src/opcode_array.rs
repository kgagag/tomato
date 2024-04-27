use crate::array;
use crate::reference;
use crate::reference::reference::Reference;
use crate::runtime_data_area::create_array;
use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
extern crate env_logger;
extern crate log;
use crate::class::*;
use crate::param::param::DataType;
use crate::runtime_data_area::get_class_name;
use crate::runtime_data_area::get_method_from_pool;
use crate::runtime_data_area::get_or_load_class;
use crate::runtime_data_area::get_reference;
use crate::u8c::u8s_to_u16;
use log::*;

pub fn newarray(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    ////info!("{:?}",v);
    let atype = frame.code.get(frame.pc + 1).unwrap();
    ////info!("{:?}",atype);
    let array_type;
    let mut len: i32 = 0;
    match v {
        StackFrameValue::Byte(l) => len = l as i32,
        StackFrameValue::Char(l) => len = l as i32,
        StackFrameValue::Int(l) => len = l as i32,
        StackFrameValue::Short(l) => len = l as i32,
        _ => panic!(),
    }
    if *atype == 4 {
        array_type = DataType::Boolean;
    } else if *atype == 5 {
        array_type = DataType::Char;
    } else if *atype == 6 {
        array_type = DataType::Float;
    } else if *atype == 7 {
        array_type = DataType::Double;
    } else if *atype == 8 {
        array_type = DataType::Byte;
    } else if *atype == 9 {
        array_type = DataType::Short;
    } else if *atype == 10 {
        array_type = DataType::Int;
    } else if *atype == 11 {
        array_type = DataType::Long;
    } else {
        panic!("wrong atype");
    }
    let reference = create_array(len as u32, array_type);
    frame.op_stack.push(StackFrameValue::Reference(reference));
    frame.pc += 2;
}

fn extract_array_base_type_code(descriptor: &str) -> Option<u8> {
    let base_type_char = descriptor.chars().skip_while(|&c| c == '[').next()?;

    match base_type_char {
        'Z' => Some(4),  // boolean
        'C' => Some(5),  // char
        'F' => Some(6),  // float
        'D' => Some(7),  // double
        'B' => Some(8),  // byte
        'S' => Some(9),  // short
        'I' => Some(10), // int
        'J' => Some(11), // long
        'L' => Some(12), // object/array of objects, use a placeholder value
        _ => None,       // Unsupported or error in the descriptor
    }
}


fn get_arr_type(atype:u8){
    
}


pub fn multianewarray(frame: &mut StackFrame) {
    //info!("{:?}", frame);
    let index = u8s_to_u16(&frame.code[frame.pc + 1..frame.pc + 3]);
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name).clone();
    let attr = this_class.constant_pool.get(&index).unwrap();
    match attr {
        ConstantPoolInfo::Class(i) => {
            let class_utf8_attr = this_class.constant_pool.get(&i).unwrap();
            match class_utf8_attr {
                ConstantPoolInfo::Utf8(class_name_str) => {
                    let atype = extract_array_base_type_code(&class_name_str).unwrap();
                    let array_type: DataType;
                    if atype == 4 {
                        array_type = DataType::Boolean;
                    } else if atype == 5 {
                        array_type = DataType::Char;
                    } else if atype == 6 {
                        array_type = DataType::Float;
                    } else if atype == 7 {
                        array_type = DataType::Double;
                    } else if atype == 8 {
                        array_type = DataType::Byte;
                    } else if atype == 9 {
                        array_type = DataType::Short;
                    } else if atype == 10 {
                        array_type = DataType::Int;
                    } else if atype == 11 {
                        array_type = DataType::Long;
                    }  else if atype == 12 {
                        array_type = DataType::Reference(class_name);
                    } else {
                        panic!("wrong atype");
                    }
                    let dimenssion = frame.code[frame.pc + 3] as i32;
                    let mut len:u32 = 0;

                    let len_value = frame.op_stack.pop().unwrap();
                    
                    match len_value {
                        StackFrameValue::Byte(l) => {
                            len = l as u32;
                        },
                        StackFrameValue::Char(l) =>{
                            len = l as u32;
                        }
                        StackFrameValue::Int(l) => {
                            len = l as u32;
                        }
                        StackFrameValue::Short(l) => {
                            len = l as u32;
                        }
                        _ => panic!(),
                    }
                    
                    let mut reference = create_array(len as u32, array_type.clone());
                    let mut v :Vec<u32>  = Vec::new();
                    v.push(reference);
                    //info!("{:?}", frame.op_stack);
                    for i in 1 .. dimenssion{
                        //info!("{:?}",i);
                        let len_value: StackFrameValue = frame.op_stack.pop().unwrap();
                        match len_value {
                            StackFrameValue::Byte(l) => {
                                len = l as u32;
                            },
                            StackFrameValue::Char(l) =>{
                                len = l as u32;
                            }
                            StackFrameValue::Int(l) => {
                                len = l as u32;
                            }
                            StackFrameValue::Short(l) => {
                                len = l as u32;
                            }
                            _ => panic!(),
                        }
                        //上一个数组
                        let a = v.pop().unwrap();
                        // 创建数组
                        let b = create_muti_array(a,len,array_type.clone());
                        v.push(b);
                    }
                    frame.op_stack.push(StackFrameValue::Reference(reference));
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
    frame.pc += 4;
}


fn create_muti_array(referenceId:u32,len: u32,array_type: DataType) -> u32 {
   let newarr =  create_array(len, array_type);
   let reference = get_reference(&referenceId);
   match reference {
    Reference::Array(array) => {
        for i in 0 .. array.len{
            array.data.insert(i as usize, StackFrameValue::Reference(newarr))
        }
      }
       _=> panic!()
   }
   return newarr;
}

pub fn iastore(frame: &mut StackFrame) {
    xastore(frame);
}

pub fn lastore(frame: &mut StackFrame) {
    xastore(frame);
}
pub fn fastore(frame: &mut StackFrame) {
    xastore(frame);
}

pub fn dastore(frame: &mut StackFrame) {
    xastore(frame);
}

pub fn aastore(frame: &mut StackFrame) {
    xastore(frame);
}

pub fn bastore(frame: &mut StackFrame) {
    xastore(frame);
}

pub fn castore(frame: &mut StackFrame) {
    xastore(frame);
}

pub fn sastore(frame: &mut StackFrame) {
    xastore(frame);
}

fn xastore(frame: &mut StackFrame) {
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    let index = frame.op_stack.pop().unwrap();
    let array = frame.op_stack.pop().unwrap();
    //info!("{:?}",array);
    let i: usize;
    match index {
        StackFrameValue::Byte(l) => i = l as usize,
        StackFrameValue::Char(l) => i = l as usize,
        StackFrameValue::Int(l) => i = l as usize,
        StackFrameValue::Short(l) => i = l as usize,
        _ => panic!(),
    }
    match array {
        StackFrameValue::Reference(reference_id) => {
            let reference: &mut Reference = get_reference(&reference_id);
            // //info!("{:?}", reference);
            match reference {
                Reference::Array(arr) => {
                    arr.data.insert(i, v);
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn iaload(frame: &mut StackFrame) {
    xaload(frame);
}

pub fn laload(frame: &mut StackFrame) {
    xaload(frame);
}

pub fn faload(frame: &mut StackFrame) {
    xaload(frame);
}

pub fn daload(frame: &mut StackFrame) {
    xaload(frame);
}

pub fn aaload(frame: &mut StackFrame) {
    xaload(frame);
}

pub fn baload(frame: &mut StackFrame) {
    xaload(frame);
}

pub fn caload(frame: &mut StackFrame) {
    xaload(frame);
}

pub fn saload(frame: &mut StackFrame) {
    xaload(frame);
}

pub fn arraylength(frame: &mut StackFrame){
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Reference(reference) =>{
            let aref = get_reference(&reference);
            match aref {
                Reference::Array(array) =>{
                    frame.op_stack.push(StackFrameValue::U32(array.len))
                }
                _=> panic!()
            }
        }
        _=> panic!()
    }
    frame.pc += 1;
}

fn xaload(frame: &mut StackFrame) {
    let index = frame.op_stack.pop().unwrap();
    let array = frame.op_stack.pop().unwrap();
    //info!("{:?}",array);
    let i;
    match index {
        StackFrameValue::Byte(l) => i = l as usize,
        StackFrameValue::Char(l) => i = l as usize,
        StackFrameValue::Int(l) => i = l as usize,
        StackFrameValue::Short(l) => i = l as usize,
        _ => panic!(),
    }
    match array {
        StackFrameValue::Reference(reference_id) => {
            let reference = get_reference(&reference_id);
            match reference {
                Reference::Array(arr) => {
                    frame
                        .op_stack
                        .push(arr.data.get(i ).unwrap().clone());
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
    frame.pc += 1;
}
