use std::collections::VecDeque;

use log::info;

use crate::{
    classfile::class::ConstantPoolInfo,
    classloader::{self, class_loader},
    common::{
        error::Throwable,
        param::DataType,
        reference::{self, Reference},
        stack_frame::StackFrame,
        value::StackFrameValue,
    },
    runtime::{
        heap::{self, Heap},
        metaspace::Metaspace,
    },
    utils::u8c::u8s_to_u16,
};

pub fn newarray(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
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
    let reference = heap.create_basic_array(*atype, len,  1);
    frame
        .op_stack
        .push(StackFrameValue::Reference(reference as u32));
    frame.pc += 2;
    Ok(())
}

fn extract_array_base_type_info(descriptor: &str) -> Option<(u8, Option<String>)> {
    // 跳过所有的'['，找到第一个非数组的字符
    let start_pos = descriptor.chars().position(|c| c != '[').unwrap_or(0);

    let remaining = &descriptor[start_pos..];
    let first_char = remaining.chars().next()?;

    match first_char {
        'Z' => Some((4, None)),  // boolean
        'C' => Some((5, None)),  // char
        'F' => Some((6, None)),  // float
        'D' => Some((7, None)),  // double
        'B' => Some((8, None)),  // byte
        'S' => Some((9, None)),  // short
        'I' => Some((10, None)), // int
        'J' => Some((11, None)), // long
        'L' => {
            // 提取类名：L<class>; 格式
            // 去掉开头的'L'和结尾的';'
            if remaining.ends_with(';') && remaining.len() > 2 {
                let class_name = &remaining[1..remaining.len() - 1];
                Some((12, Some(class_name.to_string())))
            } else {
                // 无效的类描述符
                panic!("Invalid class descriptor")
            }
        }
        _ => None, // 不支持的描述符
    }
}

pub fn anewarray(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let (class_name, len) = {
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
        let this_class: &crate::classfile::class::Class = &metaspace.classes[frame.class];
        let index: u16 = u8s_to_u16(&frame.code[frame.pc + 1..frame.pc + 3]);
        let class_constant = &this_class.constant_pool[index as usize];
        match class_constant {
            ConstantPoolInfo::Class(name_index) => {
                let class_utf8_attr: &ConstantPoolInfo =
                    &this_class.constant_pool[*name_index as usize];
                match class_utf8_attr {
                    ConstantPoolInfo::Utf8(class_name_str) => (class_name_str.clone(), len),
                    _ => panic!("class constant not found"),
                }
            }
            _ => panic!("class constant not found"),
        }
    };

    let class_id = {
        if metaspace.class_map.contains_key(&class_name) {
            let class = class_loader::find_class(&class_name, vm_stack, heap, metaspace)?;
            class.id
        } else {
            0
        }
    };
    //let class_id = class_loader::find_class(&class_name, vm_stack, heap, metaspace).id;
    //12 = 引用类型数组
    let reference = heap.create_reference_array(class_id as u32, len, 0, 12);
    vm_stack[frame_index]
        .op_stack
        .push(StackFrameValue::Reference(reference as u32));
    vm_stack[frame_index].pc += 3;
    Ok(())
}

