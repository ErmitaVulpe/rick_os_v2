#![no_std]
#![feature(abi_x86_interrupt, const_mut_refs)]

use core::{arch::asm, panic::PanicInfo};
use alloc::vec;
use multiboot2::{BootInformation, BootInformationHeader};

extern crate alloc;

mod com;
mod memory;
mod interrupts;
mod task;

use task::{executor::Executor, keyboard, Task};


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC!");
    println!("{}", info);
    loop {
        unsafe { asm!("hlt") };
    }
}


#[no_mangle]
pub extern "C" fn kmain(mbi_ptr: u32) -> ! {
    interrupts::init();

    let mbi = unsafe {
        BootInformation::load(mbi_ptr as *const BootInformationHeader).unwrap()
    };

    memory::init(&mbi);


    let framebuffer_tag = mbi.framebuffer_tag().unwrap().unwrap();
    let address = framebuffer_tag.address();
    let framebuffer_size = framebuffer_tag.pitch() as usize * framebuffer_tag.height() as usize;
    let framebuffer_ptr = address as *mut u8;


    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.spawn(Task::new(example_task()));
    executor.run();


    println!("Success");
    let mut buffer: vec::Vec<u8> = vec![0u8 ; framebuffer_size];

    loop {
        for c in 0..=255 {
            unsafe {
                framebuffer_ptr.copy_from(buffer.as_ptr(), framebuffer_size);
                buffer = vec![c ; framebuffer_size];
            }
        }
    }
}




async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
