use log::{Metadata, info};

use crate::{classfile::class::{self, Class}, classloader::class_loader, common::{error::{JvmError, Throwable}, param::DataType, stack_frame::StackFrame, value::{self, StackFrameValue}}, runtime::{heap::{self, Heap}, metaspace::{self, Metaspace}}};

pub fn create_class_object(class_name: &String,vm_stack:&mut Vec<StackFrame>,heap:&mut Heap,metaspace:&mut Metaspace) -> Result<u32,Throwable> {
    let id = create_string_object(class_name.clone(),vm_stack,heap,metaspace)?;
    let class_name = &String::from("java/lang/Class");
    let class0 =class_loader::find_class(class_name, vm_stack, heap, metaspace)?;
    let obj_id: u32 = heap.create_object(class0) as u32;
    for(key,v) in &class0.field_info {
       if key == "name" {
           heap.put_field_reference(obj_id,  v.offset, id);
           break;
       }
    }
    heap.put_into_class_constant_pool(class_name.clone(), obj_id as u32);
    Ok(obj_id as u32)
}

pub fn convert_to_rust_string(msg: StackFrameValue,vm_stack:&mut Vec<StackFrame>,heap:&mut Heap,metaspace:&mut Metaspace) ->Result<String,Throwable>{
    let mut string: String = String::from("");
    match msg {
        StackFrameValue::Reference(id) => {
            let class_id = heap.get_object_class_id(id as usize)?;
            let class: &Class = &metaspace.classes[class_id as usize];
            for (key,field) in &class.field_info {
                if key == "value" {
                  let str_value_array =  heap.get_field_ptr(id, field.offset);
                  if str_value_array.is_none() {
                      return Err(Throwable::Exception(crate::common::error::Exception::NullPointer("java.lang.NullPointerException".to_string())));
                  }
                  let len = heap.get_array_length(str_value_array.unwrap()) as usize;
                  //info!("===={}.{}.{}.{}=====",id,str_value_array,field.offset,len);
                  for i in 0..len {
                      let (_atype,value) = heap.get_array_element(str_value_array.unwrap(), i);
                      string.push(char::from_u32(value.unwrap() as u32).unwrap());
                  } 
                }
            }
        }
       // _=> panic!("create_string_object error")
       _=> return Err(Throwable::Error(JvmError::InternalError("Internal error".to_string()))),
    }
    return Ok(string);
}


pub fn create_string_object(str_value: String,vm_stack:&mut Vec<StackFrame>,heap:&mut Heap,metaspace:&mut Metaspace) -> Result<u32,Throwable> {
    let char_array_id = {
        let chars: Vec<char> = str_value.chars().collect();
       // print!("{:?}",chars);
        let char_array_id =  heap.create_basic_array(5, chars.len() as u32, 1);
        for (i, c) in chars.iter().enumerate() {
            let value = StackFrameValue::CHARACTER(*c);
            heap.put_array_element(char_array_id as u32, i, value::number_u64(&value));
        }
        char_array_id
    };
    let class_name = String::from("java/lang/String");
    let class: &mut Class = class_loader::find_class(&class_name,vm_stack,heap,metaspace)?;
    let obj_id = heap.create_object(class);
    for (key,field) in &class.field_info {
        if key == "value" {
            //info!("######{}.{}.{}.{}####",obj_id,char_array_id,field.offset,str_value.len());
            heap.put_field_reference(obj_id as u32, field.offset, char_array_id as u32 );
        }
    }
    heap.put_into_string_constant_pool(str_value, obj_id as u32);
    Ok(obj_id as u32)
}