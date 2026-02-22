// main.rs @ builder

use std::path::PathBuf;

const OVMF_CODE: &str = "/usr/share/edk2/x64/OVMF_CODE.4m.fd";

fn main() {
    let kernel_path = PathBuf::from(env!("CARGO_BIN_FILE_KERNEL_kernel"));
    let uefi_path = PathBuf::from("target/kasumi-uefi.img");

    bootloader::UefiBoot::new(&kernel_path)
        .create_disk_image(&uefi_path)
        .unwrap();

    println!("UEFI image: {}", uefi_path.display());

    let status = std::process::Command::new("qemu-system-x86_64")
        .args(["-drive", &format!("if=pflash,format=raw,readonly=on,file={OVMF_CODE}")])
        .args(["-drive", &format!("format=raw,file={}", uefi_path.display())])
        .args(["-serial", "stdio"])
        .status()
        .expect("failed to launch QEMU");

    std::process::exit(status.code().unwrap_or(-1));
}
