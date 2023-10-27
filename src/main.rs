use std::env::current_exe;
use std::fs::copy;
use std::process::{exit, Command};

use ovmf_prebuilt::ovmf_pure_efi;

fn main() {
    let current_exe = current_exe().unwrap();

    let uefi_target = current_exe.with_file_name("uefi.img");
    copy(env!("UEFI_IMAGE"), &uefi_target).unwrap();
    println!("UEFI disk image at {}", uefi_target.display());

    let mut qemu = Command::new("qemu-system-x86_64");
    qemu.arg("-drive")
        .arg(format!("format=raw,file={}", env!("UEFI_IMAGE")))
        .arg("-bios")
        .arg(ovmf_pure_efi())
        .arg("-serial")
        .arg("file:serial_output.log")
        .arg("-d")
        .arg("int")
        .arg("-no-reboot");

    println!("Starting QEMU with command: {:#?}", qemu);
    let exit_status = qemu.status().unwrap();
    exit(exit_status.code().unwrap_or(-1));
}
