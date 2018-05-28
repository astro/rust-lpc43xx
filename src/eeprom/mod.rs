#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEPROM command register"]
    pub cmd: CMD,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - EEPROM read wait state register"]
    pub rwstate: RWSTATE,
    #[doc = "0x0c - EEPROM auto programming register"]
    pub autoprog: AUTOPROG,
    #[doc = "0x10 - EEPROM wait state register"]
    pub wstate: WSTATE,
    #[doc = "0x14 - EEPROM clock divider register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x18 - EEPROM power-down register"]
    pub pwrdwn: PWRDWN,
    _reserved1: [u8; 4028usize],
    #[doc = "0xfd8 - EEPROM interrupt enable clear"]
    pub intenclr: INTENCLR,
    #[doc = "0xfdc - EEPROM interrupt enable set"]
    pub intenset: INTENSET,
    #[doc = "0xfe0 - EEPROM interrupt status"]
    pub intstat: INTSTAT,
    #[doc = "0xfe4 - EEPROM interrupt enable"]
    pub inten: INTEN,
    #[doc = "0xfe8 - EEPROM interrupt status clear"]
    pub intstatclr: INTSTATCLR,
}
#[doc = "EEPROM command register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM command register"]
pub mod cmd;
#[doc = "EEPROM read wait state register"]
pub struct RWSTATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM read wait state register"]
pub mod rwstate;
#[doc = "EEPROM auto programming register"]
pub struct AUTOPROG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM auto programming register"]
pub mod autoprog;
#[doc = "EEPROM wait state register"]
pub struct WSTATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM wait state register"]
pub mod wstate;
#[doc = "EEPROM clock divider register"]
pub struct CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM clock divider register"]
pub mod clkdiv;
#[doc = "EEPROM power-down register"]
pub struct PWRDWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM power-down register"]
pub mod pwrdwn;
#[doc = "EEPROM interrupt enable clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM interrupt enable clear"]
pub mod intenclr;
#[doc = "EEPROM interrupt enable set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM interrupt enable set"]
pub mod intenset;
#[doc = "EEPROM interrupt status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM interrupt status"]
pub mod intstat;
#[doc = "EEPROM interrupt enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM interrupt enable"]
pub mod inten;
#[doc = "EEPROM interrupt status clear"]
pub struct INTSTATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM interrupt status clear"]
pub mod intstatclr;
