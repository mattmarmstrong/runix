use crate::mmu::alloc::FrameAllocator;
use crate::mmu::physical_address::PhysicalAddress;
use crate::mmu::virtual_address::VirtualAddress;
use crate::mmu::vmm::asm::get_raw_pml4_ptr;
use crate::mmu::vmm::frame::PhysicalFrame;
use crate::mmu::vmm::page_table_entry::PageTableEntry;

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

    pub unsafe fn get_active_pml4() -> Self {
        let raw_pml4_address = get_raw_pml4_ptr();
        let table_ptr = raw_pml4_address as *mut PageTable;
        *table_ptr
    }

    fn try_get_next_page_table(offset: VirtualAddress, entry: PageTableEntry) -> Option<Self> {
        match entry.get_physical_frame() {
            Some(frame) => Some(frame.frame_to_page_table(offset)),
            None => None,
        }
    }

    // like try_get_next_page_table, except if the corresponding page table doesn't exist, it
    // creates it. Returns a flag that lets the caller know if the table was created or just
    // fetched from the existing entry
    pub fn get_next_page_table(
        offset: VirtualAddress,
        mut entry: PageTableEntry,
        flags: u64,
        frame_allocator: impl FrameAllocator,
    ) -> (Self, bool) {
        let created: bool;
        let physical_frame: PhysicalFrame;
        match entry.is_unused() {
            true => {
                physical_frame = frame_allocator.allocate_frame().unwrap();
                entry.set_physical_frame_address(physical_frame);
                created = true;
            }
            false => {
                entry.set_flags(flags);
                physical_frame = entry.get_physical_frame().unwrap();
                created = false;
            }
        }
        let mut page_table = physical_frame.frame_to_page_table(offset);
        match created {
            true => {
                page_table.set_empty();
                (page_table, created)
            }
            false => (page_table, created),
        }
    }
}

#[derive(Debug)]
pub struct MappedPageTable {
    page_table: PageTable,
    offset: VirtualAddress,
}

impl MappedPageTable {
    pub fn from_pml4(offset: VirtualAddress, page_table: PageTable) -> Self {
        Self { page_table, offset }
    }

    pub fn translate_virtual_address(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let pml4_index = virtual_address.get_pml4_index();
        let pml4_entry = self.page_table.inner[pml4_index];
        let pdpt_option = PageTable::try_get_next_page_table(self.offset, pml4_entry);
        match pdpt_option {
            Some(pdpt) => {
                let pdpt_index = virtual_address.get_pdpt_index();
                let pdpt_entry = pdpt.inner[pdpt_index];
                let pd_option = PageTable::try_get_next_page_table(self.offset, pdpt_entry);
                match pd_option {
                    Some(pd) => {
                        let pd_index = virtual_address.get_pd_index();
                        let pd_entry = pd.inner[pd_index];
                        let pt_option = PageTable::try_get_next_page_table(self.offset, pd_entry);
                        match pt_option {
                            Some(pt) => {
                                let pt_index = virtual_address.get_pt_index();
                                let pt_entry = pt.inner[pt_index];
                                let physical_frame_option = pt_entry.get_physical_frame();
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
}
