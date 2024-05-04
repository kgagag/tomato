
    use crate::stack_frame::StackFrame;
    use crate::u8c::*;
    use crate::value::value::StackFrameValue;
    extern crate log;
    extern crate env_logger;
    use crate::value::*;
    

    use self::value::number_to_u32tuple;


    pub fn fstore(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        let index = frame.code[frame.pc + 1] as usize;
        frame.local[index] = v;
        frame.pc += 2;
    }

    pub fn astore(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        let index = frame.code[frame.pc + 1] as usize;
        frame.local[index] = v;
        frame.pc += 2;
    }

    pub fn istore(frame: &mut StackFrame) {
        let v = frame.op_stack.pop().unwrap();
        let index = frame.code[frame.pc + 1] as usize;
        frame.local[index] = v;
        frame.pc += 2;
    }

    
    pub fn lstore(frame: &mut StackFrame) {
        xistore(frame);
    }

    fn xistore(frame: &mut StackFrame){
        let v: StackFrameValue = frame.op_stack.pop().unwrap();
        let index = frame.code[frame.pc + 1] as usize;
        let value:i64;
        match v {
            StackFrameValue::Byte(l) => value = l as i64,
            StackFrameValue::Char(l) => value = l as i64,
            StackFrameValue::Int(l) => value = l as i64,
            StackFrameValue::Short(l) => value = l as i64,
            _=> panic!()
        }
        let bytes: [u8; 8] = value.to_le_bytes();
        let a =  u8s_to_u32(&bytes[0..4]);
        frame.local[index] = StackFrameValue::U32(u8s_to_u32(&bytes[0..4])) ;
        frame.local[index + 1] = StackFrameValue::U32(u8s_to_u32(&bytes[4..8])) ;
        frame.pc += 2;
    }


    fn xfstore(frame: &mut StackFrame){
        let v: StackFrameValue = frame.op_stack.pop().unwrap();
        let index = frame.code[frame.pc + 1] as usize;
        let value:f64;
        match v {
            StackFrameValue::Double(l) => value = l,
            StackFrameValue::Float(l) => value = l as f64,
            _=> panic!()
        }
        let u32_tuple = f64_to_u32_tuple(value);
        frame.local[index] = StackFrameValue::U32(u32_tuple.0) ;
        frame.local[index + 1] = StackFrameValue::U32(u32_tuple.1) ;
        frame.pc += 2;
    }

    pub fn dstore(frame: &mut StackFrame) {
        xfstore(frame);
    }

    pub fn istore_0(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[0] = i;
        frame.pc += 1;
    }


    pub fn istore_1(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[1] = i;
        frame.pc += 1;
    }

    pub fn istore_2(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[2] = i;
        frame.pc += 1;
    }

    pub fn istore_3(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[3] = i;
        frame.pc += 1;
    }

    pub fn fstore_0(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[0] = i;
        frame.pc += 1;
    }

    pub fn fstore_1(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[1] = i;
        frame.pc += 1;
    }

    pub fn fstore_2(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[2] = i;
        frame.pc += 1;
    }

    pub fn fstore_3(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[3] = i;
        frame.pc += 1;
    }


    pub fn dstore_0(frame: &mut StackFrame) {
        set_u64_frame_local(frame,0);
        frame.pc += 1;
    }

    pub fn dstore_1(frame: &mut StackFrame) {
        set_u64_frame_local(frame,1);
        frame.pc += 1;
    }

    pub fn dstore_2(frame: &mut StackFrame) {
        set_u64_frame_local(frame,2);
        frame.pc += 1;
    }

    pub fn dstore_3(frame: &mut StackFrame) {
        set_u64_frame_local(frame,3);
        frame.pc += 1;
    }

    fn set_u64_frame_local(frame: &mut StackFrame,i:usize){
        let v = frame.op_stack.pop().unwrap();
        let u32tuple = number_to_u32tuple(&v);
        frame.local[i] = StackFrameValue::U32(u32tuple.0);
        frame.local[i  + 1] = StackFrameValue::U32(u32tuple.1);
    }


    pub fn lstore_0(frame: &mut StackFrame) {
        set_u64_frame_local(frame,0);
        frame.pc += 1;
    }

    pub fn lstore_1(frame: &mut StackFrame) {
        set_u64_frame_local(frame,1);
        frame.pc += 1;
    }

    pub fn lstore_2(frame: &mut StackFrame) {
        set_u64_frame_local(frame,2);
        frame.pc += 1;
    }

    pub fn lstore_3(frame: &mut StackFrame) {
        set_u64_frame_local(frame,3);
        frame.pc += 1;
    }

    pub fn astore_0(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[0] = i;
        frame.pc += 1;
    }

    pub fn astore_1(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[1] = i;
        frame.pc += 1;
    }

    pub fn astore_2(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[2] = i;
        frame.pc += 1;
    }
    
    pub fn astore_3(frame: &mut StackFrame) {
        let i = frame.op_stack.pop().unwrap();
        frame.local[3] = i;
        frame.pc += 1;
    }
