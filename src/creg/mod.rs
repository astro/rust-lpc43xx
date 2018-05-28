#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Chip configuration register 32 kHz oscillator output and BOD control register."]
    pub creg0: CREG0,
    _reserved1: [u8; 248usize],
    #[doc = "0x100 - ARM Cortex-M4 memory mapping"]
    pub m4memmap: M4MEMMAP,
    _reserved2: [u8; 20usize],
    #[doc = "0x118 - Chip configuration register 5. Controls JTAG access."]
    pub creg5: CREG5,
    #[doc = "0x11c - DMA mux control"]
    pub dmamux: DMAMUX,
    #[doc = "0x120 - Flash accelerator configuration register for flash bank A"]
    pub flashcfga: FLASHCFGA,
    #[doc = "0x124 - Flash accelerator configuration register for flash bank B"]
    pub flashcfgb: FLASHCFGB,
    #[doc = "0x128 - ETB RAM configuration"]
    pub etbcfg: ETBCFG,
    #[doc = "0x12c - Chip configuration register 6. Controls multiple functions : Ethernet interface, SCT output, I2S0/1 inputs, EMC clock."]
    pub creg6: CREG6,
    #[doc = "0x130 - Cortex-M4 TXEV event clear"]
    pub m4txevent: M4TXEVENT,
    _reserved3: [u8; 204usize],
    #[doc = "0x200 - Part ID"]
    pub chipid: CHIPID,
    _reserved4: [u8; 260usize],
    #[doc = "0x308 - ARM Cortex-M0SUB memory mapping"]
    pub m0submemmap: M0SUBMEMMAP,
    _reserved5: [u8; 8usize],
    #[doc = "0x314 - Cortex-M0SUB TXEV event clear"]
    pub m0subtxevent: M0SUBTXEVENT,
    _reserved6: [u8; 232usize],
    #[doc = "0x400 - Cortex-M0APP TXEV event clear"]
    pub m0apptxevent: M0APPTXEVENT,
    #[doc = "0x404 - ARM Cortex-M0APP memory mapping"]
    pub m0appmemmap: M0APPMEMMAP,
    _reserved7: [u8; 248usize],
    #[doc = "0x500 - USB0 frame length adjust register"]
    pub usb0fladj: USB0FLADJ,
    _reserved8: [u8; 252usize],
    #[doc = "0x600 - USB1 frame length adjust register"]
    pub usb1fladj: USB1FLADJ,
}
#[doc = "Chip configuration register 32 kHz oscillator output and BOD control register."]
pub struct CREG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip configuration register 32 kHz oscillator output and BOD control register."]
pub mod creg0;
#[doc = "ARM Cortex-M4 memory mapping"]
pub struct M4MEMMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ARM Cortex-M4 memory mapping"]
pub mod m4memmap;
#[doc = "Chip configuration register 5. Controls JTAG access."]
pub struct CREG5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip configuration register 5. Controls JTAG access."]
pub mod creg5;
#[doc = "DMA mux control"]
pub struct DMAMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA mux control"]
pub mod dmamux;
#[doc = "Flash accelerator configuration register for flash bank A"]
pub struct FLASHCFGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash accelerator configuration register for flash bank A"]
pub mod flashcfga;
#[doc = "Flash accelerator configuration register for flash bank B"]
pub struct FLASHCFGB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash accelerator configuration register for flash bank B"]
pub mod flashcfgb;
#[doc = "ETB RAM configuration"]
pub struct ETBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ETB RAM configuration"]
pub mod etbcfg;
#[doc = "Chip configuration register 6. Controls multiple functions : Ethernet interface, SCT output, I2S0/1 inputs, EMC clock."]
pub struct CREG6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip configuration register 6. Controls multiple functions : Ethernet interface, SCT output, I2S0/1 inputs, EMC clock."]
pub mod creg6;
#[doc = "Cortex-M4 TXEV event clear"]
pub struct M4TXEVENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cortex-M4 TXEV event clear"]
pub mod m4txevent;
#[doc = "Part ID"]
pub struct CHIPID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Part ID"]
pub mod chipid;
#[doc = "ARM Cortex-M0SUB memory mapping"]
pub struct M0SUBMEMMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ARM Cortex-M0SUB memory mapping"]
pub mod m0submemmap;
#[doc = "Cortex-M0SUB TXEV event clear"]
pub struct M0SUBTXEVENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cortex-M0SUB TXEV event clear"]
pub mod m0subtxevent;
#[doc = "Cortex-M0APP TXEV event clear"]
pub struct M0APPTXEVENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cortex-M0APP TXEV event clear"]
pub mod m0apptxevent;
#[doc = "ARM Cortex-M0APP memory mapping"]
pub struct M0APPMEMMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ARM Cortex-M0APP memory mapping"]
pub mod m0appmemmap;
#[doc = "USB0 frame length adjust register"]
pub struct USB0FLADJ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB0 frame length adjust register"]
pub mod usb0fladj;
#[doc = "USB1 frame length adjust register"]
pub struct USB1FLADJ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB1 frame length adjust register"]
pub mod usb1fladj;
