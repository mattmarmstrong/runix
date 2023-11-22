use crate::device::serial::Port;

pub const PIT_FREQUENCY: usize = 1193182; // Rounding 1.1931816666...

pub const PIT_CHANNEL_0_PORT_NUMBER: u16 = 0x40;
pub const PIT_CHANNEL_1_PORT_NUMBER: u16 = 0x41;
pub const PIT_CHANNEL_2_PORT_NUMBER: u16 = 0x42;
pub const PIT_COMMAND_PORT_NUMBER: u16 = 0x43;

#[derive(Debug)]
pub struct PIT {
    channel_0: Port,
    _channel_1: Port, // System-specific port
    _channel_2: Port,
    command: Port,
}

impl PIT {
    pub fn new() -> Self {
        let channel_0 = Port::new(PIT_CHANNEL_0_PORT_NUMBER, true);
        let _channel_1 = Port::new(PIT_CHANNEL_1_PORT_NUMBER, true);
        let _channel_2 = Port::new(PIT_CHANNEL_2_PORT_NUMBER, true);
        let command = Port::new(PIT_COMMAND_PORT_NUMBER, true);
        PIT {
            channel_0,
            _channel_1,
            _channel_2,
            command,
        }
    }
    pub fn set_count(&self, count: u16) {
        let low_byte: u8 = (count & 0xFF) as u8;
        let high_byte: u8 = ((count & 0xFF00) >> 8) as u8;
        self.channel_0.write_byte_to_port(low_byte);
        self.channel_0.write_byte_to_port(high_byte);
    }

    pub fn read_count(&self) -> u16 {
        self.command.write_byte_to_port(0x00);
        let low_byte = self.channel_0.read_byte_from_port() as u16;
        let high_byte = (self.channel_0.read_byte_from_port() as u16) << 8;
        high_byte | low_byte
    }
}
