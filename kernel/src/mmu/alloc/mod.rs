use bootloader_api::BootInfo;

use super::vmm::page_table::{
    MappedPageTable,
    PageTable,
};
use super::vmm::page_table_entry::PageTableEntryFlags;
use super::KERNEL_BASE_ADDRESS;
use crate::mmu::alloc::boot::BootFrameAllocator;
use crate::mmu::virtual_address::VirtualAddress;
use crate::mmu::vmm::frame::PhysicalFrame;
use crate::mmu::vmm::page::{
    VirtualPage,
    VirtualPageRange,
};

pub mod alloc;
pub mod boot;
pub mod buddy;
pub mod slab;

pub const KERNEL_HEAP_START: u64 = 0xDEAD_BEEF;
pub const KERNEL_HEAP_SIZE: u64 = 1 << 21; // 1 MB

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysicalFrame>;
    fn deallocate_frame(&self, physical_frame: PhysicalFrame);
}

fn init_kernel_heap(boot_info: &'static mut BootInfo) {
    let mut boot_allocator = BootFrameAllocator::new(&boot_info.memory_regions);
    let kernel_heap_start_page =
        VirtualPage::from_address_aligned(VirtualAddress::with_kernel_base_offset(KERNEL_HEAP_START));
    let kernel_heap_end_page = VirtualPage::from_address_aligned(VirtualAddress::with_kernel_base_offset(
        KERNEL_HEAP_START + KERNEL_HEAP_SIZE,
    ));
    let virtual_page_range = VirtualPageRange::range_inclusive(kernel_heap_start_page, kernel_heap_end_page);
    let mut active_pml4 =
        unsafe { MappedPageTable::from_pml4(VirtualAddress::new(KERNEL_BASE_ADDRESS), PageTable::get_active_pml4()) };

    virtual_page_range.for_each(|virtual_page| {
        let allocated_frame = boot_allocator.allocate_frame().unwrap();
        let entry_flags = PageTableEntryFlags::PRESENT & PageTableEntryFlags::WRITE_ACCESS;
        let table_flags = entry_flags;
        active_pml4.map_to(
            virtual_page,
            allocated_frame,
            entry_flags,
            table_flags,
            true,
            &mut boot_allocator,
        )
    })
}
