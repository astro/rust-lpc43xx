#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin configuration register for pins P0"]
    pub sfsp0: [SFSP0; 2],
    _reserved0: [u8; 120usize],
    #[doc = "0x80 - Pin configuration register for pins P1"]
    pub sfsp1: [SFSP1; 17],
    #[doc = "0xc4 - Pin configuration register for pins P1_17"]
    pub sfsp1_17: SFSP1_17,
    #[doc = "0xc8 - Pin configuration register for pins P1"]
    pub sfsp1_18: SFSP1,
    #[doc = "0xcc - Pin configuration register for pins P1"]
    pub sfsp1_19: SFSP1,
    #[doc = "0xd0 - Pin configuration register for pins P1"]
    pub sfsp1_20: SFSP1,
    _reserved1: [u8; 44usize],
    #[doc = "0x100 - Pin configuration register for pins P2"]
    pub sfsp2: [SFSP2; 3],
    #[doc = "0x10c - Pin configuration register for pins P2"]
    pub sfsp2_3: SFSP2,
    #[doc = "0x110 - Pin configuration register for pins P2"]
    pub sfsp2_4: SFSP2,
    #[doc = "0x114 - Pin configuration register for pins P2"]
    pub sfsp2_5: SFSP2,
    #[doc = "0x118 - Pin configuration register for pins P2"]
    pub sfsp2_6: SFSP2,
    #[doc = "0x11c - Pin configuration register for pins P2"]
    pub sfsp2_7: SFSP2,
    #[doc = "0x120 - Pin configuration register for pins P2"]
    pub sfsp2_8: SFSP2,
    #[doc = "0x124 - Pin configuration register for pins P2"]
    pub sfsp2_9: SFSP2,
    #[doc = "0x128 - Pin configuration register for pins P2"]
    pub sfsp2_10: SFSP2,
    #[doc = "0x12c - Pin configuration register for pins P2"]
    pub sfsp2_11: SFSP2,
    #[doc = "0x130 - Pin configuration register for pins P2"]
    pub sfsp2_12: SFSP2,
    _reserved2: [u8; 76usize],
    #[doc = "0x180 - Pin configuration register for pins P3"]
    pub sfsp3: [SFSP3; 3],
    #[doc = "0x18c - Pin configuration register for pins P3"]
    pub sfsp3_3: SFSP3_3,
    #[doc = "0x190 - Pin configuration register for pins P3"]
    pub sfsp3_4: SFSP3,
    #[doc = "0x194 - Pin configuration register for pins P3"]
    pub sfsp3_5: SFSP3,
    #[doc = "0x198 - Pin configuration register for pins P3"]
    pub sfsp3_6: SFSP3,
    #[doc = "0x19c - Pin configuration register for pins P3"]
    pub sfsp3_7: SFSP3,
    #[doc = "0x1a0 - Pin configuration register for pins P3"]
    pub sfsp3_8: SFSP3,
    _reserved3: [u8; 92usize],
    #[doc = "0x200 - Pin configuration register for pins P4"]
    pub sfsp4: [SFSP4; 11],
    _reserved4: [u8; 84usize],
    #[doc = "0x280 - Pin configuration register for pins P5"]
    pub sfsp5: [SFSP5; 8],
    _reserved5: [u8; 96usize],
    #[doc = "0x300 - Pin configuration register for pins P6"]
    pub sfsp6: [SFSP6; 13],
    _reserved6: [u8; 76usize],
    #[doc = "0x380 - Pin configuration register for pins P7"]
    pub sfsp7: [SFSP7; 8],
    _reserved7: [u8; 96usize],
    #[doc = "0x400 - Pin configuration register for pins P8"]
    pub sfsp8: [SFSP8; 3],
    #[doc = "0x40c - Pin configuration register for pins P8"]
    pub sfsp8_3: SFSP8,
    #[doc = "0x410 - Pin configuration register for pins P8"]
    pub sfsp8_4: SFSP8,
    #[doc = "0x414 - Pin configuration register for pins P8"]
    pub sfsp8_5: SFSP8,
    #[doc = "0x418 - Pin configuration register for pins P8"]
    pub sfsp8_6: SFSP8,
    #[doc = "0x41c - Pin configuration register for pins P8"]
    pub sfsp8_7: SFSP8,
    #[doc = "0x420 - Pin configuration register for pins P8"]
    pub sfsp8_8: SFSP8,
    _reserved8: [u8; 92usize],
    #[doc = "0x480 - Pin configuration register for pins P9"]
    pub sfsp9: [SFSP9; 7],
    _reserved9: [u8; 100usize],
    #[doc = "0x500 - Pin configuration register for pins PA"]
    pub sfspa_0: SFSPA_0,
    #[doc = "0x504 - Pin configuration register for pins PA"]
    pub sfspa_1: SFSPA,
    #[doc = "0x508 - Pin configuration register for pins PA"]
    pub sfspa_2: SFSPA,
    #[doc = "0x50c - Pin configuration register for pins PA"]
    pub sfspa_3: SFSPA,
    #[doc = "0x510 - Pin configuration register for pins PA"]
    pub sfspa_4: SFSPA_4,
    _reserved10: [u8; 108usize],
    #[doc = "0x580 - Pin configuration register for pins PB"]
    pub sfspb: [SFSPB; 7],
    _reserved11: [u8; 100usize],
    #[doc = "0x600 - Pin configuration register for pins PC"]
    pub sfspc: [SFSPC; 15],
    _reserved12: [u8; 68usize],
    #[doc = "0x680 - Pin configuration register for pins PD"]
    pub sfspd: [SFSPD; 17],
    _reserved13: [u8; 60usize],
    #[doc = "0x700 - Pin configuration register for pins PE"]
    pub sfspe: [SFSPE; 16],
    _reserved14: [u8; 64usize],
    #[doc = "0x780 - Pin configuration register for pins PF"]
    pub sfspf: [SFSPF; 12],
    _reserved15: [u8; 1104usize],
    #[doc = "0xc00 - Pin configuration register for pins CLK"]
    pub sfsclk: [SFSCLK; 4],
    _reserved16: [u8; 112usize],
    #[doc = "0xc80 - Pin configuration register for pins USB1_DM and USB1_DP"]
    pub sfsusb: SFSUSB,
    #[doc = "0xc84 - Pin configuration register for I2C0-bus pins"]
    pub sfsi2c0: SFSI2C0,
    #[doc = "0xc88 - ADC0 function select register"]
    pub enaio0: ENAIO0,
    #[doc = "0xc8c - ADC1 function select register"]
    pub enaio1: ENAIO1,
    #[doc = "0xc90 - Analog function select register"]
    pub enaio2: ENAIO2,
    _reserved17: [u8; 108usize],
    #[doc = "0xd00 - EMC clock delay register"]
    pub emcdelayclk: EMCDELAYCLK,
    _reserved18: [u8; 124usize],
    #[doc = "0xd80 - SD/MMC sample and drive delay register"]
    pub sddelay: SDDELAY,
    _reserved19: [u8; 124usize],
    #[doc = "0xe00 - Pin interrupt select register for pin interrupts 0 to 3."]
    pub pintsel0: PINTSEL0,
    #[doc = "0xe04 - Pin interrupt select register for pin interrupts 4 to 7."]
    pub pintsel1: PINTSEL1,
}
#[doc = "Pin configuration register for pins P0"]
pub struct SFSP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins P0"]
pub mod sfsp0;
#[doc = "Pin configuration register for pins P1"]
pub struct SFSP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins P1"]
pub mod sfsp1;
#[doc = "Pin configuration register for pins P1_17"]
pub struct SFSP1_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins P1_17"]
pub mod sfsp1_17;
#[doc = "Pin configuration register for pins P2"]
pub struct SFSP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins P2"]
pub mod sfsp2;
#[doc = "Pin configuration register for pins P3"]
pub struct SFSP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins P3"]
pub mod sfsp3;
#[doc = "Pin configuration register for pins P3"]
pub struct SFSP3_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins P3"]
pub mod sfsp3_3;
#[doc = "Pin configuration register for pins P4"]
pub struct SFSP4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins P4"]
pub mod sfsp4;
#[doc = "Pin configuration register for pins P5"]
pub struct SFSP5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins P5"]
pub mod sfsp5;
#[doc = "Pin configuration register for pins P6"]
pub struct SFSP6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins P6"]
pub mod sfsp6;
#[doc = "Pin configuration register for pins P7"]
pub struct SFSP7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins P7"]
pub mod sfsp7;
#[doc = "Pin configuration register for pins P8"]
pub struct SFSP8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins P8"]
pub mod sfsp8;
#[doc = "Pin configuration register for pins P9"]
pub struct SFSP9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins P9"]
pub mod sfsp9;
#[doc = "Pin configuration register for pins PA"]
pub struct SFSPA_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins PA"]
pub mod sfspa_0;
#[doc = "Pin configuration register for pins PA"]
pub struct SFSPA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins PA"]
pub mod sfspa;
#[doc = "Pin configuration register for pins PA"]
pub struct SFSPA_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins PA"]
pub mod sfspa_4;
#[doc = "Pin configuration register for pins PB"]
pub struct SFSPB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins PB"]
pub mod sfspb;
#[doc = "Pin configuration register for pins PC"]
pub struct SFSPC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins PC"]
pub mod sfspc;
#[doc = "Pin configuration register for pins PD"]
pub struct SFSPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins PD"]
pub mod sfspd;
#[doc = "Pin configuration register for pins PE"]
pub struct SFSPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins PE"]
pub mod sfspe;
#[doc = "Pin configuration register for pins PF"]
pub struct SFSPF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins PF"]
pub mod sfspf;
#[doc = "Pin configuration register for pins CLK"]
pub struct SFSCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins CLK"]
pub mod sfsclk;
#[doc = "Pin configuration register for pins USB1_DM and USB1_DP"]
pub struct SFSUSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for pins USB1_DM and USB1_DP"]
pub mod sfsusb;
#[doc = "Pin configuration register for I2C0-bus pins"]
pub struct SFSI2C0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin configuration register for I2C0-bus pins"]
pub mod sfsi2c0;
#[doc = "ADC0 function select register"]
pub struct ENAIO0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC0 function select register"]
pub mod enaio0;
#[doc = "ADC1 function select register"]
pub struct ENAIO1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1 function select register"]
pub mod enaio1;
#[doc = "Analog function select register"]
pub struct ENAIO2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog function select register"]
pub mod enaio2;
#[doc = "EMC clock delay register"]
pub struct EMCDELAYCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EMC clock delay register"]
pub mod emcdelayclk;
#[doc = "SD/MMC sample and drive delay register"]
pub struct SDDELAY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SD/MMC sample and drive delay register"]
pub mod sddelay;
#[doc = "Pin interrupt select register for pin interrupts 0 to 3."]
pub struct PINTSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt select register for pin interrupts 0 to 3."]
pub mod pintsel0;
#[doc = "Pin interrupt select register for pin interrupts 4 to 7."]
pub struct PINTSEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt select register for pin interrupts 4 to 7."]
pub mod pintsel1;
