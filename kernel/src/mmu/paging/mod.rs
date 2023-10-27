pub mod frame;
pub mod page;
pub mod page_table;

pub const PHYSICAL_ADDRESS_MASK: u64 = 0x000F_FFFF_FFFF_F000;

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum PageSize {}

impl PageSize {
    pub const FOUR_KIB: u64 = 1 << 12;
    pub const TWO_MB: u64 = (1 << 20) * 2;
    pub const ONE_GB: u64 = 1 << 30;
}
