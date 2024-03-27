use x86_64::instructions::port;

pub const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
pub const PCI_CONFIG_DATA: u16 = 0xCFC;


#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PciDevice {
    location: u32,
}

impl core::fmt::Debug for PciDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PciDevice")
            .field("bus", &((self.location >> 16) & 0xFF))
            .field("slot", &((self.location >> 11) & 0x1F))
            .field("func", &((self.location >> 8) & 0x7))
        .finish()
    }
}

impl PciDevice {
    pub fn from(bus: u8, slot: u8, func: u8) -> Option<PciDevice> {
        if slot > 31 || func > 7 {
            None
        } else {
            let location = ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | (1 << 31);

            let device = PciDevice {
                location,
            };

            if device.get_id_register() == PciIdRegister::empty() {
                None
            } else {                        
                Some(device)
            }
        }
    }

    pub unsafe fn from_unchecked(bus: u8, slot: u8, func: u8) -> PciDevice {
        let location = ((bus as u32) << 16)
        | ((slot as u32) << 11)
        | ((func as u32) << 8)
        | (1 << 31);
        PciDevice {
            location,
        }
    }

    pub fn read_register(&self, offset: u8) -> u32 {
        let mut config_address = port::Port::new(PCI_CONFIG_ADDRESS);
        let mut config_data = port::Port::new(PCI_CONFIG_DATA);
        
        let request = self.location | offset as u32;

        unsafe {
            config_address.write(request);
            config_data.read()
        }
    }

    pub fn get_id_register(&self) -> PciIdRegister {
        let result: u32 = self.read_register(0x0);

        PciIdRegister {
            device_id: (result >> 16) as u16,
            vendor_id: (result & 0xFFFF) as u16,
        }
    }

    pub fn get_status_register(&self) -> PciStatusRegister {
        let result: u32 = self.read_register(0x4);

        PciStatusRegister {
            status: (result >> 16) as u16,
            command: (result & 0xFFFF) as u16,
        }
    }

    pub fn get_class_register(&self) -> PciClassRegister {
        let result: u32 = self.read_register(0x8);

        PciClassRegister {
            class: (result >> 24) as u8,
            subclass: (result >> 16 & 0xFF) as u8,
            prog_if: (result >> 8 & 0xFF) as u8,
            revision: (result & 0xFF) as u8,
        }
    }

    pub fn get_info_register(&self) -> PciInfoRegister {
        let result: u32 = self.read_register(0xC);

        PciInfoRegister {
            bist: (result >> 24) as u8,
            header_type: (result >> 16 & 0xFF) as u8,
            latency_timer: (result >> 8 & 0xFF) as u8,
            cache_size: (result & 0xFF) as u8,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PciIdRegister {
    pub device_id: u16,
    pub vendor_id: u16,
}

impl PciIdRegister {
    pub const fn empty() -> Self {
        Self { 
            device_id: !0, 
            vendor_id: !0 
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PciStatusRegister {
    pub status: u16,
    pub command: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PciClassRegister {
    pub class: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub revision: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PciInfoRegister {
    pub bist: u8,
    pub header_type: u8,
    pub latency_timer: u8,
    pub cache_size: u8,
}


pub fn try_find_device<F>(test: F) -> Option<PciDevice>
where F: Fn(&PciDevice) -> bool {
    for bus in 0..=255 {
        for slot in 0..=31 {
            for func in 0..=7 {
                let pci_device = unsafe {
                    PciDevice::from_unchecked(bus, slot, func)
                };

                if test(&pci_device) {
                    return Some(pci_device);
                }
            }
        }
    }

    None
}
