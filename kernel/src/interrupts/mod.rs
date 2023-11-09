pub mod asm;
pub mod handlers;
pub mod idt;

use lazy_static::lazy_static;

use crate::interrupts::asm::enable_interrupts;
use crate::interrupts::handlers::*;
use crate::interrupts::idt::{
    GateDescriptor,
    GateOptions,
    InterruptDescriptorTable,
};
use crate::mmu::VirtualAddress;
use crate::segmentation::tss::*;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
// These are the register values that the CPU pushes to the stack when an exception occurs
pub struct ExceptionRegisters {
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

// Register values that are manually pushed to stack during interrupts.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExecutionState {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rbp: u64,
    rdi: u64,
    rsi: u64,
    rdx: u64,
    rcx: u64,
    rbx: u64,
    rax: u64,
}

impl core::fmt::Display for ExceptionRegisters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "RIP: {:#X} CS: {:#X} RFLAGS: {:#X}\nRSP: {:#X} SS: {:#X}",
            self.rip, self.cs, self.rflags, self.rsp, self.ss
        ))
    }
}

impl core::fmt::Display for ExecutionState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "RAX: {:#X} RBX: {:#X} RCX: {:#X}\nRDX: {:#X} RSI: {:#X} RDI: {:#X}\nRBP: {:#X} R8: {:#X} R9: {:#X}\nR10: {:#X} R11: {:#X} R12: {:#X}\nR13: {:#X} R14: {:#X} R15: {:#X}",
            self.rax,
            self.rbx,
            self.rcx,
            self.rdx,
            self.rsi,
            self.rdi,
            self.rbp,
            self.r8,
            self.r9,
            self.r10,
            self.r11,
            self.r12,
            self.r13,
            self.r14,
            self.r15,
        ))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExceptionStackFrame {
    execution_state: ExecutionState,
    interrupt_registers: ExceptionRegisters,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExceptionStackFrameWithErrorCode {
    execution_state: ExecutionState,
    error_code: u64,
    interrupt_registers: ExceptionRegisters,
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
    pub const APIC_IRQ: usize = 0x20;
    pub const SYSCALL: usize = 0x80;
}

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Gate descriptors
        let mut divide_by_zero_gate_descriptor = GateDescriptor::new(GateOptions::exception_gate_options());
        divide_by_zero_gate_descriptor.set_handler_address(VirtualAddress::new(divide_by_zero as u64));

        let double_fault_gate_options = GateOptions::exception_gate_options().set_stack_index(DOUBLE_FAULT_STACK_TABLE_INDEX);
        let mut double_fault_gate_descriptor = GateDescriptor::new(double_fault_gate_options);
        double_fault_gate_descriptor.set_handler_address(VirtualAddress::new(double_fault as u64));

        idt.descriptor_table[InterruptVector::DIVIDE_ERROR] = divide_by_zero_gate_descriptor;
        idt.descriptor_table[InterruptVector::DOUBLE_FAULT] = double_fault_gate_descriptor;

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
