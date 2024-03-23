use core::alloc::Layout;

pub mod fixed_sized_block;
pub mod linked_list;

pub const KERNEL_HEAP_START: usize = 0xDEAD_BEEF;
pub const KERNEL_HEAP_SIZE: usize = 1 << 21; // 1 MB

pub trait HeapAllocator {
    fn allocate(&self, layout: Layout) -> *mut u8;
    fn deallocate(&self, ptr: *mut u8, layout: Layout);
}
