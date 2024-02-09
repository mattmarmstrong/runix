use super::vmm::frame::PhysicalFrame;

pub mod alloc;
pub mod boot;
pub mod buddy;
pub mod slab;

pub trait FrameAllocator {
    fn allocate_frame(&self) -> Option<PhysicalFrame>;
    fn deallocate_frame(&self, physical_frame: PhysicalFrame);
}
