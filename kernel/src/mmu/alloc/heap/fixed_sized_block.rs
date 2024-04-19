use core::alloc::{
    GlobalAlloc,
    Layout,
};

use crate::mmu::alloc::heap::linked_list::LinkedListAllocator;

pub const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];

fn get_block_size_index(layout: Layout) -> Option<usize> {
    let required_size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&block_size| required_size <= block_size)
}

#[derive(Debug)]
struct FreeBlock {
    next: Option<&'static mut FreeBlock>,
}

impl FreeBlock {
    #[inline]
    pub fn new(next: Option<&'static mut FreeBlock>) -> Self {
        Self { next }
    }

    #[inline]
    pub fn addr(&self) -> usize {
        self as *const _ as usize
    }
}

pub struct FixedSizedBlockAllocator {
    free_lists: [Option<&'static mut FreeBlock>; BLOCK_SIZES.len()],
    fallback_allocator: LinkedListAllocator,
}

unsafe impl GlobalAlloc for FixedSizedBlockAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match get_block_size_index(layout) {
            Some(index) => match self.free_lists[index].take() {
                Some(node) => {
                    self.free_lists[index] = node.next.take();
                    node as *mut FreeBlock as *mut u8
                }
                None => {
                    let block_size = BLOCK_SIZES[index];
                    let block_align = block_size;
                    let layout = Layout::from_size_align(block_size, block_align).unwrap();
                    self.fallback_allocator.alloc(layout)
                }
            },
            None => self.fallback_allocator.alloc(layout),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match get_block_size_index(layout) {
            Some(index) => {
                let block = FreeBlock::new(self.free_lists[index].take());
                debug_assert!(core::mem::size_of::<FreeBlock>() <= BLOCK_SIZES[index]);
                debug_assert!(core::mem::align_of::<FreeBlock>() <= BLOCK_SIZES[index]);
                let block_ptr = ptr as *mut FreeBlock;
                block_ptr.write(block);
                self.free_lists[index] = Some(&mut *block_ptr);
            }
            None => self.fallback_allocator.dealloc(ptr, layout),
        }
    }
}

impl FixedSizedBlockAllocator {
    pub fn new() -> Self {
        const EMPTY: Option<&'static mut FreeBlock> = None;
        Self {
            free_lists: [EMPTY; BLOCK_SIZES.len()],
            fallback_allocator: LinkedListAllocator::new(),
        }
    }
}
