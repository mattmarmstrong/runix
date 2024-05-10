use bootloader_api::BootInfo;

use self::frame::map_frames;
use self::heap::init_allocator;

pub mod frame;
pub mod heap;

pub const KERNEL_HEAP_START: usize = 0xDEAD_BEEF;
pub const KERNEL_HEAP_SIZE: usize = 1 << 21; // 1 MB

pub fn init_kheap(boot_info: &'static BootInfo) {
    map_frames(boot_info, KERNEL_HEAP_START, KERNEL_HEAP_SIZE);
    init_allocator(KERNEL_HEAP_START, KERNEL_HEAP_SIZE);
}
