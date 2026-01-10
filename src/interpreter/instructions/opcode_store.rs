
    use log::info;

    use crate::{common::{error::Throwable, stack_frame::StackFrame, value::{StackFrameValue, number_to_u32tuple}}, utils::u8c::{f64_to_u32_tuple, i64_to_u32_tuple}};

  

    pub fn fstore(frame: &mut StackFrame) ->Result<(),Throwable>{
        let v = frame.op_stack.pop().unwrap();
        let index = frame.code[frame.pc + 1] as usize;
        frame.local[index] = v;
        frame.pc += 2;
        Ok(())
    }

    pub fn astore(frame: &mut StackFrame) ->Result<(),Throwable>{
        let v = frame.op_stack.pop().unwrap();
        let index = frame.code[frame.pc + 1] as usize;
        frame.local[index] = v;
        frame.pc += 2;
        Ok(())
    }

    pub fn istore(frame: &mut StackFrame) ->Result<(),Throwable>{
        let v = frame.op_stack.pop().unwrap();
        let index = frame.code[frame.pc + 1] as usize;
        frame.local[index] = v;
        frame.pc += 2;
        Ok(())
    }

    
    pub fn lstore(frame: &mut StackFrame) ->Result<(),Throwable>{
        xistore(frame);
        Ok(())
    }

    fn xistore(frame: &mut StackFrame){
        let v: StackFrameValue = frame.op_stack.pop().unwrap();
        let index = frame.code[frame.pc + 1] as usize;
        let value = match v {
            StackFrameValue::Byte(l) => l as i64,
            StackFrameValue::Char(l) =>  l as i64,
            StackFrameValue::Int(l) =>  l as i64,
            StackFrameValue::Short(l) =>  l as i64,
            StackFrameValue::Long(l) => l ,
            _=> panic!()
        };
        let u32_tuple = i64_to_u32_tuple(value);

        frame.local[index] = StackFrameValue::U32(u32_tuple.0) ;
        frame.local[index + 1] = StackFrameValue::U32(u32_tuple.1) ;

       // frame.local[index] = StackFrameValue::U32(c) ;
       // frame.local[index + 1] = StackFrameValue::U32(u8s_to_u32(&bytes[4..8])) ;
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

    pub fn dstore(frame: &mut StackFrame) ->Result<(),Throwable>{
        xfstore(frame);
        Ok(())
    }

    pub fn istore_0(frame: &mut StackFrame) ->Result<(),Throwable>{
        let i = frame.op_stack.pop().unwrap();
        frame.local[0] = i;
        frame.pc += 1;
        Ok(())
    }


    pub fn istore_1(frame: &mut StackFrame) ->Result<(),Throwable>{
        let i = frame.op_stack.pop().unwrap();
        frame.local[1] = i;
        frame.pc += 1;
        Ok(())
    }

    pub fn istore_2(frame: &mut StackFrame) ->Result<(),Throwable>{
        let i = frame.op_stack.pop().unwrap();
        frame.local[2] = i;
        frame.pc += 1;
        Ok(())
    }

    pub fn istore_3(frame: &mut StackFrame) ->Result<(),Throwable>{
        let i = frame.op_stack.pop().unwrap();
        frame.local[3] = i;
        frame.pc += 1;
        Ok(())
    }

    pub fn fstore_0(frame: &mut StackFrame) ->Result<(),Throwable>{
        let i = frame.op_stack.pop().unwrap();
        frame.local[0] = i;
        frame.pc += 1;
        Ok(())
    }

    pub fn fstore_1(frame: &mut StackFrame) ->Result<(),Throwable>{
        let i = frame.op_stack.pop().unwrap();
        frame.local[1] = i;
        frame.pc += 1;
        Ok(())
    }

    pub fn fstore_2(frame: &mut StackFrame) ->Result<(),Throwable>{
        let i = frame.op_stack.pop().unwrap();
        frame.local[2] = i;
        frame.pc += 1;
        Ok(())
    }

    pub fn fstore_3(frame: &mut StackFrame) ->Result<(),Throwable>{
        let i = frame.op_stack.pop().unwrap();
        frame.local[3] = i;
        frame.pc += 1;
        Ok(())
    }


    pub fn dstore_0(frame: &mut StackFrame) ->Result<(),Throwable>{
        set_u64_frame_local(frame,0);
        frame.pc += 1;
        Ok(())
    }

    pub fn dstore_1(frame: &mut StackFrame) ->Result<(),Throwable>{
        set_u64_frame_local(frame,1);
        frame.pc += 1;
        Ok(())
    }

    pub fn dstore_2(frame: &mut StackFrame) ->Result<(),Throwable>{
        set_u64_frame_local(frame,2);
        frame.pc += 1;
        Ok(())
    }

    pub fn dstore_3(frame: &mut StackFrame) ->Result<(),Throwable>{
        set_u64_frame_local(frame,3);
        frame.pc += 1;
        Ok(())
    }

    fn set_u64_frame_local(frame: &mut StackFrame,i:usize){
        let v = frame.op_stack.pop().unwrap();
        //info!("{:?}",v);
        let u32tuple = number_to_u32tuple(&v);
        frame.local[i] = StackFrameValue::U32(u32tuple.0);
        frame.local[i  + 1] = StackFrameValue::U32(u32tuple.1);
    }


    pub fn lstore_0(frame: &mut StackFrame) ->Result<(),Throwable>{
        set_u64_frame_local(frame,0);
        frame.pc += 1;
        Ok(())
    }

    pub fn lstore_1(frame: &mut StackFrame) ->Result<(),Throwable>{
        set_u64_frame_local(frame,1);
        frame.pc += 1;
        Ok(())
    }

    pub fn lstore_2(frame: &mut StackFrame) ->Result<(),Throwable>{
        set_u64_frame_local(frame,2);
        frame.pc += 1;
        Ok(())
    }

    pub fn lstore_3(frame: &mut StackFrame) ->Result<(),Throwable>{
        set_u64_frame_local(frame,3);
        frame.pc += 1;
        Ok(())
    }

    pub fn astore_0(frame: &mut StackFrame) ->Result<(),Throwable>{
        let i = frame.op_stack.pop().unwrap();
        frame.local[0] = i;
        frame.pc += 1;
        Ok(())
    }

    pub fn astore_1(frame: &mut StackFrame) ->Result<(),Throwable>{
        let i = frame.op_stack.pop().unwrap();
        frame.local[1] = i;
        frame.pc += 1;
        Ok(())
    }

    pub fn astore_2(frame: &mut StackFrame) ->Result<(),Throwable>{
        let i = frame.op_stack.pop().unwrap();
        frame.local[2] = i;
        frame.pc += 1;
        Ok(())
    }
    
    pub fn astore_3(frame: &mut StackFrame) ->Result<(),Throwable>{
        let i = frame.op_stack.pop().unwrap();
        frame.local[3] = i;
        frame.pc += 1;
        Ok(())
    }
