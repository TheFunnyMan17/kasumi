// src/drivers/uart.rs @ kernel

use spin::{Mutex, Once};
use core::fmt;
use super::port::Port;

const COM1_BASE: u16 = 0x3F8;

/// Represents a serial UART device with I/O ports for communication.
pub struct Serial {
    data:             Port<u8>,
    interrupt_enable: Port<u8>,
    fifo_control:     Port<u8>,
    line_control:     Port<u8>,
    modem_control:    Port<u8>,
    line_status:      Port<u8>,
    scratch:          Port<u8>,
}

impl Serial {
    /// Create a new Serial instance at the specified COM port base address.
    unsafe fn new(base: u16) -> Option<Self> {
        let uart = Serial {
            data:             Port::new(base + 0),
            interrupt_enable: Port::new(base + 1),
            fifo_control:     Port::new(base + 2),
            line_control:     Port::new(base + 3),
            modem_control:    Port::new(base + 4),
            line_status:      Port::new(base + 5),
            scratch:          Port::new(base + 7),
        };

        // check the hardware actually exists
        uart.scratch.write(0xAE);
        if uart.scratch.read() != 0xAE {
            return None;
        }

        // Disable interrupts while we configure
        uart.interrupt_enable.write(0x00);

        // set DLAB bit to access divisor registers
        uart.line_control.write(0x80);
        uart.data.write(0x01);            // divisor low
        uart.interrupt_enable.write(0x00); // divisor high

        // 8N1, clear DLAB
        uart.line_control.write(0x03);

        // enable FIFO, clear it, 14-byte threshold
        uart.fifo_control.write(0xC7);

        // enable DTR + RTS
        uart.modem_control.write(0x0B);

        Some(uart)
    }


    /// Check if the serial port is ready to accept more data.
    fn is_transmit_ready(&self) -> bool {
        unsafe { self.line_status.read() & 0x20 != 0 }
    }

    /// Write a byte to the serial port.
    fn write_byte(&self, byte: u8) {
        while !self.is_transmit_ready() {
            core::hint::spin_loop();
        }
        unsafe { self.data.write(byte) }
    }

    /// Read a byte from the serial port, if one is available.
    pub fn read_byte(&self) -> Option<u8> {
        unsafe {
            if self.line_status.read() & 0x01 != 0 {
                Some(self.data.read())
            } else {
                None
            }
        }
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            if byte == b'\n' {
                // we need this to avoid line staircases
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
        Ok(())
    }
}

/// Global singleton instance of the serial port driver.
pub static SERIAL: Once<Mutex<Serial>> = Once::new();

/// Initialize the serial port driver.
pub fn init() {
    SERIAL.call_once(|| {
        Mutex::new(unsafe { Serial::new(COM1_BASE).expect("COM1 not found") })
    });
}

// macros for serial printing

#[macro_export]
macro_rules! sprint {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut serial = $crate::drivers::uart::SERIAL.get().unwrap().lock();
        write!(serial, $($arg)*).ok();
    }};
}

#[macro_export]
macro_rules! sprintln {
    () => { $crate::sprint!("\n") };
    ($fmt:literal $(, $arg:expr)*) => {
        $crate::sprint!(concat!($fmt, "\n") $(, $arg)*)
    };
}