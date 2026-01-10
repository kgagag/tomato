use std::num::Wrapping;
use log::info;

use crate::common::{error::Throwable, stack_frame::StackFrame, value::StackFrameValue};



pub fn iadd(frame: &mut StackFrame) ->Result<(),Throwable>{
    let i1: i32 = frame.popi64() as i32;
    let i2: i32 = frame.popi64() as i32;
    let a: Wrapping<i32> = Wrapping(i2);
    let b: Wrapping<i32> = Wrapping(i1);
    let result = a + b;
    let value = StackFrameValue::Int(result.0);
    frame.op_stack.push(value);
    frame.pc += 1;
    Ok(())
}

pub fn fadd(frame: &mut StackFrame) ->Result<(),Throwable>{
    let i1 = frame.popf64() as f32;
    let i2 = frame.popf64() as f32;
    let result = i1 + i2;
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
    Ok(())
}

pub fn dadd(frame: &mut StackFrame) ->Result<(),Throwable>{
    let i1 = frame.popf64() ;
    let i2 = frame.popf64() ;
    let result = i1 + i2;
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
    Ok(())
}

pub fn ladd(frame: &mut StackFrame) ->Result<(),Throwable>{
    let i1 = frame.popi64() ;
    let i2 = frame.popi64() ;
    let a: Wrapping<i64> = Wrapping(i2);
    let b: Wrapping<i64> = Wrapping(i1);
    let result = a + b;
    frame.op_stack.push(StackFrameValue::Long(result.0));
    frame.pc += 1;
    Ok(())
}

pub fn isub(frame: &mut StackFrame) ->Result<(),Throwable>{
    let i2 = frame.popi64() as i32;
    let i1 = frame.popi64() as i32;
    let result = i1 - i2;
    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
    Ok(())
}

pub fn fsub(frame: &mut StackFrame) ->Result<(),Throwable>{
    let f2 = frame.popf64() as f32;
    let f1 = frame.popf64() as f32;
    let result = f1 - f2;
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
    Ok(())
}

pub fn dsub(frame: &mut StackFrame) ->Result<(),Throwable>{
    let d2 = frame.popf64() ;
    let d1 = frame.popf64() ;
    let result = d1 - d2;
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
    Ok(())
}

pub fn lsub(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    let result = l1 - l2;
    frame.op_stack.push(StackFrameValue::Long(result));
    frame.pc += 1;
    Ok(())
}

pub fn fmul(frame: &mut StackFrame) ->Result<(),Throwable>{
    let f2 = frame.popf64() as f32;
    let f1 = frame.popf64() as f32;
    let result = f1 * f2;
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
    Ok(())
}

pub fn imul(frame: &mut StackFrame) ->Result<(),Throwable>{
    let i2 = frame.popi64() as i32;
    let i1 = frame.popi64() as i32;
    let a: Wrapping<i32> = Wrapping(i2);
    let b: Wrapping<i32> = Wrapping(i1);
    let result = a * b;
    frame.op_stack.push(StackFrameValue::Int(result.0));
    frame.pc += 1;
    Ok(())
}

pub fn lmul(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    let a: Wrapping<i64> = Wrapping(l2);
    let b: Wrapping<i64> = Wrapping(l1);
    let result = a * b;
    frame.op_stack.push(StackFrameValue::Long(result.0));
    frame.pc += 1;
    Ok(())
}

pub fn dmul(frame: &mut StackFrame) ->Result<(),Throwable>{
    let d2 = frame.popf64() ;
    let d1 = frame.popf64() ;
    let result = d1 * d2;
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
    Ok(())
}

pub fn idiv(frame: &mut StackFrame) ->Result<(),Throwable>{
    let i2 = frame.popi64() as i32;
    let i1 = frame.popi64() as i32;
    if i2 == 0 {
        panic!()
    }
    let result = i1 / i2;
    //warn!("{}", format!("{}{}", "idiv add result:", result));
    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
    Ok(())
}

pub fn fdiv(frame: &mut StackFrame) ->Result<(),Throwable>{
    let f2 = frame.popf64() as f32;
    let f1 = frame.popf64() as f32;
    if f2 == 0.0 {
        panic!()
    }
    let result = f1 / f2;
    //warn!("{}", format!("{}{}", "fdiv add result:", result));
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
    Ok(())
}

pub fn ddiv(frame: &mut StackFrame) ->Result<(),Throwable>{
    let d2 = frame.popf64() ;
    let d1 = frame.popf64() ;
    if d2 == 0.0 {
        panic!()
    }
    let result = d1 - d2;
    //warn!("{}", format!("{}{}", "ddiv add result:", result));
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
    Ok(())
}

pub fn ldiv(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    if l2 == 0 {
        panic!()
    }
    let result = l1 / l2;
    //warn!("{}", format!("{}{}", "ldiv add result:", result));
    frame.op_stack.push(StackFrameValue::Long(result));
    frame.pc += 1;
    Ok(())
}

