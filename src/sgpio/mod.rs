#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin multiplexer configuration registers."]
    pub out_mux_cfg: [OUT_MUX_CFG; 16],
    #[doc = "0x40 - SGPIO multiplexer configuration registers."]
    pub sgpio_mux_cfg: [SGPIO_MUX_CFG; 16],
    #[doc = "0x80 - Slice multiplexer configuration registers."]
    pub slice_mux_cfg: [SLICE_MUX_CFG; 16],
    #[doc = "0xc0 - Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading bit 31 with data captured from DIN(n). DOUT(n) is set to REG(0)"]
    pub reg: [REG; 16],
    #[doc = "0x100 - Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is exchanged with the content of REG"]
    pub reg_ss: [REG_SS; 16],
    #[doc = "0x140 - Reload value of COUNT0, loaded when COUNT0 reaches 0x0"]
    pub preset: [PRESET; 16],
    #[doc = "0x180 - Down counter, counts down each clock cycle."]
    pub count: [COUNT; 16],
    #[doc = "0x1c0 - Each time COUNT0 reaches 0x0 POS counts down."]
    pub pos: [POS; 16],
    #[doc = "0x200 - Mask for pattern match function of slice A"]
    pub mask_a: MASK_A,
    #[doc = "0x204 - Mask for pattern match function of slice H"]
    pub mask_h: MASK_H,
    #[doc = "0x208 - Mask for pattern match function of slice I"]
    pub mask_i: MASK_I,
    #[doc = "0x20c - Mask for pattern match function of slice P"]
    pub mask_p: MASK_P,
    #[doc = "0x210 - GPIO input status register"]
    pub gpio_inreg: GPIO_INREG,
    #[doc = "0x214 - GPIO output control register"]
    pub gpio_outreg: GPIO_OUTREG,
    #[doc = "0x218 - GPIO OE control register"]
    pub gpio_oenreg: GPIO_OENREG,
    #[doc = "0x21c - Enables the slice COUNT counter"]
    pub ctrl_enable: CTRL_ENABLE,
    #[doc = "0x220 - Disables the slice POS counter"]
    pub ctrl_disable: CTRL_DISABLE,
    _reserved0: [u8; 3292usize],
    #[doc = "0xf00 - Shift clock interrupt clear mask"]
    pub clr_en_0: CLR_EN_0,
    #[doc = "0xf04 - Shift clock interrupt set mask"]
    pub set_en_0: SET_EN_0,
    #[doc = "0xf08 - Shift clock interrupt enable"]
    pub enable_0: ENABLE_0,
    #[doc = "0xf0c - Shift clock interrupt status"]
    pub status_0: STATUS_0,
    #[doc = "0xf10 - Shift clock interrupt clear status"]
    pub clr_status_0: CLR_STATUS_0,
    #[doc = "0xf14 - Shift clock interrupt set status"]
    pub set_status_0: SET_STATUS_0,
    _reserved1: [u8; 8usize],
    #[doc = "0xf20 - Exchange clock interrupt clear mask"]
    pub clr_en_1: CLR_EN_1,
    #[doc = "0xf24 - Exchange clock interrupt set mask"]
    pub set_en_1: SET_EN_1,
    #[doc = "0xf28 - Exchange clock interrupt enable"]
    pub enable_1: ENABLE_1,
    #[doc = "0xf2c - Exchange clock interrupt status"]
    pub status_1: STATUS_1,
    #[doc = "0xf30 - Exchange clock interrupt clear status"]
    pub clr_status_1: CLR_STATUS_1,
    #[doc = "0xf34 - Exchange clock interrupt set status"]
    pub set_status_1: SET_STATUS_1,
    _reserved2: [u8; 8usize],
    #[doc = "0xf40 - Pattern match interrupt clear mask"]
    pub clr_en_2: CLR_EN_2,
    #[doc = "0xf44 - Pattern match interrupt set mask"]
    pub set_en_2: SET_EN_2,
    #[doc = "0xf48 - Pattern match interrupt enable"]
    pub enable_2: ENABLE_2,
    #[doc = "0xf4c - Pattern match interrupt status"]
    pub status_2: STATUS_2,
    #[doc = "0xf50 - Pattern match interrupt clear status"]
    pub clr_status_2: CLR_STATUS_2,
    #[doc = "0xf54 - Pattern match interrupt set status"]
    pub set_status_2: SET_STATUS_2,
    _reserved3: [u8; 8usize],
    #[doc = "0xf60 - Input interrupt clear mask"]
    pub clr_en_3: CLR_EN_3,
    #[doc = "0xf64 - Input bit match interrupt set mask"]
    pub set_en_3: SET_EN_3,
    #[doc = "0xf68 - Input bit match interrupt enable"]
    pub enable_3: ENABLE_3,
    #[doc = "0xf6c - Input bit match interrupt status"]
    pub status_3: STATUS_3,
    #[doc = "0xf70 - Input bit match interrupt clear status"]
    pub clr_status_3: CLR_STATUS_3,
    #[doc = "0xf74 - Input bit match interrupt set status"]
    pub set_status_3: SET_STATUS_3,
}
#[doc = "Pin multiplexer configuration registers."]
pub struct OUT_MUX_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin multiplexer configuration registers."]
pub mod out_mux_cfg;
#[doc = "SGPIO multiplexer configuration registers."]
pub struct SGPIO_MUX_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SGPIO multiplexer configuration registers."]
pub mod sgpio_mux_cfg;
#[doc = "Slice multiplexer configuration registers."]
pub struct SLICE_MUX_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slice multiplexer configuration registers."]
pub mod slice_mux_cfg;
#[doc = "Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading bit 31 with data captured from DIN(n). DOUT(n) is set to REG(0)"]
pub struct REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading bit 31 with data captured from DIN(n). DOUT(n) is set to REG(0)"]
pub mod reg;
#[doc = "Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is exchanged with the content of REG"]
pub struct REG_SS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is exchanged with the content of REG"]
pub mod reg_ss;
#[doc = "Reload value of COUNT0, loaded when COUNT0 reaches 0x0"]
pub struct PRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reload value of COUNT0, loaded when COUNT0 reaches 0x0"]
pub mod preset;
#[doc = "Down counter, counts down each clock cycle."]
pub struct COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Down counter, counts down each clock cycle."]
pub mod count;
#[doc = "Each time COUNT0 reaches 0x0 POS counts down."]
pub struct POS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Each time COUNT0 reaches 0x0 POS counts down."]
pub mod pos;
#[doc = "Mask for pattern match function of slice A"]
pub struct MASK_A {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask for pattern match function of slice A"]
pub mod mask_a;
#[doc = "Mask for pattern match function of slice H"]
pub struct MASK_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask for pattern match function of slice H"]
pub mod mask_h;
#[doc = "Mask for pattern match function of slice I"]
pub struct MASK_I {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask for pattern match function of slice I"]
pub mod mask_i;
#[doc = "Mask for pattern match function of slice P"]
pub struct MASK_P {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask for pattern match function of slice P"]
pub mod mask_p;
#[doc = "GPIO input status register"]
pub struct GPIO_INREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO input status register"]
pub mod gpio_inreg;
#[doc = "GPIO output control register"]
pub struct GPIO_OUTREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO output control register"]
pub mod gpio_outreg;
#[doc = "GPIO OE control register"]
pub struct GPIO_OENREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO OE control register"]
pub mod gpio_oenreg;
#[doc = "Enables the slice COUNT counter"]
pub struct CTRL_ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enables the slice COUNT counter"]
pub mod ctrl_enable;
#[doc = "Disables the slice POS counter"]
pub struct CTRL_DISABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disables the slice POS counter"]
pub mod ctrl_disable;
#[doc = "Shift clock interrupt clear mask"]
pub struct CLR_EN_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shift clock interrupt clear mask"]
pub mod clr_en_0;
#[doc = "Shift clock interrupt set mask"]
pub struct SET_EN_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shift clock interrupt set mask"]
pub mod set_en_0;
#[doc = "Shift clock interrupt enable"]
pub struct ENABLE_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shift clock interrupt enable"]
pub mod enable_0;
#[doc = "Shift clock interrupt status"]
pub struct STATUS_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shift clock interrupt status"]
pub mod status_0;
#[doc = "Shift clock interrupt clear status"]
pub struct CLR_STATUS_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shift clock interrupt clear status"]
pub mod clr_status_0;
#[doc = "Shift clock interrupt set status"]
pub struct SET_STATUS_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shift clock interrupt set status"]
pub mod set_status_0;
#[doc = "Exchange clock interrupt clear mask"]
pub struct CLR_EN_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Exchange clock interrupt clear mask"]
pub mod clr_en_1;
#[doc = "Exchange clock interrupt set mask"]
pub struct SET_EN_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Exchange clock interrupt set mask"]
pub mod set_en_1;
#[doc = "Exchange clock interrupt enable"]
pub struct ENABLE_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Exchange clock interrupt enable"]
pub mod enable_1;
#[doc = "Exchange clock interrupt status"]
pub struct STATUS_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Exchange clock interrupt status"]
pub mod status_1;
#[doc = "Exchange clock interrupt clear status"]
pub struct CLR_STATUS_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Exchange clock interrupt clear status"]
pub mod clr_status_1;
#[doc = "Exchange clock interrupt set status"]
pub struct SET_STATUS_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Exchange clock interrupt set status"]
pub mod set_status_1;
#[doc = "Pattern match interrupt clear mask"]
pub struct CLR_EN_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pattern match interrupt clear mask"]
pub mod clr_en_2;
#[doc = "Pattern match interrupt set mask"]
pub struct SET_EN_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pattern match interrupt set mask"]
pub mod set_en_2;
#[doc = "Pattern match interrupt enable"]
pub struct ENABLE_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pattern match interrupt enable"]
pub mod enable_2;
#[doc = "Pattern match interrupt status"]
pub struct STATUS_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pattern match interrupt status"]
pub mod status_2;
#[doc = "Pattern match interrupt clear status"]
pub struct CLR_STATUS_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pattern match interrupt clear status"]
pub mod clr_status_2;
#[doc = "Pattern match interrupt set status"]
pub struct SET_STATUS_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pattern match interrupt set status"]
pub mod set_status_2;
#[doc = "Input interrupt clear mask"]
pub struct CLR_EN_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input interrupt clear mask"]
pub mod clr_en_3;
#[doc = "Input bit match interrupt set mask"]
pub struct SET_EN_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input bit match interrupt set mask"]
pub mod set_en_3;
#[doc = "Input bit match interrupt enable"]
pub struct ENABLE_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input bit match interrupt enable"]
pub mod enable_3;
#[doc = "Input bit match interrupt status"]
pub struct STATUS_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input bit match interrupt status"]
pub mod status_3;
#[doc = "Input bit match interrupt clear status"]
pub struct CLR_STATUS_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input bit match interrupt clear status"]
pub mod clr_status_3;
#[doc = "Input bit match interrupt set status"]
pub struct SET_STATUS_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input bit match interrupt set status"]
pub mod set_status_3;
