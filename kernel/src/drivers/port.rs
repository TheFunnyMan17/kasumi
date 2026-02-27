// src/drivers/port.rs @ kernel

use core::marker::PhantomData;

// private asm implementation

unsafe fn port_read_u8(port: u16) -> u8 {
    let val: u8;
    core::arch::asm!("in al, dx", out("al") val, in("dx") port,
    options(nomem, nostack, preserves_flags));
    val
}

unsafe fn port_read_u16(port: u16) -> u16 {
    let val: u16;
    core::arch::asm!("in ax, dx", out("ax") val, in("dx") port,
    options(nomem, nostack, preserves_flags));
    val
}

unsafe fn port_read_u32(port: u16) -> u32 {
    let val: u32;
    core::arch::asm!("in eax, dx", out("eax") val, in("dx") port,
    options(nomem, nostack, preserves_flags));
    val
}

unsafe fn port_write_u8(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val,
    options(nomem, nostack, preserves_flags));
}

unsafe fn port_write_u16(port: u16, val: u16) {
    core::arch::asm!("out dx, ax", in("dx") port, in("ax") val,
    options(nomem, nostack, preserves_flags));
}

unsafe fn port_write_u32(port: u16, val: u32) {
    core::arch::asm!("out dx, eax", in("dx") port, in("eax") val,
    options(nomem, nostack, preserves_flags));
}

// public trait implementation

pub trait PortValue: Sized {
    unsafe fn read(port: u16) -> Self;
    unsafe fn write(port: u16, val: Self);
}

impl PortValue for u8 {
    unsafe fn read(port: u16) -> Self { port_read_u8(port) }
    unsafe fn write(port: u16, val: Self) { port_write_u8(port, val) }
}

impl PortValue for u16 {
    unsafe fn read(port: u16) -> Self { port_read_u16(port) }
    unsafe fn write(port: u16, val: Self) { port_write_u16(port, val) }
}

impl PortValue for u32 {
    unsafe fn read(port: u16) -> Self { port_read_u32(port) }
    unsafe fn write(port: u16, val: Self) { port_write_u32(port, val) }
}

// Port<T> for easier type inference

/// A hardware I/O port with a specific data type.
pub struct Port<T: PortValue> {
    address: u16,
    _phantom: PhantomData<T>,
}

impl<T: PortValue> Port<T> {
    /// Create a new Port instance at the specified address.
    pub const unsafe fn new(address: u16) -> Self {
        Port { address, _phantom: PhantomData }
    }

    /// Read value of width `T` from the port.
    pub unsafe fn read(&self) -> T {
        T::read(self.address)
    }

    /// Write value of width `T` to the port.
    pub unsafe fn write(&self, val: T) {
        T::write(self.address, val)
    }
}