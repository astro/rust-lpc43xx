#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Horizontal Timing Control register"]
    pub timh: TIMH,
    #[doc = "0x04 - Vertical Timing Control register"]
    pub timv: TIMV,
    #[doc = "0x08 - Clock and Signal Polarity Control register"]
    pub pol: POL,
    #[doc = "0x0c - Line End Control register"]
    pub le: LE,
    #[doc = "0x10 - Upper Panel Frame Base Address register"]
    pub upbase: UPBASE,
    #[doc = "0x14 - Lower Panel Frame Base Address register"]
    pub lpbase: LPBASE,
    #[doc = "0x18 - LCD Control register"]
    pub ctrl: CTRL,
    #[doc = "0x1c - Interrupt Mask register"]
    pub intmsk: INTMSK,
    #[doc = "0x20 - Raw Interrupt Status register"]
    pub intraw: INTRAW,
    #[doc = "0x24 - Masked Interrupt Status register"]
    pub intstat: INTSTAT,
    #[doc = "0x28 - Interrupt Clear register"]
    pub intclr: INTCLR,
    #[doc = "0x2c - Upper Panel Current Address Value register"]
    pub upcurr: UPCURR,
    #[doc = "0x30 - Lower Panel Current Address Value register"]
    pub lpcurr: LPCURR,
    _reserved0: [u8; 460usize],
    #[doc = "0x200 - 256x16-bit Color Palette registers"]
    pub pal: [PAL; 256],
    _reserved1: [u8; 512usize],
    #[doc = "0x800 - Cursor Image registers"]
    pub crsr_img: [CRSR_IMG; 256],
    #[doc = "0xc00 - Cursor Control register"]
    pub crsr_ctrl: CRSR_CTRL,
    #[doc = "0xc04 - Cursor Configuration register"]
    pub crsr_cfg: CRSR_CFG,
    #[doc = "0xc08 - Cursor Palette register 0"]
    pub crsr_pal0: CRSR_PAL0,
    #[doc = "0xc0c - Cursor Palette register 1"]
    pub crsr_pal1: CRSR_PAL1,
    #[doc = "0xc10 - Cursor XY Position register"]
    pub crsr_xy: CRSR_XY,
    #[doc = "0xc14 - Cursor Clip Position register"]
    pub crsr_clip: CRSR_CLIP,
    _reserved2: [u8; 8usize],
    #[doc = "0xc20 - Cursor Interrupt Mask register"]
    pub crsr_intmsk: CRSR_INTMSK,
    #[doc = "0xc24 - Cursor Interrupt Clear register"]
    pub crsr_intclr: CRSR_INTCLR,
    #[doc = "0xc28 - Cursor Raw Interrupt Status register"]
    pub crsr_intraw: CRSR_INTRAW,
    #[doc = "0xc2c - Cursor Masked Interrupt Status register"]
    pub crsr_intstat: CRSR_INTSTAT,
}
#[doc = "Horizontal Timing Control register"]
pub struct TIMH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Horizontal Timing Control register"]
pub mod timh;
#[doc = "Vertical Timing Control register"]
pub struct TIMV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vertical Timing Control register"]
pub mod timv;
#[doc = "Clock and Signal Polarity Control register"]
pub struct POL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock and Signal Polarity Control register"]
pub mod pol;
#[doc = "Line End Control register"]
pub struct LE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line End Control register"]
pub mod le;
#[doc = "Upper Panel Frame Base Address register"]
pub struct UPBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Upper Panel Frame Base Address register"]
pub mod upbase;
#[doc = "Lower Panel Frame Base Address register"]
pub struct LPBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lower Panel Frame Base Address register"]
pub mod lpbase;
#[doc = "LCD Control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCD Control register"]
pub mod ctrl;
#[doc = "Interrupt Mask register"]
pub struct INTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask register"]
pub mod intmsk;
#[doc = "Raw Interrupt Status register"]
pub struct INTRAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Interrupt Status register"]
pub mod intraw;
#[doc = "Masked Interrupt Status register"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked Interrupt Status register"]
pub mod intstat;
#[doc = "Interrupt Clear register"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear register"]
pub mod intclr;
#[doc = "Upper Panel Current Address Value register"]
pub struct UPCURR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Upper Panel Current Address Value register"]
pub mod upcurr;
#[doc = "Lower Panel Current Address Value register"]
pub struct LPCURR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lower Panel Current Address Value register"]
pub mod lpcurr;
#[doc = "256x16-bit Color Palette registers"]
pub struct PAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "256x16-bit Color Palette registers"]
pub mod pal;
#[doc = "Cursor Image registers"]
pub struct CRSR_IMG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cursor Image registers"]
pub mod crsr_img;
#[doc = "Cursor Control register"]
pub struct CRSR_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cursor Control register"]
pub mod crsr_ctrl;
#[doc = "Cursor Configuration register"]
pub struct CRSR_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cursor Configuration register"]
pub mod crsr_cfg;
#[doc = "Cursor Palette register 0"]
pub struct CRSR_PAL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cursor Palette register 0"]
pub mod crsr_pal0;
#[doc = "Cursor Palette register 1"]
pub struct CRSR_PAL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cursor Palette register 1"]
pub mod crsr_pal1;
#[doc = "Cursor XY Position register"]
pub struct CRSR_XY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cursor XY Position register"]
pub mod crsr_xy;
#[doc = "Cursor Clip Position register"]
pub struct CRSR_CLIP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cursor Clip Position register"]
pub mod crsr_clip;
#[doc = "Cursor Interrupt Mask register"]
pub struct CRSR_INTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cursor Interrupt Mask register"]
pub mod crsr_intmsk;
#[doc = "Cursor Interrupt Clear register"]
pub struct CRSR_INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cursor Interrupt Clear register"]
pub mod crsr_intclr;
#[doc = "Cursor Raw Interrupt Status register"]
pub struct CRSR_INTRAW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cursor Raw Interrupt Status register"]
pub mod crsr_intraw;
#[doc = "Cursor Masked Interrupt Status register"]
pub struct CRSR_INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cursor Masked Interrupt Status register"]
pub mod crsr_intstat;
