use std::{f32::consts::E, io::Read, net::TcpListener};

use log::{error, info};

use crate::{
    common::{reference::Reference, stack_frame::StackFrame, value::StackFrameValue}};

pub fn accept(frame: &mut StackFrame) {
    let sfv = frame.op_stack.pop().unwrap();
    let (bind, reference_id) = match sfv {
        StackFrameValue::Reference(id) => {
            let reference = get_reference(&id).unwrap();
            match reference {
                Reference::Object(object) => (
                    object.field.get("bind").unwrap(),
                    id
                ),
                _ => panic!(),
            }
        }
        _ => panic!(),
    };

    let (address, port) = match bind {
        StackFrameValue::Reference(id) => {
            let reference = get_reference(id).unwrap();
            match reference {
                Reference::Object(object) => (
                    object.field.get("address").unwrap(),
                    object.field.get("port").unwrap(),
                ),
                _ => panic!(),
            }
        }
        _ => panic!(),
    };

    let mut url = String::from("");
    match address {
        StackFrameValue::Reference(id) => {
            let reference = get_reference(id).unwrap();
            match reference {
                Reference::Array(array) => {
                    for i in 0..array.data.len() {
                        let op: &StackFrameValue = array.data.get(i).unwrap();
                        match op {
                            StackFrameValue::Int(p) => {
                                url.push_str(p.to_string().as_str());
                                if i != array.data.len() - 1 {
                                    url.push('.');
                                }
                            }
                            StackFrameValue::Short(p) => {
                                url.push_str(p.to_string().as_str());
                                if i != array.data.len() - 1 {
                                    url.push('.');
                                }
                            }
                            StackFrameValue::Byte(p) => {
                                url.push_str(p.to_string().as_str());
                                if i != array.data.len() - 1 {
                                    url.push('.');
                                }
                            }
                            _ => panic!(),
                        }
                    }
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }

    url.push(':');
    match port {
        StackFrameValue::Int(p) => {
            url.push_str(p.to_string().as_str());
        }
        StackFrameValue::Short(p) => {
            url.push_str(p.to_string().as_str());
        }
        _ => panic!(),
    };
    let listener = TcpListener::bind(url);
    match listener {
        Ok(tcp) => {
            for stream in tcp.incoming() {
                match stream {
                    Ok(stream) => {
                       // info!("{:?}","connected..");
                        runtime_data_area::put_tcp(&reference_id, stream);
                        break;
                    }
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            }
        }
        Err(err) => {
            error!("{:?}", err);
        }
    }
}
