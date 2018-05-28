#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Downcounter register"]
    pub downcounter: DOWNCOUNTER,
    #[doc = "0x04 - Preset value register"]
    pub preset: PRESET,
    _reserved0: [u8; 4048usize],
    #[doc = "0xfd8 - Interrupt clear enable register"]
    pub clr_en: CLR_EN,
    #[doc = "0xfdc - Interrupt set enable register"]
    pub set_en: SET_EN,
    #[doc = "0xfe0 - Status register"]
    pub status: STATUS,
    #[doc = "0xfe4 - Enable register"]
    pub enable: ENABLE,
    #[doc = "0xfe8 - Clear register"]
    pub clr_stat: CLR_STAT,
    #[doc = "0xfec - Set register"]
    pub set_stat: SET_STAT,
}
#[doc = "Downcounter register"]
pub struct DOWNCOUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Downcounter register"]
pub mod downcounter;
#[doc = "Preset value register"]
pub struct PRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Preset value register"]
pub mod preset;
#[doc = "Interrupt clear enable register"]
pub struct CLR_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt clear enable register"]
pub mod clr_en;
#[doc = "Interrupt set enable register"]
pub struct SET_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt set enable register"]
pub mod set_en;
#[doc = "Status register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod status;
#[doc = "Enable register"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable register"]
pub mod enable;
#[doc = "Clear register"]
pub struct CLR_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear register"]
pub mod clr_stat;
#[doc = "Set register"]
pub struct SET_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set register"]
pub mod set_stat;
