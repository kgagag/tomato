use log::info;

use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;
extern crate env_logger;
extern crate log;
use crate::u8c::*;


pub fn fcmpg(frame: &mut StackFrame) {
    let f2 = frame.popf64();
    let f1 = frame.popf64();

    let result = match f1.partial_cmp(&f2) {
        Some(std::cmp::Ordering::Less) => -1,
        Some(std::cmp::Ordering::Equal) => 0,
        Some(std::cmp::Ordering::Greater) => 1,
        None => {
            // Handle NaN case, which is unordered
            // In Java, if either operand is NaN, the result is 1
            if f1.is_nan() || f2.is_nan() {
                1
            } else {
                // If neither operand is NaN and one is positive infinity,
                // the result is 1; otherwise, the result is -1.
                if f1.is_infinite() || f2.is_infinite() {
                    1
                } else {
                    -1
                }
            }
        }
    };

    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
}




pub fn lcmp(frame: &mut StackFrame) {
    let l2 = frame.popi64();
    let l1 = frame.popi64();
    
    let result = match l1.cmp(&l2) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };
    
    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
}

pub fn fcmpl(frame: &mut StackFrame) {
    let f2 = frame.popf64();
    let f1 = frame.popf64();

    let result = match f1.partial_cmp(&f2) {
        Some(std::cmp::Ordering::Less) => -1,
        Some(std::cmp::Ordering::Equal) => 0,
        Some(std::cmp::Ordering::Greater) => 1,
        None => {
            1
        }
    };

    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
}


pub fn dcmpg(frame: &mut StackFrame) {
    let d2 = frame.popf64();
    let d1 = frame.popf64();

    let result = match d1.partial_cmp(&d2) {
        Some(std::cmp::Ordering::Less) => -1,
        Some(std::cmp::Ordering::Equal) => 0,
        Some(std::cmp::Ordering::Greater) => 1,
        None => {
            // Handle NaN case, which is unordered
            // In Java, if either operand is NaN, the result is 1
            if d1.is_nan() || d2.is_nan() {
                1
            } else {
                // If neither operand is NaN and one is positive infinity,
                // the result is 1; otherwise, the result is -1.
                if d1.is_infinite() || d2.is_infinite() {
                    1
                } else {
                    -1
                }
            }
        }
    };

    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
}



pub fn dcmpl(frame: &mut StackFrame) {
    let d2 = frame.popf64();
    let d1 = frame.popf64();

    let result = match d1.partial_cmp(&d2) {
        Some(std::cmp::Ordering::Less) => -1,
        Some(std::cmp::Ordering::Equal) => 0,
        Some(std::cmp::Ordering::Greater) => 1,
        None => {
            // Handle NaN case, which is unordered
            // In Java, if either operand is NaN, the result is -1
            if d1.is_nan() || d2.is_nan() {
                -1
            } else {
                // If neither operand is NaN and one is positive infinity,
                // the result is 1; otherwise, the result is -1.
                if d1.is_infinite() || d2.is_infinite() {
                    1
                } else {
                    -1
                }
            }
        }
    };

    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
}


pub fn ifeq(frame: &mut StackFrame) {
    let value = frame.popi64();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);
    if value == 0 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}

pub fn ifne(frame: &mut StackFrame) {
    let value = frame.popi64();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]) as i16;
    if value != 0 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}

pub fn iflt(frame: &mut StackFrame) {
    let value = frame.popi64();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);
    if value < 0 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}


pub fn ifge(frame: &mut StackFrame) {
    let value = frame.popi64();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);
    if value >= 0 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}


pub fn ifgt(frame: &mut StackFrame) {
    let value = frame.popi64();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);
    if value > 0 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}

pub fn ifle(frame: &mut StackFrame) {
    let value = frame.popi64();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);
    if value <= 0 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}


pub fn if_icmpeq(frame: &mut StackFrame) {
    let value2 = frame.popi64();
    let value1 = frame.popi64();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);

    if value1 == value2 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}


pub fn if_icmpne(frame: &mut StackFrame) {
    let value2 = frame.popi64();
    let value1 = frame.popi64();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);
    if value1 != value2 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}


pub fn if_icmplt(frame: &mut StackFrame) {
    let value2 = frame.popi64();
    let value1 = frame.popi64();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);

    if value1 < value2 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}


pub fn if_icmpge(frame: &mut StackFrame) {
    let value2 = frame.popi64();
    let value1 = frame.popi64();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);

    if value1 >= value2 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}

pub fn if_icmpgt(frame: &mut StackFrame) {
    let value2 = frame.popi64();
    let value1 = frame.popi64();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);

    if value1 > value2 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}

pub fn if_icmple(frame: &mut StackFrame) {
    let value2 = frame.popi64();
    let value1 = frame.popi64();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);
    if value1 <= value2 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}


pub fn if_acmpeq(frame: &mut StackFrame) {
    let value2 = frame.pop_reference();
    let value1 = frame.pop_reference();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);
    if value1 == value2 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}

pub fn if_acmpne(frame: &mut StackFrame) {
    let value2 = frame.pop_reference();
    let value1 = frame.pop_reference();
    let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);

    if value1 != value2 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}

pub fn ifnonnull(frame: &mut StackFrame) {
    let value = frame.op_stack.pop().unwrap();
    //info!("{:?}",value);
    match value {
        StackFrameValue::Null =>{
            frame.pc += 3; // 跳转失败，继续执行下一条指令
        }
        _=>{
            let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);
            frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
        }
    }
}

pub fn ifnull(frame: &mut StackFrame) {
    let value = frame.op_stack.pop().unwrap();
    //info!("{:?}",value);
    match value {
        StackFrameValue::Null =>{
            let branch_offset = u8s_to_u16(&frame.code[frame.pc + 1.. frame.pc + 3]);
            frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
        }
        _=>{
            frame.pc += 3; // 跳转失败，继续执行下一条指令
        }
    }
}

