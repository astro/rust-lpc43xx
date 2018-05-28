#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPIFI control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - SPIFI command register"]
    pub cmd: CMD,
    #[doc = "0x08 - SPIFI address register"]
    pub addr: ADDR,
    #[doc = "0x0c - SPIFI intermediate data register"]
    pub idata: IDATA,
    #[doc = "0x10 - SPIFI cache limit register"]
    pub climit: CLIMIT,
    #[doc = "0x14 - SPIFI data register"]
    pub data: DATA,
    #[doc = "0x18 - SPIFI memory command register"]
    pub mcmd: MCMD,
    #[doc = "0x1c - SPIFI status register"]
    pub stat: STAT,
}
#[doc = "SPIFI control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIFI control register"]
pub mod ctrl;
#[doc = "SPIFI command register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIFI command register"]
pub mod cmd;
#[doc = "SPIFI address register"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIFI address register"]
pub mod addr;
#[doc = "SPIFI intermediate data register"]
pub struct IDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIFI intermediate data register"]
pub mod idata;
#[doc = "SPIFI cache limit register"]
pub struct CLIMIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIFI cache limit register"]
pub mod climit;
#[doc = "SPIFI data register"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIFI data register"]
pub mod data;
#[doc = "SPIFI memory command register"]
pub struct MCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIFI memory command register"]
pub mod mcmd;
#[doc = "SPIFI status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIFI status register"]
pub mod stat;
