use std::{f32::consts::E, io::Read, net::TcpListener};

use log::{error, info};

use crate::{classloader::class_loader, common::{array, object, reference::{self, Reference}, stack_frame::StackFrame, value::StackFrameValue}, runtime::runtime_data_area::{self, get_reference}};

pub fn accept(frame: &mut StackFrame) {
    let sfv = frame.op_stack.pop().unwrap();
    //创建一个链接,返回一个socket对象
    let bind = match sfv {
        StackFrameValue::Reference(id) =>{
           let reference =  get_reference(&id).unwrap();
           match reference {
               Reference::Object(object) =>{
                    object.field.get("bind").unwrap()
               }
               _=> panic!()
           }
        }
        _=> panic!()
    };
    let (address,port) = 
    match bind {
        StackFrameValue::Reference(id) =>{
           let reference =  get_reference(id).unwrap();
           match reference {
               Reference::Object(object) => {
                    (object.field.get("address").unwrap(),object.field.get("port").unwrap())
               }
               _=> panic!()
           }
        }
        _=> panic!()
    };

    let mut url = String::from("");

    match address {
        StackFrameValue::Reference(id) =>{
            let reference = get_reference(id).unwrap();
            match reference {
                Reference::Array(array) =>{
                    for i in 0 .. array.data.len(){
                       let op: &StackFrameValue = array.data.get(i).unwrap();
                       info!("{:?}",op);
                       match op {
                            StackFrameValue::Int(p) =>{
                                url.push_str(p.to_string().as_str());
                                if i != array.data.len() - 1 {
                                    url.push('.');
                                }
                            }
                            StackFrameValue::Short(p) =>{
                                url.push_str(p.to_string().as_str());
                                if i != array.data.len() - 1 {
                                    url.push('.');
                                }
                            }
                            StackFrameValue::Byte(p) =>{
                                url.push_str(p.to_string().as_str());
                                if i != array.data.len() - 1 {
                                    url.push('.');
                                }
                            }
                            _=> panic!()
                       }
                    }
                }
                _=> panic!()
            }
        }
        _=> panic!()
    }

    url.push(':');
    match port {
        StackFrameValue::Int(p) =>{
            url.push_str(p.to_string().as_str());
        }
        StackFrameValue::Short(p) =>{
            url.push_str(p.to_string().as_str());
        }
        _=> panic!()
    };

    info!("{:?}",url);
    let listener = TcpListener::bind(url);
    match listener {
        Ok(tcp) => {
            for stream in tcp.incoming() {
                match stream {
                    Ok(stream) =>{
                      let class =  runtime_data_area::get_or_load_class(&String::from("tomato/net/Socket"));
                      let socket_object = runtime_data_area::create_object(class.id);
                      break;
                    },
                    Err(e) =>{
                        
                    }
                }
            }
        }
        Err(err) => {
            error!("{:?}",err);
        }
    }
 }