use core::alloc::{
    GlobalAlloc,
    Layout,
};

use super::linked_list::LinkedListAllocator;
use super::Locked;
use crate::mmu::address::VirtualAddress;

const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];

fn get_block_size_index(layout: Layout) -> Option<usize> {
    let required_size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&block_size| required_size <= block_size)
}

#[derive(Debug)]
struct Block {
    next: Option<&'static mut Block>,
}

impl Block {
    #[inline]
    pub fn new(next: Option<&'static mut Block>) -> Self {
        Self { next }
    }
}

pub struct BlockAllocator {
    free_lists: [Option<&'static mut Block>; BLOCK_SIZES.len()],
    fallback_allocator: LinkedListAllocator,
}

unsafe impl GlobalAlloc for Locked<BlockAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();
        match get_block_size_index(layout) {
            Some(index) => match allocator.free_lists[index].take() {
                Some(node) => {
                    allocator.free_lists[index] = node.next.take();
                    node as *mut Block as *mut u8
                }
                None => {
                    let block_size = BLOCK_SIZES[index];
                    let block_align = block_size;
                    let layout = Layout::from_size_align(block_size, block_align).unwrap();
                    allocator.fallback_allocator.alloc_first_fit(layout)
                }
            },
            None => allocator.fallback_allocator.alloc_first_fit(layout),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut allocator = self.lock();
        match get_block_size_index(layout) {
            Some(index) => {
                let block = Block::new(allocator.free_lists[index].take());
                debug_assert!(core::mem::size_of::<Block>() <= BLOCK_SIZES[index]);
                debug_assert!(core::mem::align_of::<Block>() <= BLOCK_SIZES[index]);
                let block_ptr = ptr as *mut Block;
                block_ptr.write(block);
                allocator.free_lists[index] = Some(&mut *block_ptr);
            }
            None => allocator.fallback_allocator.free(ptr, layout),
        }
    }
}

impl BlockAllocator {
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut Block> = None;
        Self {
            free_lists: [EMPTY; BLOCK_SIZES.len()],
            fallback_allocator: LinkedListAllocator::new(),
        }
    }

    pub unsafe fn init(&mut self, start: VirtualAddress, size: usize) {
        self.fallback_allocator.init(start, size);
    }
}
