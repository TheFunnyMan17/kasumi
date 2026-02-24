// src/main.rs @ kernel

#![no_std]
#![no_main]

pub mod drivers;

use bootloader_api::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    drivers::uart::init();

    sprintln!(":3");

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    sprintln!("PANIC: {}", _info);
    loop {}
}
