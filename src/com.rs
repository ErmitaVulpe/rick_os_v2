use core::{
    arch::asm,
    fmt::{self, Write}
};
use spin::Mutex;

const SERIAL_PORT_ADDR: u16 = 0x3F8;

static mut SERIAL_WRITER: Mutex<SerialWriter> = Mutex::new(SerialWriter);
struct SerialWriter;

impl fmt::Write for SerialWriter {
    #[no_mangle]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            for byte in s.bytes() {
                match byte {
                    // '\n' doesn't insert the carry return by itself so it has to be done manually
                    b'\n' => {
                        asm!(
                            "mov al, 0x0A",
                            "out dx, al",
                            "mov al, 0x0D",
                            "out dx, al",
                            in("dx") SERIAL_PORT_ADDR,
                        );
                    }
                        
                    _ => {
                        asm!(
                            "out dx, al",
                            in("dx") SERIAL_PORT_ADDR,
                            in("al") byte,
                        );
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
    unsafe {
        SERIAL_WRITER.lock().write_fmt(args).unwrap();
    }
}

