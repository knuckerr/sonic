use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

pub struct MemoryBlock {
    data: NonNull<u8>,
    size: usize,
}

unsafe impl Send for MemoryBlock{}

pub struct MemoryPool {
    blocks: Vec<MemoryBlock>,
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryPool {
    pub fn new() -> Self {
        MemoryPool { blocks: Vec::new() }
    }
    pub fn allocate(&mut self, size: usize) -> *mut u8 {
        for block in &mut self.blocks {
            if block.size >= size {
                return block.data.as_ptr();
            }
        }
        let layout = Layout::from_size_align(size, std::mem::size_of::<u8>()).unwrap();
        let ptr = unsafe { alloc(layout) };
        if !ptr.is_null() {
            let block = MemoryBlock {
                data: unsafe { NonNull::new_unchecked(ptr) },
                size,
            };
            self.blocks.push(block)
        }
        ptr
    }
    pub fn deallocate(&mut self, ptr: *mut u8) {
        if ptr.is_null() {
            return;
        }

        if let Some(index) = self
            .blocks
            .iter()
            .position(|block| block.data.as_ptr() == ptr)
        {
            let block = self.blocks.remove(index);
            let layout = Layout::from_size_align(block.size, std::mem::align_of::<u8>()).unwrap();
            unsafe { dealloc(block.data.as_ptr(), layout) };
        } else {
            panic!("Attempted to deallocate non-pooled memory");
        }
    }
}

#[test]
pub fn allocate() {
    let mut pool = MemoryPool::new();
    pool.alocate(12);
    assert!(true);
}

#[test]
pub fn deallocate() {
    let mut pool = MemoryPool::new();
    let ptr = pool.alocate(12);
    pool.deallocate(ptr);
    assert!(true);
}
