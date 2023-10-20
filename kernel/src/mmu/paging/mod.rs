pub mod frame;
pub mod page;
pub mod page_table;

#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum PageSize {
    Standard = 1 << 1,         // 4KB
    Large = 1 << 2 * 2,        // 2MB
    Huge = 1024 * 1024 * 1024, // 1 GB
}
pub const PHYSICAL_ADDRESS_MASK: u64 = 0x000F_FFFF_FFFF_F000;
