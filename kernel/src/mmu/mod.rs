// memory management software
pub mod address;
pub mod alloc;
pub mod vmm;

pub const KERNEL_BASE_ADDRESS: usize = 0xFFFF_8880_0000_0000;
