#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flushes FIFO"]
    pub flush: FLUSH,
    #[doc = "0x04 - Set or clear DMA write request"]
    pub dma_req: DMA_REQ,
    #[doc = "0x08 - Indicates FIFO fill level status"]
    pub fifo_sts: FIFO_STS,
    #[doc = "0x0c - Configures FIFO fill level that triggers interrupt and packing 1 or 2 samples per word."]
    pub fifo_cfg: FIFO_CFG,
    #[doc = "0x10 - Enable software trigger to start descriptor processing"]
    pub trigger: TRIGGER,
    #[doc = "0x14 - Indicates active descriptor table and descriptor entry"]
    pub dscr_sts: DSCR_STS,
    #[doc = "0x18 - Set or clear power down mode"]
    pub power_down: POWER_DOWN,
    #[doc = "0x1c - Configures external trigger mode, store channel ID in FIFO and walk-up recovery time from power down."]
    pub config: CONFIG,
    #[doc = "0x20 - Configures window comparator A levels."]
    pub thr_a: THR_A,
    #[doc = "0x24 - Configures window comparator B levels."]
    pub thr_b: THR_B,
    #[doc = "0x28 - Contains last converted sample of input M [M=0..5) and result of window comparator."]
    pub last_sample: [LAST_SAMPLE; 6],
    _reserved0: [u8; 196usize],
    #[doc = "0x104 - ADC speed control"]
    pub adc_speed: ADC_SPEED,
    #[doc = "0x108 - Configures ADC power vs. speed, DC-in biasing, output format and power gating."]
    pub power_control: POWER_CONTROL,
    _reserved1: [u8; 244usize],
    #[doc = "0x200 - FIFO output mapped to 16 consecutive address locations. An output contains the value and input channel ID of one or two converted samples"]
    pub fifo_output: [FIFO_OUTPUT; 16],
    _reserved2: [u8; 192usize],
    #[doc = "0x300 - Table 0 descriptor n, n= 0 to 7"]
    pub descriptor0: [DESCRIPTOR0; 8],
    #[doc = "0x320 - Table 1 descriptors n, n=0 to 7"]
    pub descriptor1: [DESCRIPTOR1; 8],
    _reserved3: [u8; 3008usize],
    #[doc = "0xf00 - Interrupt 0 clear mask"]
    pub clr_en0: CLR_EN0,
    #[doc = "0xf04 - Interrupt 0 set mask"]
    pub set_en0: SET_EN0,
    #[doc = "0xf08 - Interrupt 0 mask"]
    pub mask0: MASK0,
    #[doc = "0xf0c - Interrupt 0 status. Interrupt 0 contains FIFO fill level, descriptor status and ADC range under/overflow"]
    pub status0: STATUS0,
    #[doc = "0xf10 - Interrupt 0 clear status"]
    pub clr_stat0: CLR_STAT0,
    #[doc = "0xf14 - Interrupt 0 set status"]
    pub set_stat0: SET_STAT0,
    _reserved4: [u8; 8usize],
    #[doc = "0xf20 - Interrupt 1 mask clear enable."]
    pub clr_en1: CLR_EN1,
    #[doc = "0xf24 - Interrupt 1 mask set enable"]
    pub set_en1: SET_EN1,
    #[doc = "0xf28 - Interrupt 1 mask"]
    pub mask1: MASK1,
    #[doc = "0xf2c - Interrupt 1 status. Interrupt 1 contains window comparator results and register last LAST_SAMPLE[M] overrun."]
    pub status1: STATUS1,
    #[doc = "0xf30 - Interrupt 1 clear status"]
    pub clr_stat1: CLR_STAT1,
    #[doc = "0xf34 - Interrupt 1 set status"]
    pub set_stat1: SET_STAT1,
}
#[doc = "Flushes FIFO"]
pub struct FLUSH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flushes FIFO"]
pub mod flush;
#[doc = "Set or clear DMA write request"]
pub struct DMA_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set or clear DMA write request"]
pub mod dma_req;
#[doc = "Indicates FIFO fill level status"]
pub struct FIFO_STS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indicates FIFO fill level status"]
pub mod fifo_sts;
#[doc = "Configures FIFO fill level that triggers interrupt and packing 1 or 2 samples per word."]
pub struct FIFO_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configures FIFO fill level that triggers interrupt and packing 1 or 2 samples per word."]
pub mod fifo_cfg;
#[doc = "Enable software trigger to start descriptor processing"]
pub struct TRIGGER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable software trigger to start descriptor processing"]
pub mod trigger;
#[doc = "Indicates active descriptor table and descriptor entry"]
pub struct DSCR_STS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indicates active descriptor table and descriptor entry"]
pub mod dscr_sts;
#[doc = "Set or clear power down mode"]
pub struct POWER_DOWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set or clear power down mode"]
pub mod power_down;
#[doc = "Configures external trigger mode, store channel ID in FIFO and walk-up recovery time from power down."]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configures external trigger mode, store channel ID in FIFO and walk-up recovery time from power down."]
pub mod config;
#[doc = "Configures window comparator A levels."]
pub struct THR_A {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configures window comparator A levels."]
pub mod thr_a;
#[doc = "Configures window comparator B levels."]
pub struct THR_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configures window comparator B levels."]
pub mod thr_b;
#[doc = "Contains last converted sample of input M [M=0..5) and result of window comparator."]
pub struct LAST_SAMPLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains last converted sample of input M [M=0..5) and result of window comparator."]
pub mod last_sample;
#[doc = "ADC speed control"]
pub struct ADC_SPEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC speed control"]
pub mod adc_speed;
#[doc = "Configures ADC power vs. speed, DC-in biasing, output format and power gating."]
pub struct POWER_CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configures ADC power vs. speed, DC-in biasing, output format and power gating."]
pub mod power_control;
#[doc = "FIFO output mapped to 16 consecutive address locations. An output contains the value and input channel ID of one or two converted samples"]
pub struct FIFO_OUTPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO output mapped to 16 consecutive address locations. An output contains the value and input channel ID of one or two converted samples"]
pub mod fifo_output;
#[doc = "Table 0 descriptor n, n= 0 to 7"]
pub struct DESCRIPTOR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Table 0 descriptor n, n= 0 to 7"]
pub mod descriptor0;
#[doc = "Table 1 descriptors n, n=0 to 7"]
pub struct DESCRIPTOR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Table 1 descriptors n, n=0 to 7"]
pub mod descriptor1;
#[doc = "Interrupt 0 clear mask"]
pub struct CLR_EN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt 0 clear mask"]
pub mod clr_en0;
#[doc = "Interrupt 0 set mask"]
pub struct SET_EN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt 0 set mask"]
pub mod set_en0;
#[doc = "Interrupt 0 mask"]
pub struct MASK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt 0 mask"]
pub mod mask0;
#[doc = "Interrupt 0 status. Interrupt 0 contains FIFO fill level, descriptor status and ADC range under/overflow"]
pub struct STATUS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt 0 status. Interrupt 0 contains FIFO fill level, descriptor status and ADC range under/overflow"]
pub mod status0;
#[doc = "Interrupt 0 clear status"]
pub struct CLR_STAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt 0 clear status"]
pub mod clr_stat0;
#[doc = "Interrupt 0 set status"]
pub struct SET_STAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt 0 set status"]
pub mod set_stat0;
#[doc = "Interrupt 1 mask clear enable."]
pub struct CLR_EN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt 1 mask clear enable."]
pub mod clr_en1;
#[doc = "Interrupt 1 mask set enable"]
pub struct SET_EN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt 1 mask set enable"]
pub mod set_en1;
#[doc = "Interrupt 1 mask"]
pub struct MASK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt 1 mask"]
pub mod mask1;
#[doc = "Interrupt 1 status. Interrupt 1 contains window comparator results and register last LAST_SAMPLE[M] overrun."]
pub struct STATUS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt 1 status. Interrupt 1 contains window comparator results and register last LAST_SAMPLE[M] overrun."]
pub mod status1;
#[doc = "Interrupt 1 clear status"]
pub struct CLR_STAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt 1 clear status"]
pub mod clr_stat1;
#[doc = "Interrupt 1 set status"]
pub struct SET_STAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt 1 set status"]
pub mod set_stat1;
