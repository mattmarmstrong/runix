use crate::mmu::address::VirtualAddress;

pub mod file;
pub mod kernel;
pub mod process;

// FIXME
const MAX_THREADS: usize = 1;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RegisterState {
    r15: usize,
    r14: usize,
    r13: usize,
    r12: usize,
    r11: usize,
    r10: usize,
    r9: usize,
    r8: usize,
    rbp: usize,
    rdi: usize,
    rsi: usize,
    rdx: usize,
    rcx: usize,
    rbx: usize,
    rax: usize,
}

impl core::fmt::Display for RegisterState {
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
pub enum ThreadState {
    Ready,
    Running,
    Blocked,
    Sleeping,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Thread {
    register_state: RegisterState,
    program_counter: usize, // RIP
    stack_pointer: usize,   // RSP
    state: ThreadState,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Process {
    process_id: usize,
    parent_pid: Option<usize>,
    thread_table: [Thread; MAX_THREADS],
    scheduling_priority: usize,
    ticks_left: usize,
    cpu_time_used: usize,
    text_seg_addr: VirtualAddress,
    data_seg_addr: VirtualAddress,
    bss_seg_addr: VirtualAddress,
}
