pub mod stack_frame {
    use crate::value::value::StackFrameValue;
    /**
     * 栈桢
     */
    #[derive(Debug, Clone)]
    pub struct StackFrame {
        // //指令
        // pub code: Vec<u8>,
        //程序计数器
        pub pc: usize,
        // //局部变量表
        pub local: Vec<StackFrameValue>,
        // //操作数栈
        pub op_stack: Vec<StackFrameValue>,
        // //类
        pub class: usize,

        pub max_stack: u16,

        pub max_locals: u16,

        pub code: Vec<u8>,

        //所属虚拟机栈id
        pub vm_stack_id: u32,
    }

    impl StackFrame {
        pub fn new(class: usize, max_stack: u16, max_locals: u16, code: Vec<u8>) -> StackFrame {
            let mut stake_frame = StackFrame {
                pc: 0,
                class,
                local: Vec::new(),
                op_stack: Vec::new(),
                max_stack,
                max_locals,
                code,
                vm_stack_id: 0,
            };
            for _i in 0..stake_frame.max_locals as usize {
                stake_frame.local.push(StackFrameValue::Byte(0));
            }
            return stake_frame;
        }

        pub fn store(&mut self, index: usize, stack_value: StackFrameValue) {
            self.local[index] = stack_value;
        }

        pub fn loadu32(&mut self,index: usize) ->u32{
            let value = self.local.get(index).unwrap();
            match value {
                StackFrameValue::Int(data) =>{
                     *data as u32
                }
                StackFrameValue::Byte(data) => {
                   *data as u32
                }
                StackFrameValue::Char(data) => {
                     *data as u32
                }
                StackFrameValue::Double(data) => {
                     *data as u32
                }
                StackFrameValue::Float(data) =>{
                     *data as u32
                }
                StackFrameValue::Long(data) => {
                    *data as u32
                }
                StackFrameValue::Short(data) => {
                    *data as u32
                }
                _ => {
                    panic!("wrong value type");
                }
            }
        }

        pub fn popu64(&mut self) ->u64{
            let value = self.op_stack.pop().unwrap();
            match value {
                StackFrameValue::Int(data) =>{
                     data as u64
                }
                StackFrameValue::Byte(data) => {
                     data as u64
                }
                StackFrameValue::Char(data) => {
                     data as u64
                }
                StackFrameValue::Double(data) => {
                     data as u64
                }
                StackFrameValue::Float(data) =>{
                     data as u64
                }
                StackFrameValue::Long(data) => {
                    data as u64
                }
                StackFrameValue::Short(data) => {
                    data as u64
                }
                _ => {
                    panic!("wrong value type");
                }
            }
        }



    }
}
