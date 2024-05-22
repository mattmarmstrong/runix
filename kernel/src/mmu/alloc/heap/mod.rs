use self::fixed_sized_block::BlockAllocator;
use crate::mmu::address::VirtualAddress;

pub mod fixed_sized_block;
pub mod linked_list;

pub struct Locked<Alloc> {
    inner: spin::Mutex<Alloc>,
}

impl<Alloc> Locked<Alloc> {
    pub const fn new(inner: Alloc) -> Self {
        Self {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<Alloc> {
        self.inner.lock()
    }
}

#[global_allocator]
static HEAP_ALLOCATOR: Locked<BlockAllocator> = Locked::new(BlockAllocator::new());

pub(super) fn init_allocator(start: usize, size: usize) {
    log::info!("Initializing kernel heap allocator");
    let start_addr = VirtualAddress::with_kernel_base_offset(start);
    let mut allocator = HEAP_ALLOCATOR.lock();
    unsafe { allocator.init(start_addr, size) }
}
