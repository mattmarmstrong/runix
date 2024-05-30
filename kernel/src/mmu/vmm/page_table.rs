use super::asm::flush;
use super::asm::get_raw_pml4_ptr;
use super::frame::PhysicalFrame;
use super::page::VirtualPage;
use super::page_table_entry::{
    PageTableEntry,
    PHYSICAL_ADDRESS_MASK,
};
use crate::mmu::address::{
    PhysicalAddress,
    VirtualAddress,
};
use crate::mmu::alloc::frame::FrameAllocator;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageTable {
    inner: [PageTableEntry; 512],
}

impl PageTable {
    pub fn empty() -> Self {
        PageTable {
            inner: [PageTableEntry::new_unused(); 512],
        }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PageTableEntry> {
        let slice_ptr = self.inner.as_mut_ptr();
        (0..512).map(move |item| unsafe { &mut *slice_ptr.add(item) })
    }

    pub fn set_empty(&mut self) {
        for entry in self.iter_mut() {
            entry.set_unused();
        }
    }

    pub unsafe fn get_active_pml4() -> &'static mut Self {
        let raw_pml4_address = get_raw_pml4_ptr();
        let addr = PhysicalAddress::new(PHYSICAL_ADDRESS_MASK & raw_pml4_address);
        let frame = PhysicalFrame::from_address_aligned(addr);
        let virt_addr = VirtualAddress::with_kernel_base_offset(frame.start_address());
        let table_ptr = virt_addr.inner as *mut PageTable;
        &mut *table_ptr
    }

    fn next_table_opt(offset: VirtualAddress, entry: PageTableEntry) -> Option<Self> {
        match entry.get_frame() {
            Some(frame) => Some(frame.frame_to_page_table(offset)),
            None => None,
        }
    }

    pub fn get_next_page_table(
        &self,
        offset: VirtualAddress,
        mut entry: PageTableEntry,
        flags: usize,
        frame_allocator: &mut impl FrameAllocator,
    ) -> Self {
        let created: bool;
        let physical_frame: PhysicalFrame;
        match entry.is_unused() {
            true => {
                physical_frame = frame_allocator.allocate_frame().unwrap();
                entry.set_frame_addr(physical_frame);
                created = true;
            }
            false => {
                entry.set_flags(flags);
                physical_frame = entry.get_frame().unwrap();
                created = false;
            }
        }
        let mut page_table = physical_frame.frame_to_page_table(offset);
        if created {
            page_table.set_empty();
        }
        page_table
    }
}

#[derive(Debug)]
pub struct MappedPageTable {
    page_table: &'static mut PageTable,
    offset: VirtualAddress,
}

impl MappedPageTable {
    pub fn new(offset: VirtualAddress, page_table: &'static mut PageTable) -> Self {
        Self { page_table, offset }
    }

    pub fn translate_virtual_address(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let pml4_index = virtual_address.get_pml4_index();
        let pml4_entry = self.page_table.inner[pml4_index];
        let pdpt_option = PageTable::next_table_opt(self.offset, pml4_entry);
        match pdpt_option {
            Some(pdpt) => {
                let pdpt_index = virtual_address.get_pdpt_index();
                let pdpt_entry = pdpt.inner[pdpt_index];
                let pd_option = PageTable::next_table_opt(self.offset, pdpt_entry);
                match pd_option {
                    Some(pd) => {
                        let pd_index = virtual_address.get_pd_index();
                        let pd_entry = pd.inner[pd_index];
                        let pt_option = PageTable::next_table_opt(self.offset, pd_entry);
                        match pt_option {
                            Some(pt) => {
                                let pt_index = virtual_address.get_pt_index();
                                let pt_entry = pt.inner[pt_index];
                                let physical_frame_option = pt_entry.get_frame();
                                match physical_frame_option {
                                    Some(physical_frame) => Some(PhysicalAddress::new(
                                        physical_frame.start_address() + virtual_address.get_page_offset(),
                                    )),
                                    None => None,
                                }
                            }
                            None => None,
                        }
                    }
                    None => None,
                }
            }
            None => None,
        }
    }

    pub fn map_to(
        &mut self,
        page: VirtualPage,
        frame: PhysicalFrame,
        entry_flags: usize,
        table_flags: usize,
        should_flush_page: bool,
        frame_allocator: &mut impl FrameAllocator,
    ) {
        let virtual_address = page.offset;
        let pml4 = &mut self.page_table;
        let pml4_entry = pml4.inner[virtual_address.get_pml4_index()];
        let pdpt = pml4.get_next_page_table(self.offset, pml4_entry, table_flags, frame_allocator);
        let pdpt_entry = pdpt.inner[virtual_address.get_pdpt_index()];
        let pd = pdpt.get_next_page_table(self.offset, pdpt_entry, table_flags, frame_allocator);
        let pd_entry = pd.inner[virtual_address.get_pd_index()];
        let mut pt = pd.get_next_page_table(self.offset, pd_entry, table_flags, frame_allocator);
        let mut pt_entry = pt.inner[virtual_address.get_pt_index()];

        match pt_entry.is_unused() {
            true => {
                pt_entry.set_frame_addr(frame);
                pt_entry.set_flags(entry_flags);
                // update the page table.
                pt.inner[virtual_address.get_pt_index()] = pt_entry;
                if should_flush_page {
                    unsafe {
                        flush(virtual_address.inner);
                    }
                }
            }
            false => panic!("Page is already mapped!"),
        }
    }
}