pub fn multianewarray(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let (atype, dimenssion, array_class_name) = {
        let frame = &mut vm_stack[frame_index];
        let index = u8s_to_u16(&frame.code[frame.pc + 1..frame.pc + 3]);
        let this_class = &metaspace.classes[frame.class];
        let attr = &this_class.constant_pool[index as usize];
        match attr {
            ConstantPoolInfo::Class(i) => {
                let class_utf8_attr: &ConstantPoolInfo = &this_class.constant_pool[*i as usize];
                match class_utf8_attr {
                    ConstantPoolInfo::Utf8(class_name_str) => {
                        let (atype, array_class_name) =
                            extract_array_base_type_info(&class_name_str).unwrap();
                        (atype, frame.code[frame.pc + 3], array_class_name)
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    };

    let class_id = {
        if atype == 12 {
            let class_name = &array_class_name.unwrap();
            let class = class_loader::find_class(class_name, vm_stack, heap, metaspace)?;
            Some(class.id)
        } else {
            None
        }
    };

    let mut queue: VecDeque<(u32, usize)> = VecDeque::new();
    let mut sfv_vec = Vec::new();
    //全部弹出
    for _i in 0..dimenssion {
        // _ = vm_stack[frame_index].op_stack.pop()
        let sfv = vm_stack[frame_index].op_stack.pop().unwrap();
        sfv_vec.push(sfv);
    }
    for i in 0..dimenssion {
        let len = {
            let len_value = sfv_vec.pop().unwrap();
            match len_value {
                StackFrameValue::Byte(l) => l as u32,
                StackFrameValue::Char(l) => l as u32,
                StackFrameValue::Int(l) => l as u32,
                StackFrameValue::Short(l) => l as u32,
                _ => panic!(),
            }
        };
        if queue.len() == 0 {
            let reference_id: usize =
             if class_id.is_none() {
                heap.create_reference_array(0, len, dimenssion - i, atype)
            } else {
                heap.create_reference_array(class_id.unwrap() as u32, len, dimenssion - i, atype)
            };
            // heap.create_reference_array(0, len, dimenssion - i, atype);
            //info!("create array reference:{}", reference_id);
            vm_stack[frame_index]
                .op_stack
                .push(StackFrameValue::Reference(reference_id as u32));
            queue.push_back((len, reference_id));
        } else {
            let size = queue.len();
            for _j in 0..size {
                let (plen, reference) = queue.pop_front().unwrap();
                //在这个reference中创建新的数组,个数为len
                for index in 0..plen {
                    //不是引用类型
                    if atype != 12 {
                        if i == dimenssion - 1 {
                            let id = heap.create_basic_array(atype, len, dimenssion - i);
                            heap.put_array_element(reference as u32, index as usize, id as u64);
                            queue.push_back((len, id));
                        } else {
                            let id: usize =
                                heap.create_reference_array(0, len, dimenssion - i, atype);
                            heap.put_array_element(reference as u32, index as usize, id as u64);
                            queue.push_back((len, id));
                        }
                    } else {
                        let id = heap.create_reference_array(
                            class_id.unwrap() as u32,
                            len,
                            dimenssion - i,
                            atype,
                        );
                        heap.put_array_element(reference as u32, index as usize, id as u64);
                        queue.push_back((len, id));
                    }
                }
            }
        }
    }
    vm_stack[frame_index].pc += 4;
    Ok(())
}

pub fn iastore(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xastore(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn lastore(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xastore(vm_stack, heap, metaspace)?;
    Ok(())
}
pub fn fastore(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xastore(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn dastore(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xastore(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn aastore(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xastore(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn bastore(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xastore(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn castore(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xastore(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn sastore(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xastore(vm_stack, heap, metaspace)?;
    Ok(())
}

fn xastore(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    _metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let frame = &mut vm_stack[frame_index];
    let v: StackFrameValue = frame.op_stack.pop().unwrap();
    let index = frame.op_stack.pop().unwrap();
    let array = frame.op_stack.pop().unwrap();
    let index = {
        match index {
            StackFrameValue::Byte(i) => i as usize,
            StackFrameValue::Char(i) => i as usize,
            StackFrameValue::Int(i) => i as usize,
            StackFrameValue::Short(i) => i as usize,
            _ => panic!(),
        }
    };
    match array {
        StackFrameValue::Reference(reference_id) => {
            //todo 实现不同类型的put_basic_array_element
            match v {
                StackFrameValue::Byte(val) => {
                    heap.put_array_element(reference_id, index, val as u64);
                }
                StackFrameValue::Char(val) => {
                    heap.put_array_element(reference_id, index, val as u64);
                }
                StackFrameValue::Double(val) => {
                    heap.put_array_element(reference_id, index, val as u64);
                }
                StackFrameValue::Float(val) => {
                    heap.put_array_element(reference_id, index, val as u64);
                }
                StackFrameValue::Int(val) => {
                    heap.put_array_element(reference_id, index, val as u64);
                }
                StackFrameValue::Long(val) => {
                    heap.put_array_element(reference_id, index, val as u64);
                }
                StackFrameValue::Short(val) => {
                    heap.put_array_element(reference_id, index, val as u64);
                }
                StackFrameValue::Boolean(val) => {
                    heap.put_array_element(reference_id, index, val as u64);
                }
                StackFrameValue::U32(val) => {
                    heap.put_array_element(reference_id, index, val as u64);
                }
                StackFrameValue::U64(val) => {
                    heap.put_array_element(reference_id, index, val as u64);
                }
                StackFrameValue::CHARACTER(val) => {
                    heap.put_array_element(reference_id, index, val as u64);
                }
                StackFrameValue::Reference(val) => {
                    heap.put_array_element(reference_id, index, val as u64);
                }
                StackFrameValue::Null => {
                    heap.put_array_element(reference_id, index, 0);
                }
                _ => panic!(""),
            }
        }
        _ => panic!(),
    }
    frame.pc += 1;
    Ok(())
}

pub fn iaload(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xaload(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn laload(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xaload(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn faload(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xaload(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn daload(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xaload(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn aaload(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xaload(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn baload(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xaload(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn caload(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xaload(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn saload(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
    xaload(vm_stack, heap, metaspace)?;
    Ok(())
}

pub fn arraylength(vm_stack: &mut Vec<StackFrame>, heap: &mut Heap) -> Result<(), Throwable> {
    let frame_index = vm_stack.len() - 1;
    let frame = &mut vm_stack[frame_index];
    let v = frame.op_stack.pop().unwrap();
    //info!("v = {:?}",v);
    match v {
        StackFrameValue::Reference(reference) => frame
            .op_stack
            .push(StackFrameValue::U32(heap.get_array_length(reference))),
        StackFrameValue::Null=>{
            return Err(Throwable::Exception(
               crate::common::error::Exception::NullPointer("java/lang/NullPointerException".to_string())
            ));
        }
        _ => {
            return Err(Throwable::Error(crate::common::error::JvmError::UnknownError("unknown error".to_string())));
        }
    }
    frame.pc += 1;
    Ok(())
}

fn xaload(
    vm_stack: &mut Vec<StackFrame>,
    heap: &mut Heap,
    metaspace: &mut Metaspace,
) -> Result<(), Throwable> {
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
            let (atype, value) = heap.get_array_element(reference_id, i);
            match atype {
                4 => {
                    if value.is_none() {
                       return  Err(Throwable::Exception(
                            crate::common::error::Exception::NullPointer(
                                "null pointer exception".to_owned(),
                            ),
                        ))
                    };
                    // boolean
                    frame
                        .op_stack
                        .push(StackFrameValue::Boolean(value.unwrap() != 0));
                }
                5 => {
                    if value.is_none() {
                      return  Err(Throwable::Exception(
                            crate::common::error::Exception::NullPointer(
                                "null pointer exception".to_owned(),
                            ),
                        ))
                    }
                    // char
                    frame.op_stack.push(StackFrameValue::CHARACTER(
                        char::from_u32(value.unwrap() as u32).unwrap(),
                    ));
                }
                6 => {
                    if value.is_none() {
                    return    Err(Throwable::Exception(
                            crate::common::error::Exception::NullPointer(
                                "null pointer exception".to_owned(),
                            ),
                        ))
                    }
                    // float
                    frame
                        .op_stack
                        .push(StackFrameValue::Float(f32::from_bits(value.unwrap() as u32)));
                }
                7 => {
                    if value.is_none() {
                     return   Err(Throwable::Exception(
                            crate::common::error::Exception::NullPointer(
                                "null pointer exception".to_owned(),
                            ),
                        ))
                    }
                    // double
                    frame
                        .op_stack
                        .push(StackFrameValue::Double(f64::from_bits(value.unwrap())));
                }
                8 => {
                    if value.is_none() {
                      return  Err(Throwable::Exception(
                            crate::common::error::Exception::NullPointer(
                                "null pointer exception".to_owned(),
                            ),
                        ))
                    }
                    // byte
                    frame.op_stack.push(StackFrameValue::Byte(value.unwrap() as i8));
                }
                9 => {
                    if value.is_none() {
                       return Err(Throwable::Exception(
                            crate::common::error::Exception::NullPointer(
                                "null pointer exception".to_owned(),
                            ),
                        ))
                    }
                    // short
                    frame.op_stack.push(StackFrameValue::Short(value.unwrap() as i16));
                }
                10 => {
                    if value.is_none() {
                       return Err(Throwable::Exception(
                            crate::common::error::Exception::NullPointer(
                                "null pointer exception".to_owned(),
                            ),
                        ))
                    }
                    // int
                    frame.op_stack.push(StackFrameValue::Int(value.unwrap() as i32));
                }
                11 => {
                    if value.is_none() {
                       return Err(Throwable::Exception(
                            crate::common::error::Exception::NullPointer(
                                "null pointer exception".to_owned(),
                            ),
                        ))
                    }
                    // long
                    frame.op_stack.push(StackFrameValue::Long(value.unwrap() as i64));
                }
                12 => {
                    if value.is_none() {
                       frame
                        .op_stack
                        .push(StackFrameValue::Null);
                    }else{
                        frame
                            .op_stack
                            .push(StackFrameValue::Reference(value.unwrap() as u32));
                        }
                }
                _ => panic!("wrong atype"),
            }
        }
        _ => panic!(),
    }
    frame.pc += 1;
    Ok(())
}
