#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - Reset control register 0"]
    pub reset_ctrl0: RESET_CTRL0,
    #[doc = "0x104 - Reset control register 1"]
    pub reset_ctrl1: RESET_CTRL1,
    _reserved1: [u8; 8usize],
    #[doc = "0x110 - Reset status register 0"]
    pub reset_status0: RESET_STATUS0,
    #[doc = "0x114 - Reset status register 1"]
    pub reset_status1: RESET_STATUS1,
    #[doc = "0x118 - Reset status register 2"]
    pub reset_status2: RESET_STATUS2,
    #[doc = "0x11c - Reset status register 3"]
    pub reset_status3: RESET_STATUS3,
    _reserved2: [u8; 48usize],
    #[doc = "0x150 - Reset active status register 0"]
    pub reset_active_status0: RESET_ACTIVE_STATUS0,
    #[doc = "0x154 - Reset active status register 1"]
    pub reset_active_status1: RESET_ACTIVE_STATUS1,
    _reserved3: [u8; 684usize],
    #[doc = "0x404 - Reset external status register 1 for PERIPH_RST"]
    pub reset_ext_stat1: RESET_EXT_STAT1,
    #[doc = "0x408 - Reset external status register 2 for MASTER_RST"]
    pub reset_ext_stat2: RESET_EXT_STAT2,
    _reserved4: [u8; 8usize],
    #[doc = "0x414 - Reset external status register 5 for CREG_RST"]
    pub reset_ext_stat5: RESET_EXT_STAT5,
    _reserved5: [u8; 8usize],
    #[doc = "0x420 - Reset external status register"]
    pub reset_ext_stat8: RESET_EXT_STAT8,
    #[doc = "0x424 - Reset external status register"]
    pub reset_ext_stat9: RESET_EXT_STAT9,
    _reserved6: [u8; 8usize],
    #[doc = "0x430 - Reset external status register"]
    pub reset_ext_stat12: RESET_EXT_STAT12,
    #[doc = "0x434 - Reset external status register"]
    pub reset_ext_stat13: RESET_EXT_STAT13,
    _reserved7: [u8; 8usize],
    #[doc = "0x440 - Reset external status register"]
    pub reset_ext_stat16: RESET_EXT_STAT16,
    #[doc = "0x444 - Reset external status register"]
    pub reset_ext_stat17: RESET_EXT_STAT17,
    #[doc = "0x448 - Reset external status register"]
    pub reset_ext_stat18: RESET_EXT_STAT18,
    #[doc = "0x44c - Reset external status register"]
    pub reset_ext_stat19: RESET_EXT_STAT19,
    #[doc = "0x450 - Reset external status register"]
    pub reset_ext_stat20: RESET_EXT_STAT20,
    #[doc = "0x454 - Reset external status register"]
    pub reset_ext_stat21: RESET_EXT_STAT21,
    #[doc = "0x458 - Reset external status register"]
    pub reset_ext_stat22: RESET_EXT_STAT22,
    _reserved8: [u8; 8usize],
    #[doc = "0x464 - Reset external status register"]
    pub reset_ext_stat25: RESET_EXT_STAT25,
    _reserved9: [u8; 4usize],
    #[doc = "0x46c - Reset external status register"]
    pub reset_ext_stat27: RESET_EXT_STAT27,
    #[doc = "0x470 - Reset external status register"]
    pub reset_ext_stat28: RESET_EXT_STAT28,
    #[doc = "0x474 - Reset external status register"]
    pub reset_ext_stat29: RESET_EXT_STAT29,
    _reserved10: [u8; 8usize],
    #[doc = "0x480 - Reset external status register"]
    pub reset_ext_stat32: RESET_EXT_STAT32,
    #[doc = "0x484 - Reset external status register"]
    pub reset_ext_stat33: RESET_EXT_STAT33,
    #[doc = "0x488 - Reset external status register"]
    pub reset_ext_stat34: RESET_EXT_STAT34,
    #[doc = "0x48c - Reset external status register"]
    pub reset_ext_stat35: RESET_EXT_STAT35,
    #[doc = "0x490 - Reset external status register"]
    pub reset_ext_stat36: RESET_EXT_STAT36,
    #[doc = "0x494 - Reset external status register"]
    pub reset_ext_stat37: RESET_EXT_STAT37,
    #[doc = "0x498 - Reset external status register"]
    pub reset_ext_stat38: RESET_EXT_STAT38,
    #[doc = "0x49c - Reset external status register"]
    pub reset_ext_stat39: RESET_EXT_STAT39,
    #[doc = "0x4a0 - Reset external status register"]
    pub reset_ext_stat40: RESET_EXT_STAT40,
    #[doc = "0x4a4 - Reset external status register"]
    pub reset_ext_stat41: RESET_EXT_STAT41,
    #[doc = "0x4a8 - Reset external status register"]
    pub reset_ext_stat42: RESET_EXT_STAT42,
    _reserved11: [u8; 4usize],
    #[doc = "0x4b0 - Reset external status register"]
    pub reset_ext_stat44: RESET_EXT_STAT44,
    #[doc = "0x4b4 - Reset external status register"]
    pub reset_ext_stat45: RESET_EXT_STAT45,
    #[doc = "0x4b8 - Reset external status register"]
    pub reset_ext_stat46: RESET_EXT_STAT46,
    #[doc = "0x4bc - Reset external status register"]
    pub reset_ext_stat47: RESET_EXT_STAT47,
    #[doc = "0x4c0 - Reset external status register"]
    pub reset_ext_stat48: RESET_EXT_STAT48,
    #[doc = "0x4c4 - Reset external status register"]
    pub reset_ext_stat49: RESET_EXT_STAT49,
    #[doc = "0x4c8 - Reset external status register"]
    pub reset_ext_stat50: RESET_EXT_STAT50,
    #[doc = "0x4cc - Reset external status register"]
    pub reset_ext_stat51: RESET_EXT_STAT51,
    #[doc = "0x4d0 - Reset external status register"]
    pub reset_ext_stat52: RESET_EXT_STAT52,
    #[doc = "0x4d4 - Reset external status register"]
    pub reset_ext_stat53: RESET_EXT_STAT53,
    #[doc = "0x4d8 - Reset external status register"]
    pub reset_ext_stat54: RESET_EXT_STAT54,
    #[doc = "0x4dc - Reset external status register"]
    pub reset_ext_stat55: RESET_EXT_STAT55,
    #[doc = "0x4e0 - Reset external status register"]
    pub reset_ext_stat56: RESET_EXT_STAT56,
    #[doc = "0x4e4 - Reset external status register"]
    pub reset_ext_stat57: RESET_EXT_STAT57,
    #[doc = "0x4e8 - Reset external status register"]
    pub reset_ext_stat58: RESET_EXT_STAT58,
    _reserved12: [u8; 4usize],
    #[doc = "0x4f0 - Reset external status register"]
    pub reset_ext_stat60: RESET_EXT_STAT60,
}
#[doc = "Reset control register 0"]
pub struct RESET_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset control register 0"]
pub mod reset_ctrl0;
#[doc = "Reset control register 1"]
pub struct RESET_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset control register 1"]
pub mod reset_ctrl1;
#[doc = "Reset status register 0"]
pub struct RESET_STATUS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset status register 0"]
pub mod reset_status0;
#[doc = "Reset status register 1"]
pub struct RESET_STATUS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset status register 1"]
pub mod reset_status1;
#[doc = "Reset status register 2"]
pub struct RESET_STATUS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset status register 2"]
pub mod reset_status2;
#[doc = "Reset status register 3"]
pub struct RESET_STATUS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset status register 3"]
pub mod reset_status3;
#[doc = "Reset active status register 0"]
pub struct RESET_ACTIVE_STATUS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset active status register 0"]
pub mod reset_active_status0;
#[doc = "Reset active status register 1"]
pub struct RESET_ACTIVE_STATUS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset active status register 1"]
pub mod reset_active_status1;
#[doc = "Reset external status register 1 for PERIPH_RST"]
pub struct RESET_EXT_STAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register 1 for PERIPH_RST"]
pub mod reset_ext_stat1;
#[doc = "Reset external status register 2 for MASTER_RST"]
pub struct RESET_EXT_STAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register 2 for MASTER_RST"]
pub mod reset_ext_stat2;
#[doc = "Reset external status register 5 for CREG_RST"]
pub struct RESET_EXT_STAT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register 5 for CREG_RST"]
pub mod reset_ext_stat5;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat8;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat9;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat12;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat13;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat16;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat17;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat18;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat19;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat20;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat21;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat22;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat25;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat27;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat28;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat29;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat32;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat33;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat34;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat35;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat36;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat37;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat38;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat39;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat40;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat41;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat42;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat44;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat45;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat46;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat47;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat48;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat49;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat50;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat51;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat52;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat53;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat54;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat55;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat56;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT57 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat57;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT58 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat58;
#[doc = "Reset external status register"]
pub struct RESET_EXT_STAT60 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset external status register"]
pub mod reset_ext_stat60;
