#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer 0 CAP0_0 capture input multiplexer (GIMA output 0)"]
    pub cap0_0_in: CAP0_0_IN,
    #[doc = "0x04 - Timer 0 CAP0_1 capture input multiplexer (GIMA output 1)"]
    pub cap0_1_in: CAP0_1_IN,
    #[doc = "0x08 - Timer 0 CAP0_2 capture input multiplexer (GIMA output 2)"]
    pub cap0_2_in: CAP0_2_IN,
    #[doc = "0x0c - Timer 0 CAP0_3 capture input multiplexer (GIMA output 3)"]
    pub cap0_3_in: CAP0_3_IN,
    #[doc = "0x10 - Timer 1 CAP1_0 capture input multiplexer (GIMA output 4)"]
    pub cap1_0_in: CAP1_0_IN,
    #[doc = "0x14 - Timer 1 CAP1_1 capture input multiplexer (GIMA output 5)"]
    pub cap1_1_in: CAP1_1_IN,
    #[doc = "0x18 - Timer 1 CAP1_2 capture input multiplexer (GIMA output 6)"]
    pub cap1_2_in: CAP1_2_IN,
    #[doc = "0x1c - Timer 1 CAP1_3 capture input multiplexer (GIMA output 7)"]
    pub cap1_3_in: CAP1_3_IN,
    #[doc = "0x20 - Timer 2 CAP2_0 capture input multiplexer (GIMA output 8)"]
    pub cap2_0_in: CAP2_0_IN,
    #[doc = "0x24 - Timer 2 CAP2_1 capture input multiplexer (GIMA output 9)"]
    pub cap2_1_in: CAP2_1_IN,
    #[doc = "0x28 - Timer 2 CAP2_2 capture input multiplexer (GIMA output 10)"]
    pub cap2_2_in: CAP2_2_IN,
    #[doc = "0x2c - Timer 2 CAP2_3 capture input multiplexer (GIMA output 11)"]
    pub cap2_3_in: CAP2_3_IN,
    #[doc = "0x30 - Timer 3 CAP3_0 capture input multiplexer (GIMA output 12)"]
    pub cap3_0_in: CAP3_0_IN,
    #[doc = "0x34 - Timer 3 CAP3_1 capture input multiplexer (GIMA output 13)"]
    pub cap3_1_in: CAP3_1_IN,
    #[doc = "0x38 - Timer 3 CAP3_2 capture input multiplexer (GIMA output 14)"]
    pub cap3_2_in: CAP3_2_IN,
    #[doc = "0x3c - Timer 3 CAP3_3 capture input multiplexer (GIMA output 15)"]
    pub cap3_3_in: CAP3_3_IN,
    #[doc = "0x40 - SCT CTIN_0 capture input multiplexer (GIMA output 16)"]
    pub ctin_0_in: CTIN_0_IN,
    #[doc = "0x44 - SCT CTIN_1 capture input multiplexer (GIMA output 17)"]
    pub ctin_1_in: CTIN_1_IN,
    #[doc = "0x48 - SCT CTIN_2 capture input multiplexer (GIMA output 18)"]
    pub ctin_2_in: CTIN_2_IN,
    #[doc = "0x4c - SCT CTIN_3 capture input multiplexer (GIMA output 19)"]
    pub ctin_3_in: CTIN_3_IN,
    #[doc = "0x50 - SCT CTIN_4 capture input multiplexer (GIMA output 20)"]
    pub ctin_4_in: CTIN_4_IN,
    #[doc = "0x54 - SCT CTIN_5 capture input multiplexer (GIMA output 21)"]
    pub ctin_5_in: CTIN_5_IN,
    #[doc = "0x58 - SCT CTIN_6 capture input multiplexer (GIMA output 22)"]
    pub ctin_6_in: CTIN_6_IN,
    #[doc = "0x5c - SCT CTIN_7 capture input multiplexer (GIMA output 23)"]
    pub ctin_7_in: CTIN_7_IN,
    #[doc = "0x60 - ADCHS trigger input multiplexer (GIMA output 24)"]
    pub adchs_trigger_in: ADCHS_TRIGGER_IN,
    #[doc = "0x64 - Event router input 13 multiplexer (GIMA output 25)"]
    pub eventrouter_13_in: EVENTROUTER_13_IN,
    #[doc = "0x68 - Event router input 14 multiplexer (GIMA output 26)"]
    pub eventrouter_14_in: EVENTROUTER_14_IN,
    #[doc = "0x6c - Event router input 16 multiplexer (GIMA output 27)"]
    pub eventrouter_16_in: EVENTROUTER_16_IN,
    #[doc = "0x70 - ADC start0 input multiplexer (GIMA output 28)"]
    pub adcstart0_in: ADCSTART0_IN,
    #[doc = "0x74 - ADC start1 input multiplexer (GIMA output 29)"]
    pub adcstart1_in: ADCSTART1_IN,
}
#[doc = "Timer 0 CAP0_0 capture input multiplexer (GIMA output 0)"]
pub struct CAP0_0_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 0 CAP0_0 capture input multiplexer (GIMA output 0)"]
pub mod cap0_0_in;
#[doc = "Timer 0 CAP0_1 capture input multiplexer (GIMA output 1)"]
pub struct CAP0_1_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 0 CAP0_1 capture input multiplexer (GIMA output 1)"]
pub mod cap0_1_in;
#[doc = "Timer 0 CAP0_2 capture input multiplexer (GIMA output 2)"]
pub struct CAP0_2_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 0 CAP0_2 capture input multiplexer (GIMA output 2)"]
pub mod cap0_2_in;
#[doc = "Timer 0 CAP0_3 capture input multiplexer (GIMA output 3)"]
pub struct CAP0_3_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 0 CAP0_3 capture input multiplexer (GIMA output 3)"]
pub mod cap0_3_in;
#[doc = "Timer 1 CAP1_0 capture input multiplexer (GIMA output 4)"]
pub struct CAP1_0_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 1 CAP1_0 capture input multiplexer (GIMA output 4)"]
pub mod cap1_0_in;
#[doc = "Timer 1 CAP1_1 capture input multiplexer (GIMA output 5)"]
pub struct CAP1_1_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 1 CAP1_1 capture input multiplexer (GIMA output 5)"]
pub mod cap1_1_in;
#[doc = "Timer 1 CAP1_2 capture input multiplexer (GIMA output 6)"]
pub struct CAP1_2_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 1 CAP1_2 capture input multiplexer (GIMA output 6)"]
pub mod cap1_2_in;
#[doc = "Timer 1 CAP1_3 capture input multiplexer (GIMA output 7)"]
pub struct CAP1_3_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 1 CAP1_3 capture input multiplexer (GIMA output 7)"]
pub mod cap1_3_in;
#[doc = "Timer 2 CAP2_0 capture input multiplexer (GIMA output 8)"]
pub struct CAP2_0_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 2 CAP2_0 capture input multiplexer (GIMA output 8)"]
pub mod cap2_0_in;
#[doc = "Timer 2 CAP2_1 capture input multiplexer (GIMA output 9)"]
pub struct CAP2_1_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 2 CAP2_1 capture input multiplexer (GIMA output 9)"]
pub mod cap2_1_in;
#[doc = "Timer 2 CAP2_2 capture input multiplexer (GIMA output 10)"]
pub struct CAP2_2_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 2 CAP2_2 capture input multiplexer (GIMA output 10)"]
pub mod cap2_2_in;
#[doc = "Timer 2 CAP2_3 capture input multiplexer (GIMA output 11)"]
pub struct CAP2_3_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 2 CAP2_3 capture input multiplexer (GIMA output 11)"]
pub mod cap2_3_in;
#[doc = "Timer 3 CAP3_0 capture input multiplexer (GIMA output 12)"]
pub struct CAP3_0_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 3 CAP3_0 capture input multiplexer (GIMA output 12)"]
pub mod cap3_0_in;
#[doc = "Timer 3 CAP3_1 capture input multiplexer (GIMA output 13)"]
pub struct CAP3_1_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 3 CAP3_1 capture input multiplexer (GIMA output 13)"]
pub mod cap3_1_in;
#[doc = "Timer 3 CAP3_2 capture input multiplexer (GIMA output 14)"]
pub struct CAP3_2_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 3 CAP3_2 capture input multiplexer (GIMA output 14)"]
pub mod cap3_2_in;
#[doc = "Timer 3 CAP3_3 capture input multiplexer (GIMA output 15)"]
pub struct CAP3_3_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 3 CAP3_3 capture input multiplexer (GIMA output 15)"]
pub mod cap3_3_in;
#[doc = "SCT CTIN_0 capture input multiplexer (GIMA output 16)"]
pub struct CTIN_0_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT CTIN_0 capture input multiplexer (GIMA output 16)"]
pub mod ctin_0_in;
#[doc = "SCT CTIN_1 capture input multiplexer (GIMA output 17)"]
pub struct CTIN_1_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT CTIN_1 capture input multiplexer (GIMA output 17)"]
pub mod ctin_1_in;
#[doc = "SCT CTIN_2 capture input multiplexer (GIMA output 18)"]
pub struct CTIN_2_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT CTIN_2 capture input multiplexer (GIMA output 18)"]
pub mod ctin_2_in;
#[doc = "SCT CTIN_3 capture input multiplexer (GIMA output 19)"]
pub struct CTIN_3_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT CTIN_3 capture input multiplexer (GIMA output 19)"]
pub mod ctin_3_in;
#[doc = "SCT CTIN_4 capture input multiplexer (GIMA output 20)"]
pub struct CTIN_4_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT CTIN_4 capture input multiplexer (GIMA output 20)"]
pub mod ctin_4_in;
#[doc = "SCT CTIN_5 capture input multiplexer (GIMA output 21)"]
pub struct CTIN_5_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT CTIN_5 capture input multiplexer (GIMA output 21)"]
pub mod ctin_5_in;
#[doc = "SCT CTIN_6 capture input multiplexer (GIMA output 22)"]
pub struct CTIN_6_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT CTIN_6 capture input multiplexer (GIMA output 22)"]
pub mod ctin_6_in;
#[doc = "SCT CTIN_7 capture input multiplexer (GIMA output 23)"]
pub struct CTIN_7_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT CTIN_7 capture input multiplexer (GIMA output 23)"]
pub mod ctin_7_in;
#[doc = "ADCHS trigger input multiplexer (GIMA output 24)"]
pub struct ADCHS_TRIGGER_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADCHS trigger input multiplexer (GIMA output 24)"]
pub mod adchs_trigger_in;
#[doc = "Event router input 13 multiplexer (GIMA output 25)"]
pub struct EVENTROUTER_13_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event router input 13 multiplexer (GIMA output 25)"]
pub mod eventrouter_13_in;
#[doc = "Event router input 14 multiplexer (GIMA output 26)"]
pub struct EVENTROUTER_14_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event router input 14 multiplexer (GIMA output 26)"]
pub mod eventrouter_14_in;
#[doc = "Event router input 16 multiplexer (GIMA output 27)"]
pub struct EVENTROUTER_16_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event router input 16 multiplexer (GIMA output 27)"]
pub mod eventrouter_16_in;
#[doc = "ADC start0 input multiplexer (GIMA output 28)"]
pub struct ADCSTART0_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC start0 input multiplexer (GIMA output 28)"]
pub mod adcstart0_in;
#[doc = "ADC start1 input multiplexer (GIMA output 29)"]
pub struct ADCSTART1_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC start1 input multiplexer (GIMA output 29)"]
pub mod adcstart1_in;
