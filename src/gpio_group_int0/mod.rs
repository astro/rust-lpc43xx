#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO grouped interrupt control register"]
    pub ctrl: CTRL,
    _reserved0: [u8; 28usize],
    #[doc = "0x20 - GPIO grouped interrupt port polarity register"]
    pub port_pol: [PORT_POL; 8],
    #[doc = "0x40 - GPIO grouped interrupt port m enable register"]
    pub port_ena: [PORT_ENA; 8],
}
#[doc = "GPIO grouped interrupt control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO grouped interrupt control register"]
pub mod ctrl;
#[doc = "GPIO grouped interrupt port polarity register"]
pub struct PORT_POL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO grouped interrupt port polarity register"]
pub mod port_pol;
#[doc = "GPIO grouped interrupt port m enable register"]
pub struct PORT_ENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO grouped interrupt port m enable register"]
pub mod port_ena;
