use bootloader_api::info::FrameBufferInfo;
use bootloader_api::BootInfo;
use bootloader_x86_64_common::logger::LockedLogger;
use log;

use crate::LOGGER;

// The bootloader provides most of the abstractions here
pub fn init_kernel_logging(boot_info: &'static mut BootInfo) {
    // this is a Option<Optional<FrameBuffer>>. The API is kind of ugly
    let boot_info_framebuffer = &mut boot_info.framebuffer;
    let framebuffer_option = boot_info_framebuffer.as_mut();
    let framebuffer = framebuffer_option.unwrap();
    let framebuffer_info: FrameBufferInfo = framebuffer.info().clone();
    let raw_char_buffer: &'static mut [u8] = framebuffer.buffer_mut();
    let logger = LOGGER.get_or_init(move || LockedLogger::new(raw_char_buffer, framebuffer_info, true, false));
    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("RUNIX kernel logging enabled");
}
