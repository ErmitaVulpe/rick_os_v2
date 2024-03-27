use bit_field::BitField;
use bitflags::bitflags;


const IOREDTBL_OFFSET: u8 = 0x10;


bitflags! {
    #[derive(Clone, PartialEq)]
    pub struct IrqEntry: u64 {
        const DESTIONATION_MODE = 1 << 11;
        const DELIVERY_STATUS = 1 << 12;
        const PIN_POLARITY = 1 << 13;
        const REMOTE_IRR = 1 << 14;
        const TRIGGER_MODE = 1 << 15;
        const MASK = 1 << 16;
    }
}

impl IrqEntry {
    pub fn get_vector(&self) -> u8 {
        self.bits().get_bits(0..8) as u8
    }

    pub fn set_vector(&mut self, vec: u8) {
        let new_bits = *self.bits().set_bits(0..8, vec as u64);
        *self = IrqEntry::from_bits_retain(new_bits);
    }

    pub fn get_delivery_mode(&self) -> DeliveryMode {
        DeliveryMode::from_u8( self.bits().get_bits(8..11) as u8 )
    }

    pub fn set_delivery_mode(&mut self, mode: DeliveryMode) {
        let new_bits = *self.bits().set_bits(8..11, mode as u64);
        *self = IrqEntry::from_bits_retain(new_bits);
    }

    pub fn get_destination(&self) -> u8 {
        self.bits().get_bits(56..64) as u8
    }

    pub fn set_destination(&mut self, apic_id: u8) {
        let new_bits = *self.bits().set_bits(56..64, apic_id as u64);
        *self = IrqEntry::from_bits_retain(new_bits);
    }
}

impl core::fmt::Debug for IrqEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IrqEntry")
            .field("vector",                &self.get_vector())
            .field("delivery_mode",         &self.get_delivery_mode())
            .field("DESTIONATION_MODE",     &self.contains(Self::DESTIONATION_MODE))
            .field("DELIVERY_STATUS",       &self.contains(Self::DELIVERY_STATUS))
            .field("PIN_POLARITY",          &self.contains(Self::PIN_POLARITY))
            .field("REMOTE_IRR",            &self.contains(Self::REMOTE_IRR))
            .field("TRIGGER_MODE",          &self.contains(Self::TRIGGER_MODE))
            .field("MASK",                  &self.contains(Self::MASK))
            .field("destination",           &self.get_destination())
        .finish()
    }
}


#[derive(Debug)]
#[repr(u8)]
pub enum DeliveryMode {
    Fixed = 0b000,
    LowestPriority = 0b001,
    SMI = 0b010,
    NMI = 0b100,
    INIT = 0b101,
    ExtINT = 0b111,
}

impl DeliveryMode {
    fn from_u8(value: u8) -> Self {
        match value {
            x if x == Self::Fixed as u8             => Self::Fixed,
            x if x == Self::LowestPriority as u8    => Self::LowestPriority,
            x if x == Self::SMI as u8               => Self::SMI,
            x if x == Self::NMI as u8               => Self::NMI,
            x if x == Self::INIT as u8              => Self::INIT,
            x if x == Self::ExtINT as u8            => Self::ExtINT,
            _ => unreachable!(),
        }
    }
}


pub struct IoApic {
    // IOREGSEL
    sel: *mut u32,
    // IOREGWIN
    win: *mut u32,
}

impl IoApic {
    pub unsafe fn new(addr: usize) -> Self {
        Self {
            sel: addr as *mut u32,
            win: (addr + 0x10) as *mut u32,
        }
    }

    pub unsafe fn read_reg(&mut self, reg: u8) -> u32 {
        self.sel.write_volatile(reg as u32);
        self.win.read_volatile()
    }

    pub unsafe fn write_reg(&mut self, reg: u8, data: u32) {
        self.sel.write_volatile(reg as u32);
        self.win.write_volatile(data);
    }

    pub fn read_id(&mut self) -> u8 {
        unsafe {
            self.read_reg(0).get_bits(24..28) as u8
        }
    }

    pub fn read_ver_and_max_entry(&mut self) -> (u8, u16) {
        unsafe {
            let reg = self.read_reg(1);
            (
                reg.get_bits(0..8) as u8,
                (reg.get_bits(16..24) + 1) as u16,
            )
        }
    }

    pub fn read_arb(&mut self) -> u8 {
        unsafe {
            self.read_reg(2).get_bits(24..28) as u8
        }
    }

    pub fn read_irq(&mut self, irq: u8) -> IrqEntry {
        let entry: u64 = unsafe {
            (self.read_reg(IOREDTBL_OFFSET + 2 * irq) as u64) |
            (self.read_reg(IOREDTBL_OFFSET + 2 * irq + 1) as u64) << 32
        };
        IrqEntry::from_bits_retain(entry)
    }

    pub fn write_irq(&mut self, irq: u8, irq_entry: IrqEntry) {
        let bits = irq_entry.bits();
        unsafe {
            self.write_reg(IOREDTBL_OFFSET + 2 * irq, bits as u32);
            self.write_reg(IOREDTBL_OFFSET + 2 * irq + 1, (bits >> 32) as u32);
        }
    }
}
