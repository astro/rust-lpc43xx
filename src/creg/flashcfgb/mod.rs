#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASHCFGB {
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
#[doc = "Possible values of the field `FLASHTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHTIMR {
    #[doc = "1 BASE_M4_CLK clock. Use for BASE_M4_CLK up to 21 MHz."]
    _1_BASE_M4_CLK_CLOCK,
    #[doc = "2 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 43 MHz."]
    _2_BASE_M4_CLK_CLOCKS,
    #[doc = "3 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 64 MHz."]
    _3_BASE_M4_CLK_CLOCKS,
    #[doc = "4 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 86 MHz."]
    _4_BASE_M4_CLK_CLOCKS,
    #[doc = "5 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 107 MHz."]
    _5_BASE_M4_CLK_CLOCKS,
    #[doc = "6 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 129 MHz."]
    _6_BASE_M4_CLK_CLOCKS,
    #[doc = "7 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 150 MHz."]
    _7_BASE_M4_CLK_CLOCKS,
    #[doc = "8 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 172 MHz."]
    _8_BASE_M4_CLK_CLOCKS,
    #[doc = "9 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 193 MHz."]
    _9_BASE_M4_CLK_CLOCKS,
    #[doc = "10 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 204 MHz. Safe setting for all allowed conditions."]
    _10_BASE_M4_CLK_CLOCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLASHTIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASHTIMR::_1_BASE_M4_CLK_CLOCK => 0,
            FLASHTIMR::_2_BASE_M4_CLK_CLOCKS => 1,
            FLASHTIMR::_3_BASE_M4_CLK_CLOCKS => 2,
            FLASHTIMR::_4_BASE_M4_CLK_CLOCKS => 3,
            FLASHTIMR::_5_BASE_M4_CLK_CLOCKS => 4,
            FLASHTIMR::_6_BASE_M4_CLK_CLOCKS => 5,
            FLASHTIMR::_7_BASE_M4_CLK_CLOCKS => 6,
            FLASHTIMR::_8_BASE_M4_CLK_CLOCKS => 7,
            FLASHTIMR::_9_BASE_M4_CLK_CLOCKS => 8,
            FLASHTIMR::_10_BASE_M4_CLK_CLOCK => 9,
            FLASHTIMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLASHTIMR {
        match value {
            0 => FLASHTIMR::_1_BASE_M4_CLK_CLOCK,
            1 => FLASHTIMR::_2_BASE_M4_CLK_CLOCKS,
            2 => FLASHTIMR::_3_BASE_M4_CLK_CLOCKS,
            3 => FLASHTIMR::_4_BASE_M4_CLK_CLOCKS,
            4 => FLASHTIMR::_5_BASE_M4_CLK_CLOCKS,
            5 => FLASHTIMR::_6_BASE_M4_CLK_CLOCKS,
            6 => FLASHTIMR::_7_BASE_M4_CLK_CLOCKS,
            7 => FLASHTIMR::_8_BASE_M4_CLK_CLOCKS,
            8 => FLASHTIMR::_9_BASE_M4_CLK_CLOCKS,
            9 => FLASHTIMR::_10_BASE_M4_CLK_CLOCK,
            i => FLASHTIMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_BASE_M4_CLK_CLOCK`"]
    #[inline]
    pub fn is_1_base_m4_clk_clock(&self) -> bool {
        *self == FLASHTIMR::_1_BASE_M4_CLK_CLOCK
    }
    #[doc = "Checks if the value of the field is `_2_BASE_M4_CLK_CLOCKS`"]
    #[inline]
    pub fn is_2_base_m4_clk_clocks(&self) -> bool {
        *self == FLASHTIMR::_2_BASE_M4_CLK_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_3_BASE_M4_CLK_CLOCKS`"]
    #[inline]
    pub fn is_3_base_m4_clk_clocks(&self) -> bool {
        *self == FLASHTIMR::_3_BASE_M4_CLK_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_4_BASE_M4_CLK_CLOCKS`"]
    #[inline]
    pub fn is_4_base_m4_clk_clocks(&self) -> bool {
        *self == FLASHTIMR::_4_BASE_M4_CLK_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_5_BASE_M4_CLK_CLOCKS`"]
    #[inline]
    pub fn is_5_base_m4_clk_clocks(&self) -> bool {
        *self == FLASHTIMR::_5_BASE_M4_CLK_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_6_BASE_M4_CLK_CLOCKS`"]
    #[inline]
    pub fn is_6_base_m4_clk_clocks(&self) -> bool {
        *self == FLASHTIMR::_6_BASE_M4_CLK_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_7_BASE_M4_CLK_CLOCKS`"]
    #[inline]
    pub fn is_7_base_m4_clk_clocks(&self) -> bool {
        *self == FLASHTIMR::_7_BASE_M4_CLK_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_8_BASE_M4_CLK_CLOCKS`"]
    #[inline]
    pub fn is_8_base_m4_clk_clocks(&self) -> bool {
        *self == FLASHTIMR::_8_BASE_M4_CLK_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_9_BASE_M4_CLK_CLOCKS`"]
    #[inline]
    pub fn is_9_base_m4_clk_clocks(&self) -> bool {
        *self == FLASHTIMR::_9_BASE_M4_CLK_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_10_BASE_M4_CLK_CLOCK`"]
    #[inline]
    pub fn is_10_base_m4_clk_clock(&self) -> bool {
        *self == FLASHTIMR::_10_BASE_M4_CLK_CLOCK
    }
}
#[doc = "Possible values of the field `POW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POWR {
    #[doc = "Power-down"]
    POWER_DOWN,
    #[doc = "Active (Default)"]
    ACTIVE,
}
impl POWR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            POWR::POWER_DOWN => false,
            POWR::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POWR {
        match value {
            false => POWR::POWER_DOWN,
            true => POWR::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline]
    pub fn is_power_down(&self) -> bool {
        *self == POWR::POWER_DOWN
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == POWR::ACTIVE
    }
}
#[doc = "Values that can be written to the field `FLASHTIM`"]
pub enum FLASHTIMW {
    #[doc = "1 BASE_M4_CLK clock. Use for BASE_M4_CLK up to 21 MHz."]
    _1_BASE_M4_CLK_CLOCK,
    #[doc = "2 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 43 MHz."]
    _2_BASE_M4_CLK_CLOCKS,
    #[doc = "3 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 64 MHz."]
    _3_BASE_M4_CLK_CLOCKS,
    #[doc = "4 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 86 MHz."]
    _4_BASE_M4_CLK_CLOCKS,
    #[doc = "5 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 107 MHz."]
    _5_BASE_M4_CLK_CLOCKS,
    #[doc = "6 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 129 MHz."]
    _6_BASE_M4_CLK_CLOCKS,
    #[doc = "7 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 150 MHz."]
    _7_BASE_M4_CLK_CLOCKS,
    #[doc = "8 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 172 MHz."]
    _8_BASE_M4_CLK_CLOCKS,
    #[doc = "9 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 193 MHz."]
    _9_BASE_M4_CLK_CLOCKS,
    #[doc = "10 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 204 MHz. Safe setting for all allowed conditions."]
    _10_BASE_M4_CLK_CLOCK,
}
impl FLASHTIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASHTIMW::_1_BASE_M4_CLK_CLOCK => 0,
            FLASHTIMW::_2_BASE_M4_CLK_CLOCKS => 1,
            FLASHTIMW::_3_BASE_M4_CLK_CLOCKS => 2,
            FLASHTIMW::_4_BASE_M4_CLK_CLOCKS => 3,
            FLASHTIMW::_5_BASE_M4_CLK_CLOCKS => 4,
            FLASHTIMW::_6_BASE_M4_CLK_CLOCKS => 5,
            FLASHTIMW::_7_BASE_M4_CLK_CLOCKS => 6,
            FLASHTIMW::_8_BASE_M4_CLK_CLOCKS => 7,
            FLASHTIMW::_9_BASE_M4_CLK_CLOCKS => 8,
            FLASHTIMW::_10_BASE_M4_CLK_CLOCK => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHTIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHTIMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 BASE_M4_CLK clock. Use for BASE_M4_CLK up to 21 MHz."]
    #[inline]
    pub fn _1_base_m4_clk_clock(self) -> &'a mut W {
        self.variant(FLASHTIMW::_1_BASE_M4_CLK_CLOCK)
    }
    #[doc = "2 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 43 MHz."]
    #[inline]
    pub fn _2_base_m4_clk_clocks(self) -> &'a mut W {
        self.variant(FLASHTIMW::_2_BASE_M4_CLK_CLOCKS)
    }
    #[doc = "3 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 64 MHz."]
    #[inline]
    pub fn _3_base_m4_clk_clocks(self) -> &'a mut W {
        self.variant(FLASHTIMW::_3_BASE_M4_CLK_CLOCKS)
    }
    #[doc = "4 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 86 MHz."]
    #[inline]
    pub fn _4_base_m4_clk_clocks(self) -> &'a mut W {
        self.variant(FLASHTIMW::_4_BASE_M4_CLK_CLOCKS)
    }
    #[doc = "5 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 107 MHz."]
    #[inline]
    pub fn _5_base_m4_clk_clocks(self) -> &'a mut W {
        self.variant(FLASHTIMW::_5_BASE_M4_CLK_CLOCKS)
    }
    #[doc = "6 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 129 MHz."]
    #[inline]
    pub fn _6_base_m4_clk_clocks(self) -> &'a mut W {
        self.variant(FLASHTIMW::_6_BASE_M4_CLK_CLOCKS)
    }
    #[doc = "7 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 150 MHz."]
    #[inline]
    pub fn _7_base_m4_clk_clocks(self) -> &'a mut W {
        self.variant(FLASHTIMW::_7_BASE_M4_CLK_CLOCKS)
    }
    #[doc = "8 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 172 MHz."]
    #[inline]
    pub fn _8_base_m4_clk_clocks(self) -> &'a mut W {
        self.variant(FLASHTIMW::_8_BASE_M4_CLK_CLOCKS)
    }
    #[doc = "9 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 193 MHz."]
    #[inline]
    pub fn _9_base_m4_clk_clocks(self) -> &'a mut W {
        self.variant(FLASHTIMW::_9_BASE_M4_CLK_CLOCKS)
    }
    #[doc = "10 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 204 MHz. Safe setting for all allowed conditions."]
    #[inline]
    pub fn _10_base_m4_clk_clock(self) -> &'a mut W {
        self.variant(FLASHTIMW::_10_BASE_M4_CLK_CLOCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POW`"]
pub enum POWW {
    #[doc = "Power-down"]
    POWER_DOWN,
    #[doc = "Active (Default)"]
    ACTIVE,
}
impl POWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POWW::POWER_DOWN => false,
            POWW::ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POWW<'a> {
    w: &'a mut W,
}
impl<'a> _POWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power-down"]
    #[inline]
    pub fn power_down(self) -> &'a mut W {
        self.variant(POWW::POWER_DOWN)
    }
    #[doc = "Active (Default)"]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(POWW::ACTIVE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of BASE_M4_CLK clocks used for a flash access. Warning: Improper setting of this value may result in incorrect operation of the device. All other values are allowed but may not be optimal for the supported clock frequencies."]
    #[inline]
    pub fn flashtim(&self) -> FLASHTIMR {
        FLASHTIMR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Flash bank A power control"]
    #[inline]
    pub fn pow(&self) -> POWR {
        POWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147545146 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of BASE_M4_CLK clocks used for a flash access. Warning: Improper setting of this value may result in incorrect operation of the device. All other values are allowed but may not be optimal for the supported clock frequencies."]
    #[inline]
    pub fn flashtim(&mut self) -> _FLASHTIMW {
        _FLASHTIMW { w: self }
    }
    #[doc = "Bit 31 - Flash bank A power control"]
    #[inline]
    pub fn pow(&mut self) -> _POWW {
        _POWW { w: self }
    }
}
