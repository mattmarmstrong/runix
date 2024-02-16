pub mod asm;
pub mod frame;
pub mod page;
pub mod page_table;
pub mod page_table_entry;

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Size {}

impl Size {
    pub const FOUR_KIB: u64 = 1 << 12;
    pub const TWO_MB: u64 = (1 << 20) * 2;
    pub const ONE_GB: u64 = 1 << 30;
}
