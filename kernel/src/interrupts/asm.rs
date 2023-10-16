use core::arch::asm;

#[macro_export]
macro_rules! interrupt {
    ($interrupt_name:ident, $rust_secondary_handler:ident) => {
        #[naked]
        pub unsafe extern "C" fn $interrupt_name() {
            core::arch::asm!(
                    "cld;",
                    "push rax",
                    "push rbx",
                    "push rcx",
                    "push rdx",
                    "push rsi",
                    "push rdi",
                    "push rbp",
                    "push r8",
                    "push r9",
                    "push r10",
                    "push r11",
                    "push r12",
                    "push r13",
                    "push r14",
                    "push r15",
                    "mov ax, 0x10",
                    "mov ds, ax",
                    "mov es, ax",
                    "mov fs, ax",
                    "mov gs, ax",
                    "mov rdi, rsp",
                    "sub rsp, 8",
                    "call {}",
                    "pop r15",
                    "pop r14",
                    "pop r13",
                    "pop r12",
                    "pop r11",
                    "pop r10",
                    "pop r9",
                    "pop r8",
                    "pop rbp",
                    "pop rdi",
                    "pop rsi",
                    "pop rdx",
                    "pop rcx",
                    "pop rbx",
                    "pop rax",
                    "iretq",
                    sym $rust_secondary_handler,
                options(noreturn))
        }
    };

}

#[macro_export]
macro_rules! interrupt_with_error_code {
    ($interrupt_name:ident, $rust_secondary_handler:ident) => {
        #[naked]
        pub unsafe extern "C" fn $interrupt_name() {
            core::arch::asm!(
                    "cld;",
                    "push rax",
                    "push rbx",
                    "push rcx",
                    "push rdx",
                    "push rsi",
                    "push rdi",
                    "push rbp",
                    "push r8",
                    "push r9",
                    "push r10",
                    "push r11",
                    "push r12",
                    "push r13",
                    "push r14",
                    "push r15",
                    "mov ax, 0x10",
                    "mov ds, ax",
                    "mov es, ax",
                    "mov fs, ax",
                    "mov gs, ax",
                    "mov rdi, rsp",
                    "sub rsp, 8",
                    "call {}",
                    "pop r15",
                    "pop r14",
                    "pop r13",
                    "pop r12",
                    "pop r11",
                    "pop r10",
                    "pop r9",
                    "pop r8",
                    "pop rbp",
                    "pop rdi",
                    "pop rsi",
                    "pop rdx",
                    "pop rcx",
                    "pop rbx",
                    "pop rax",
                    "add rsp, 8",
                    "iretq",
                    sym $rust_secondary_handler,
                    options(noreturn),
                    )
        }
    };
}

pub unsafe fn enable_interrupts() {
    asm!("sti", options(nomem, nostack));
}
