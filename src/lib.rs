#![no_std]
#![feature(abi_x86_interrupt, const_mut_refs)]

use core::{arch::asm, panic::PanicInfo};
use alloc::vec;
use lazy_static::lazy_static;
use multiboot2::{BootInformation, BootInformationHeader};
use x86::cpuid::CpuId;

extern crate alloc;

mod acpi;
mod com;
mod memory;
mod pci;
mod interrupts;
mod task;
mod tests;

use acpi::acpi::madt::MadtEntry;
use task::{executor::Executor, keyboard, Task};
use interrupts::apic::LOCAL_APIC;


lazy_static! {
    pub static ref CPUID: CpuId = CpuId::new();
}


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
    interrupts::idt::init();
    tests::run_tests();
    
    let mbi = unsafe {
        BootInformation::load(mbi_ptr as *const BootInformationHeader).unwrap()
    };
    
    // init allocator
    unsafe { memory::init(&mbi) };
    println!("|||||| Multiboot memmory map:");
    println!("{:#?}", mbi.memory_map_tag().unwrap());

    // parse acpi
    let acpi_tables = acpi::read(&mbi);
    let madt = acpi::read_madt(&acpi_tables);
    let fadt = acpi::read_fadt(&acpi_tables);

    // parse madt
    let mut madt_lapic = None;
    let mut madt_ioapic = None;
    let mut madt_lapic_override = None;
    for entry in madt.entries() {
        println!("{:#?}", entry);
        match entry {
            MadtEntry::LocalApic(val) => madt_lapic = Some(val),
            MadtEntry::IoApic(val) => madt_ioapic = Some(val),
            MadtEntry::LocalApicAddressOverride(val) => madt_lapic_override = Some(val),
            _ => {},
        }
    }
    let madt_ioapic = madt_ioapic.unwrap();


    println!("|||||| io apic madt entry:");
    println!("{:#?}", madt_ioapic);

    // check for address override
    let lapic_addr = if let Some(lapic_override) = madt_lapic_override {
        lapic_override.local_apic_address
    } else {
        madt.local_apic_address as u64
    };

    // init lapic
    unsafe {
        LOCAL_APIC.init( lapic_addr as *mut u32 );
        LOCAL_APIC.enable_timer();
    }
    
    // init iopic
    let mut ioapic = unsafe {
        interrupts::ioapic::IoApic::new(madt_ioapic.io_apic_address as usize)
    };

    // println!("{:#?}", fadt.dsdt_address()); // TODO


    let framebuffer_tag = mbi.framebuffer_tag().unwrap().unwrap();
    let address = framebuffer_tag.address();
    let framebuffer_size = framebuffer_tag.pitch() as usize * framebuffer_tag.height() as usize;
    let framebuffer_ptr = address as *mut u8;


    { // PCIE
        // use pci::*;

        // let hda_device = try_find_device(|d| {
        //     let class = d.get_class_register();
        //     class.class == 4 && class.subclass == 3
        // }).unwrap();

        
        // let hda_memory_ptr = {
        //     let bar0 = hda_device.read_register(0x10);
        //     // check for register location
        //     match bar0 & 0b1 == 1 {
        //         true => { // I/O Space
        //             (bar0 & !0b11) as u64
        //         }
        //         false => { // Memory Space
        //             match bar0 >> 1 & 0b11 {
        //                 0 => (bar0 & !0xF) as u64,
        //                 0x2 => {
        //                     let bar1 = hda_device.read_register(0x14);
        //                     ((bar1 as u64) << 32) & ((bar0 as u64) & !0xF)
        //                 }
        //                 _ => unreachable!()
        //             }
        //         }
        //     }
        // };

        // println!("{:0b}", hda_memory_ptr);
        // {
        //     let reg = unsafe {
        //         (hda_memory_ptr as *const u32).read()
        //     };
        //     println!("Number of Output streams: {}", reg >> 12 & 0xF);
        //     println!("Number of Input streams: {}", reg >> 8 & 0xF);
        //     println!("Number of Bidirectional streams: {}", reg >> 3 & 0x1F);
        //     println!("64 bit addressing is supported: {}", reg & 0x1);
        // }
        // {
        //     let reg = unsafe {
        //         ((hda_memory_ptr + 0x44) as *const u32).read()
        //     };
        //     println!("{:0b}", reg);
        // }
    }

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
