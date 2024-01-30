#![no_std]
#![feature(abi_x86_interrupt)]

use core::{arch::asm, panic::PanicInfo};
use multiboot2::{BootInformation, BootInformationHeader};

mod com;
mod idt;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC!");
    println!("{}", info);
    unsafe { asm!("hlt") };
    loop {}
}


#[no_mangle]
pub extern "C" fn kmain(init_eax: u32, init_ebx: u32) -> ! {
    // Check for magic number
    if init_eax != multiboot2::MAGIC {
        panic!("Bootloader is not multiboot2 compliant.");
    }

    idt::init();

    let boot_info = unsafe {
        BootInformation::load(init_ebx as *const BootInformationHeader).unwrap()
    };


    let framebuffer_info = boot_info.framebuffer_tag().unwrap().unwrap();
    println!("{:#?}", framebuffer_info);

    let address = framebuffer_info.address();
    let framebuffer_ptr = address as *mut u8;

    println!("{:#?}", boot_info.basic_memory_info_tag().unwrap());
    println!("{:#?}", boot_info.memory_map_tag().unwrap());
    


    println!("Success");
    println!("{}", framebuffer_ptr as u64);
    static mut BUFFER: [u8 ; 3145728] = [0u8 ; 3145728];

    loop {
        for c in 0..=255 {
            unsafe {
                core::ptr::copy(BUFFER.as_ptr(), framebuffer_ptr, 3145728);
                BUFFER = [c ; 3145728];
            }
        }
    }
}
