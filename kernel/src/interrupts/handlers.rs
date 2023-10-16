use log;

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
    log::error!("REGISTERS: {}", exception_stack_frame.registers);
    panic!();
}

#[no_mangle]
pub extern "C" fn double_fault_secondary_handler(exception_stack_frame: &mut ExceptionStackFrameWithErrorCode) {
    log::error!("DOUBLE FAULT EXCEPTION");
    log::error!("EXCEPTION REGISTERS: {}", exception_stack_frame.interrupt_registers);
    log::error!("REGISTERS: {}", exception_stack_frame.registers);
    log::error!("ERROR CODE: {:#X}", exception_stack_frame.error_code);
    panic!();
}

interrupt!(divide_by_zero, divide_by_zero_secondary_handler);
interrupt_with_error_code!(double_fault, double_fault_secondary_handler);
