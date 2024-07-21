use log::info;

use crate::common::value::StackFrameValue;

use super::java;



pub fn dprint(msg: StackFrameValue) {
   let rust_string =  java::convert_to_rust_string(msg);
   if rust_string.is_some(){
    println!("{}", rust_string.unwrap());
   }else {
    println!("{}", "null");
   }
}
