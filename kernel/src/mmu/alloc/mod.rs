use bootloader_api::BootInfo;

use self::heap::KERNEL_HEAP_SIZE;
use crate::mmu::address::VirtualAddress;
use crate::mmu::alloc::frame::boot::BootFrameAllocator;
use crate::mmu::alloc::heap::KERNEL_HEAP_START;

pub mod alloc;
pub mod frame;
pub mod heap;

fn map_kheap_frames(boot_info: &'static BootInfo) {
    let mut boot_allocator = BootFrameAllocator::new(&boot_info.memory_regions);
    let kheap_start_addr = VirtualAddress::with_kernel_base_offset(KERNEL_HEAP_START);
    let kheap_end_addr = kheap_start_addr + KERNEL_HEAP_SIZE;
    unsafe { boot_allocator.allocate_region(kheap_start_addr, kheap_end_addr) }
}
