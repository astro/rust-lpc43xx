#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLL1_CTRL {
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
#[doc = "Possible values of the field `PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDR {
    #[doc = "PLL1 enabled"]
    PLL1_ENABLED,
    #[doc = "PLL1 powered down"]
    PLL1_POWERED_DOWN,
}
impl PDR {
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
            PDR::PLL1_ENABLED => false,
            PDR::PLL1_POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDR {
        match value {
            false => PDR::PLL1_ENABLED,
            true => PDR::PLL1_POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `PLL1_ENABLED`"]
    #[inline]
    pub fn is_pll1_enabled(&self) -> bool {
        *self == PDR::PLL1_ENABLED
    }
    #[doc = "Checks if the value of the field is `PLL1_POWERED_DOWN`"]
    #[inline]
    pub fn is_pll1_powered_down(&self) -> bool {
        *self == PDR::PLL1_POWERED_DOWN
    }
}
#[doc = "Possible values of the field `BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSR {
    #[doc = "Normal. CCO clock sent to post-dividers. Use for normal operation."]
    NORMAL,
    #[doc = "Input clock. PLL1 input clock sent to post-dividers (default)."]
    INPUT_CLOCK,
}
impl BYPASSR {
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
            BYPASSR::NORMAL => false,
            BYPASSR::INPUT_CLOCK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASSR {
        match value {
            false => BYPASSR::NORMAL,
            true => BYPASSR::INPUT_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == BYPASSR::NORMAL
    }
    #[doc = "Checks if the value of the field is `INPUT_CLOCK`"]
    #[inline]
    pub fn is_input_clock(&self) -> bool {
        *self == BYPASSR::INPUT_CLOCK
    }
}
#[doc = "Possible values of the field `FBSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBSELR {
    #[doc = "CCO out. CCO output is used as feedback divider input clock."]
    CCO_OUT,
    #[doc = "PLL out. PLL output clock (clkout) is used as feedback divider input clock. Use for normal operation."]
    PLL_OUT,
}
impl FBSELR {
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
            FBSELR::CCO_OUT => false,
            FBSELR::PLL_OUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FBSELR {
        match value {
            false => FBSELR::CCO_OUT,
            true => FBSELR::PLL_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `CCO_OUT`"]
    #[inline]
    pub fn is_cco_out(&self) -> bool {
        *self == FBSELR::CCO_OUT
    }
    #[doc = "Checks if the value of the field is `PLL_OUT`"]
    #[inline]
    pub fn is_pll_out(&self) -> bool {
        *self == FBSELR::PLL_OUT
    }
}
#[doc = "Possible values of the field `DIRECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DIRECTR {
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
            DIRECTR::DISABLED => false,
            DIRECTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRECTR {
        match value {
            false => DIRECTR::DISABLED,
            true => DIRECTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DIRECTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DIRECTR::ENABLED
    }
}
#[doc = "Possible values of the field `PSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELR {
    #[doc = "1"]
    _1,
    #[doc = "2 (default)"]
    PEQ2,
    #[doc = "4"]
    PEQ4,
    #[doc = "8"]
    _8,
}
impl PSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSELR::_1 => 0,
            PSELR::PEQ2 => 1,
            PSELR::PEQ4 => 2,
            PSELR::_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSELR {
        match value {
            0 => PSELR::_1,
            1 => PSELR::PEQ2,
            2 => PSELR::PEQ4,
            3 => PSELR::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PSELR::_1
    }
    #[doc = "Checks if the value of the field is `PEQ2`"]
    #[inline]
    pub fn is_peq2(&self) -> bool {
        *self == PSELR::PEQ2
    }
    #[doc = "Checks if the value of the field is `PEQ4`"]
    #[inline]
    pub fn is_peq4(&self) -> bool {
        *self == PSELR::PEQ4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == PSELR::_8
    }
}
#[doc = "Possible values of the field `AUTOBLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOBLOCKR {
    #[doc = "Disabled. Autoblocking disabled"]
    DISABLED,
    #[doc = "Enabled. Autoblocking enabled"]
    ENABLED,
}
impl AUTOBLOCKR {
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
            AUTOBLOCKR::DISABLED => false,
            AUTOBLOCKR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTOBLOCKR {
        match value {
            false => AUTOBLOCKR::DISABLED,
            true => AUTOBLOCKR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOBLOCKR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOBLOCKR::ENABLED
    }
}
#[doc = "Possible values of the field `NSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSELR {
    #[doc = "1"]
    _1,
    #[doc = "2"]
    NEQ2,
    #[doc = "3 (default)"]
    NEQ3,
    #[doc = "4"]
    _4,
}
impl NSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NSELR::_1 => 0,
            NSELR::NEQ2 => 1,
            NSELR::NEQ3 => 2,
            NSELR::_4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NSELR {
        match value {
            0 => NSELR::_1,
            1 => NSELR::NEQ2,
            2 => NSELR::NEQ3,
            3 => NSELR::_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NSELR::_1
    }
    #[doc = "Checks if the value of the field is `NEQ2`"]
    #[inline]
    pub fn is_neq2(&self) -> bool {
        *self == NSELR::NEQ2
    }
    #[doc = "Checks if the value of the field is `NEQ3`"]
    #[inline]
    pub fn is_neq3(&self) -> bool {
        *self == NSELR::NEQ3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == NSELR::_4
    }
}
#[doc = r" Value of the field"]
pub struct MSELR {
    bits: u8,
}
impl MSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_SELR {
    #[doc = "32 kHz oscillator"]
    _32_KHZ_OSCILLATOR,
    #[doc = "IRC (default)"]
    IRC_DEFAULT,
    #[doc = "ENET_RX_CLK"]
    ENET_RX_CLK,
    #[doc = "ENET_TX_CLK"]
    ENET_TX_CLK,
    #[doc = "GP_CLKIN"]
    GP_CLKIN,
    #[doc = "Crystal oscillator"]
    CRYSTAL_OSCILLATOR,
    #[doc = "PLL0USB"]
    PLL0USB,
    #[doc = "PLL0AUDIO"]
    PLL0AUDIO,
    #[doc = "IDIVA"]
    IDIVA,
    #[doc = "IDIVB"]
    IDIVB,
    #[doc = "IDIVC"]
    IDIVC,
    #[doc = "IDIVD"]
    IDIVD,
    #[doc = "IDIVE"]
    IDIVE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_SELR::_32_KHZ_OSCILLATOR => 0,
            CLK_SELR::IRC_DEFAULT => 1,
            CLK_SELR::ENET_RX_CLK => 2,
            CLK_SELR::ENET_TX_CLK => 3,
            CLK_SELR::GP_CLKIN => 4,
            CLK_SELR::CRYSTAL_OSCILLATOR => 6,
            CLK_SELR::PLL0USB => 7,
            CLK_SELR::PLL0AUDIO => 8,
            CLK_SELR::IDIVA => 12,
            CLK_SELR::IDIVB => 13,
            CLK_SELR::IDIVC => 14,
            CLK_SELR::IDIVD => 15,
            CLK_SELR::IDIVE => 16,
            CLK_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_SELR {
        match value {
            0 => CLK_SELR::_32_KHZ_OSCILLATOR,
            1 => CLK_SELR::IRC_DEFAULT,
            2 => CLK_SELR::ENET_RX_CLK,
            3 => CLK_SELR::ENET_TX_CLK,
            4 => CLK_SELR::GP_CLKIN,
            6 => CLK_SELR::CRYSTAL_OSCILLATOR,
            7 => CLK_SELR::PLL0USB,
            8 => CLK_SELR::PLL0AUDIO,
            12 => CLK_SELR::IDIVA,
            13 => CLK_SELR::IDIVB,
            14 => CLK_SELR::IDIVC,
            15 => CLK_SELR::IDIVD,
            16 => CLK_SELR::IDIVE,
            i => CLK_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_32_KHZ_OSCILLATOR`"]
    #[inline]
    pub fn is_32_khz_oscillator(&self) -> bool {
        *self == CLK_SELR::_32_KHZ_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `IRC_DEFAULT`"]
    #[inline]
    pub fn is_irc_default(&self) -> bool {
        *self == CLK_SELR::IRC_DEFAULT
    }
    #[doc = "Checks if the value of the field is `ENET_RX_CLK`"]
    #[inline]
    pub fn is_enet_rx_clk(&self) -> bool {
        *self == CLK_SELR::ENET_RX_CLK
    }
    #[doc = "Checks if the value of the field is `ENET_TX_CLK`"]
    #[inline]
    pub fn is_enet_tx_clk(&self) -> bool {
        *self == CLK_SELR::ENET_TX_CLK
    }
    #[doc = "Checks if the value of the field is `GP_CLKIN`"]
    #[inline]
    pub fn is_gp_clkin(&self) -> bool {
        *self == CLK_SELR::GP_CLKIN
    }
    #[doc = "Checks if the value of the field is `CRYSTAL_OSCILLATOR`"]
    #[inline]
    pub fn is_crystal_oscillator(&self) -> bool {
        *self == CLK_SELR::CRYSTAL_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `PLL0USB`"]
    #[inline]
    pub fn is_pll0usb(&self) -> bool {
        *self == CLK_SELR::PLL0USB
    }
    #[doc = "Checks if the value of the field is `PLL0AUDIO`"]
    #[inline]
    pub fn is_pll0audio(&self) -> bool {
        *self == CLK_SELR::PLL0AUDIO
    }
    #[doc = "Checks if the value of the field is `IDIVA`"]
    #[inline]
    pub fn is_idiva(&self) -> bool {
        *self == CLK_SELR::IDIVA
    }
    #[doc = "Checks if the value of the field is `IDIVB`"]
    #[inline]
    pub fn is_idivb(&self) -> bool {
        *self == CLK_SELR::IDIVB
    }
    #[doc = "Checks if the value of the field is `IDIVC`"]
    #[inline]
    pub fn is_idivc(&self) -> bool {
        *self == CLK_SELR::IDIVC
    }
    #[doc = "Checks if the value of the field is `IDIVD`"]
    #[inline]
    pub fn is_idivd(&self) -> bool {
        *self == CLK_SELR::IDIVD
    }
    #[doc = "Checks if the value of the field is `IDIVE`"]
    #[inline]
    pub fn is_idive(&self) -> bool {
        *self == CLK_SELR::IDIVE
    }
}
#[doc = "Values that can be written to the field `PD`"]
pub enum PDW {
    #[doc = "PLL1 enabled"]
    PLL1_ENABLED,
    #[doc = "PLL1 powered down"]
    PLL1_POWERED_DOWN,
}
impl PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDW::PLL1_ENABLED => false,
            PDW::PLL1_POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDW<'a> {
    w: &'a mut W,
}
impl<'a> _PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL1 enabled"]
    #[inline]
    pub fn pll1_enabled(self) -> &'a mut W {
        self.variant(PDW::PLL1_ENABLED)
    }
    #[doc = "PLL1 powered down"]
    #[inline]
    pub fn pll1_powered_down(self) -> &'a mut W {
        self.variant(PDW::PLL1_POWERED_DOWN)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BYPASS`"]
pub enum BYPASSW {
    #[doc = "Normal. CCO clock sent to post-dividers. Use for normal operation."]
    NORMAL,
    #[doc = "Input clock. PLL1 input clock sent to post-dividers (default)."]
    INPUT_CLOCK,
}
impl BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSW::NORMAL => false,
            BYPASSW::INPUT_CLOCK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. CCO clock sent to post-dividers. Use for normal operation."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(BYPASSW::NORMAL)
    }
    #[doc = "Input clock. PLL1 input clock sent to post-dividers (default)."]
    #[inline]
    pub fn input_clock(self) -> &'a mut W {
        self.variant(BYPASSW::INPUT_CLOCK)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FBSEL`"]
pub enum FBSELW {
    #[doc = "CCO out. CCO output is used as feedback divider input clock."]
    CCO_OUT,
    #[doc = "PLL out. PLL output clock (clkout) is used as feedback divider input clock. Use for normal operation."]
    PLL_OUT,
}
impl FBSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FBSELW::CCO_OUT => false,
            FBSELW::PLL_OUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FBSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FBSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FBSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCO out. CCO output is used as feedback divider input clock."]
    #[inline]
    pub fn cco_out(self) -> &'a mut W {
        self.variant(FBSELW::CCO_OUT)
    }
    #[doc = "PLL out. PLL output clock (clkout) is used as feedback divider input clock. Use for normal operation."]
    #[inline]
    pub fn pll_out(self) -> &'a mut W {
        self.variant(FBSELW::PLL_OUT)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIRECT`"]
pub enum DIRECTW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DIRECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRECTW::DISABLED => false,
            DIRECTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRECTW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIRECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIRECTW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIRECTW::ENABLED)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSEL`"]
pub enum PSELW {
    #[doc = "1"]
    _1,
    #[doc = "2 (default)"]
    PEQ2,
    #[doc = "4"]
    PEQ4,
    #[doc = "8"]
    _8,
}
impl PSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSELW::_1 => 0,
            PSELW::PEQ2 => 1,
            PSELW::PEQ4 => 2,
            PSELW::_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSELW::_1)
    }
    #[doc = "2 (default)"]
    #[inline]
    pub fn peq2(self) -> &'a mut W {
        self.variant(PSELW::PEQ2)
    }
    #[doc = "4"]
    #[inline]
    pub fn peq4(self) -> &'a mut W {
        self.variant(PSELW::PEQ4)
    }
    #[doc = "8"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(PSELW::_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUTOBLOCK`"]
pub enum AUTOBLOCKW {
    #[doc = "Disabled. Autoblocking disabled"]
    DISABLED,
    #[doc = "Enabled. Autoblocking enabled"]
    ENABLED,
}
impl AUTOBLOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOBLOCKW::DISABLED => false,
            AUTOBLOCKW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTOBLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOBLOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTOBLOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Autoblocking disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOBLOCKW::DISABLED)
    }
    #[doc = "Enabled. Autoblocking enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOBLOCKW::ENABLED)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NSEL`"]
