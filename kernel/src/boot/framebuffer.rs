use bootloader_api::info::{
    FrameBuffer,
    FrameBufferInfo,
};
use bootloader_x86_64_common::logger::LockedLogger;
use log;

use crate::LOGGER;

pub fn init_kernel_logging(framebuffer: FrameBuffer) {
    let framebuffer_info: FrameBufferInfo = framebuffer.info().clone();
    let raw_char_buffer: &'static mut [u8] = framebuffer.into_buffer();
    let logger = LOGGER.get_or_init(move || LockedLogger::new(raw_char_buffer, framebuffer_info, true, false));
    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("RUNIX kernel logging enabled");
}
