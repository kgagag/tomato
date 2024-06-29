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
    let branch_offset = u8s_to_i16(&frame.code[frame.pc + 1.. frame.pc + 3]) as i32;
    if value == 0 {
        frame.pc = (frame.pc as i32 + branch_offset ) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}

pub fn ifne(frame: &mut StackFrame) {
    let value = frame.popi64();
    let branch_offset = u8s_to_i16(&frame.code[frame.pc + 1.. frame.pc + 3]) as i16;
    if value != 0 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}

pub fn iflt(frame: &mut StackFrame) {
    let value = frame.popi64();
    let branch_offset = u8s_to_i16(&frame.code[frame.pc + 1.. frame.pc + 3]);
    if value < 0 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}


pub fn ifge(frame: &mut StackFrame) {
    let value = frame.popi64();
    let branch_offset = u8s_to_i16(&frame.code[frame.pc + 1.. frame.pc + 3]);
    if value >= 0 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}


pub fn ifgt(frame: &mut StackFrame) {
    let value = frame.popi64();
    let branch_offset = u8s_to_i16(&frame.code[frame.pc + 1.. frame.pc + 3]);
    if value > 0 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}

pub fn ifle(frame: &mut StackFrame) {
    let value = frame.popi64();
    let branch_offset = u8s_to_i16(&frame.code[frame.pc + 1.. frame.pc + 3]);
    if value <= 0 {
        frame.pc = (frame.pc as i32 + branch_offset as i32) as usize;
    } else {
        frame.pc += 3; // 跳转失败，继续执行下一条指令
    }
}


pub fn if_icmpeq(frame: &mut StackFrame) {
    let value2 = frame.popi64();
    let value1 = frame.popi64();
    let branch_offset = u8s_to_i16(&frame.code[frame.pc + 1.. frame.pc + 3]);
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
    // info!("{:?}",value);
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


/**
 * gpt4o 实现的 lookupswitch
 */
pub fn lookupswitch(frame: &mut StackFrame) {
    let pc0 = frame.pc;
    // 确保PC是4字节对齐的
    while frame.pc % 4 != 0 {
        frame.pc += 1;
    }

    // 获取默认跳转地址
    let default_offset = u8s_to_i32(&frame.code[frame.pc..frame.pc + 4]);
    frame.pc += 4;

    // 获取匹配的键值对数量
    let npairs = u8s_to_i32(&frame.code[frame.pc..frame.pc + 4]);
    frame.pc += 4;

    // 弹出栈顶的key
    let key = frame.popi64() as i32;

    for _ in 0..npairs {
        let match_key = u8s_to_i32(&frame.code[frame.pc..frame.pc + 4]);
        let match_offset = u8s_to_i32(&frame.code[frame.pc + 4..frame.pc + 8]);
        //frame.pc += 8;
        if key == match_key {
            frame.pc = (pc0 as i32 + match_offset) as usize;
            return;
        }
    }

    // 没有匹配，跳转到默认地址
    frame.pc = (frame.pc as i32 + default_offset) as usize;
}


/**
 *  * gpt4o 实现的 lookupswitch
 */
pub fn tableswitch(frame: &mut StackFrame) {
    let pc0 = frame.pc;

    // 确保PC是4字节对齐的
    frame.pc = (frame.pc + 3) & !3;

    // 获取默认跳转地址
    let default_offset = u8s_to_i32(&frame.code[frame.pc..frame.pc + 4]);
    frame.pc += 4;

    // 获取最低值和最高值
    let low = u8s_to_i32(&frame.code[frame.pc..frame.pc + 4]);
    frame.pc += 4;
    let high = u8s_to_i32(&frame.code[frame.pc..frame.pc + 4]);
    frame.pc += 4;

    // 计算跳转表的长度
    let jump_table_length = (high - low + 1) as usize;

    // 弹出栈顶的key
    let key = frame.popi64() as i32;

    if key < low || key > high {
        // 如果key不在范围内，跳转到默认地址
        frame.pc = (pc0 as i32 + default_offset) as usize;
    } else {
        // 在跳转表中查找跳转偏移量
        let index = (key - low) as usize;
        let offset = u8s_to_i32(&frame.code[frame.pc + index * 4..frame.pc + (index + 1) * 4]);
        frame.pc = (pc0 as i32 + offset) as usize;
    }
}


// 辅助函数：将四个 u8 转换为 i32
fn u8s_to_i32(bytes: &[u8]) -> i32 {
    ((bytes[0] as i32) << 24)
        | ((bytes[1] as i32) << 16)
        | ((bytes[2] as i32) << 8)
        | (bytes[3] as i32)
}


