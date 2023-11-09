use core::arch::asm;

#[derive(Debug, Clone, Copy)]
pub struct Port {
    port_number: u16,
    writable: bool,
}

impl Port {
    pub const fn new(port_number: u16, writable: bool) -> Self {
        Port { port_number, writable }
    }

    pub fn read_byte_from_port(&self) -> u8 {
        let read_value: u8;
        unsafe {
            asm!("in al, dx", out("al") read_value, in("dx") self.port_number, options(nomem, nostack, preserves_flags));
        }
        read_value
    }

    pub fn write_byte_to_port(&self, write_value: u8) {
        if self.writable {
            unsafe {
                asm!("out dx, al", in("dx") self.port_number, in("al") write_value, options(nomem, nostack, preserves_flags));
            }
        } else {
            log::error!("Port: {:#X} not writable!", self.port_number)
        }
    }

    pub fn read_word_from_port(&self) -> u16 {
        let read_value: u16;
        unsafe {
            asm!("in ax, dx", out("ax") read_value, in("dx") self.port_number, options(nomem, nostack, preserves_flags));
        }
        read_value
    }

    pub fn write_word_to_port(&self, write_value: u16) {
        if self.writable {
            unsafe {
                asm!("out dx, ax", in("dx") self.port_number, in("ax") write_value, options(nomem, nostack, preserves_flags));
            }
        } else {
            log::error!("Port: {:#X} not writable!", self.port_number)
        }
    }

    pub fn read_long_from_port(&self) -> u32 {
        let read_value: u32;
        unsafe {
            asm!("in eax, dx", out("eax") read_value, in("dx") self.port_number, options(nomem, nostack, preserves_flags));
        }
        read_value
    }

    pub fn write_long_to_port(&self, write_value: u32) {
        if self.writable {
            unsafe {
                asm!("out dx, eax", in("dx") self.port_number, in("eax") write_value, options(nomem, nostack, preserves_flags));
            }
        } else {
            log::error!("Port: {:#X} not writable!", self.port_number)
        }
    }
}
