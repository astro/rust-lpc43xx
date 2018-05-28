#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub con: CON,
    #[doc = "0x04 - Encoder status register"]
    pub stat: STAT,
    #[doc = "0x08 - Configuration register"]
    pub conf: CONF,
    #[doc = "0x0c - Position register"]
    pub pos: POS,
    #[doc = "0x10 - Maximum position register"]
    pub maxpos: MAXPOS,
    #[doc = "0x14 - position compare register 0"]
    pub cmpos0: CMPOS0,
    #[doc = "0x18 - position compare register 1"]
    pub cmpos1: CMPOS1,
    #[doc = "0x1c - position compare register 2"]
    pub cmpos2: CMPOS2,
    #[doc = "0x20 - Index count register"]
    pub inxcnt: INXCNT,
    #[doc = "0x24 - Index compare register 0"]
    pub inxcmp0: INXCMP0,
    #[doc = "0x28 - Velocity timer reload register"]
    pub load: LOAD,
    #[doc = "0x2c - Velocity timer register"]
    pub time: TIME,
    #[doc = "0x30 - Velocity counter register"]
    pub vel: VEL,
    #[doc = "0x34 - Velocity capture register"]
    pub cap: CAP,
    #[doc = "0x38 - Velocity compare register"]
    pub velcomp: VELCOMP,
    #[doc = "0x3c - Digital filter register on input phase A (QEI_A)"]
    pub filterpha: FILTERPHA,
    #[doc = "0x40 - Digital filter register on input phase B (QEI_B)"]
    pub filterphb: FILTERPHB,
    #[doc = "0x44 - Digital filter register on input index (QEI_IDX)"]
    pub filterinx: FILTERINX,
    #[doc = "0x48 - Index acceptance window register"]
    pub window: WINDOW,
    #[doc = "0x4c - Index compare register 1"]
    pub inxcmp1: INXCMP1,
    #[doc = "0x50 - Index compare register 2"]
    pub inxcmp2: INXCMP2,
    _reserved0: [u8; 3972usize],
    #[doc = "0xfd8 - Interrupt enable clear register"]
    pub iec: IEC,
    #[doc = "0xfdc - Interrupt enable set register"]
    pub ies: IES,
    #[doc = "0xfe0 - Interrupt status register"]
    pub intstat: INTSTAT,
    #[doc = "0xfe4 - Interrupt enable register"]
    pub ie: IE,
    #[doc = "0xfe8 - Interrupt status clear register"]
    pub clr: CLR,
    #[doc = "0xfec - Interrupt status set register"]
    pub set: SET,
}
#[doc = "Control register"]
pub struct CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod con;
#[doc = "Configuration register"]
pub struct CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod conf;
#[doc = "Encoder status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Encoder status register"]
pub mod stat;
#[doc = "Position register"]
pub struct POS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Position register"]
pub mod pos;
#[doc = "Maximum position register"]
pub struct MAXPOS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum position register"]
pub mod maxpos;
#[doc = "position compare register 0"]
pub struct CMPOS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "position compare register 0"]
pub mod cmpos0;
#[doc = "position compare register 1"]
pub struct CMPOS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "position compare register 1"]
pub mod cmpos1;
#[doc = "position compare register 2"]
pub struct CMPOS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "position compare register 2"]
pub mod cmpos2;
#[doc = "Index count register"]
pub struct INXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Index count register"]
pub mod inxcnt;
#[doc = "Index compare register 0"]
pub struct INXCMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Index compare register 0"]
pub mod inxcmp0;
#[doc = "Velocity timer reload register"]
pub struct LOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Velocity timer reload register"]
pub mod load;
#[doc = "Velocity timer register"]
pub struct TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Velocity timer register"]
pub mod time;
#[doc = "Velocity counter register"]
pub struct VEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Velocity counter register"]
pub mod vel;
#[doc = "Velocity capture register"]
pub struct CAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Velocity capture register"]
pub mod cap;
#[doc = "Velocity compare register"]
pub struct VELCOMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Velocity compare register"]
pub mod velcomp;
#[doc = "Digital filter register on input phase A (QEI_A)"]
pub struct FILTERPHA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital filter register on input phase A (QEI_A)"]
pub mod filterpha;
#[doc = "Digital filter register on input phase B (QEI_B)"]
pub struct FILTERPHB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital filter register on input phase B (QEI_B)"]
pub mod filterphb;
#[doc = "Digital filter register on input index (QEI_IDX)"]
pub struct FILTERINX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital filter register on input index (QEI_IDX)"]
pub mod filterinx;
#[doc = "Index acceptance window register"]
pub struct WINDOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Index acceptance window register"]
pub mod window;
#[doc = "Index compare register 1"]
pub struct INXCMP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Index compare register 1"]
pub mod inxcmp1;
#[doc = "Index compare register 2"]
pub struct INXCMP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Index compare register 2"]
pub mod inxcmp2;
#[doc = "Interrupt enable clear register"]
pub struct IEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable clear register"]
pub mod iec;
#[doc = "Interrupt enable set register"]
pub struct IES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable set register"]
pub mod ies;
#[doc = "Interrupt status register"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status register"]
pub mod intstat;
#[doc = "Interrupt enable register"]
pub struct IE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable register"]
pub mod ie;
#[doc = "Interrupt status clear register"]
pub struct CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status clear register"]
pub mod clr;
#[doc = "Interrupt status set register"]
pub struct SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status set register"]
pub mod set;
