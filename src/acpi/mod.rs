pub use acpi;
use acpi::madt::Madt;
use acpi::fadt::Fadt;
use acpi::sdt::Signature;
use acpi::handler::AcpiHandler;
use core::mem;
use crate::println;
use acpi::AcpiTables;



#[derive(Clone, Debug)]
pub struct DummyHandler {}

impl AcpiHandler for DummyHandler {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> acpi::PhysicalMapping<Self, T> {
        acpi::PhysicalMapping::new(
            physical_address, 
            core::ptr::NonNull::new(physical_address as *mut T).unwrap(), 
            size, 
            size, 
            self.clone(),
        )
    }
    
    fn unmap_physical_region<T>(region: &acpi::PhysicalMapping<Self, T>) {}
}


pub fn read(mbi: &multiboot2::BootInformation) -> AcpiTables<DummyHandler> {
    let rsdp = mbi.rsdp_v1_tag().unwrap();
    let rsdp_ptr = match mbi.rsdp_v2_tag() {
        // 8 is the size of the multiboot2 header
        Some(rsdp) => rsdp as *const multiboot2::RsdpV2Tag as usize + 8,
        None => mbi.rsdp_v1_tag().unwrap() as *const multiboot2::RsdpV1Tag as usize + 8,
    };

    unsafe {
        acpi::AcpiTables::from_rsdp(
            DummyHandler{}, 
            rsdp_ptr,
        ).unwrap()
    }
}


pub fn read_madt<'a>(acpi_tables: &AcpiTables<DummyHandler>) -> &'a Madt {
    let madt = unsafe {
        acpi_tables.find_table::<Madt>().unwrap().virtual_start().as_ref()
    };
    madt.header.validate(acpi::sdt::Signature::MADT).unwrap();
    madt
}

pub fn read_fadt<'a>(acpi_tables: &AcpiTables<DummyHandler>) -> &'a Fadt {
    let fadt = unsafe {
        acpi_tables.find_table::<Fadt>().unwrap().virtual_start().as_ref()
    };
    fadt.validate().unwrap();
    fadt
}


/*
LocalApic(
    LocalApicEntry {
        header: EntryHeader {
            entry_type: 0,
            length: 8,
        },
        processor_id: 0,
        apic_id: 0,
        flags: 1,
    },
)
IoApic(
    IoApicEntry {
        header: EntryHeader {
            entry_type: 1,
            length: 12,
        },
        io_apic_id: 0,
        _reserved: 0,
        io_apic_address: 4273995776,
        global_system_interrupt_base: 0,
    },
)
InterruptSourceOverride(
    InterruptSourceOverrideEntry {
        header: EntryHeader {
            entry_type: 2,
            length: 10,
        },
        bus: 0,
        irq: 0,
        global_system_interrupt: 2,
        flags: 0,
    },
)
InterruptSourceOverride(
    InterruptSourceOverrideEntry {
        header: EntryHeader {
            entry_type: 2,
            length: 10,
        },
        bus: 0,
        irq: 5,
        global_system_interrupt: 5,
        flags: 13,
    },
)
InterruptSourceOverride(
    InterruptSourceOverrideEntry {
        header: EntryHeader {
            entry_type: 2,
            length: 10,
        },
        bus: 0,
        irq: 9,
        global_system_interrupt: 9,
        flags: 13,
    },
)
InterruptSourceOverride(
    InterruptSourceOverrideEntry {
        header: EntryHeader {
            entry_type: 2,
            length: 10,
        },
        bus: 0,
        irq: 10,
        global_system_interrupt: 10,
        flags: 13,
    },
)
InterruptSourceOverride(
    InterruptSourceOverrideEntry {
        header: EntryHeader {
            entry_type: 2,
            length: 10,
        },
        bus: 0,
        irq: 11,
        global_system_interrupt: 11,
        flags: 13,
    },
)
LocalApicNmi(
    LocalApicNmiEntry {
        header: EntryHeader {
            entry_type: 4,
            length: 6,
        },
        processor_id: 255,
        flags: 0,
        nmi_line: 1,
    },
)
 */