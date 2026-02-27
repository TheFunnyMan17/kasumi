// src/arch/x86_64/idt.rs @ kernel

use spin::Once;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use super::tss::DOUBLE_FAULT_IST_INDEX;

static IDT: Once<InterruptDescriptorTable> = Once::new();

pub fn init() {
    let idt = IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);

        // double fault needs a known-good stack via IST
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }

        idt.page_fault.set_handler_fn(page_fault_handler);

        idt
    });

    idt.load();
}

// handlers

extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    crate::sprintln!("EXCEPTION: breakpoint\n{:#?}", frame);
    // execution continues after the int3
}

extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: double fault\n{:#?}", frame);
}

extern "x86-interrupt" fn page_fault_handler(
    frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    crate::sprintln!("EXCEPTION: page fault");
    crate::sprintln!("  accessed address : {:?}", Cr2::read());
    crate::sprintln!("  error code       : {:?}", error_code);
    crate::sprintln!("{:#?}", frame);

    loop {}
}
