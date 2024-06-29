use std::num::Wrapping;
use log::info;
use crate::stack_frame::StackFrame;
use crate::value::value::StackFrameValue;


pub fn iadd(frame: &mut StackFrame) {
    let i1: i32 = frame.popi64() as i32;
    let i2: i32 = frame.popi64() as i32;
    let a: Wrapping<i32> = Wrapping(i2);
    let b: Wrapping<i32> = Wrapping(i1);
    let result = a + b;
    let value = StackFrameValue::Int(result.0);
    frame.op_stack.push(value);
    frame.pc += 1;
}

pub fn fadd(frame: &mut StackFrame) {
    let i1 = frame.popf64() as f32;
    let i2 = frame.popf64() as f32;
    let result = i1 + i2;
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
}

pub fn dadd(frame: &mut StackFrame) {
    let i1 = frame.popf64() ;
    let i2 = frame.popf64() ;
    let result = i1 + i2;
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
}

pub fn ladd(frame: &mut StackFrame) {
    let i1 = frame.popi64() ;
    let i2 = frame.popi64() ;
    let a: Wrapping<i64> = Wrapping(i2);
    let b: Wrapping<i64> = Wrapping(i1);
    let result = a + b;
    frame.op_stack.push(StackFrameValue::Long(result.0));
    frame.pc += 1;
}

pub fn isub(frame: &mut StackFrame) {
    let i2 = frame.popi64() as i32;
    let i1 = frame.popi64() as i32;
    let result = i1 - i2;
    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
}

pub fn fsub(frame: &mut StackFrame) {
    let f2 = frame.popf64() as f32;
    let f1 = frame.popf64() as f32;
    let result = f1 - f2;
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
}

pub fn dsub(frame: &mut StackFrame) {
    let d2 = frame.popf64() ;
    let d1 = frame.popf64() ;
    let result = d1 - d2;
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
}

pub fn lsub(frame: &mut StackFrame) {
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    let result = l1 - l2;
    frame.op_stack.push(StackFrameValue::Long(result));
    frame.pc += 1;
}

pub fn fmul(frame: &mut StackFrame) {
    let f2 = frame.popf64() as f32;
    let f1 = frame.popf64() as f32;
    let result = f1 * f2;
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
}

pub fn imul(frame: &mut StackFrame) {
    let i2 = frame.popi64() as i32;
    let i1 = frame.popi64() as i32;
    let a: Wrapping<i32> = Wrapping(i2);
    let b: Wrapping<i32> = Wrapping(i1);
    let result = a * b;
    frame.op_stack.push(StackFrameValue::Int(result.0));
    frame.pc += 1;
}

pub fn lmul(frame: &mut StackFrame) {
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    let a: Wrapping<i64> = Wrapping(l2);
    let b: Wrapping<i64> = Wrapping(l1);
    let result = a * b;
    frame.op_stack.push(StackFrameValue::Long(result.0));
    frame.pc += 1;
}

pub fn dmul(frame: &mut StackFrame) {
    let d2 = frame.popf64() ;
    let d1 = frame.popf64() ;
    let result = d1 * d2;
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
}

pub fn idiv(frame: &mut StackFrame) {
    let i2 = frame.popi64() as i32;
    let i1 = frame.popi64() as i32;
    if i2 == 0 {
        panic!()
    }
    let result = i1 / i2;
    //warn!("{}", format!("{}{}", "idiv add result:", result));
    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
}

pub fn fdiv(frame: &mut StackFrame) {
    let f2 = frame.popf64() as f32;
    let f1 = frame.popf64() as f32;
    if f2 == 0.0 {
        panic!()
    }
    let result = f1 / f2;
    //warn!("{}", format!("{}{}", "fdiv add result:", result));
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
}

pub fn ddiv(frame: &mut StackFrame) {
    let d2 = frame.popf64() ;
    let d1 = frame.popf64() ;
    if d2 == 0.0 {
        panic!()
    }
    let result = d1 - d2;
    //warn!("{}", format!("{}{}", "ddiv add result:", result));
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
}

pub fn ldiv(frame: &mut StackFrame) {
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    if l2 == 0 {
        panic!()
    }
    let result = l1 / l2;
    //warn!("{}", format!("{}{}", "ldiv add result:", result));
    frame.op_stack.push(StackFrameValue::Long(result));
    frame.pc += 1;
}

pub fn irem(frame: &mut StackFrame) {
    let l2 = frame.popi64() as i32;
    let l1 = frame.popi64() as i32;
    let result = l1 % l2;
    //warn!("{}", format!("{}{}", "ldiv add result:", result));
    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
}

pub fn frem(frame: &mut StackFrame) {
    let f2 = frame.popi64() as f32;
    let f1 = frame.popi64() as f32;
    let result = f1 % f2;
    //warn!("{}", format!("{}{}", "ldiv add result:", result));
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
}

