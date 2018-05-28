#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hardware sleep event enable register"]
    pub pd0_sleep0_hw_ena: PD0_SLEEP0_HW_ENA,
    _reserved0: [u8; 24usize],
    #[doc = "0x1c - Sleep power mode register"]
    pub pd0_sleep0_mode: PD0_SLEEP0_MODE,
}
#[doc = "Hardware sleep event enable register"]
pub struct PD0_SLEEP0_HW_ENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware sleep event enable register"]
pub mod pd0_sleep0_hw_ena;
#[doc = "Sleep power mode register"]
pub struct PD0_SLEEP0_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sleep power mode register"]
pub mod pd0_sleep0_mode;
