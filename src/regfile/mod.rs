#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General purpose storage register"]
    pub regfile: [REGFILE; 64],
}
#[doc = "General purpose storage register"]
pub struct REGFILE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General purpose storage register"]
pub mod regfile;
