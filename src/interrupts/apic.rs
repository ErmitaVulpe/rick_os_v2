use pic8259::ChainedPics;
use x86::apic::{xapic, ApicControl};
use core::{mem::ManuallyDrop, ops::{Deref, DerefMut}, ptr::{read_volatile, write_volatile}};


pub static mut PICS: ChainedPics = unsafe { ChainedPics::new(32, 8) };
pub static mut LOCAL_APIC: LocalApic = LocalApic { public: PublicXapic::empty() };


pub union LocalApic {
    apic: ManuallyDrop<xapic::XAPIC>,
    public: PublicXapic,
}

impl LocalApic {
    pub fn get(&self) -> &xapic::XAPIC {
        unsafe { self.apic.deref() }
    }

    pub fn get_mut(&mut self) -> &mut xapic::XAPIC {
        unsafe { self.apic.deref_mut() }
    }

    pub unsafe fn init(&mut self, mimo: *mut u32) {
        PICS.initialize();
        PICS.disable();

        let mut local_apic = xapic::XAPIC::new(core::slice::from_raw_parts_mut(mimo, 256));
        local_apic.attach();
        *self.get_mut() = local_apic;

        x86_64::instructions::interrupts::enable();
    }

    pub fn enable_timer(&mut self) {
        unsafe {
            let mimo_addr = self.public.mimo_region_ptr;
            
            let svr = read_volatile((mimo_addr + xapic::XAPIC_SVR as u64) as *const u32);
            write_volatile((mimo_addr + xapic::XAPIC_SVR as u64) as *mut u32, svr | 0x100);

            // set to periodic mode and map to 32nd idt entry
            write_volatile(
                (mimo_addr + xapic::XAPIC_LVT_TIMER as u64) as *mut u32,
                super::InterruptIndex::APICTimer as u32 | (1 << 17)
            );

            // timer divider
            write_volatile((mimo_addr + xapic::XAPIC_TIMER_DIV_CONF as u64) as *mut u32, 0b1010);
            
            // timer count
                write_volatile((mimo_addr + xapic::XAPIC_TIMER_INIT_COUNT as u64) as *mut u32, 0x200000);

            let apic_version = LOCAL_APIC.get().version() & 0xFF;
            match apic_version {
                0x0..=0xF => {
                    todo!("Local APIC is 82489DX discrete APIC");
                },
                0x10..=0x15 => {
                    // todo!("Integrated APIC");
                },
                _ => unreachable!("Reserved value"),
            }
        }
    }
}

#[derive(Clone, Copy)]
struct PublicXapic {
    mimo_region_ptr: u64,
    mimo_region_lenght: u64,
    base: u64,
}

impl PublicXapic {
    const fn empty() -> Self {
        Self {
            mimo_region_ptr: 0,
            mimo_region_lenght: 0,
            base: 0,
        }
    }
}

