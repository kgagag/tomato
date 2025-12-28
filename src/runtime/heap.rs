use std::f32::consts::E;

pub struct Heap {
    memory: Box<[u8]>,
    memory_block: Box<Vec<(u32, u32)>>,
    address_map: Box<[u32]>,
    address_map_index: usize,
    address_malloc_method: u8,
}

impl Heap {
    //创建堆
    pub fn create() -> Heap {
        Heap {
            memory: Box::new([0u8; 1024 * 1024 * 256]),
            address_map: Box::new([0u32; 1024 * 1024 * 16]),
            memory_block: Box::new(vec![(0, 1024 * 1024 * 256)]),
            address_map_index: 0,
            address_malloc_method: 0,
        }
    }

    //分配内存,如果内存块足够，则分配成功,更新内存块信息
    pub fn malloc(&mut self, size: u32) ->usize{
        let index = self.address_map_index;
        for i in 0..self.memory_block.len() {
            let (address, block_size) = self.memory_block.get(i).unwrap().clone();
            if block_size >= size {
                let new_address = address + size;
                let new_block_size = block_size - size;
                self.memory_block.remove(i);
                if new_block_size > 0 {
                    self.memory_block[i] = (new_address, new_block_size);
                } else {
                    self.memory_block.remove(i);
                }
                self.address_map[self.address_map_index] = address;
                if self.address_malloc_method == 0 {
                    if self.address_map_index < (self.address_map.len() - 1) {
                        self.address_map_index += 1;
                    } else {
                        self.address_malloc_method = 1;
                        //在address_map 中寻找指向0的位置
                        for j in 0..self.address_map.len() {
                            if self.address_map[j] == 0 {
                                self.address_map_index = j;
                                break;
                            }
                        }
                    }
                } else {
                    //在address_map 中寻找指向0的位置
                    for j in 0..self.address_map.len() {
                        if self.address_map[j] == 0 {
                            self.address_map_index = j;
                            break;
                        }
                    }
                }
            }
            panic!("heap out of memory");
        }
        index
    }
}
