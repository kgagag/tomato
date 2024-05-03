use crate::stack_frame::StackFrame;
extern crate env_logger;
extern crate log;
use crate::class::*;
use crate::runtime_data_area::*;


pub fn putstatic(frame: &mut StackFrame) {
    let index: u16 = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);
    let field_ref = this_class.constant_pool.get(&index).expect("Field reference not found");

    if let ConstantPoolInfo::Fieldref(class_index, name_and_type_index) = field_ref {
        let class_name_utf8 = match this_class.constant_pool.get(&class_index) {
            Some(ConstantPoolInfo::Class(class_name_index)) => this_class.constant_pool.get(class_name_index),
            _ => panic!(),
        }.expect("Class name UTF-8 not found");

        if let ConstantPoolInfo::Utf8(class_name) = class_name_utf8 {
            let target_class = get_or_load_class(&class_name);
            let name_and_type = this_class.constant_pool.get(name_and_type_index).expect("Name and type not found");

            if let ConstantPoolInfo::NameAndType(name_index, _) = name_and_type {
                let field_name_utf8 = this_class.constant_pool.get(name_index).expect("Field name UTF-8 not found");

                if let ConstantPoolInfo::Utf8(field_name) = field_name_utf8 {
                    let field = target_class.field_info.get_mut(field_name).unwrap();
                    field.value = frame.op_stack.pop().expect("Stack underflow");
                } else {
                    panic!("Unexpected constant pool info type");
                }
            } else {
                panic!("Unexpected name and type");
            }
        } else {
            panic!("Unexpected class name UTF-8");
        }
    } else {
        panic!("Unexpected field reference");
    }
    frame.pc += 3;
}

pub fn getstatic(frame: &mut StackFrame) {
    let index: u16 = u16::from_be_bytes([frame.code[frame.pc + 1], frame.code[frame.pc + 2]]);
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);
    let field_ref = this_class.constant_pool.get(&index).expect("Field reference not found");

    if let ConstantPoolInfo::Fieldref(class_index, name_and_type_index) = field_ref {
        let class_name_utf8 = match this_class.constant_pool.get(&class_index) {
            Some(ConstantPoolInfo::Class(class_name_index)) => this_class.constant_pool.get(class_name_index),
            _ => panic!(),
        }.expect("Class name UTF-8 not found");

        if let ConstantPoolInfo::Utf8(class_name) = class_name_utf8 {
            let target_class = get_or_load_class(&class_name);
            let name_and_type = this_class.constant_pool.get(name_and_type_index).expect("Name and type not found");

            if let ConstantPoolInfo::NameAndType(name_index, _) = name_and_type {
                let field_name_utf8 = this_class.constant_pool.get(name_index).expect("Field name UTF-8 not found");

                if let ConstantPoolInfo::Utf8(field_name) = field_name_utf8 {
                    let field = target_class.field_info.get(field_name).unwrap();
                    frame.op_stack.push(field.value.clone());
                } else {
                    panic!("Unexpected constant pool info type");
                }
            } else {
                panic!("Unexpected name and type");
            }
        } else {
            panic!("Unexpected class name UTF-8");
        }
    } else {
        panic!("Unexpected field reference");
    }
    frame.pc += 3;
}
