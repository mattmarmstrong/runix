use std::env;
use std::path::PathBuf;

use bootloader::DiskImageBuilder;

fn main() {
    let kernel_path = env::var("CARGO_BIN_FILE_KERNEL").unwrap();
    let kernel_disk_image_builder = DiskImageBuilder::new(PathBuf::from(kernel_path.clone()));
    println!("{}", &kernel_path);
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let uefi_path = out_dir.join("runix-uefi.img");

    kernel_disk_image_builder.create_uefi_image(&uefi_path).unwrap();

    // Exporting the UEFI_IMAGE variable for the rust compiler to use
    println!("cargo:rustc-env=UEFI_IMAGE={}", uefi_path.display());
}
