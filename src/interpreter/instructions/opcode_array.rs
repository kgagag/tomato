use log::info;

use crate::{classfile::class::ConstantPoolInfo, common::{param::DataType, reference::Reference, stack_frame::StackFrame, value::StackFrameValue}, runtime::{heap::{self, Heap}, metaspace::{self, Metaspace}}, utils::u8c::u8s_to_u16};


pub fn newarray(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    let frame = vm_stack.last_mut().unwrap() ;
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    ////info!("{:?}",v);
    let atype = frame.code.get(frame.pc + 1).unwrap();
    ////info!("{:?}",atype);
    let array_type;
    let  len: u32 ;
    match v {
        StackFrameValue::Byte(l) => len = l as u32,
        StackFrameValue::Char(l) => len = l as u32,
        StackFrameValue::Int(l) => len = l as u32,
        StackFrameValue::Short(l) => len = l as u32,
        StackFrameValue::U32(l) => len = l,
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
    let reference = heap.create_array(len, DataType::Array { element_type: Box::new(array_type), depth: (1) });
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


pub fn anewarray(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap){
    let frame = vm_stack.last_mut().unwrap() ;
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    let  len: u32 ;
    match v {
        StackFrameValue::Byte(l) => len = l as u32,
        StackFrameValue::Char(l) => len = l as u32,
        StackFrameValue::Int(l) => len = l as u32,
        StackFrameValue::Short(l) => len = l as u32,
        StackFrameValue::U32(l) => len = l,
        _ => panic!(),
    }
    let reference = heap.create_array(len as u32, DataType::Array{
       element_type:Box::new(DataType::Reference(frame.class_name.clone())),
       depth: (1)
    });
    frame.op_stack.push(StackFrameValue::Reference(reference));
    frame.pc += 3;
}



pub fn multianewarray(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap,metaspace : &mut Metaspace) {
    let frame = vm_stack.last_mut().unwrap();
    //info!("{:?}", frame);
    let index = u8s_to_u16(&frame.code[frame.pc + 1..frame.pc + 3]);
    let this_class = metaspace.get_class(&frame.class_name).unwrap();
    let attr = this_class.constant_pool.get(&index).unwrap();
    match attr {
        ConstantPoolInfo::Class(i) => {
            let class_utf8_attr: &ConstantPoolInfo = this_class.constant_pool.get(&i).unwrap();
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
                        array_type = DataType::Reference(class_name_str.clone());
                    } else {
                        panic!("wrong atype");
                    }
                    let dimenssion: u8 = frame.code[frame.pc + 3];
                    let mut len:u32;

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
                    
                    let reference = heap.create_array(len as u32, DataType::Array { element_type: (Box::new(array_type.clone())), depth: (dimenssion) });
                    let mut v :Vec<u64>  = Vec::new();
                    v.push(reference);
                    for i in 1 .. dimenssion{
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
                        // 创建数组
                        let b = create_muti_array(v.pop().unwrap(),len,DataType::Array { element_type: (Box::new(array_type.clone())), depth: (dimenssion - i) },heap);
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


fn create_muti_array(reference_id:u64,len: u32 ,array_type: DataType,heap : &mut Heap) -> u64 {
   let newarr =  heap.create_array(len, array_type);
   let mut reference = heap.get_reference_mut(&reference_id).unwrap();
   match &mut *reference {
    Reference::Array(array) => {
        for i in 0 .. array.len{
            array.data.insert(i as usize, StackFrameValue::Reference(newarr))
        }
      }
       _=> panic!()
   }
    newarr
}

pub fn iastore(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xastore(vm_stack,heap);
}

pub fn lastore(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xastore(vm_stack,heap);
}
pub fn fastore(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xastore(vm_stack,heap);
}

pub fn dastore(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xastore(vm_stack,heap);
}

pub fn aastore(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xastore(vm_stack,heap);
}

pub fn bastore(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xastore(vm_stack,heap);
}

pub fn castore(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xastore(vm_stack,heap);
}

pub fn sastore(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xastore(vm_stack,heap);
}

fn xastore(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap ) {
    let frame = vm_stack.last_mut().unwrap();
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    let index = frame.op_stack.pop().unwrap();
    let array = frame.op_stack.pop().unwrap();
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
            let mut reference = heap.get_reference_mut(&reference_id).unwrap();
            match &mut * reference {
                Reference::Array(arr) => {
                    arr.data[i] = v;
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn iaload(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xaload(vm_stack,heap);
}

pub fn laload(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xaload(vm_stack,heap);
}

pub fn faload(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xaload(vm_stack,heap);
}

pub fn daload(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xaload(vm_stack,heap);
}

pub fn aaload(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xaload(vm_stack,heap);
}

pub fn baload(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xaload(vm_stack,heap);
}

pub fn caload(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xaload(vm_stack,heap);
}

pub fn saload(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap) {
    xaload(vm_stack,heap);
}

pub fn arraylength(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap){
    let frame =  vm_stack.last_mut().unwrap();
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Reference(reference) =>{
            let mut aref = heap.get_reference_mut(&reference).unwrap();
            match &mut * aref {
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

fn xaload(vm_stack:&mut Vec<StackFrame> , heap : &mut Heap ) {
    let frame =  vm_stack.last_mut().unwrap();
    let index = frame.op_stack.pop().unwrap();
    let array = frame.op_stack.pop().unwrap();
    let i;
    match index {
        StackFrameValue::Byte(l) => i = l as usize,
        StackFrameValue::Char(l) => i = l as usize,
        StackFrameValue::Int(l) => i = l as usize,
        StackFrameValue::Short(l) => i = l as usize,
        StackFrameValue::CHARACTER(l) => i = l as usize,
        _ => panic!(),
    }
    match array {
        StackFrameValue::Reference(reference_id) => {
            let mut reference = heap.get_reference_mut(&reference_id).unwrap();
            match &mut * reference {
                Reference::Array(arr) => {
                    &mut frame
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
