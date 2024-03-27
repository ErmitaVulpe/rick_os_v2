use crate::{print, println};
use core::arch::asm;
use lazy_static::lazy_static;
use x86::apic::ApicControl;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};


#[derive(Debug, PartialEq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum InterruptIndex {
    DivideError,
    Debug,
    NonMaskableInterrupt,
    Breakpoint,
    Overflow,
    BoundRangeExceeded,
    InvalidOpcode,
    DeviceNotAvailable,
    DoubleFault,
    CoprocessorSegmentOverrun,
    InvalidTSS,
    SegmentNotPResent,
    StackSegmentFault,
    GeneralProtectionFault,
    PageFault,
    // 15 (0xF) is reserved
    X87FloatingPoint = 16,
    AlignmentCheck,
    MachineCheck,
    SimdFloatingPoint,
    
    // PIC 8259
    PicTimer = 32,
    Keyboard,

    // APIC
    APICTimer = 48,
}


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

        idt[InterruptIndex::PicTimer as usize].set_handler_fn(pic_timer_interrupt_handler);
        idt[InterruptIndex::Keyboard as usize].set_handler_fn(keyboard_interrupt_handler);

        idt[InterruptIndex::APICTimer as usize].set_handler_fn(apic_timer_interrupt_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler (
    _stack_frame: InterruptStackFrame
) {
    println!("EXCEPTION: BREAKPOINT");
    unsafe { asm!("hlt") };
}

extern "x86-interrupt" fn page_fault_handler (
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    panic!("EXCEPTION: PAGE FAULT (0x{:0X})\n{:#?}", error_code, stack_frame);
}

#[no_mangle]
extern "x86-interrupt" fn double_fault_handler (
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT (0x{:0X})\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn stack_segment_fault_handler (
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("EXCEPTION: STACK SEGMENT FAULT (0x{:0X})\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn general_protection_fault_handler (
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("EXCEPTION: GENERAL PROTECTION FAULT (0x{:0X})\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn divide_error_handler (
    stack_frame: InterruptStackFrame,
) {
    panic!("EXCEPTION: DIVIDE ERROR\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_tss_handler (
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("EXCEPTION: INVALID TSS (0x{:0X})\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn segment_not_present_handler (
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!("EXCEPTION: SEGMENT NOT PRESENT (0x{:0X})\n{:#?}", error_code, stack_frame);
}


extern "x86-interrupt" fn pic_timer_interrupt_handler (
    _stack_frame: InterruptStackFrame)
{
    print!("_");
    unsafe {
        super::apic::PICS
            .notify_end_of_interrupt(InterruptIndex::PicTimer as u8);
    }
}


extern "x86-interrupt" fn keyboard_interrupt_handler (
    _stack_frame: InterruptStackFrame)
{
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    crate::task::keyboard::add_scancode(scancode);

    unsafe {
        super::apic::PICS
            .notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}



extern "x86-interrupt" fn apic_timer_interrupt_handler (
    _stack_frame: InterruptStackFrame)
{
    print!(".");
    unsafe {
        super::apic::LOCAL_APIC.get_mut().eoi();
    }
}
