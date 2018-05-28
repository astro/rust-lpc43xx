#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Interrupt Status Register"]
    pub intstat: INTSTAT,
    #[doc = "0x04 - DMA Interrupt Terminal Count Request Status Register"]
    pub inttcstat: INTTCSTAT,
    #[doc = "0x08 - DMA Interrupt Terminal Count Request Clear Register"]
    pub inttcclear: INTTCCLEAR,
    #[doc = "0x0c - DMA Interrupt Error Status Register"]
    pub interrstat: INTERRSTAT,
    #[doc = "0x10 - DMA Interrupt Error Clear Register"]
    pub interrclr: INTERRCLR,
    #[doc = "0x14 - DMA Raw Interrupt Terminal Count Status Register"]
    pub rawinttcstat: RAWINTTCSTAT,
    #[doc = "0x18 - DMA Raw Error Interrupt Status Register"]
    pub rawinterrstat: RAWINTERRSTAT,
    #[doc = "0x1c - DMA Enabled Channel Register"]
    pub enbldchns: ENBLDCHNS,
    #[doc = "0x20 - DMA Software Burst Request Register"]
    pub softbreq: SOFTBREQ,
    #[doc = "0x24 - DMA Software Single Request Register"]
    pub softsreq: SOFTSREQ,
    #[doc = "0x28 - DMA Software Last Burst Request Register"]
    pub softlbreq: SOFTLBREQ,
    #[doc = "0x2c - DMA Software Last Single Request Register"]
    pub softlsreq: SOFTLSREQ,
    #[doc = "0x30 - DMA Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x34 - DMA Synchronization Register"]
    pub sync: SYNC,
    _reserved0: [u8; 200usize],
    #[doc = "0x100 - DMA Channel Source Address Register"]
    pub c0srcaddr: CSRCADDR,
    #[doc = "0x104 - DMA Channel Destination Address Register"]
    pub c0destaddr: CDESTADDR,
    #[doc = "0x108 - DMA Channel Linked List Item Register"]
    pub c0lli: CLLI,
    #[doc = "0x10c - DMA Channel Control Register"]
    pub c0control: CCONTROL,
    #[doc = "0x110 - DMA Channel Configuration Register"]
    pub c0config: CCONFIG,
    _reserved1: [u8; 12usize],
    #[doc = "0x120 - DMA Channel Source Address Register"]
    pub c1srcaddr: CSRCADDR,
    #[doc = "0x124 - DMA Channel Destination Address Register"]
    pub c1destaddr: CDESTADDR,
    #[doc = "0x128 - DMA Channel Linked List Item Register"]
    pub c1lli: CLLI,
    #[doc = "0x12c - DMA Channel Control Register"]
    pub c1control: CCONTROL,
    #[doc = "0x130 - DMA Channel Configuration Register"]
    pub c1config: CCONFIG,
    _reserved2: [u8; 12usize],
    #[doc = "0x140 - DMA Channel Source Address Register"]
    pub c2srcaddr: CSRCADDR,
    #[doc = "0x144 - DMA Channel Destination Address Register"]
    pub c2destaddr: CDESTADDR,
    #[doc = "0x148 - DMA Channel Linked List Item Register"]
    pub c2lli: CLLI,
    #[doc = "0x14c - DMA Channel Control Register"]
    pub c2control: CCONTROL,
    #[doc = "0x150 - DMA Channel Configuration Register"]
    pub c2config: CCONFIG,
    _reserved3: [u8; 12usize],
    #[doc = "0x160 - DMA Channel Source Address Register"]
    pub c3srcaddr: CSRCADDR,
    #[doc = "0x164 - DMA Channel Destination Address Register"]
    pub c3destaddr: CDESTADDR,
    #[doc = "0x168 - DMA Channel Linked List Item Register"]
    pub c3lli: CLLI,
    #[doc = "0x16c - DMA Channel Control Register"]
    pub c3control: CCONTROL,
    #[doc = "0x170 - DMA Channel Configuration Register"]
    pub c3config: CCONFIG,
    _reserved4: [u8; 12usize],
    #[doc = "0x180 - DMA Channel Source Address Register"]
    pub c4srcaddr: CSRCADDR,
    #[doc = "0x184 - DMA Channel Destination Address Register"]
    pub c4destaddr: CDESTADDR,
    #[doc = "0x188 - DMA Channel Linked List Item Register"]
    pub c4lli: CLLI,
    #[doc = "0x18c - DMA Channel Control Register"]
    pub c4control: CCONTROL,
    #[doc = "0x190 - DMA Channel Configuration Register"]
    pub c4config: CCONFIG,
    _reserved5: [u8; 12usize],
    #[doc = "0x1a0 - DMA Channel Source Address Register"]
    pub c5srcaddr: CSRCADDR,
    #[doc = "0x1a4 - DMA Channel Destination Address Register"]
    pub c5destaddr: CDESTADDR,
    #[doc = "0x1a8 - DMA Channel Linked List Item Register"]
    pub c5lli: CLLI,
    #[doc = "0x1ac - DMA Channel Control Register"]
    pub c5control: CCONTROL,
    #[doc = "0x1b0 - DMA Channel Configuration Register"]
    pub c5config: CCONFIG,
    _reserved6: [u8; 12usize],
    #[doc = "0x1c0 - DMA Channel Source Address Register"]
    pub c6srcaddr: CSRCADDR,
    #[doc = "0x1c4 - DMA Channel Destination Address Register"]
    pub c6destaddr: CDESTADDR,
    #[doc = "0x1c8 - DMA Channel Linked List Item Register"]
    pub c6lli: CLLI,
    #[doc = "0x1cc - DMA Channel Control Register"]
    pub c6control: CCONTROL,
    #[doc = "0x1d0 - DMA Channel Configuration Register"]
    pub c6config: CCONFIG,
    _reserved7: [u8; 12usize],
    #[doc = "0x1e0 - DMA Channel Source Address Register"]
    pub c7srcaddr: CSRCADDR,
    #[doc = "0x1e4 - DMA Channel Destination Address Register"]
    pub c7destaddr: CDESTADDR,
    #[doc = "0x1e8 - DMA Channel Linked List Item Register"]
    pub c7lli: CLLI,
    #[doc = "0x1ec - DMA Channel Control Register"]
    pub c7control: CCONTROL,
    #[doc = "0x1f0 - DMA Channel Configuration Register"]
    pub c7config: CCONFIG,
}
#[doc = "DMA Interrupt Status Register"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Status Register"]
pub mod intstat;
#[doc = "DMA Interrupt Terminal Count Request Status Register"]
pub struct INTTCSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Terminal Count Request Status Register"]
pub mod inttcstat;
#[doc = "DMA Interrupt Terminal Count Request Clear Register"]
pub struct INTTCCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Terminal Count Request Clear Register"]
pub mod inttcclear;
#[doc = "DMA Interrupt Error Status Register"]
pub struct INTERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Error Status Register"]
pub mod interrstat;
#[doc = "DMA Interrupt Error Clear Register"]
pub struct INTERRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Error Clear Register"]
pub mod interrclr;
#[doc = "DMA Raw Interrupt Terminal Count Status Register"]
pub struct RAWINTTCSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Raw Interrupt Terminal Count Status Register"]
pub mod rawinttcstat;
#[doc = "DMA Raw Error Interrupt Status Register"]
pub struct RAWINTERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Raw Error Interrupt Status Register"]
pub mod rawinterrstat;
#[doc = "DMA Enabled Channel Register"]
pub struct ENBLDCHNS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Enabled Channel Register"]
pub mod enbldchns;
#[doc = "DMA Software Burst Request Register"]
pub struct SOFTBREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Software Burst Request Register"]
pub mod softbreq;
#[doc = "DMA Software Single Request Register"]
pub struct SOFTSREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Software Single Request Register"]
pub mod softsreq;
#[doc = "DMA Software Last Burst Request Register"]
pub struct SOFTLBREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Software Last Burst Request Register"]
pub mod softlbreq;
#[doc = "DMA Software Last Single Request Register"]
pub struct SOFTLSREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Software Last Single Request Register"]
pub mod softlsreq;
#[doc = "DMA Configuration Register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Configuration Register"]
pub mod config;
#[doc = "DMA Synchronization Register"]
pub struct SYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Synchronization Register"]
pub mod sync;
#[doc = "DMA Channel Source Address Register"]
pub struct CSRCADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Source Address Register"]
pub mod csrcaddr;
#[doc = "DMA Channel Destination Address Register"]
pub struct CDESTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Destination Address Register"]
pub mod cdestaddr;
#[doc = "DMA Channel Linked List Item Register"]
pub struct CLLI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Linked List Item Register"]
pub mod clli;
#[doc = "DMA Channel Control Register"]
pub struct CCONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Control Register"]
pub mod ccontrol;
#[doc = "DMA Channel Configuration Register"]
pub struct CCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Configuration Register"]
pub mod cconfig;
