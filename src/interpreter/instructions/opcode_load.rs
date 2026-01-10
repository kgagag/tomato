

extern crate env_logger;
extern crate log;
use log::info;

use crate::{common::{error::Throwable, stack_frame::StackFrame, value::StackFrameValue}, utils::u8c::{u32_tuple_to_f64, u32_tuple_to_i64}};
pub fn aload_1(frame: &mut StackFrame) ->Result<(),Throwable>{
    frame.op_stack.push(frame.local[1]);
    frame.pc += 1;
    Ok(())
}

pub fn aload_2(frame: &mut StackFrame) ->Result<(),Throwable>{
    frame.op_stack.push(frame.local[2]);
    frame.pc += 1;
    Ok(())
}

pub fn aload_3(frame: &mut StackFrame) ->Result<(),Throwable>{
    frame.op_stack.push(frame.local[3]);
    frame.pc += 1;
    Ok(())
}

pub fn aload_0(frame: &mut StackFrame) ->Result<(),Throwable>{
    frame.op_stack.push(frame.local[0]);
    frame.pc += 1;
    Ok(())
}

pub fn iload(frame: &mut StackFrame) ->Result<(),Throwable>{
    let index = frame.code[frame.pc + 1] as usize;
    frame.op_stack.push(frame.local[index]);
    frame.pc += 2;
    Ok(())
}

pub fn fload(frame: &mut StackFrame) ->Result<(),Throwable>{
    let index = frame.code[frame.pc + 1] as usize;
    //info!("{:?}",frame);
    frame.op_stack.push(frame.local[index]);
    frame.pc += 2;
    Ok(())
}

pub fn aload(frame: &mut StackFrame) ->Result<(),Throwable>{
    let index = frame.code[frame.pc + 1] as usize;
    frame.op_stack.push(frame.local[index]);
    frame.pc += 2;
    Ok(())
}

pub fn dload(frame: &mut StackFrame) ->Result<(),Throwable>{
    let index = frame.code[frame.pc + 1] as usize;
    let v1 = frame.local[index];
    let v2: StackFrameValue = frame.local[index + 1];

    let u1 = match v1 {
        StackFrameValue::Byte(l) => l as u32,
        StackFrameValue::Char(l) => l as u32,
        StackFrameValue::Int(l) => l as u32,
        StackFrameValue::Short(l) => l as u32,
        StackFrameValue::U32(l) => l,
        _ => panic!(),
    };
    let u2 = match v2 {
        StackFrameValue::Byte(l) => l as u32,
        StackFrameValue::Char(l) => l as u32,
        StackFrameValue::Int(l) => l as u32,
        StackFrameValue::Short(l) => l as u32,
        StackFrameValue::U32(l) => l,
        _ => panic!(),
    };

    let d = StackFrameValue::Double(u32_tuple_to_f64((u1, u2)));
    // info!("{:?}",d);
    frame.op_stack.push(d);
    frame.pc += 2;
    Ok(())
}

pub fn lload(frame: &mut StackFrame) ->Result<(),Throwable>{
    let index = frame.code[frame.pc + 1] as usize;
    let v1 = frame.local[index];
    let v2: StackFrameValue = frame.local[index + 1];
    let u1 = match v1 {
        StackFrameValue::Byte(l) => l as u32,
        StackFrameValue::Char(l) => l as u32,
        StackFrameValue::Int(l) => l as u32,
        StackFrameValue::Short(l) => l as u32,
        StackFrameValue::U32(l) => l as u32,
        _ => panic!(),
    };
    let u2 = match v2 {
        StackFrameValue::Byte(l) => l as u32,
        StackFrameValue::Char(l) => l as u32,
        StackFrameValue::Int(l) => l as u32,
        StackFrameValue::Short(l) => l as u32,
        StackFrameValue::U32(l) => l as u32,
        _ => panic!(),
    };
    let d = StackFrameValue::Long(u32_tuple_to_i64((u1, u2)));
    // info!("{:?}",d);
    frame.op_stack.push(d);
    frame.pc += 2;
    Ok(())
}
pub fn iload_0(frame: &mut StackFrame) ->Result<(),Throwable>{
    frame.op_stack.push(frame.local[0]);
    frame.pc += 1;
    Ok(())
}

pub fn iload_1(frame: &mut StackFrame) ->Result<(),Throwable>{
    let v = frame.local[1];
    frame.op_stack.push(v);
    frame.pc += 1;
    Ok(())
}

pub fn iload_2(frame: &mut StackFrame) ->Result<(),Throwable>{
    frame.op_stack.push(frame.local[2]);
    frame.pc += 1;
    Ok(())
}

