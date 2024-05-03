use std::f32::consts::E;

use log::info;

use crate::class::ConstantPoolInfo;

use crate::object::Object;
use crate::runtime_data_area::create_object;
use crate::runtime_data_area::get_constant_pool_str;
use crate::stack_frame::StackFrame;
use crate::u8c::*;


use crate::value::value::StackFrameValue;
use crate::runtime_data_area::get_class_name;
use crate::runtime_data_area::get_or_load_class;
extern crate log;
extern crate env_logger;

pub fn ldc(frame: &mut StackFrame) {
    let index = frame.code[frame.pc + 1];
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);
    let constant_pool_data = this_class.constant_pool.get(&(index as u16)).unwrap();
    match constant_pool_data {
        ConstantPoolInfo::Float(f) =>{
            frame.op_stack.push(StackFrameValue::Float(*f));
        }
        ConstantPoolInfo::Utf8(_) => todo!(),
        ConstantPoolInfo::Integer(i) => {
            frame.op_stack.push(StackFrameValue::Int(*i));
        }
        ConstantPoolInfo::Long(_) => todo!(),
        ConstantPoolInfo::Double(_) => todo!(),
        ConstantPoolInfo::Class(_) => todo!(),
        ConstantPoolInfo::String(index) => {
            let utf8_constant  = this_class.constant_pool.get(index).unwrap();
            match utf8_constant {
                ConstantPoolInfo::Utf8(str) =>{
                let str_obj = get_constant_pool_str(str);
                  if str_obj != None {
                    frame.op_stack.push(StackFrameValue::Reference(*str_obj.unwrap()))
                  }else {
                    let class_name = String::from("java/lang/String");
                    let class = get_or_load_class(&class_name);
                    let obj = create_object(class.id);
                    frame.op_stack.push(StackFrameValue::Reference(obj));
                  }
                }
                _=> panic!()
            }
        },
        ConstantPoolInfo::Fieldref(_, _) => todo!(),
        ConstantPoolInfo::Methodref(_, _) => todo!(),
        ConstantPoolInfo::InterfaceMethodref(_, _) => todo!(),
        ConstantPoolInfo::NameAndType(_, _) => todo!(),
        ConstantPoolInfo::MethodHandle(_, _) => todo!(),
        ConstantPoolInfo::MethodType(_) => todo!(),
        ConstantPoolInfo::InvokeDynamic(_, _) => todo!(),
        ConstantPoolInfo::Module(_) => todo!(),
        ConstantPoolInfo::Package(_) => todo!(),
        ConstantPoolInfo::MethodPointer(_, _) => todo!(),
        ConstantPoolInfo::InvokeStaticDynamic(_, _) => todo!(),
        ConstantPoolInfo::BootstrapMethod(_, _) => todo!(),
        ConstantPoolInfo::MethodTypeReference(_) => todo!(),
    }
    //frame.op_stack.push(frame.local.get(1).unwrap().clone());
    frame.pc += 2;
}

pub fn ldc_w(frame: &mut StackFrame) {
    frame.op_stack.push(frame.local.get(1).unwrap().clone());
    frame.pc += 1;
}

pub fn ldc2_w(frame: &mut StackFrame) {
    let index = u8s_to_u16(&frame.code[frame.pc + 1 .. frame.pc + 3]);
    let class_name = get_class_name(&frame.class);
    let this_class = get_or_load_class(&class_name);
    let constant_pool_data = this_class.constant_pool.get(&(index as u16)).unwrap();
    match constant_pool_data {
        ConstantPoolInfo::Long(l) => {
            frame.op_stack.push(StackFrameValue::Long(*l));
        }
        ConstantPoolInfo::Double(d) => {
            frame.op_stack.push(StackFrameValue::Double(*d));
        }
        _=> panic!()
    }
    frame.pc += 3;

}
