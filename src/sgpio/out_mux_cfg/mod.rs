#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUT_MUX_CFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `P_OUT_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P_OUT_CFGR {
    #[doc = "dout_doutm1 (1-bit mode)"]
    DOUT_DOUTM1,
    #[doc = "dout_doutm2a (2-bit mode 2a)"]
    DOUT_DOUTM2A,
    #[doc = "dout_doutm2b (2-bit mode 2b)"]
    DOUT_DOUTM2B,
    #[doc = "dout_doutm2c (2-bit mode 2c)"]
    DOUT_DOUTM2C,
    #[doc = "gpio_out (level set by GPIO_OUTREG)"]
    GPIO_OUT_LEVEL_SET,
    #[doc = "dout_doutm4a (4-bit mode 4a)"]
    DOUT_DOUTM4A,
    #[doc = "dout_doutm4b (4-bit mode 4b)"]
    DOUT_DOUTM4B,
    #[doc = "dout_doutm4c (4-bit mode 4c)"]
    DOUT_DOUTM4C,
    #[doc = "clk_out"]
    CLK_OUT,
    #[doc = "dout_doutm8a (8-bit mode 8a)"]
    DOUT_DOUTM8A,
    #[doc = "dout_doutm8b (8-bit mode 8b)"]
    DOUT_DOUTM8B,
    #[doc = "dout_doutm8c (8-bit mode 8c)"]
    DOUT_DOUTM8C,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl P_OUT_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P_OUT_CFGR::DOUT_DOUTM1 => 0,
            P_OUT_CFGR::DOUT_DOUTM2A => 1,
            P_OUT_CFGR::DOUT_DOUTM2B => 2,
            P_OUT_CFGR::DOUT_DOUTM2C => 3,
            P_OUT_CFGR::GPIO_OUT_LEVEL_SET => 4,
            P_OUT_CFGR::DOUT_DOUTM4A => 5,
            P_OUT_CFGR::DOUT_DOUTM4B => 6,
            P_OUT_CFGR::DOUT_DOUTM4C => 7,
            P_OUT_CFGR::CLK_OUT => 8,
            P_OUT_CFGR::DOUT_DOUTM8A => 9,
            P_OUT_CFGR::DOUT_DOUTM8B => 10,
            P_OUT_CFGR::DOUT_DOUTM8C => 11,
            P_OUT_CFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P_OUT_CFGR {
        match value {
            0 => P_OUT_CFGR::DOUT_DOUTM1,
            1 => P_OUT_CFGR::DOUT_DOUTM2A,
            2 => P_OUT_CFGR::DOUT_DOUTM2B,
            3 => P_OUT_CFGR::DOUT_DOUTM2C,
            4 => P_OUT_CFGR::GPIO_OUT_LEVEL_SET,
            5 => P_OUT_CFGR::DOUT_DOUTM4A,
            6 => P_OUT_CFGR::DOUT_DOUTM4B,
            7 => P_OUT_CFGR::DOUT_DOUTM4C,
            8 => P_OUT_CFGR::CLK_OUT,
            9 => P_OUT_CFGR::DOUT_DOUTM8A,
            10 => P_OUT_CFGR::DOUT_DOUTM8B,
            11 => P_OUT_CFGR::DOUT_DOUTM8C,
            i => P_OUT_CFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DOUT_DOUTM1`"]
    #[inline]
    pub fn is_dout_doutm1(&self) -> bool {
        *self == P_OUT_CFGR::DOUT_DOUTM1
    }
    #[doc = "Checks if the value of the field is `DOUT_DOUTM2A`"]
    #[inline]
    pub fn is_dout_doutm2a(&self) -> bool {
        *self == P_OUT_CFGR::DOUT_DOUTM2A
    }
    #[doc = "Checks if the value of the field is `DOUT_DOUTM2B`"]
    #[inline]
    pub fn is_dout_doutm2b(&self) -> bool {
        *self == P_OUT_CFGR::DOUT_DOUTM2B
    }
    #[doc = "Checks if the value of the field is `DOUT_DOUTM2C`"]
    #[inline]
    pub fn is_dout_doutm2c(&self) -> bool {
        *self == P_OUT_CFGR::DOUT_DOUTM2C
    }
    #[doc = "Checks if the value of the field is `GPIO_OUT_LEVEL_SET`"]
    #[inline]
    pub fn is_gpio_out_level_set(&self) -> bool {
        *self == P_OUT_CFGR::GPIO_OUT_LEVEL_SET
    }
    #[doc = "Checks if the value of the field is `DOUT_DOUTM4A`"]
    #[inline]
    pub fn is_dout_doutm4a(&self) -> bool {
        *self == P_OUT_CFGR::DOUT_DOUTM4A
    }
    #[doc = "Checks if the value of the field is `DOUT_DOUTM4B`"]
    #[inline]
    pub fn is_dout_doutm4b(&self) -> bool {
        *self == P_OUT_CFGR::DOUT_DOUTM4B
    }
    #[doc = "Checks if the value of the field is `DOUT_DOUTM4C`"]
    #[inline]
    pub fn is_dout_doutm4c(&self) -> bool {
        *self == P_OUT_CFGR::DOUT_DOUTM4C
    }
    #[doc = "Checks if the value of the field is `CLK_OUT`"]
    #[inline]
    pub fn is_clk_out(&self) -> bool {
        *self == P_OUT_CFGR::CLK_OUT
    }
    #[doc = "Checks if the value of the field is `DOUT_DOUTM8A`"]
    #[inline]
    pub fn is_dout_doutm8a(&self) -> bool {
        *self == P_OUT_CFGR::DOUT_DOUTM8A
    }
    #[doc = "Checks if the value of the field is `DOUT_DOUTM8B`"]
    #[inline]
    pub fn is_dout_doutm8b(&self) -> bool {
        *self == P_OUT_CFGR::DOUT_DOUTM8B
    }
    #[doc = "Checks if the value of the field is `DOUT_DOUTM8C`"]
    #[inline]
    pub fn is_dout_doutm8c(&self) -> bool {
        *self == P_OUT_CFGR::DOUT_DOUTM8C
    }
}
#[doc = "Possible values of the field `P_OE_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P_OE_CFGR {
    #[doc = "gpio_oe (state set by GPIO_OEREG)"]
    GPIO_OE_STATE_SET_B,
    #[doc = "dout_oem1 (1-bit mode)"]
    DOUT_OEM1_1_BIT_MOD,
    #[doc = "dout_oem2 (2-bit mode)"]
    DOUT_OEM2_2_BIT_MOD,
    #[doc = "dout_oem4 (4-bit mode)"]
    DOUT_OEM4_4_BIT_MOD,
    #[doc = "dout_oem8 (8-bit mode)"]
    DOUT_OEM8_8_BIT_MOD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl P_OE_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P_OE_CFGR::GPIO_OE_STATE_SET_B => 0,
            P_OE_CFGR::DOUT_OEM1_1_BIT_MOD => 4,
            P_OE_CFGR::DOUT_OEM2_2_BIT_MOD => 5,
            P_OE_CFGR::DOUT_OEM4_4_BIT_MOD => 6,
            P_OE_CFGR::DOUT_OEM8_8_BIT_MOD => 7,
            P_OE_CFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P_OE_CFGR {
        match value {
            0 => P_OE_CFGR::GPIO_OE_STATE_SET_B,
            4 => P_OE_CFGR::DOUT_OEM1_1_BIT_MOD,
            5 => P_OE_CFGR::DOUT_OEM2_2_BIT_MOD,
            6 => P_OE_CFGR::DOUT_OEM4_4_BIT_MOD,
            7 => P_OE_CFGR::DOUT_OEM8_8_BIT_MOD,
            i => P_OE_CFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_OE_STATE_SET_B`"]
    #[inline]
    pub fn is_gpio_oe_state_set_b(&self) -> bool {
        *self == P_OE_CFGR::GPIO_OE_STATE_SET_B
    }
    #[doc = "Checks if the value of the field is `DOUT_OEM1_1_BIT_MOD`"]
    #[inline]
    pub fn is_dout_oem1_1_bit_mod(&self) -> bool {
        *self == P_OE_CFGR::DOUT_OEM1_1_BIT_MOD
    }
    #[doc = "Checks if the value of the field is `DOUT_OEM2_2_BIT_MOD`"]
    #[inline]
    pub fn is_dout_oem2_2_bit_mod(&self) -> bool {
        *self == P_OE_CFGR::DOUT_OEM2_2_BIT_MOD
    }
    #[doc = "Checks if the value of the field is `DOUT_OEM4_4_BIT_MOD`"]
    #[inline]
    pub fn is_dout_oem4_4_bit_mod(&self) -> bool {
        *self == P_OE_CFGR::DOUT_OEM4_4_BIT_MOD
    }
    #[doc = "Checks if the value of the field is `DOUT_OEM8_8_BIT_MOD`"]
    #[inline]
    pub fn is_dout_oem8_8_bit_mod(&self) -> bool {
        *self == P_OE_CFGR::DOUT_OEM8_8_BIT_MOD
    }
}
#[doc = "Values that can be written to the field `P_OUT_CFG`"]
pub enum P_OUT_CFGW {
    #[doc = "dout_doutm1 (1-bit mode)"]
    DOUT_DOUTM1,
    #[doc = "dout_doutm2a (2-bit mode 2a)"]
    DOUT_DOUTM2A,
    #[doc = "dout_doutm2b (2-bit mode 2b)"]
    DOUT_DOUTM2B,
    #[doc = "dout_doutm2c (2-bit mode 2c)"]
    DOUT_DOUTM2C,
    #[doc = "gpio_out (level set by GPIO_OUTREG)"]
    GPIO_OUT_LEVEL_SET,
    #[doc = "dout_doutm4a (4-bit mode 4a)"]
    DOUT_DOUTM4A,
    #[doc = "dout_doutm4b (4-bit mode 4b)"]
    DOUT_DOUTM4B,
    #[doc = "dout_doutm4c (4-bit mode 4c)"]
    DOUT_DOUTM4C,
    #[doc = "clk_out"]
    CLK_OUT,
    #[doc = "dout_doutm8a (8-bit mode 8a)"]
    DOUT_DOUTM8A,
    #[doc = "dout_doutm8b (8-bit mode 8b)"]
    DOUT_DOUTM8B,
    #[doc = "dout_doutm8c (8-bit mode 8c)"]
    DOUT_DOUTM8C,
}
impl P_OUT_CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P_OUT_CFGW::DOUT_DOUTM1 => 0,
            P_OUT_CFGW::DOUT_DOUTM2A => 1,
            P_OUT_CFGW::DOUT_DOUTM2B => 2,
            P_OUT_CFGW::DOUT_DOUTM2C => 3,
            P_OUT_CFGW::GPIO_OUT_LEVEL_SET => 4,
            P_OUT_CFGW::DOUT_DOUTM4A => 5,
            P_OUT_CFGW::DOUT_DOUTM4B => 6,
            P_OUT_CFGW::DOUT_DOUTM4C => 7,
            P_OUT_CFGW::CLK_OUT => 8,
            P_OUT_CFGW::DOUT_DOUTM8A => 9,
            P_OUT_CFGW::DOUT_DOUTM8B => 10,
            P_OUT_CFGW::DOUT_DOUTM8C => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P_OUT_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _P_OUT_CFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P_OUT_CFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "dout_doutm1 (1-bit mode)"]
    #[inline]
    pub fn dout_doutm1(self) -> &'a mut W {
        self.variant(P_OUT_CFGW::DOUT_DOUTM1)
    }
    #[doc = "dout_doutm2a (2-bit mode 2a)"]
    #[inline]
    pub fn dout_doutm2a(self) -> &'a mut W {
        self.variant(P_OUT_CFGW::DOUT_DOUTM2A)
    }
    #[doc = "dout_doutm2b (2-bit mode 2b)"]
    #[inline]
    pub fn dout_doutm2b(self) -> &'a mut W {
        self.variant(P_OUT_CFGW::DOUT_DOUTM2B)
    }
    #[doc = "dout_doutm2c (2-bit mode 2c)"]
    #[inline]
    pub fn dout_doutm2c(self) -> &'a mut W {
        self.variant(P_OUT_CFGW::DOUT_DOUTM2C)
    }
    #[doc = "gpio_out (level set by GPIO_OUTREG)"]
    #[inline]
    pub fn gpio_out_level_set(self) -> &'a mut W {
        self.variant(P_OUT_CFGW::GPIO_OUT_LEVEL_SET)
    }
    #[doc = "dout_doutm4a (4-bit mode 4a)"]
    #[inline]
    pub fn dout_doutm4a(self) -> &'a mut W {
        self.variant(P_OUT_CFGW::DOUT_DOUTM4A)
    }
    #[doc = "dout_doutm4b (4-bit mode 4b)"]
    #[inline]
    pub fn dout_doutm4b(self) -> &'a mut W {
        self.variant(P_OUT_CFGW::DOUT_DOUTM4B)
    }
    #[doc = "dout_doutm4c (4-bit mode 4c)"]
    #[inline]
    pub fn dout_doutm4c(self) -> &'a mut W {
        self.variant(P_OUT_CFGW::DOUT_DOUTM4C)
    }
    #[doc = "clk_out"]
    #[inline]
    pub fn clk_out(self) -> &'a mut W {
        self.variant(P_OUT_CFGW::CLK_OUT)
    }
    #[doc = "dout_doutm8a (8-bit mode 8a)"]
    #[inline]
    pub fn dout_doutm8a(self) -> &'a mut W {
        self.variant(P_OUT_CFGW::DOUT_DOUTM8A)
    }
    #[doc = "dout_doutm8b (8-bit mode 8b)"]
    #[inline]
    pub fn dout_doutm8b(self) -> &'a mut W {
        self.variant(P_OUT_CFGW::DOUT_DOUTM8B)
    }
    #[doc = "dout_doutm8c (8-bit mode 8c)"]
    #[inline]
    pub fn dout_doutm8c(self) -> &'a mut W {
        self.variant(P_OUT_CFGW::DOUT_DOUTM8C)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P_OE_CFG`"]
pub enum P_OE_CFGW {
    #[doc = "gpio_oe (state set by GPIO_OEREG)"]
    GPIO_OE_STATE_SET_B,
    #[doc = "dout_oem1 (1-bit mode)"]
    DOUT_OEM1_1_BIT_MOD,
    #[doc = "dout_oem2 (2-bit mode)"]
    DOUT_OEM2_2_BIT_MOD,
    #[doc = "dout_oem4 (4-bit mode)"]
    DOUT_OEM4_4_BIT_MOD,
    #[doc = "dout_oem8 (8-bit mode)"]
    DOUT_OEM8_8_BIT_MOD,
}
impl P_OE_CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P_OE_CFGW::GPIO_OE_STATE_SET_B => 0,
            P_OE_CFGW::DOUT_OEM1_1_BIT_MOD => 4,
            P_OE_CFGW::DOUT_OEM2_2_BIT_MOD => 5,
            P_OE_CFGW::DOUT_OEM4_4_BIT_MOD => 6,
            P_OE_CFGW::DOUT_OEM8_8_BIT_MOD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P_OE_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _P_OE_CFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P_OE_CFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "gpio_oe (state set by GPIO_OEREG)"]
    #[inline]
    pub fn gpio_oe_state_set_b(self) -> &'a mut W {
        self.variant(P_OE_CFGW::GPIO_OE_STATE_SET_B)
    }
    #[doc = "dout_oem1 (1-bit mode)"]
    #[inline]
    pub fn dout_oem1_1_bit_mod(self) -> &'a mut W {
        self.variant(P_OE_CFGW::DOUT_OEM1_1_BIT_MOD)
    }
    #[doc = "dout_oem2 (2-bit mode)"]
    #[inline]
    pub fn dout_oem2_2_bit_mod(self) -> &'a mut W {
        self.variant(P_OE_CFGW::DOUT_OEM2_2_BIT_MOD)
    }
    #[doc = "dout_oem4 (4-bit mode)"]
    #[inline]
    pub fn dout_oem4_4_bit_mod(self) -> &'a mut W {
        self.variant(P_OE_CFGW::DOUT_OEM4_4_BIT_MOD)
    }
    #[doc = "dout_oem8 (8-bit mode)"]
    #[inline]
    pub fn dout_oem8_8_bit_mod(self) -> &'a mut W {
        self.variant(P_OE_CFGW::DOUT_OEM8_8_BIT_MOD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Output control of output SGPIOn. All other values are reserved."]
    #[inline]
    pub fn p_out_cfg(&self) -> P_OUT_CFGR {
        P_OUT_CFGR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Output enable source. All other values are reserved."]
    #[inline]
    pub fn p_oe_cfg(&self) -> P_OE_CFGR {
        P_OE_CFGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Output control of output SGPIOn. All other values are reserved."]
    #[inline]
    pub fn p_out_cfg(&mut self) -> _P_OUT_CFGW {
        _P_OUT_CFGW { w: self }
    }
    #[doc = "Bits 4:6 - Output enable source. All other values are reserved."]
    #[inline]
    pub fn p_oe_cfg(&mut self) -> _P_OE_CFGW {
        _P_OE_CFGW { w: self }
    }
}
