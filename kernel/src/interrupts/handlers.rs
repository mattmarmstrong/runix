use log;

use crate::cpu::LOCAL_APIC;
use crate::interrupts::{
    ExceptionStackFrame,
    ExceptionStackFrameWithErrorCode,
};
use crate::{
    interrupt,
    interrupt_with_error_code,
};

#[no_mangle]
pub extern "C" fn divide_by_zero_secondary_handler(exception_stack_frame: &mut ExceptionStackFrame) {
    log::error!("DIVIDE BY ZERO EXCEPTION");
    log::error!("EXCEPTION REGISTERS: {}", exception_stack_frame.interrupt_registers);
    log::error!("EXECUTION STATE: {}", exception_stack_frame.execution_state);
    panic!();
}

#[no_mangle]
pub extern "C" fn double_fault_secondary_handler(exception_stack_frame: &mut ExceptionStackFrameWithErrorCode) {
    log::error!("DOUBLE FAULT EXCEPTION");
    log::error!("EXCEPTION REGISTERS: {}", exception_stack_frame.interrupt_registers);
    log::error!("EXECUTION STATE: {}", exception_stack_frame.execution_state);
    log::error!("ERROR CODE: {:#X}", exception_stack_frame.error_code);
    panic!();
}

#[no_mangle]
pub extern "C" fn page_fault_secondary_handler(exception_stack_frame: &mut ExceptionStackFrameWithErrorCode) {
    log::error!("PAGE FAULT EXCEPTION");
    log::error!("EXCEPTION REGISTERS: {}", exception_stack_frame.interrupt_registers);
    log::error!("EXECUTION STATE: {}", exception_stack_frame.execution_state);
    log::error!("ERROR CODE: {:#X}", exception_stack_frame.error_code);
    panic!();
}

#[no_mangle]
pub extern "C" fn timer_interrupt_secondary_handler(_exception_stack_frame: &mut ExceptionStackFrame) {
    log::info!("LAPIC TIMER IRQ");
    LOCAL_APIC.get().unwrap().signal_end_of_interrupt();
}

#[no_mangle]
pub extern "C" fn spurious_interrupt_secondary_handler(_exception_stack_frame: &mut ExceptionStackFrame) {
    log::error!("SPURIOUS INTERRUPT RECIEVED");
    LOCAL_APIC.get().unwrap().signal_end_of_interrupt();
}

// Exceptions
interrupt!(divide_by_zero, divide_by_zero_secondary_handler);
interrupt_with_error_code!(double_fault, double_fault_secondary_handler);
interrupt_with_error_code!(page_fault, page_fault_secondary_handler);

// IRQs
interrupt!(lapic_timer_interrupt, timer_interrupt_secondary_handler);
interrupt!(lapic_spurious_interrupt, spurious_interrupt_secondary_handler);
