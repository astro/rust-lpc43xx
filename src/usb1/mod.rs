#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - Capability register length"]
    pub caplength: CAPLENGTH,
    #[doc = "0x104 - Host controller structural parameters"]
    pub hcsparams: HCSPARAMS,
    #[doc = "0x108 - Host controller capability parameters"]
    pub hccparams: HCCPARAMS,
    _reserved1: [u8; 20usize],
    #[doc = "0x120 - Device interface version number"]
    pub dciversion: DCIVERSION,
    _reserved2: [u8; 28usize],
    #[doc = "0x140 - USB command (device mode)"]
    pub usbcmd_d: USBCMD_D,
    #[doc = "0x144 - USB status (device mode)"]
    pub usbsts_d: USBSTS_D,
    #[doc = "0x148 - USB interrupt enable (device mode)"]
    pub usbintr_d: USBINTR_D,
    #[doc = "0x14c - USB frame index (device mode)"]
    pub frindex_d: FRINDEX_D,
    _reserved3: [u8; 4usize],
    #[doc = "0x154 - USB device address"]
    pub deviceaddr: DEVICEADDR,
    #[doc = "0x158 - Address of endpoint list in memory (device mode)"]
    pub endpointlistaddr: ENDPOINTLISTADDR,
    #[doc = "0x15c - Asynchronous buffer status for embedded TT (host mode)"]
    pub ttctrl: TTCTRL,
    #[doc = "0x160 - Programmable burst size"]
    pub burstsize: BURSTSIZE,
    #[doc = "0x164 - Host transmit pre-buffer packet tuning (host mode)"]
    pub txfilltuning: TXFILLTUNING,
    _reserved4: [u8; 8usize],
    #[doc = "0x170 - ULPI viewport"]
    pub ulpiviewport: ULPIVIEWPORT,
    #[doc = "0x174 - Length of virtual frame"]
    pub binterval: BINTERVAL,
    #[doc = "0x178 - Endpoint NAK (device mode)"]
    pub endptnak: ENDPTNAK,
    #[doc = "0x17c - Endpoint NAK Enable (device mode)"]
    pub endptnaken: ENDPTNAKEN,
    _reserved5: [u8; 4usize],
    #[doc = "0x184 - Port 1 status/control (device mode)"]
    pub portsc1_d: PORTSC1_D,
    _reserved6: [u8; 32usize],
    #[doc = "0x1a8 - USB mode (device mode)"]
    pub usbmode_d: USBMODE_D,
    #[doc = "0x1ac - Endpoint setup status"]
    pub endptsetupstat: ENDPTSETUPSTAT,
    #[doc = "0x1b0 - Endpoint initialization"]
    pub endptprime: ENDPTPRIME,
    #[doc = "0x1b4 - Endpoint de-initialization"]
    pub endptflush: ENDPTFLUSH,
    #[doc = "0x1b8 - Endpoint status"]
    pub endptstat: ENDPTSTAT,
    #[doc = "0x1bc - Endpoint complete"]
    pub endptcomplete: ENDPTCOMPLETE,
    #[doc = "0x1c0 - Endpoint control 0"]
    pub endptctrl0: ENDPTCTRL0,
    #[doc = "0x1c4 - Endpoint control"]
    pub endptctrl1: ENDPTCTRL,
    #[doc = "0x1c8 - Endpoint control"]
    pub endptctrl2: ENDPTCTRL,
    #[doc = "0x1cc - Endpoint control"]
    pub endptctrl3: ENDPTCTRL,
}
#[doc = "Capability register length"]
pub struct CAPLENGTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capability register length"]
pub mod caplength;
#[doc = "Host controller structural parameters"]
pub struct HCSPARAMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host controller structural parameters"]
pub mod hcsparams;
#[doc = "Host controller capability parameters"]
pub struct HCCPARAMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host controller capability parameters"]
pub mod hccparams;
#[doc = "Device interface version number"]
pub struct DCIVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device interface version number"]
pub mod dciversion;
#[doc = "USB command (device mode)"]
pub struct USBCMD_D {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB command (device mode)"]
pub mod usbcmd_d;
#[doc = "USB command (host mode)"]
pub struct USBCMD_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB command (host mode)"]
pub mod usbcmd_h;
#[doc = "USB status (device mode)"]
pub struct USBSTS_D {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB status (device mode)"]
pub mod usbsts_d;
#[doc = "USB status (host mode)"]
pub struct USBSTS_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB status (host mode)"]
pub mod usbsts_h;
#[doc = "USB interrupt enable (device mode)"]
pub struct USBINTR_D {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB interrupt enable (device mode)"]
pub mod usbintr_d;
#[doc = "USB interrupt enable (host mode)"]
pub struct USBINTR_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB interrupt enable (host mode)"]
pub mod usbintr_h;
#[doc = "USB frame index (device mode)"]
pub struct FRINDEX_D {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB frame index (device mode)"]
pub mod frindex_d;
#[doc = "USB frame index (host mode)"]
pub struct FRINDEX_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB frame index (host mode)"]
pub mod frindex_h;
#[doc = "USB device address"]
pub struct DEVICEADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB device address"]
pub mod deviceaddr;
#[doc = "Frame list base address"]
pub struct PERIODICLISTBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frame list base address"]
pub mod periodiclistbase;
#[doc = "Address of endpoint list in memory (device mode)"]
pub struct ENDPOINTLISTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address of endpoint list in memory (device mode)"]
pub mod endpointlistaddr;
#[doc = "Address of endpoint list in memory (host mode)"]
pub struct ASYNCLISTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address of endpoint list in memory (host mode)"]
pub mod asynclistaddr;
#[doc = "Asynchronous buffer status for embedded TT (host mode)"]
pub struct TTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Asynchronous buffer status for embedded TT (host mode)"]
pub mod ttctrl;
#[doc = "Programmable burst size"]
pub struct BURSTSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Programmable burst size"]
pub mod burstsize;
#[doc = "Host transmit pre-buffer packet tuning (host mode)"]
pub struct TXFILLTUNING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host transmit pre-buffer packet tuning (host mode)"]
pub mod txfilltuning;
#[doc = "ULPI viewport"]
pub struct ULPIVIEWPORT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ULPI viewport"]
pub mod ulpiviewport;
#[doc = "Length of virtual frame"]
pub struct BINTERVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Length of virtual frame"]
pub mod binterval;
#[doc = "Endpoint NAK (device mode)"]
pub struct ENDPTNAK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint NAK (device mode)"]
pub mod endptnak;
#[doc = "Endpoint NAK Enable (device mode)"]
pub struct ENDPTNAKEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint NAK Enable (device mode)"]
pub mod endptnaken;
#[doc = "Port 1 status/control (device mode)"]
pub struct PORTSC1_D {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port 1 status/control (device mode)"]
pub mod portsc1_d;
#[doc = "Port 1 status/control (host mode)"]
pub struct PORTSC1_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port 1 status/control (host mode)"]
pub mod portsc1_h;
#[doc = "USB mode (device mode)"]
pub struct USBMODE_D {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB mode (device mode)"]
pub mod usbmode_d;
#[doc = "USB mode (host mode)"]
pub struct USBMODE_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB mode (host mode)"]
pub mod usbmode_h;
#[doc = "Endpoint setup status"]
pub struct ENDPTSETUPSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint setup status"]
pub mod endptsetupstat;
#[doc = "Endpoint initialization"]
pub struct ENDPTPRIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint initialization"]
pub mod endptprime;
#[doc = "Endpoint de-initialization"]
pub struct ENDPTFLUSH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint de-initialization"]
pub mod endptflush;
#[doc = "Endpoint status"]
pub struct ENDPTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint status"]
pub mod endptstat;
#[doc = "Endpoint complete"]
pub struct ENDPTCOMPLETE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint complete"]
pub mod endptcomplete;
#[doc = "Endpoint control 0"]
pub struct ENDPTCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint control 0"]
pub mod endptctrl0;
#[doc = "Endpoint control"]
pub struct ENDPTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint control"]
pub mod endptctrl;
