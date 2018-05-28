#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC register. Holds the conversion data."]
    pub cr: CR,
    #[doc = "0x04 - DAC control register."]
    pub ctrl: CTRL,
    #[doc = "0x08 - DAC counter value register."]
    pub cntval: CNTVAL,
}
#[doc = "DAC register. Holds the conversion data."]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC register. Holds the conversion data."]
pub mod cr;
#[doc = "DAC control register."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC control register."]
pub mod ctrl;
#[doc = "DAC counter value register."]
pub struct CNTVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC counter value register."]
pub mod cntval;
