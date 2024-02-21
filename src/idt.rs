use crate::println;
use core::arch::asm;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    _stack_frame: InterruptStackFrame
) {
    println!("EXCEPTION: BREAKPOINT");
    unsafe { asm!("hlt") };
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    panic!("EXCEPTION: PAGE FAULT (0x{:0X})\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT (0x{:0X})\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn stack_segment_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("EXCEPTION: STACK SEGMENT FAULT (0x{:0X})\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("EXCEPTION: GENERAL PROTECTION FAULT (0x{:0X})\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn divide_error_handler(
    stack_frame: InterruptStackFrame,
) {
    panic!("EXCEPTION: DIVIDE ERROR\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_tss_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("EXCEPTION: INVALID TSS (0x{:0X})\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("EXCEPTION: SEGMENT NOT PRESENT (0x{:0X})\n{:#?}", error_code, stack_frame);
}
