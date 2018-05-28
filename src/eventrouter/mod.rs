#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Level configuration register"]
    pub hilo: HILO,
    #[doc = "0x04 - Edge configuration"]
    pub edge: EDGE,
    _reserved0: [u8; 4048usize],
    #[doc = "0xfd8 - Clear event enable register"]
    pub clr_en: CLR_EN,
    #[doc = "0xfdc - Set event enable register"]
    pub set_en: SET_EN,
    #[doc = "0xfe0 - Event Status register"]
    pub status: STATUS,
    #[doc = "0xfe4 - Event Enable register"]
    pub enable: ENABLE,
    #[doc = "0xfe8 - Clear event status register"]
    pub clr_stat: CLR_STAT,
    #[doc = "0xfec - Set event status register"]
    pub set_stat: SET_STAT,
}
#[doc = "Level configuration register"]
pub struct HILO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Level configuration register"]
pub mod hilo;
#[doc = "Edge configuration"]
pub struct EDGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Edge configuration"]
pub mod edge;
#[doc = "Clear event enable register"]
pub struct CLR_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear event enable register"]
pub mod clr_en;
#[doc = "Set event enable register"]
pub struct SET_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set event enable register"]
pub mod set_en;
#[doc = "Event Status register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Status register"]
pub mod status;
#[doc = "Event Enable register"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Enable register"]
pub mod enable;
#[doc = "Clear event status register"]
pub struct CLR_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear event status register"]
pub mod clr_stat;
#[doc = "Set event status register"]
pub struct SET_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set event status register"]
pub mod set_stat;
