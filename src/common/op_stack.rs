use crate::common::value::StackFrameValue;

#[derive(Debug, Clone)]
pub struct OpStack{
    small_stack:[StackFrameValue;128],
    small_size:usize,
    top:usize,
    large_stack:Vec<StackFrameValue>,
    flag:bool//是否启用large_stack
}


impl OpStack{
    pub fn new(flag:bool)->OpStack{
        OpStack{
            small_stack:[StackFrameValue::UNSET;128],
            large_stack:Vec::new(),
            flag:flag,
            small_size: 0,
            top: 0,
        }
    }

    pub fn push(&mut self,value:StackFrameValue){
        if !self.flag{
            //0 空着
            self.top = self.top + 1;
            self.small_stack[self.top] = value;
            self.small_size = self.small_size + 1;
        }else{
            self.large_stack.push(value);
        }
    }

    pub fn pop(&mut self)->Option<StackFrameValue>{
        if !self.flag{
            if self.small_size == 0{
                panic!("stack is empty")
            }else{
                self.small_size = self.small_size - 1;
                self.top =  self.top - 1 ;
                return  Some(self.small_stack[self.top + 1])
            }
        }else{
            self.large_stack.pop()
        }
    }

    pub fn len (&self)->usize{
        if !self.flag{
            self.small_size
        }else{
            self.large_stack.len()
        }
    }

    pub fn last(&self)->Option<StackFrameValue>{
        if !self.flag{
            if self.small_size == 0{
                panic!("stack is empty")
            }else{
                Some(self.small_stack[self.top])
            }
        }else{
            self.large_stack.last().cloned()
        }
    }

    pub fn get(&self,index : usize) -> StackFrameValue{
        if !self.flag{
            if self.small_size == 0{
                panic!("stack is empty")
            }else{
                self.small_stack[index + 1]
            }
        }else{
            *self.large_stack.get(index).unwrap()
        }
    }

    pub fn peek(&self) ->StackFrameValue{
        if !self.flag{
            if self.small_size == 0{
                panic!("stack is empty")
            }else{
                self.small_stack[self.top]
            }
        }else{
            self.large_stack[self.large_stack.len() - 1]
        }
    }


    // //按照Vec的插入方式分别处理small_stack和large_stack
    // pub fn insert(&mut self,index:usize,value:StackFrameValue){
        
    // }

}