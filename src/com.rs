use core::fmt::{self, Write};
use spin::Mutex;
use x86_64::instructions::port::Port;

static mut SERIAL_WRITER: Mutex<SerialWriter> = Mutex::new(SerialWriter::new(0x3F8));
struct SerialWriter {
    port: Port<u8>,
}

impl SerialWriter {
    const fn new(port_addr: u16) -> Self {
        Self {
            port: Port::new(port_addr)
        }
    }
}

impl fmt::Write for SerialWriter {
    #[no_mangle]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            for byte in s.bytes() {
                match byte {
                    // '\n' doesn't insert the carry return by itself so it has to be done manually
                    b'\n' => {
                        self.port.write(0x0A);
                        self.port.write(0x0D);
                    }
                        
                    _ => {
                        self.port.write(byte);
                    }
                } 
            }
        }
        Ok(())
    }
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::com::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| unsafe {
        SERIAL_WRITER.lock().write_fmt(args).unwrap();
    });
}

