use crate::mmu::vmm::frame::PhysicalFrame;

pub mod boot;
pub mod buddy;

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysicalFrame>;
    fn deallocate_frame(&self, physical_frame: PhysicalFrame);
}
