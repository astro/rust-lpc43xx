#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCT configuration register"]
    pub config: CONFIG,
    #[doc = "0x04 - SCT control register"]
    pub ctrl: CTRL,
    #[doc = "0x08 - SCT limit register"]
    pub limit: LIMIT,
    #[doc = "0x0c - SCT halt condition register"]
    pub halt: HALT,
    #[doc = "0x10 - SCT stop condition register"]
    pub stop: STOP,
    #[doc = "0x14 - SCT start condition register"]
    pub start: START,
    #[doc = "0x18 - SCT dither condition register"]
    pub dither: DITHER,
    _reserved0: [u8; 36usize],
    #[doc = "0x40 - SCT counter register"]
    pub count: COUNT,
    #[doc = "0x44 - SCT state register"]
    pub state: STATE,
    #[doc = "0x48 - SCT input register"]
    pub input: INPUT,
    #[doc = "0x4c - SCT match/capture registers mode register"]
    pub regmode: REGMODE,
    #[doc = "0x50 - SCT output register"]
    pub output: OUTPUT,
    #[doc = "0x54 - SCT output counter direction control register"]
    pub outputdirctrl: OUTPUTDIRCTRL,
    #[doc = "0x58 - SCT conflict resolution register"]
    pub res: RES,
    #[doc = "0x5c - SCT DMA request 0 register"]
    pub dmareq0: DMAREQ0,
    #[doc = "0x60 - SCT DMA request 1 register"]
    pub dmareq1: DMAREQ1,
    _reserved1: [u8; 140usize],
    #[doc = "0xf0 - SCT event enable register"]
    pub even: EVEN,
    #[doc = "0xf4 - SCT event flag register"]
    pub evflag: EVFLAG,
    #[doc = "0xf8 - SCT conflict enable register"]
    pub conen: CONEN,
    #[doc = "0xfc - SCT conflict flag register"]
    pub conflag: CONFLAG,
    #[doc = "0x100 - SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0"]
    pub match_: [MATCH; 16],
    #[doc = "0x140 - Fractional match registers 0 to 5 for SCT match value registers 0 to 5."]
    pub fracmat: [FRACMAT; 6],
    _reserved2: [u8; 168usize],
    #[doc = "0x200 - SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0"]
    pub matchrel: [MATCHREL; 16],
    #[doc = "0x240 - Fractional match reload registers 0 to 5 for SCT match value registers 0 to 5."]
    pub fracmatrel: [FRACMATREL; 6],
    _reserved3: [u8; 168usize],
    #[doc = "0x300 - SCT event state register 0"]
    pub ev0_state: EV_STATE,
    #[doc = "0x304 - SCT event control register 0"]
    pub ev0_ctrl: EV_CTRL,
    #[doc = "0x308 - SCT event state register 0"]
    pub ev1_state: EV_STATE,
    #[doc = "0x30c - SCT event control register 0"]
    pub ev1_ctrl: EV_CTRL,
    #[doc = "0x310 - SCT event state register 0"]
    pub ev2_state: EV_STATE,
    #[doc = "0x314 - SCT event control register 0"]
    pub ev2_ctrl: EV_CTRL,
    #[doc = "0x318 - SCT event state register 0"]
    pub ev3_state: EV_STATE,
    #[doc = "0x31c - SCT event control register 0"]
    pub ev3_ctrl: EV_CTRL,
    #[doc = "0x320 - SCT event state register 0"]
    pub ev4_state: EV_STATE,
    #[doc = "0x324 - SCT event control register 0"]
    pub ev4_ctrl: EV_CTRL,
    #[doc = "0x328 - SCT event state register 0"]
    pub ev5_state: EV_STATE,
    #[doc = "0x32c - SCT event control register 0"]
    pub ev5_ctrl: EV_CTRL,
    #[doc = "0x330 - SCT event state register 0"]
    pub ev6_state: EV_STATE,
    #[doc = "0x334 - SCT event control register 0"]
    pub ev6_ctrl: EV_CTRL,
    #[doc = "0x338 - SCT event state register 0"]
    pub ev7_state: EV_STATE,
    #[doc = "0x33c - SCT event control register 0"]
    pub ev7_ctrl: EV_CTRL,
    #[doc = "0x340 - SCT event state register 0"]
    pub ev8_state: EV_STATE,
    #[doc = "0x344 - SCT event control register 0"]
    pub ev8_ctrl: EV_CTRL,
    #[doc = "0x348 - SCT event state register 0"]
    pub ev9_state: EV_STATE,
    #[doc = "0x34c - SCT event control register 0"]
    pub ev9_ctrl: EV_CTRL,
    #[doc = "0x350 - SCT event state register 0"]
    pub ev10_state: EV_STATE,
    #[doc = "0x354 - SCT event control register 0"]
    pub ev10_ctrl: EV_CTRL,
    #[doc = "0x358 - SCT event state register 0"]
    pub ev11_state: EV_STATE,
    #[doc = "0x35c - SCT event control register 0"]
    pub ev11_ctrl: EV_CTRL,
    #[doc = "0x360 - SCT event state register 0"]
    pub ev12_state: EV_STATE,
    #[doc = "0x364 - SCT event control register 0"]
    pub ev12_ctrl: EV_CTRL,
    #[doc = "0x368 - SCT event state register 0"]
    pub ev13_state: EV_STATE,
    #[doc = "0x36c - SCT event control register 0"]
    pub ev13_ctrl: EV_CTRL,
    #[doc = "0x370 - SCT event state register 0"]
    pub ev14_state: EV_STATE,
    #[doc = "0x374 - SCT event control register 0"]
    pub ev14_ctrl: EV_CTRL,
    #[doc = "0x378 - SCT event state register 0"]
    pub ev15_state: EV_STATE,
    #[doc = "0x37c - SCT event control register 0"]
    pub ev15_ctrl: EV_CTRL,
    _reserved4: [u8; 384usize],
    #[doc = "0x500 - SCT output 0 set register"]
    pub out0_set: OUT_SET,
    #[doc = "0x504 - SCT output 0 clear register"]
    pub out0_clr: OUT_CLR,
    #[doc = "0x508 - SCT output 0 set register"]
    pub out1_set: OUT_SET,
    #[doc = "0x50c - SCT output 0 clear register"]
    pub out1_clr: OUT_CLR,
    #[doc = "0x510 - SCT output 0 set register"]
    pub out2_set: OUT_SET,
    #[doc = "0x514 - SCT output 0 clear register"]
    pub out2_clr: OUT_CLR,
    #[doc = "0x518 - SCT output 0 set register"]
    pub out3_set: OUT_SET,
    #[doc = "0x51c - SCT output 0 clear register"]
    pub out3_clr: OUT_CLR,
    #[doc = "0x520 - SCT output 0 set register"]
    pub out4_set: OUT_SET,
    #[doc = "0x524 - SCT output 0 clear register"]
    pub out4_clr: OUT_CLR,
    #[doc = "0x528 - SCT output 0 set register"]
    pub out5_set: OUT_SET,
    #[doc = "0x52c - SCT output 0 clear register"]
    pub out5_clr: OUT_CLR,
    #[doc = "0x530 - SCT output 0 set register"]
    pub out6_set: OUT_SET,
    #[doc = "0x534 - SCT output 0 clear register"]
    pub out6_clr: OUT_CLR,
    #[doc = "0x538 - SCT output 0 set register"]
    pub out7_set: OUT_SET,
    #[doc = "0x53c - SCT output 0 clear register"]
    pub out7_clr: OUT_CLR,
    #[doc = "0x540 - SCT output 0 set register"]
    pub out8_set: OUT_SET,
    #[doc = "0x544 - SCT output 0 clear register"]
    pub out8_clr: OUT_CLR,
    #[doc = "0x548 - SCT output 0 set register"]
    pub out9_set: OUT_SET,
    #[doc = "0x54c - SCT output 0 clear register"]
    pub out9_clr: OUT_CLR,
    #[doc = "0x550 - SCT output 0 set register"]
    pub out10_set: OUT_SET,
    #[doc = "0x554 - SCT output 0 clear register"]
    pub out10_clr: OUT_CLR,
    #[doc = "0x558 - SCT output 0 set register"]
    pub out11_set: OUT_SET,
    #[doc = "0x55c - SCT output 0 clear register"]
    pub out11_clr: OUT_CLR,
    #[doc = "0x560 - SCT output 0 set register"]
    pub out12_set: OUT_SET,
    #[doc = "0x564 - SCT output 0 clear register"]
    pub out12_clr: OUT_CLR,
    #[doc = "0x568 - SCT output 0 set register"]
    pub out13_set: OUT_SET,
    #[doc = "0x56c - SCT output 0 clear register"]
    pub out13_clr: OUT_CLR,
    #[doc = "0x570 - SCT output 0 set register"]
    pub out14_set: OUT_SET,
    #[doc = "0x574 - SCT output 0 clear register"]
    pub out14_clr: OUT_CLR,
    #[doc = "0x578 - SCT output 0 set register"]
    pub out15_set: OUT_SET,
    #[doc = "0x57c - SCT output 0 clear register"]
    pub out15_clr: OUT_CLR,
}
#[doc = "SCT configuration register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT configuration register"]
pub mod config;
#[doc = "SCT control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT control register"]
pub mod ctrl;
#[doc = "SCT limit register"]
pub struct LIMIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT limit register"]
pub mod limit;
#[doc = "SCT halt condition register"]
pub struct HALT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT halt condition register"]
pub mod halt;
#[doc = "SCT stop condition register"]
pub struct STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT stop condition register"]
pub mod stop;
#[doc = "SCT start condition register"]
pub struct START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT start condition register"]
pub mod start;
#[doc = "SCT dither condition register"]
pub struct DITHER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT dither condition register"]
pub mod dither;
#[doc = "SCT counter register"]
pub struct COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT counter register"]
pub mod count;
#[doc = "SCT state register"]
pub struct STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT state register"]
pub mod state;
#[doc = "SCT input register"]
pub struct INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT input register"]
pub mod input;
#[doc = "SCT match/capture registers mode register"]
pub struct REGMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match/capture registers mode register"]
pub mod regmode;
#[doc = "SCT output register"]
pub struct OUTPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output register"]
pub mod output;
#[doc = "SCT output counter direction control register"]
pub struct OUTPUTDIRCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output counter direction control register"]
pub mod outputdirctrl;
#[doc = "SCT conflict resolution register"]
pub struct RES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT conflict resolution register"]
pub mod res;
#[doc = "SCT DMA request 0 register"]
pub struct DMAREQ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT DMA request 0 register"]
pub mod dmareq0;
#[doc = "SCT DMA request 1 register"]
pub struct DMAREQ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT DMA request 1 register"]
pub mod dmareq1;
#[doc = "SCT event enable register"]
pub struct EVEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event enable register"]
pub mod even;
#[doc = "SCT event flag register"]
pub struct EVFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event flag register"]
pub mod evflag;
#[doc = "SCT conflict enable register"]
pub struct CONEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT conflict enable register"]
pub mod conen;
#[doc = "SCT conflict flag register"]
pub struct CONFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT conflict flag register"]
pub mod conflag;
#[doc = "SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0"]
pub struct MATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0"]
pub mod match_;
#[doc = "Fractional match registers 0 to 5 for SCT match value registers 0 to 5."]
pub struct FRACMAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional match registers 0 to 5 for SCT match value registers 0 to 5."]
pub mod fracmat;
#[doc = "SCT capture register of capture channel 0 to 15; REGMOD0 to REGMODE15 = 1"]
pub struct CAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel 0 to 15; REGMOD0 to REGMODE15 = 1"]
pub mod cap;
#[doc = "SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0"]
pub struct MATCHREL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0"]
pub mod matchrel;
#[doc = "Fractional match reload registers 0 to 5 for SCT match value registers 0 to 5."]
pub struct FRACMATREL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional match reload registers 0 to 5 for SCT match value registers 0 to 5."]
pub mod fracmatrel;
#[doc = "SCT capture control register 0 to 15; REGMOD0 = 1 to REGMODE15 = 1"]
pub struct CAPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register 0 to 15; REGMOD0 = 1 to REGMODE15 = 1"]
pub mod capctrl;
#[doc = "SCT event state register 0"]
pub struct EV_STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event state register 0"]
pub mod ev_state;
#[doc = "SCT event control register 0"]
pub struct EV_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event control register 0"]
pub mod ev_ctrl;
#[doc = "SCT output 0 set register"]
pub struct OUT_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output 0 set register"]
pub mod out_set;
#[doc = "SCT output 0 clear register"]
pub struct OUT_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output 0 clear register"]
pub mod out_clr;
