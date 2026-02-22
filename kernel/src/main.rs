// main.rs

#![no_std]
#![no_main]

pub mod drivers;

use bootloader_api::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