pub fn irem(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() as i32;
    let l1 = frame.popi64() as i32;
    let result = l1 % l2;
    //warn!("{}", format!("{}{}", "ldiv add result:", result));
    frame.op_stack.push(StackFrameValue::Int(result));
    frame.pc += 1;
    Ok(())
}

pub fn frem(frame: &mut StackFrame) ->Result<(),Throwable>{
    let f2 = frame.popi64() as f32;
    let f1 = frame.popi64() as f32;
    let result = f1 % f2;
    //warn!("{}", format!("{}{}", "ldiv add result:", result));
    frame.op_stack.push(StackFrameValue::Float(result));
    frame.pc += 1;
    Ok(())
}

pub fn lrem(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    let result = l1 % l2;
    //warn!("{}", format!("{}{}", "ldiv add result:", result));
    frame.op_stack.push(StackFrameValue::Long(result));
    frame.pc += 1;
    Ok(())
}

pub fn drem(frame: &mut StackFrame) ->Result<(),Throwable>{
    let d2 = frame.popi64() as f64;
    let d1 = frame.popi64() as f64;
    let result = d1 % d2;
    //warn!("{}", format!("{}{}", "ldiv add result:", result));
    frame.op_stack.push(StackFrameValue::Double(result));
    frame.pc += 1;
    Ok(())
}

// 取负
pub fn ineg(frame: &mut StackFrame) ->Result<(),Throwable>{
    let i = frame.popi64() as i32;
    frame.op_stack.push(StackFrameValue::Int(0 - i));
    frame.pc += 1;
    Ok(())
}

pub fn lneg(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l = frame.popi64() ;
    frame.op_stack.push(StackFrameValue::Long(0 - l));
    frame.pc += 1;
    Ok(())
}

pub fn fneg(frame: &mut StackFrame) ->Result<(),Throwable>{
    let f = frame.popi64() as f32;
    frame.op_stack.push(StackFrameValue::Float(0.0 - f));
    frame.pc += 1;
    Ok(())
}

pub fn dneg(frame: &mut StackFrame) ->Result<(),Throwable>{
    let d = frame.popi64() as f64;
    frame.op_stack.push(StackFrameValue::Double(0.0 - d));
    frame.pc += 1;
    Ok(())
}

pub fn ishl(frame: &mut StackFrame) ->Result<(),Throwable>{
    let i2 = frame.popi64() as i32;
    let i1 = frame.popi64() as i32;
    frame.op_stack.push(StackFrameValue::Int(i1 << i2));
    frame.pc += 1;
    Ok(())
}

pub fn lshl(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    frame.op_stack.push(StackFrameValue::Long(l1 << l2));
    frame.pc += 1;
    Ok(())
}

pub fn ishr(frame: &mut StackFrame) ->Result<(),Throwable>{
    let i2 = frame.popi64() as i32;
    let i1 = frame.popi64() as i32;
    frame.op_stack.push(StackFrameValue::Int(i1 >> i2));
    frame.pc += 1;
    Ok(())
}

pub fn lshr(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    frame.op_stack.push(StackFrameValue::Long(l1 >> l2));
    frame.pc += 1;
    Ok(())
}

pub fn iushr(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() as u32;
    let l1 = frame.popi64() as u32;
    frame.op_stack.push(StackFrameValue::Int((l1 >> l2) as i32));
    frame.pc += 1;
    Ok(())
}

pub fn lushr(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    frame
        .op_stack
        .push(StackFrameValue::Long(l1 >> l2));
    frame.pc += 1;
    Ok(())
}

pub fn iand(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() as i32;
    let l1 = frame.popi64() as i32;
    frame.op_stack.push(StackFrameValue::Int(l1 & l2));
    frame.pc += 1;
    Ok(())
}

pub fn land(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    frame.op_stack.push(StackFrameValue::Long(l1 & l2));
    frame.pc += 1;
    Ok(())
}

pub fn ior(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() as i32;
    let l1 = frame.popi64() as i32;
    frame.op_stack.push(StackFrameValue::Int(l1 | l2));
    frame.pc += 1;
    Ok(())
}

pub fn lor(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    frame.op_stack.push(StackFrameValue::Long(l1 | l2));
    frame.pc += 1;
    Ok(())
}

pub fn ixor(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() as i32;
    let l1 = frame.popi64() as i32;
    frame.op_stack.push(StackFrameValue::Int(l1 ^ l2));
    frame.pc += 1;
    Ok(())
}

pub fn lxor(frame: &mut StackFrame) ->Result<(),Throwable>{
    let l2 = frame.popi64() ;
    let l1 = frame.popi64() ;
    frame.op_stack.push(StackFrameValue::Long(l1 ^ l2));
    frame.pc += 1;
    Ok(())
}

pub fn iinc(frame: &mut StackFrame) ->Result<(),Throwable>{
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
    Ok(())
}
