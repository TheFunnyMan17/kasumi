// src/arch/x86_64/gdt.rs @ kernel

use spin::Once;
use x86_64::structures::gdt::{
    GlobalDescriptorTable,
    Descriptor,
    SegmentSelector,
};
use x86_64::registers::segmentation::{CS, DS, Segment};
use x86_64::instructions::tables::load_tss;
use super::tss;

/// GDT selectors.
struct Selectors {
    code: SegmentSelector,
    data: SegmentSelector,
    tss:  SegmentSelector,
}

static GDT: Once<(GlobalDescriptorTable, Selectors)> = Once::new();

pub fn init() {
    // build the table and store it statically
    // (null descriptor added automatically by new())
    let (gdt, selectors) = GDT.call_once(|| {
        let mut gdt = GlobalDescriptorTable::new();
        let code = gdt.append(Descriptor::kernel_code_segment());
        let data = gdt.append(Descriptor::kernel_data_segment());
        let tss  = gdt.append(Descriptor::tss_segment(tss::tss()));
        (gdt, Selectors { code, data, tss })
    });

    // load the table (really just call `lgdt`)
    gdt.load();

    // reload segment registers to point at our new entries, then load TSS
    unsafe {
        CS::set_reg(selectors.code);
        DS::set_reg(selectors.data);
        load_tss(selectors.tss);
    }
}