pub fn iload_3(frame: &mut StackFrame) ->Result<(),Throwable>{
    frame.op_stack.push(frame.local[3]);
    frame.pc += 1;
    Ok(())
}

pub fn fload_0(frame: &mut StackFrame) ->Result<(),Throwable>{
    frame.op_stack.push(frame.local[0]);
    frame.pc += 1;
    Ok(())
}

pub fn fload_1(frame: &mut StackFrame) ->Result<(),Throwable>{
    frame.op_stack.push(frame.local[1]);
    frame.pc += 1;
    Ok(())
}

pub fn fload_2(frame: &mut StackFrame) ->Result<(),Throwable>{
    frame.op_stack.push(frame.local[2]);
    frame.pc += 1;
    Ok(())
}

pub fn fload_3(frame: &mut StackFrame) ->Result<(),Throwable>{
    frame.op_stack.push(frame.local[3]);
    frame.pc += 1;
    Ok(())
}

pub fn lload_0(frame: &mut StackFrame) ->Result<(),Throwable>{
    let v1 = frame.local[0];
    let v2 = frame.local[1];
    match v1 {
        StackFrameValue::U32(u1) => match v2 {
            StackFrameValue::U32(u2) => {
                frame
                    .op_stack
                    .push(StackFrameValue::Long(u32_tuple_to_i64((u1, u2))));
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
    frame.pc += 1;
    Ok(())
}

pub fn lload_1(frame: &mut StackFrame) ->Result<(),Throwable>{
    let v1 = frame.local[1];
    let v2 = frame.local[2];
    match v1 {
        StackFrameValue::U32(u1) => match v2 {
            StackFrameValue::U32(u2) => {
                frame
                    .op_stack
                    .push(StackFrameValue::Long(u32_tuple_to_i64((u1, u2))));
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
    frame.pc += 1;
    Ok(())
}

pub fn lload_2(frame: &mut StackFrame) ->Result<(),Throwable>{
    let v1 = frame.local[2];
    let v2 = frame.local[3];
    match v1 {
        StackFrameValue::U32(u1) => match v2 {
            StackFrameValue::U32(u2) => {
                frame
                    .op_stack
                    .push(StackFrameValue::Long(u32_tuple_to_i64((u1, u2))));
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
    frame.pc += 1;
    Ok(())
}

pub fn lload_3(frame: &mut StackFrame) ->Result<(),Throwable>{
   let v1 = frame.local[3];
    let v2 = frame.local[4];
    match v1 {
        StackFrameValue::U32(u1) => match v2 {
            StackFrameValue::U32(u2) => {
                frame
                    .op_stack
                    .push(StackFrameValue::Long(u32_tuple_to_i64((u1, u2))));
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
    frame.pc += 1;
    Ok(())
}

pub fn dload_0(frame: &mut StackFrame) ->Result<(),Throwable>{
    let v1 = frame.local[0];
    let v2 = frame.local[1];
    match v1 {
        StackFrameValue::U32(u1) => match v2 {
            StackFrameValue::U32(u2) => {
                frame
                    .op_stack
                    .push(StackFrameValue::Double(u32_tuple_to_f64((u1, u2))));
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
    frame.pc += 1;
    Ok(())
}

pub fn dload_1(frame: &mut StackFrame) ->Result<(),Throwable>{
    let v1 = frame.local[1];
    let v2 = frame.local[2];
    match v1 {
        StackFrameValue::U32(u1) => match v2 {
            StackFrameValue::U32(u2) => {
                frame
                    .op_stack
                    .push(StackFrameValue::Double(u32_tuple_to_f64((u1, u2))));
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
    frame.pc += 1;
    Ok(())
}

pub fn dload_2(frame: &mut StackFrame) ->Result<(),Throwable>{
    let v1 = frame.local[2];
    let v2 = frame.local[3];
    //info!("{:?}",frame);
    match v1 {
        StackFrameValue::U32(u1) => match v2 {
            StackFrameValue::U32(u2) => {
                frame
                    .op_stack
                    .push(StackFrameValue::Double(u32_tuple_to_f64((u1, u2))));
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
    //info!("{:?}",frame);
    frame.pc += 1;
    Ok(())
}

pub fn dload_3(frame: &mut StackFrame) ->Result<(),Throwable>{
    let v1 = frame.local[3];
    let v2 = frame.local[4];
    match v1 {
        StackFrameValue::U32(u1) => match v2 {
            StackFrameValue::U32(u2) => {
                frame
                    .op_stack
                    .push(StackFrameValue::Double(u32_tuple_to_f64((u1, u2)) as f64));
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
    frame.pc += 1;
    Ok(())
}
