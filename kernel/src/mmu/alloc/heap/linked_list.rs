use core::alloc::Layout;
use core::ptr;

use crate::mmu::address::VirtualAddress;

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    pub const fn empty() -> Self {
        const EMPTY: Option<&'static mut ListNode> = None;
        Self { size: 0, next: EMPTY }
    }

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

#[inline]
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

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        Self {
            head: ListNode::empty(),
        }
    }

    pub unsafe fn init(&mut self, heap_start: VirtualAddress, size: usize) {
        self.add_free_region(heap_start, size);
    }

    unsafe fn add_free_region(&mut self, addr: VirtualAddress, size: usize) {
        log::info!(
            "Adding free region\nStart: {:#X}, End: {:#X}",
            addr.inner,
            (addr.inner + size)
        );
        check_region(addr, size);
        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let ptr = addr.inner as *mut ListNode;
        ptr.write(node);
        self.head.next = Some(&mut *ptr);
    }

    fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<VirtualAddress, ()> {
        let alloc_start = region.start_addr().align_up(align);
        let alloc_end = alloc_start.add_checked(size); // panics if the checked add fails
        let region_end = region.end_addr();
        match alloc_end < region_end {
            true => {
                let excess = (region_end - alloc_end).inner;
                if excess > 0 && excess < core::mem::size_of::<ListNode>() {
                    Err(())
                } else {
                    Ok(alloc_start)
                }
            }
            false => Err(()),
        }
    }

    fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static ListNode, VirtualAddress)> {
        let mut current = &mut self.head;
        // Dear future self,
        // ref keyword: https://doc.rust-lang.org/std/keyword.ref.html
        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
                let next = region.next.take();
                let node = current.next.take().unwrap();
                current.next = next;
                return Some((node, alloc_start));
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        None
    }

    pub unsafe fn alloc_first_fit(&mut self, layout: Layout) -> *mut u8 {
        let (size, align) = size_align(layout);
        if let Some((region, alloc_start)) = self.find_region(size, align) {
            let alloc_end = alloc_start.add_checked(size);
            let excess = (region.end_addr() - alloc_end).inner;
            if excess > 0 {
                self.add_free_region(alloc_end, size);
            }
            alloc_start.inner as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    pub unsafe fn free(&mut self, ptr: *mut u8, layout: Layout) {
        let (size, _) = size_align(layout);
        let region_start = VirtualAddress::new(ptr as usize);
        self.add_free_region(region_start, size);
    }
}
