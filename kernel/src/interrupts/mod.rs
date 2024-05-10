pub mod asm;
pub mod handlers;
pub mod idt;

use core::fmt;

use asm::enable_interrupts;
use handlers::*;
use idt::{
    GateDescriptor,
    GateOptions,
    InterruptDescriptorTable,
};
use lazy_static::lazy_static;

use crate::mmu::address::VirtualAddress;
use crate::process::RegisterState;
use crate::segmentation::tss::*;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
// These are the register values that the CPU pushes to the stack when an exception occurs
pub struct ExcRegisterState {
    rip: usize,
    cs: usize,
    rflags: usize,
    rsp: usize,
    ss: usize,
}

impl fmt::Display for ExcRegisterState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "RIP: {:#X} CS: {:#X} RFLAGS: {:#X}\nRSP: {:#X} SS: {:#X}",
            self.rip, self.cs, self.rflags, self.rsp, self.ss
        ))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExceptionStackFrame {
    execution_state: RegisterState,
    interrupt_registers: ExcRegisterState,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExceptionStackFrameWithErrorCode {
    execution_state: RegisterState,
    error_code: usize,
    interrupt_registers: ExcRegisterState,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum InterruptVector {}

impl InterruptVector {
    // Intel Manual - Section 6.3.1
    // These are the vector numbers CPU-generated interrupts.
    // They are all predefined
    pub const DIVIDE_ERROR: usize = 0x00;
    pub const DEBUG_EXCEPTION: usize = 0x01;
    pub const NMI: usize = 0x02; // Non-maskable (hardware) Interrupt
    pub const BREAKPOINT: usize = 0x03;
    pub const OVERFLOW: usize = 0x04;
    pub const BOUND_RANGE_EXCEEDED: usize = 0x05;
    pub const INVALID_OPCODE: usize = 0x06;
    pub const DEVICE_NOT_AVAILABLE: usize = 0x07;
    pub const DOUBLE_FAULT: usize = 0x08;
    pub const COPROCESSOR_SEGMENT_OVERRUN: usize = 0x09; //reserved, not used
    pub const INVALID_TSS: usize = 0x0A;
    pub const SEGMENT_NOT_PRESENT: usize = 0x0B;
    pub const STACK_SEGMENT_FAULT: usize = 0x0C;
    pub const GENERAL_PROTECTION: usize = 0x0D;
    pub const PAGE_FAULT: usize = 0x0E;
    // Vector #15 is reserved and not in use by modern x86_64 processors
    pub const X87_FLOATING_POINT_ERROR: usize = 0x10;
    pub const ALIGNMENT_CHECK: usize = 0x11;
    pub const MACHINE_CHECK: usize = 0x12;
    pub const SIMD_FLOATING_POINT_EXCEPTION: usize = 0x13;
    pub const VIRTUALIZATION_EXCEPTION: usize = 0x14;
    // Vectors #21-31 are reserved, and #32-255 are reserved for user defined interrupts
    // TODO: define IRQ numbers here
    pub const APIC_TIMER: usize = 0x20;
    pub const APIC_SPURIOUS: usize = 0xFF;
    pub const SYSCALL: usize = 0x80;
}

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Gate descriptors - Exceptions
        let mut div_by_zero_gate_desc = GateDescriptor::new(GateOptions::exception_gate_options());
        div_by_zero_gate_desc.set_handler_address(VirtualAddress::new(divide_by_zero as usize));

        let df_gate_opts = GateOptions::exception_gate_options().set_stack_index(DOUBLE_FAULT_STACK_TABLE_INDEX);
        let mut df_gate_desc = GateDescriptor::new(df_gate_opts);
        df_gate_desc.set_handler_address(VirtualAddress::new(double_fault as usize));

        let pf_gate_opts = GateOptions::exception_gate_options().set_stack_index(PAGE_FAULT_STACK_TABLE_INDEX);
        let mut pf_gate_desc = GateDescriptor::new(pf_gate_opts);
        pf_gate_desc.set_handler_address(VirtualAddress::new(page_fault as usize));


        // Gate descriptors - IRQs
        let mut lapic_timer_irq_gate_desc= GateDescriptor::new(GateOptions::trap_gate_options());
        lapic_timer_irq_gate_desc.set_handler_address(VirtualAddress::new(lapic_timer_interrupt as usize));

        let mut lapic_spurious_irq_gate_desc = GateDescriptor::new(GateOptions::trap_gate_options());
        lapic_spurious_irq_gate_desc.set_handler_address(VirtualAddress::new(lapic_spurious_interrupt as usize));

        // Exceptions
        idt.descriptor_table[InterruptVector::DIVIDE_ERROR] = div_by_zero_gate_desc;
        idt.descriptor_table[InterruptVector::DOUBLE_FAULT] = df_gate_desc;
        idt.descriptor_table[InterruptVector::PAGE_FAULT] = pf_gate_desc;

        // IRQs
        idt.descriptor_table[InterruptVector::APIC_TIMER] = lapic_timer_irq_gate_desc;
        idt.descriptor_table[InterruptVector::APIC_SPURIOUS] = lapic_spurious_irq_gate_desc;

        idt
    };
}

pub fn init_idt() {
    unsafe {
        InterruptDescriptorTable::load_idt(&IDT.pointer());
        log::info!("IDT initalized and loaded");
        enable_interrupts();
        log::info!("CPU interrupts enabled");
    }
    // TODO -> See if we need to load any other registers here too
}
