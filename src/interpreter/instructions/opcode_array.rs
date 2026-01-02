use log::info;

use crate::{
    classfile::class::ConstantPoolInfo,
    classloader::{self, class_loader},
    common::{
        param::DataType,
        reference::{self, Reference},
        stack_frame::StackFrame,
        value::StackFrameValue,
    },
    runtime::{
        heap::{self, Heap},
        metaspace::Metaspace,
        runtime_data_area::{create_array, get_class_name, get_or_load_class, get_reference},
    },
    utils::u8c::u8s_to_u16,
};

pub fn newarray(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    let frame_index = vm_stack.len() - 1;
    let frame = &mut vm_stack[frame_index];
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    let atype = frame.code.get(frame.pc + 1).unwrap();
    let len: u32 = {
        match v {
            StackFrameValue::Byte(l) => l as u32,
            StackFrameValue::Char(l) => l as u32,
            StackFrameValue::Int(l) => l as u32,
            StackFrameValue::Short(l) => l as u32,
            StackFrameValue::U32(l) => l,
            _ => panic!(),
        }
    };
    let reference = heap.create_basic_array(*atype, len);
    frame
        .op_stack
        .push(StackFrameValue::Reference(reference as u32));
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

pub fn anewarray(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    let frame_index = vm_stack.len() - 1;
    let (class_name,len) = {
        let frame = &mut vm_stack[frame_index];
        let v: StackFrameValue = frame.op_stack.pop().unwrap();
        let len: u32;
        match v {
            StackFrameValue::Byte(l) => len = l as u32,
            StackFrameValue::Char(l) => len = l as u32,
            StackFrameValue::Int(l) => len = l as u32,
            StackFrameValue::Short(l) => len = l as u32,
            StackFrameValue::U32(l) => len = l,
            _ => panic!(),
        }
        let this_class = &metaspace.classes[frame.class];
        let index = u8s_to_u16(&frame.code[frame.pc + 1..frame.pc + 3]);
        let class_constant = this_class.constant_pool.get(&index).unwrap();
        match class_constant {
            ConstantPoolInfo::Class(name_index) => {
                let class_utf8_attr: &ConstantPoolInfo =
                    this_class.constant_pool.get(&name_index).unwrap();
                match class_utf8_attr {
                    ConstantPoolInfo::Utf8(class_name_str) => (class_name_str.clone(),len),
                    _ => panic!("class constant not found"),
                }
            }
            _ => panic!("class constant not found"),
        }
    };
    let class_id = class_loader::find_class(&class_name, vm_stack, heap, metaspace).id;
    let reference = heap.create_reference_array(class_id as u32, len);
    vm_stack[frame_index]
        .op_stack
        .push(StackFrameValue::Reference(reference as u32));
    vm_stack[frame_index].pc += 3;
}

pub fn multianewarray(frame: &mut StackFrame) {
    //info!("{:?}", frame);
    let index = u8s_to_u16(&frame.code[frame.pc + 1..frame.pc + 3]);
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);
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
                    } else if atype == 12 {
                        array_type = DataType::Reference(class_name_str.clone());
                    } else {
                        panic!("wrong atype");
                    }
                    let dimenssion: u8 = frame.code[frame.pc + 3];
                    let mut len: u32;

                    let len_value = frame.op_stack.pop().unwrap();

                    match len_value {
                        StackFrameValue::Byte(l) => {
                            len = l as u32;
                        }
                        StackFrameValue::Char(l) => {
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

                    let reference = create_array(
                        len as u32,
                        DataType::Array {
                            element_type: (Box::new(array_type.clone())),
                            depth: (dimenssion),
                        },
                    );
                    let mut v: Vec<u32> = Vec::new();
                    v.push(reference);
                    for i in 1..dimenssion {
                        let len_value: StackFrameValue = frame.op_stack.pop().unwrap();
                        match len_value {
                            StackFrameValue::Byte(l) => {
                                len = l as u32;
                            }
                            StackFrameValue::Char(l) => {
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
                        let b = create_muti_array(
                            v.pop().unwrap(),
                            len,
                            DataType::Array {
                                element_type: (Box::new(array_type.clone())),
                                depth: (dimenssion - i),
                            },
                        );
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

fn create_muti_array(reference_id: u32, len: u32, array_type: DataType) -> u32 {
    let newarr = create_array(len, array_type);
    let reference = get_reference(&reference_id).unwrap();
    match reference {
        Reference::Array(array) => {
            for i in 0..array.len {
                array
                    .data
                    .insert(i as usize, StackFrameValue::Reference(newarr))
            }
        }
        _ => panic!(),
    }
    newarr
}

pub fn iastore(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xastore(vm_stack, heap, metaspace);
}

pub fn lastore(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xastore(vm_stack, heap, metaspace);
}
pub fn fastore(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xastore(vm_stack, heap, metaspace);
}

pub fn dastore(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xastore(vm_stack, heap, metaspace);
}

pub fn aastore(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xastore(vm_stack, heap, metaspace);
}

pub fn bastore(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xastore(vm_stack, heap, metaspace);
}

pub fn castore(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
   xastore(vm_stack, heap, metaspace);
}

pub fn sastore(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xastore(vm_stack, heap, metaspace);
}

fn xastore(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, _metaspace: &mut Metaspace) {
    let frame_index = vm_stack.len() - 1;
    let frame = &mut vm_stack[frame_index];
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    let index = frame.op_stack.pop().unwrap();
    let array = frame.op_stack.pop().unwrap();
    let index = {
    match index {
        StackFrameValue::Byte(i) => i  as usize,
        StackFrameValue::Char(i) => i  as usize,
        StackFrameValue::Int(i) => i  as usize,
        StackFrameValue::Short(i) => i as usize,
        _ => panic!(),
    }};
    match array {
        StackFrameValue::Reference(reference_id) => {
            //todo 实现不同类型的put_basic_array_element
            match v {
                StackFrameValue::Byte(val) => {
                    heap.put_basic_array_element(reference_id, index, val as u64);
                },
                StackFrameValue::Char(val) => {
                    heap.put_basic_array_element(reference_id, index, val as u64);
                },
                StackFrameValue::Double(val) =>{
                    heap.put_basic_array_element(reference_id, index, val as u64);
                },
                StackFrameValue::Float(val) => {
                    heap.put_basic_array_element(reference_id, index, val as u64);
                },
                StackFrameValue::Int(val) => {
                    heap.put_basic_array_element(reference_id, index, val as u64);
                },
                StackFrameValue::Long(val) => {
                    heap.put_basic_array_element(reference_id, index, val as u64);
                },
                StackFrameValue::Short(val) => {
                    heap.put_basic_array_element(reference_id, index, val as u64);
                },
                StackFrameValue::Boolean(val) => {
                    heap.put_basic_array_element(reference_id, index, val as u64);
                },
                StackFrameValue::U32(val) => {
                    heap.put_basic_array_element(reference_id, index, val as u64);
                },
                StackFrameValue::U64(val) => {
                    heap.put_basic_array_element(reference_id, index, val as u64);
                },
                StackFrameValue::CHARACTER(val) => {
                    heap.put_basic_array_element(reference_id, index, val as u64);
                },
                _=>panic!("wrong value type")
            }

            //算出开始位置
            // 位置 = 元素大小 * i
            //
            // let reference: &mut Reference = get_reference(&reference_id).unwrap();
            // match reference {
            //     Reference::Array(arr) => {
            //         arr.data[i] = v;
            //     }
            //     _ => panic!(),
            // }
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

pub fn iaload(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xaload(vm_stack, heap, metaspace);
}

pub fn laload(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xaload(vm_stack, heap, metaspace);
}

pub fn faload(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xaload(vm_stack, heap, metaspace);
}

pub fn daload(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xaload(vm_stack, heap, metaspace);
}

pub fn aaload(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xaload(vm_stack, heap, metaspace);
}

pub fn baload(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xaload(vm_stack, heap, metaspace);
}

pub fn caload(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xaload(vm_stack, heap, metaspace);
}

pub fn saload(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    xaload(vm_stack, heap, metaspace);
}

pub fn arraylength(frame: &mut StackFrame) {
    let v = frame.op_stack.pop().unwrap();
    match v {
        StackFrameValue::Reference(reference) => {
            let aref = get_reference(&reference).unwrap();
            match aref {
                Reference::Array(array) => frame.op_stack.push(StackFrameValue::U32(array.len)),
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
    frame.pc += 1;
}

fn xaload(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap, metaspace: &mut Metaspace) {
    let frame_index = vm_stack.len() - 1;
    let frame = &mut vm_stack[frame_index];
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
            let (atype, value) = heap.get_basic_array_element(reference_id, i);
            info!("array type: {}, value: {}", atype, value);
            match atype {
                4 => { // boolean
                    frame.op_stack.push(StackFrameValue::Boolean(value != 0));
                },
                5 => { // char
                  //  frame.op_stack.push(StackFrameValue::CHARACTER(value as u16 as char));
                },
                6 => { // float
                    frame.op_stack.push(StackFrameValue::Float(f32::from_bits(value as u32)));
                },
                7 => { // double
                    frame.op_stack.push(StackFrameValue::Double(f64::from_bits(value)));
                },
                8 => { // byte
                    frame.op_stack.push(StackFrameValue::Byte(value as i8));
                },
                9 => { // short
                    frame.op_stack.push(StackFrameValue::Short(value as i16));
                },
                10 => { // int
                    frame.op_stack.push(StackFrameValue::Int(value as i32));
                },
                11 => { // long
                    frame.op_stack.push(StackFrameValue::Long(value as i64));
                },
                12 => { // object/array reference
                    frame.op_stack.push(StackFrameValue::Reference(value as u32));
                },
                _ => panic!("wrong atype"),
            }
        }
        _ => panic!(),
    }
    frame.pc += 1;
}