// The device, cpu and interrupt modules are fairly intertwined. We need devices to generate interrupts,
// and we need interrupt handlers to tell the CPU what do do during those interrupts.

pub mod asm;
pub mod keyboard;
pub mod mouse;
pub mod serial;
