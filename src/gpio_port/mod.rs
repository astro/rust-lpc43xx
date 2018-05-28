#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31"]
    pub b: [B; 256],
    _reserved0: [u8; 3840usize],
    #[doc = "0x1000 - Word pin registers port 0 to 5"]
    pub w: [W; 256],
    _reserved1: [u8; 3072usize],
    #[doc = "0x2000 - Direction registers port m"]
    pub dir: [DIR; 8],
    _reserved2: [u8; 96usize],
    #[doc = "0x2080 - Mask register port m"]
    pub mask: [MASK; 8],
    _reserved3: [u8; 96usize],
    #[doc = "0x2100 - Port pin register port m"]
    pub pin: [PIN; 8],
    _reserved4: [u8; 96usize],
    #[doc = "0x2180 - Masked port register port m"]
    pub mpin: [MPIN; 8],
    _reserved5: [u8; 96usize],
    #[doc = "0x2200 - Write: Set register for port m Read: output bits for port m"]
    pub set: [SET; 8],
    _reserved6: [u8; 96usize],
    #[doc = "0x2280 - Clear port m"]
    pub clr: [CLR; 8],
    _reserved7: [u8; 96usize],
    #[doc = "0x2300 - Toggle port m"]
    pub not: [NOT; 8],
}
#[doc = "Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31"]
pub struct B {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31"]
pub mod b;
#[doc = "Word pin registers port 0 to 5"]
pub struct W {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Word pin registers port 0 to 5"]
pub mod w;
#[doc = "Direction registers port m"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direction registers port m"]
pub mod dir;
#[doc = "Mask register port m"]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask register port m"]
pub mod mask;
#[doc = "Port pin register port m"]
pub struct PIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port pin register port m"]
pub mod pin;
#[doc = "Masked port register port m"]
pub struct MPIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked port register port m"]
pub mod mpin;
#[doc = "Write: Set register for port m Read: output bits for port m"]
pub struct SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write: Set register for port m Read: output bits for port m"]
pub mod set;
#[doc = "Clear port m"]
pub struct CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear port m"]
pub mod clr;
#[doc = "Toggle port m"]
pub struct NOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Toggle port m"]
pub mod not;
