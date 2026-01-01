use crate::{common::{reference::Reference, stack_frame::StackFrame, value::StackFrameValue}, runtime::runtime_data_area::{self, get_reference}};

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
                          runtime_data_area::close_tcp(&(*fd as u32));
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