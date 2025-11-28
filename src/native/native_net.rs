use std::{f32::consts::E, io::Read, net::TcpListener};

use log::{error, info};

use crate::{
    common::{reference::Reference, stack_frame::StackFrame, value::StackFrameValue},
    runtime::runtime_data_area::{self, get_reference},
};

pub fn accept(frame: &mut StackFrame) {
    let sfv = frame.op_stack.pop().unwrap();
    let (bind_ref_id, reference_id) = match sfv {
        StackFrameValue::Reference(id) => {
            let reference = get_reference(&id).unwrap();
            match reference {
                Reference::Object(object) => {
                    let bind_ref = object.field.get("bind").unwrap().clone();
                    match bind_ref {
                        StackFrameValue::Reference(bind_id) => (bind_id, id),
                        _ => panic!(),
                    }
                },
                _ => panic!(),
            }
        }
        _ => panic!(),
    };

    let (address_ref_id, port_value) = {
        let reference = get_reference(&bind_ref_id).unwrap();
        match reference {
            Reference::Object(object) => {
                let address_ref = object.field.get("address").unwrap().clone();
                let port_val = object.field.get("port").unwrap().clone();
                (address_ref, port_val)
            },
            _ => panic!(),
        }
    };

    let mut url = String::new();

    match address_ref_id {
        StackFrameValue::Reference(id) => {
            let reference = get_reference(&id).unwrap();
            match reference {
                Reference::Array(array) => {
                    for (i, op) in array.data.iter().enumerate() {
                        match op {
                            StackFrameValue::Int(p) => {
                                url.push_str(&p.to_string());
                            }
                            StackFrameValue::Short(p) => {
                                url.push_str(&p.to_string());
                            }
                            StackFrameValue::Byte(p) => {
                                url.push_str(&p.to_string());
                            }
                            _ => panic!(),
                        }
                        if i != array.data.len() - 1 {
                            url.push('.');
                        }
                    }
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }

    url.push(':');
    match port_value {
        StackFrameValue::Int(p) => {
            url.push_str(&p.to_string());
        }
        StackFrameValue::Short(p) => {
            url.push_str(&p.to_string());
        }
        _ => panic!(),
    };

    let listener = TcpListener::bind(&url);
    match listener {
        Ok(tcp) => {
            for stream in tcp.incoming() {
                match stream {
                    Ok(stream) => {
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