pub fn lrem(frame: &mut StackFrame) {
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    let result = l1 % l2;
    //warn!("{}", format!("{}{}", "ldiv add result:", result));
    frame.op_stack.push(StackFrameValue::Long(result));
    frame.pc += 1;
}

pub fn drem(frame: &mut StackFrame) {
    let d2 = frame.popi64() as f64;
    let d1 = frame.popi64() as f64;
    let result = d1 % d2;
    //warn!("{}", format!("{}{}", "ldiv add result:", result));
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
}

// 取负
pub fn ineg(frame: &mut StackFrame) {
    let i = frame.popi64() as i32;
    frame.op_stack.push(StackFrameValue::Int(0 - i));
    frame.pc += 1;
}

pub fn lneg(frame: &mut StackFrame) {
    let l = frame.popi64() ;
    frame.op_stack.push(StackFrameValue::Long(0 - l));
    frame.pc += 1;
}

pub fn fneg(frame: &mut StackFrame) {
    let f = frame.popi64() as f32;
    frame.op_stack.push(StackFrameValue::Float(0.0 - f));
    frame.pc += 1;
}

pub fn dneg(frame: &mut StackFrame) {
    let d = frame.popi64() as f64;
    frame.op_stack.push(StackFrameValue::Double(0.0 - d));
    frame.pc += 1;
}

pub fn ishl(frame: &mut StackFrame) {
    let i2 = frame.popi64() as i32;
    let i1 = frame.popi64() as i32;
    frame.op_stack.push(StackFrameValue::Int(i1 << i2));
    frame.pc += 1;
}

pub fn lshl(frame: &mut StackFrame) {
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    frame.op_stack.push(StackFrameValue::Long(l1 << l2));
    frame.pc += 1;
}

pub fn ishr(frame: &mut StackFrame) {
    let i2 = frame.popi64() as i32;
    let i1 = frame.popi64() as i32;
    frame.op_stack.push(StackFrameValue::Int(i1 >> i2));
    frame.pc += 1;
}

pub fn lshr(frame: &mut StackFrame) {
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    frame.op_stack.push(StackFrameValue::Long(l1 >> l2));
    frame.pc += 1;
}

pub fn iushr(frame: &mut StackFrame) {
    let l2 = frame.popi64() as u32;
    let l1 = frame.popi64() as u32;
    frame.op_stack.push(StackFrameValue::Int((l1 >> l2) as i32));
    frame.pc += 1;
}

pub fn lushr(frame: &mut StackFrame) {
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    frame
        .op_stack
        .push(StackFrameValue::Long(l1 >> l2));
    frame.pc += 1;
}

pub fn iand(frame: &mut StackFrame) {
    let l2 = frame.popi64() as i32;
    let l1 = frame.popi64() as i32;
    frame.op_stack.push(StackFrameValue::Int(l1 & l2));
    frame.pc += 1;
}

pub fn land(frame: &mut StackFrame) {
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    frame.op_stack.push(StackFrameValue::Long(l1 & l2));
    frame.pc += 1;
}

pub fn ior(frame: &mut StackFrame) {
    let l2 = frame.popi64() as i32;
    let l1 = frame.popi64() as i32;
    frame.op_stack.push(StackFrameValue::Int(l1 | l2));
    frame.pc += 1;
}

pub fn lor(frame: &mut StackFrame) {
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    frame.op_stack.push(StackFrameValue::Long(l1 | l2));
    frame.pc += 1;
}

pub fn ixor(frame: &mut StackFrame) {
    let l2 = frame.popi64() as i32;
    let l1 = frame.popi64() as i32;
    frame.op_stack.push(StackFrameValue::Int(l1 ^ l2));
    frame.pc += 1;
}

pub fn lxor(frame: &mut StackFrame) {
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    frame.op_stack.push(StackFrameValue::Long(l1 ^ l2));
    frame.pc += 1;
}

pub fn iinc(frame: &mut StackFrame) {
    let index = frame.code[frame.pc + 1] as i32;
    let _const = frame.code[frame.pc + 2] as i8;
    let v: &StackFrameValue = frame.local.get(index as usize).unwrap();
    let i = match v {
        StackFrameValue::Byte(data) => *data as i64,
        StackFrameValue::Char(data) => *data as i64,
        StackFrameValue::Int(data) =>  *data as i64,
        StackFrameValue::Long(data) => *data,
        StackFrameValue::Short(data) =>  *data as i64,
        StackFrameValue::U32(data) =>  *data as i64,
        _ => panic!(),
    };
    frame.local[index as usize] =  StackFrameValue::Int((i as i32) + i32::from(_const));
    frame.pc += 3;
}
