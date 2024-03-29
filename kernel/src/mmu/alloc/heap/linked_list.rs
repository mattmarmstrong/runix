use core::alloc::{
    GlobalAlloc,
    Layout,
};

use crate::mmu::address::VirtualAddress;

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    pub fn new(size: usize) -> Self {
        Self { size, next: None }
    }

    pub fn start_addr(&self) -> VirtualAddress {
        VirtualAddress::new(self as *const _ as usize)
    }

    pub fn end_addr(&self) -> VirtualAddress {
        self.start_addr() + self.size
    }
}

fn check_region(addr: VirtualAddress, size: usize) {
    assert_eq!(addr.align_up(core::mem::align_of::<ListNode>()), addr);
    assert!(size >= core::mem::size_of::<ListNode>());
}

#[inline]
fn size_align(layout: Layout) -> (usize, usize) {
    let layout = layout
        .align_to(core::mem::align_of::<ListNode>())
        .expect("Alignment failed!")
        .pad_to_align();
    let size = layout.size().max(core::mem::size_of::<ListNode>());
    (size, layout.align())
}
