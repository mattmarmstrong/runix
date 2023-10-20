use core::mem::size_of;

use crate::mmu::VirtualAddress;

const STACK_SIZE: usize = 4096 * 5;

pub const PRIVILEGE_LEVEL_ZERO_STACK_TABLE_INDEX: usize = 0x00;
pub const PRIVILEGE_LEVEL_THREE_STACK_TABLE_INDEX: usize = 0x02;

pub const DOUBLE_FAULT_STACK_TABLE_INDEX: usize = 0x00;
pub const PAGE_FAULT_STACK_TABLE_INDEX: usize = 0x01;

pub static mut DOUBLE_FAULT_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
pub static mut PAGE_FAULT_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

pub enum StackTableType {
    Privilege,
    Interrupt,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct TaskStateSegment {
    _reserved_1: u32,
    privilege_stack_table: [VirtualAddress; 3],
    _reserved_2: u64,
    interrupt_stack_table: [VirtualAddress; 7],
    _reserved_3: u64,
    _reserved_4: u16,
    iomap_base: u16,
}

impl TaskStateSegment {
    pub const fn new() -> TaskStateSegment {
        TaskStateSegment {
            _reserved_1: 0,
            privilege_stack_table: [VirtualAddress::zero(); 3],
            _reserved_2: 0,
            interrupt_stack_table: [VirtualAddress::zero(); 7],
            _reserved_3: 0,
            _reserved_4: 0,
            iomap_base: (size_of::<TaskStateSegment>() - 1) as u16,
        }
    }

    pub fn address(&self) -> VirtualAddress {
        VirtualAddress::new(self as *const _ as u64)
    }

    pub fn init_interrupt_stack_table(&mut self, stack_table_index: usize, stack: [u8; STACK_SIZE]) {
        let stack_ptr: u64 = (&stack as *const _ as usize + STACK_SIZE) as u64;
        let canonical_stack_ptr = VirtualAddress::new(stack_ptr);
        self.interrupt_stack_table[stack_table_index] = canonical_stack_ptr;
    }

    pub fn init_priviledge_stack_table(&mut self, stack_table_index: usize, stack: [u8; STACK_SIZE]) {
        let stack_ptr: u64 = (&stack as *const _ as usize + STACK_SIZE) as u64;
        let canonical_stack_ptr = VirtualAddress::new(stack_ptr);
        self.privilege_stack_table[stack_table_index] = canonical_stack_ptr;
    }
}
