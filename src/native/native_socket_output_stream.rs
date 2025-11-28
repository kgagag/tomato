
use std::{io::Write, net::TcpStream, sync::Arc};

use log::{info, warn};
use zip::unstable::stream;

use crate::{common::{reference::Reference, stack_frame::StackFrame, value::StackFrameValue}, runtime::runtime_data_area::{self, get_reference}};
pub fn write0( frame: &mut StackFrame) {
    //info!("write .....");
    let reference_id = frame.pop_reference();    
    let reference = get_reference(&reference_id).unwrap();
    let mut bytes:Vec<u8> = Vec::new();
    match reference {
        Reference::Array(array) =>{
            for i in 0.. array.data.len(){
               let sfv = array.data.get(i).unwrap();
               match sfv {
                   StackFrameValue::Byte(b) =>{
                        bytes.push(*b as u8);
                   }
                   _=> panic!()
               }
            }
        }
        _=> panic!()
    }
    let java_stream = frame.local.get(0).unwrap();
    match java_stream {
        StackFrameValue::Reference(id) =>{
            let java_stream_referencte = get_reference(id).unwrap();
            match java_stream_referencte {
                Reference::Object(object) =>{
                   let fdsfv =  object.field.get("fd").unwrap();
                   match fdsfv {
                       StackFrameValue::Int(fd) =>{
                        let data: std::sync::MutexGuard<'_, std::collections::HashMap<u64, TcpStream>> = runtime_data_area::TCP_CONNECT.lock().unwrap();
                        let mut stream = data.get(id).unwrap();
                         let _ = stream.write_all(&bytes).unwrap();
                         let _ = stream.flush().unwrap();
                         drop(data)
                       }
                       _=> panic!()
                   }
                }
                _=> panic!()
            }
        }
        _=> panic!()
    }
}


pub fn close0( frame: &mut StackFrame){
    let java_stream = frame.local.get(0).unwrap();
    match java_stream {
        StackFrameValue::Reference(id) =>{
            let java_stream_referencte = get_reference(id).unwrap();
            match java_stream_referencte {
                Reference::Object(object) =>{
                   let fdsfv =  object.field.get("fd").unwrap();
                   match fdsfv {
                       StackFrameValue::Int(fd) =>{
                          runtime_data_area::close_tcp(&(*fd as u64));
                       }
                       _=> panic!()
                   }
                }
                _=> panic!()
            }
        }
        _=> panic!()
    }
}