pub enum NSELW {
    #[doc = "1"]
    _1,
    #[doc = "2"]
    NEQ2,
    #[doc = "3 (default)"]
    NEQ3,
    #[doc = "4"]
    _4,
}
impl NSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NSELW::_1 => 0,
            NSELW::NEQ2 => 1,
            NSELW::NEQ3 => 2,
            NSELW::_4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSELW<'a> {
    w: &'a mut W,
}
impl<'a> _NSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NSELW::_1)
    }
    #[doc = "2"]
    #[inline]
    pub fn neq2(self) -> &'a mut W {
        self.variant(NSELW::NEQ2)
    }
    #[doc = "3 (default)"]
    #[inline]
    pub fn neq3(self) -> &'a mut W {
        self.variant(NSELW::NEQ3)
    }
    #[doc = "4"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(NSELW::_4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLK_SEL`"]
pub enum CLK_SELW {
    #[doc = "32 kHz oscillator"]
    _32_KHZ_OSCILLATOR,
    #[doc = "IRC (default)"]
    IRC_DEFAULT,
    #[doc = "ENET_RX_CLK"]
    ENET_RX_CLK,
    #[doc = "ENET_TX_CLK"]
    ENET_TX_CLK,
    #[doc = "GP_CLKIN"]
    GP_CLKIN,
    #[doc = "Crystal oscillator"]
    CRYSTAL_OSCILLATOR,
    #[doc = "PLL0USB"]
    PLL0USB,
    #[doc = "PLL0AUDIO"]
    PLL0AUDIO,
    #[doc = "IDIVA"]
    IDIVA,
    #[doc = "IDIVB"]
    IDIVB,
    #[doc = "IDIVC"]
    IDIVC,
    #[doc = "IDIVD"]
    IDIVD,
    #[doc = "IDIVE"]
    IDIVE,
}
impl CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLK_SELW::_32_KHZ_OSCILLATOR => 0,
            CLK_SELW::IRC_DEFAULT => 1,
            CLK_SELW::ENET_RX_CLK => 2,
            CLK_SELW::ENET_TX_CLK => 3,
            CLK_SELW::GP_CLKIN => 4,
            CLK_SELW::CRYSTAL_OSCILLATOR => 6,
            CLK_SELW::PLL0USB => 7,
            CLK_SELW::PLL0AUDIO => 8,
            CLK_SELW::IDIVA => 12,
            CLK_SELW::IDIVB => 13,
            CLK_SELW::IDIVC => 14,
            CLK_SELW::IDIVD => 15,
            CLK_SELW::IDIVE => 16,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLK_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "32 kHz oscillator"]
    #[inline]
    pub fn _32_khz_oscillator(self) -> &'a mut W {
        self.variant(CLK_SELW::_32_KHZ_OSCILLATOR)
    }
    #[doc = "IRC (default)"]
    #[inline]
    pub fn irc_default(self) -> &'a mut W {
        self.variant(CLK_SELW::IRC_DEFAULT)
    }
    #[doc = "ENET_RX_CLK"]
    #[inline]
    pub fn enet_rx_clk(self) -> &'a mut W {
        self.variant(CLK_SELW::ENET_RX_CLK)
    }
    #[doc = "ENET_TX_CLK"]
    #[inline]
    pub fn enet_tx_clk(self) -> &'a mut W {
        self.variant(CLK_SELW::ENET_TX_CLK)
    }
    #[doc = "GP_CLKIN"]
    #[inline]
    pub fn gp_clkin(self) -> &'a mut W {
        self.variant(CLK_SELW::GP_CLKIN)
    }
    #[doc = "Crystal oscillator"]
    #[inline]
    pub fn crystal_oscillator(self) -> &'a mut W {
        self.variant(CLK_SELW::CRYSTAL_OSCILLATOR)
    }
    #[doc = "PLL0USB"]
    #[inline]
    pub fn pll0usb(self) -> &'a mut W {
        self.variant(CLK_SELW::PLL0USB)
    }
    #[doc = "PLL0AUDIO"]
    #[inline]
    pub fn pll0audio(self) -> &'a mut W {
        self.variant(CLK_SELW::PLL0AUDIO)
    }
    #[doc = "IDIVA"]
    #[inline]
    pub fn idiva(self) -> &'a mut W {
        self.variant(CLK_SELW::IDIVA)
    }
    #[doc = "IDIVB"]
    #[inline]
    pub fn idivb(self) -> &'a mut W {
        self.variant(CLK_SELW::IDIVB)
    }
    #[doc = "IDIVC"]
    #[inline]
    pub fn idivc(self) -> &'a mut W {
        self.variant(CLK_SELW::IDIVC)
    }
    #[doc = "IDIVD"]
    #[inline]
    pub fn idivd(self) -> &'a mut W {
        self.variant(CLK_SELW::IDIVD)
    }
    #[doc = "IDIVE"]
    #[inline]
    pub fn idive(self) -> &'a mut W {
        self.variant(CLK_SELW::IDIVE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - PLL1 power down"]
    #[inline]
    pub fn pd(&self) -> PDR {
        PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Input clock bypass control"]
    #[inline]
    pub fn bypass(&self) -> BYPASSR {
        BYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PLL feedback select."]
    #[inline]
    pub fn fbsel(&self) -> FBSELR {
        FBSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - PLL direct CCO output"]
    #[inline]
    pub fn direct(&self) -> DIRECTR {
        DIRECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Post-divider division ratio P. The value applied is 2xP."]
    #[inline]
    pub fn psel(&self) -> PSELR {
        PSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Block clock automatically during frequency change"]
    #[inline]
    pub fn autoblock(&self) -> AUTOBLOCKR {
        AUTOBLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - Pre-divider division ratio N"]
    #[inline]
    pub fn nsel(&self) -> NSELR {
        NSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Feedback-divider division ratio (M) 00000000 = 1 00000001 = 2 ... 11111111 = 256"]
    #[inline]
    pub fn msel(&self) -> MSELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MSELR { bits }
    }
    #[doc = "Bits 24:28 - Clock-source selection."]
    #[inline]
    pub fn clk_sel(&self) -> CLK_SELR {
        CLK_SELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16777219 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - PLL1 power down"]
    #[inline]
    pub fn pd(&mut self) -> _PDW {
        _PDW { w: self }
    }
    #[doc = "Bit 1 - Input clock bypass control"]
    #[inline]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 6 - PLL feedback select."]
    #[inline]
    pub fn fbsel(&mut self) -> _FBSELW {
        _FBSELW { w: self }
    }
    #[doc = "Bit 7 - PLL direct CCO output"]
    #[inline]
    pub fn direct(&mut self) -> _DIRECTW {
        _DIRECTW { w: self }
    }
    #[doc = "Bits 8:9 - Post-divider division ratio P. The value applied is 2xP."]
    #[inline]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
    #[doc = "Bit 11 - Block clock automatically during frequency change"]
    #[inline]
    pub fn autoblock(&mut self) -> _AUTOBLOCKW {
        _AUTOBLOCKW { w: self }
    }
    #[doc = "Bits 12:13 - Pre-divider division ratio N"]
    #[inline]
    pub fn nsel(&mut self) -> _NSELW {
        _NSELW { w: self }
    }
    #[doc = "Bits 16:23 - Feedback-divider division ratio (M) 00000000 = 1 00000001 = 2 ... 11111111 = 256"]
    #[inline]
    pub fn msel(&mut self) -> _MSELW {
        _MSELW { w: self }
    }
    #[doc = "Bits 24:28 - Clock-source selection."]
    #[inline]
    pub fn clk_sel(&mut self) -> _CLK_SELW {
        _CLK_SELW { w: self }
    }
}
