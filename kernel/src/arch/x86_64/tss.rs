// src/arch/x86_64/tss.rs @ kernel

use spin::Once;
use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;

/// IST slot used by the double fault handler
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

const STACK_SIZE: usize = 4096 * 5;

// aligned so the stack top is 16-byte aligned
#[repr(align(16))]
struct Stack([u8; STACK_SIZE]);

static DOUBLE_FAULT_STACK: Stack = Stack([0; STACK_SIZE]);

static TSS: Once<TaskStateSegment> = Once::new();

/// returns a reference to the (already-initialised) TSS
/// panics if called before `init()`
pub fn tss() -> &'static TaskStateSegment {
    TSS.get().expect("TSS not initialized — call tss::init() before gdt::init()")
}

pub fn init() {
    TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();

        // point IST slot 0 at the top (high address) of our dedicated stack
        // The stack grows downward, so the IST entry is the end of the array
        let stack_start = VirtAddr::from_ptr(DOUBLE_FAULT_STACK.0.as_ptr());
        let stack_top   = stack_start + STACK_SIZE as u64;
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = stack_top;

        tss
    });
}
