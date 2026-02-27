// src/main.rs @ kernel

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod drivers;
pub mod arch;
pub mod macros;

use bootloader_api::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    steps! {
        "uart" => drivers::uart::init(),
        "tss"  => arch::x86_64::tss::init(),
        "gdt"  => arch::x86_64::gdt::init(),
        "idt"  => arch::x86_64::idt::init(),
    }

    x86_64::instructions::interrupts::int3();

    sprintln!("we are still alive :)");

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    sprintln!("PANIC: {}", _info);
    loop {}
}
