use std::array;

use log::info;

use crate::{
    common::{
        param::DataType,
        reference::{self, Reference},
        stack_frame::{self, StackFrame},
        value::StackFrameValue,
    },
    runtime::runtime_data_area::{self, get_reference},
    utils::u8c,
};

/**
 * char数组转换成byte数组
 */
pub fn encode0(frame: &mut StackFrame) {
    let _len = frame.popi64();
    let _off = frame.popi64();
    let reference_id = frame.pop_reference();
    let reference = get_reference(&reference_id).unwrap();
    let mut new_len = 0;
    match reference {
        Reference::Array(array) => {
          //  info!("{:?}", array);
            let mut bytes: Vec<StackFrameValue> = Vec::new();
            for i in 0..array.data.len() {
                let sfv = array.data.get(i).unwrap();
                //info!("{:?}", sfv);
                match sfv {
                    StackFrameValue::CHARACTER(ch) => {
                        let items = u8c::char_to_bytes(*ch);
                        new_len += items.len();
                        for j in 0..items.len() {
                            bytes.push(StackFrameValue::Byte(*items.get(j).unwrap() as i8))
                        }
                    }
                    _ => panic!(),
                }
            }
            let new_array_id: u64 = runtime_data_area::create_array(
                new_len as u32,
                DataType::Array {
                    element_type: Box::new(DataType::Byte),
                    depth: (1),
                },
            );
            let reference = get_reference(&new_array_id).unwrap();
            //info!("{:?}", bytes);
            match reference {
                Reference::Array(array) => {
                    array.data = bytes;
                }
                _ => panic!(),
            }
            frame
                .op_stack
                .push(StackFrameValue::Reference(new_array_id))
        }
        _ => panic!(),
    }
}

/**
     * char数组转换成byte数组
     */
    pub fn decode0(frame: &mut StackFrame) {
        let _len = frame.popi64();
        let _off = frame.popi64();
        let reference_id = frame.pop_reference();
        let reference = get_reference(&reference_id).unwrap();
        match reference {
            Reference::Array(array) => {
               // info!("{:?}", array);
                let mut chars: Vec<StackFrameValue> = Vec::new();
                let mut tmp : Vec<u8> = Vec::new();
                for i in 0..array.data.len() {
                    let sfv = array.data.get(i).unwrap();
                    //info!("{:?}",sfv);
                    match sfv {
                        StackFrameValue::Byte(b) => {
                            tmp.push(*b as u8);
                        }
                        StackFrameValue::Int(b) => {
                            tmp.push(*b as u8);
                        }
                        _ => panic!(),
                    }
                }
                let char_vec = u8c::bytes_to_chars(tmp);
                for i in 0.. char_vec.len(){
                    chars.push(StackFrameValue::CHARACTER(*char_vec.get(i).unwrap()))
                }
                let new_array_id: u64 = runtime_data_area::create_array(
                    char_vec.len() as u32,
                    DataType::Array {
                        element_type: Box::new(DataType::Char),
                        depth: (1),
                    },
                );
                let reference = get_reference(&new_array_id).unwrap();
                match reference {
                    Reference::Array(array) => {
                        array.data = chars;
                    }
                    _ => panic!(),
                }
                frame
                    .op_stack
                    .push(StackFrameValue::Reference(new_array_id))
            }
            _ => panic!(),
        }
    }
