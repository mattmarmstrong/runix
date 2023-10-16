use core::arch::asm;
use core::mem::size_of;

use crate::segmentation::asm::get_cs;
use crate::segmentation::gdt::SegmentSelector;
use crate::vmm::VirtualAddress;

#[derive(Debug, Clone)]
#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    pub descriptor_table: [GateDescriptor; 256],
}

impl InterruptDescriptorTable {
    pub fn new() -> Self {
        let descriptor_table = [GateDescriptor::new(GateOptions::minimal()); 256];
        InterruptDescriptorTable { descriptor_table }
    }

    pub fn pointer(&self) -> IdtPointer {
        let limit = (size_of::<Self>() - 1) as u16;
        let base = VirtualAddress::new(self as *const _ as u64);
        IdtPointer { limit, base }
    }

    pub unsafe fn load_idt(idt_ptr: &IdtPointer) {
        asm!("lidt [{}]", in(reg) idt_ptr, options(readonly, nostack, preserves_flags))
    }
}

#[repr(C, packed(2))]
pub struct IdtPointer {
    limit: u16,
    base: VirtualAddress,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GateDescriptor {
    offset_low: u16,
    segment: SegmentSelector,
    gate_options: GateOptions,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

impl GateDescriptor {
    pub fn new(gate_options: GateOptions) -> Self {
        GateDescriptor {
            offset_low: 0,
            segment: SegmentSelector { inner: 0 },
            gate_options,
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    pub fn set_handler_address(&mut self, handler_address: VirtualAddress) -> &Self {
        self.offset_low = (handler_address.inner & 0x0000_0000_0000_FFFF) as u16;
        self.offset_middle = ((handler_address.inner & 0x0000_0000_FFFF_0000) >> 16) as u16;
        self.offset_high = ((handler_address.inner & 0xFFFF_FFFF_0000_0000) >> 32) as u32;
        unsafe {
            self.segment = get_cs();
        }
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct GateOptions {
    inner: u16,
}

impl GateOptions {
    pub fn minimal() -> Self {
        // interrupt gate by default, meaning interrupts are automatically be disabled/re-enabled by the CPU
        GateOptions { inner: 0x0E00 }
    }

    pub fn set_present(mut self) -> Self {
        self.inner |= 0x8000;
        self
    }

    pub fn dpl_0(mut self) -> Self {
        // clear bits 13 & 14
        self.inner &= 0x9FFF;
        self
    }

    pub fn dpl_3(mut self) -> Self {
        // set bits 13 & 14
        self.inner |= 0x6000;
        self
    }

    // converts an trap gate to a interrupt gate
    pub fn enable_interrupts(mut self) -> Self {
        self.inner |= 0x0100;
        self
    }

    pub fn exception_gate_options() -> Self {
        GateOptions::minimal().set_present()
    }

    pub fn trap_gate_options() -> Self {
        GateOptions::minimal().set_present().enable_interrupts()
    }

    pub fn set_stack_index(mut self, stack_index: usize) -> Self {
        self.inner = (self.inner & 0xFFF8) | ((stack_index + 1) as u16);
        self
    }
}

pub unsafe fn load_idt(idt_ptr: &IdtPointer) {
    asm!("lidt [{}]", in(reg) idt_ptr, options(readonly, nostack, preserves_flags))
}
