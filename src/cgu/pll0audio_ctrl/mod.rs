#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLL0AUDIO_CTRL {
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
    #[doc = "PLL0 enabled"]
    PLL0_ENABLED,
    #[doc = "PLL0 powered down"]
    PLL0_POWERED_DOWN,
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
            PDR::PLL0_ENABLED => false,
            PDR::PLL0_POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDR {
        match value {
            false => PDR::PLL0_ENABLED,
            true => PDR::PLL0_POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `PLL0_ENABLED`"]
    #[inline]
    pub fn is_pll0_enabled(&self) -> bool {
        *self == PDR::PLL0_ENABLED
    }
    #[doc = "Checks if the value of the field is `PLL0_POWERED_DOWN`"]
    #[inline]
    pub fn is_pll0_powered_down(&self) -> bool {
        *self == PDR::PLL0_POWERED_DOWN
    }
}
#[doc = "Possible values of the field `BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSR {
    #[doc = "CCO clock sent to post-dividers. Use this in normal operation."]
    CCO_CLOCK_SENT_TO_PO,
    #[doc = "PLL0 input clock sent to post-dividers (default)."]
    PLL0_INPUT_CLOCK_SEN,
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
            BYPASSR::CCO_CLOCK_SENT_TO_PO => false,
            BYPASSR::PLL0_INPUT_CLOCK_SEN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASSR {
        match value {
            false => BYPASSR::CCO_CLOCK_SENT_TO_PO,
            true => BYPASSR::PLL0_INPUT_CLOCK_SEN,
        }
    }
    #[doc = "Checks if the value of the field is `CCO_CLOCK_SENT_TO_PO`"]
    #[inline]
    pub fn is_cco_clock_sent_to_po(&self) -> bool {
        *self == BYPASSR::CCO_CLOCK_SENT_TO_PO
    }
    #[doc = "Checks if the value of the field is `PLL0_INPUT_CLOCK_SEN`"]
    #[inline]
    pub fn is_pll0_input_clock_sen(&self) -> bool {
        *self == BYPASSR::PLL0_INPUT_CLOCK_SEN
    }
}
#[doc = r" Value of the field"]
pub struct DIRECTIR {
    bits: bool,
}
impl DIRECTIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct DIRECTOR {
    bits: bool,
}
impl DIRECTOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CLKENR {
    bits: bool,
}
impl CLKENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct FRMR {
    bits: bool,
}
impl FRMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
#[doc = r" Value of the field"]
pub struct PLLFRACT_REQR {
    bits: bool,
}
impl PLLFRACT_REQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `SEL_EXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_EXTR {
    #[doc = "FRAC Enabled. Enable fractional divider."]
    FRAC_ENABLED,
    #[doc = "MDEC enabled. Fractional divider not used."]
    MDEC_ENABLED,
}
impl SEL_EXTR {
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
            SEL_EXTR::FRAC_ENABLED => false,
            SEL_EXTR::MDEC_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEL_EXTR {
        match value {
            false => SEL_EXTR::FRAC_ENABLED,
            true => SEL_EXTR::MDEC_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `FRAC_ENABLED`"]
    #[inline]
    pub fn is_frac_enabled(&self) -> bool {
        *self == SEL_EXTR::FRAC_ENABLED
    }
    #[doc = "Checks if the value of the field is `MDEC_ENABLED`"]
    #[inline]
    pub fn is_mdec_enabled(&self) -> bool {
        *self == SEL_EXTR::MDEC_ENABLED
    }
}
#[doc = "Possible values of the field `MOD_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_PDR {
    #[doc = "Enabled. Sigma-Delta modulator enabled"]
    ENABLED,
    #[doc = "Disabled. Sigma-Delta modulator powered down"]
    DISABLED,
}
impl MOD_PDR {
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
            MOD_PDR::ENABLED => false,
            MOD_PDR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MOD_PDR {
        match value {
            false => MOD_PDR::ENABLED,
            true => MOD_PDR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MOD_PDR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MOD_PDR::DISABLED
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
    #[doc = "PLL1"]
    PLL1,
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
            CLK_SELR::PLL1 => 9,
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
            9 => CLK_SELR::PLL1,
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
    #[doc = "Checks if the value of the field is `PLL1`"]
    #[inline]
    pub fn is_pll1(&self) -> bool {
        *self == CLK_SELR::PLL1
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
    #[doc = "PLL0 enabled"]
    PLL0_ENABLED,
    #[doc = "PLL0 powered down"]
    PLL0_POWERED_DOWN,
}
impl PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDW::PLL0_ENABLED => false,
            PDW::PLL0_POWERED_DOWN => true,
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
    #[doc = "PLL0 enabled"]
    #[inline]
    pub fn pll0_enabled(self) -> &'a mut W {
        self.variant(PDW::PLL0_ENABLED)
    }
    #[doc = "PLL0 powered down"]
    #[inline]
    pub fn pll0_powered_down(self) -> &'a mut W {
        self.variant(PDW::PLL0_POWERED_DOWN)
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
    #[doc = "CCO clock sent to post-dividers. Use this in normal operation."]
    CCO_CLOCK_SENT_TO_PO,
    #[doc = "PLL0 input clock sent to post-dividers (default)."]
    PLL0_INPUT_CLOCK_SEN,
}
impl BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSW::CCO_CLOCK_SENT_TO_PO => false,
            BYPASSW::PLL0_INPUT_CLOCK_SEN => true,
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
    #[doc = "CCO clock sent to post-dividers. Use this in normal operation."]
    #[inline]
    pub fn cco_clock_sent_to_po(self) -> &'a mut W {
        self.variant(BYPASSW::CCO_CLOCK_SENT_TO_PO)
    }
    #[doc = "PLL0 input clock sent to post-dividers (default)."]
    #[inline]
    pub fn pll0_input_clock_sen(self) -> &'a mut W {
        self.variant(BYPASSW::PLL0_INPUT_CLOCK_SEN)
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
#[doc = r" Proxy"]
pub struct _DIRECTIW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRECTIW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIRECTOW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRECTOW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKENW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRMW<'a> {
    w: &'a mut W,
}
impl<'a> _FRMW<'a> {
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
#[doc = r" Proxy"]
pub struct _PLLFRACT_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLFRACT_REQW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEL_EXT`"]
pub enum SEL_EXTW {
    #[doc = "FRAC Enabled. Enable fractional divider."]
    FRAC_ENABLED,
    #[doc = "MDEC enabled. Fractional divider not used."]
    MDEC_ENABLED,
}
impl SEL_EXTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEL_EXTW::FRAC_ENABLED => false,
            SEL_EXTW::MDEC_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEL_EXTW<'a> {
    w: &'a mut W,
}
impl<'a> _SEL_EXTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEL_EXTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FRAC Enabled. Enable fractional divider."]
    #[inline]
    pub fn frac_enabled(self) -> &'a mut W {
        self.variant(SEL_EXTW::FRAC_ENABLED)
    }
    #[doc = "MDEC enabled. Fractional divider not used."]
    #[inline]
    pub fn mdec_enabled(self) -> &'a mut W {
        self.variant(SEL_EXTW::MDEC_ENABLED)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MOD_PD`"]
pub enum MOD_PDW {
    #[doc = "Enabled. Sigma-Delta modulator enabled"]
    ENABLED,
    #[doc = "Disabled. Sigma-Delta modulator powered down"]
    DISABLED,
}
impl MOD_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MOD_PDW::ENABLED => false,
            MOD_PDW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MOD_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _MOD_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MOD_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled. Sigma-Delta modulator enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MOD_PDW::ENABLED)
    }
    #[doc = "Disabled. Sigma-Delta modulator powered down"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MOD_PDW::DISABLED)
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
        const OFFSET: u8 = 14;
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
    #[doc = "PLL1"]
    PLL1,
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
            CLK_SELW::PLL1 => 9,
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
    #[doc = "PLL1"]
    #[inline]
    pub fn pll1(self) -> &'a mut W {
        self.variant(CLK_SELW::PLL1)
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
    #[doc = "Bit 0 - PLL0 power down"]
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
    #[doc = "Bit 2 - PLL0 direct input"]
    #[inline]
    pub fn directi(&self) -> DIRECTIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIRECTIR { bits }
    }
    #[doc = "Bit 3 - PLL0 direct output"]
    #[inline]
    pub fn directo(&self) -> DIRECTOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIRECTOR { bits }
    }
    #[doc = "Bit 4 - PLL0 clock enable"]
    #[inline]
    pub fn clken(&self) -> CLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKENR { bits }
    }
    #[doc = "Bit 6 - Free running mode"]
    #[inline]
    pub fn frm(&self) -> FRMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRMR { bits }
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
    #[doc = "Bit 12 - Fractional PLL word write request. Set this bit to 1 if the fractional divider is enabled in the SEL_EXT bit."]
    #[inline]
    pub fn pllfract_req(&self) -> PLLFRACT_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLFRACT_REQR { bits }
    }
    #[doc = "Bit 13 - Select fractional divider."]
    #[inline]
    pub fn sel_ext(&self) -> SEL_EXTR {
        SEL_EXTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Sigma-Delta modulator power-down"]
    #[inline]
    pub fn mod_pd(&self) -> MOD_PDR {
        MOD_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:28 - Clock source selection. All other values are reserved."]
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
        W { bits: 16793603 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - PLL0 power down"]
    #[inline]
    pub fn pd(&mut self) -> _PDW {
        _PDW { w: self }
    }
    #[doc = "Bit 1 - Input clock bypass control"]
    #[inline]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 2 - PLL0 direct input"]
    #[inline]
    pub fn directi(&mut self) -> _DIRECTIW {
        _DIRECTIW { w: self }
    }
    #[doc = "Bit 3 - PLL0 direct output"]
    #[inline]
    pub fn directo(&mut self) -> _DIRECTOW {
        _DIRECTOW { w: self }
    }
    #[doc = "Bit 4 - PLL0 clock enable"]
    #[inline]
    pub fn clken(&mut self) -> _CLKENW {
        _CLKENW { w: self }
    }
    #[doc = "Bit 6 - Free running mode"]
    #[inline]
    pub fn frm(&mut self) -> _FRMW {
        _FRMW { w: self }
    }
    #[doc = "Bit 11 - Block clock automatically during frequency change"]
    #[inline]
    pub fn autoblock(&mut self) -> _AUTOBLOCKW {
        _AUTOBLOCKW { w: self }
    }
    #[doc = "Bit 12 - Fractional PLL word write request. Set this bit to 1 if the fractional divider is enabled in the SEL_EXT bit."]
    #[inline]
    pub fn pllfract_req(&mut self) -> _PLLFRACT_REQW {
        _PLLFRACT_REQW { w: self }
    }
    #[doc = "Bit 13 - Select fractional divider."]
    #[inline]
    pub fn sel_ext(&mut self) -> _SEL_EXTW {
        _SEL_EXTW { w: self }
    }
    #[doc = "Bit 14 - Sigma-Delta modulator power-down"]
    #[inline]
    pub fn mod_pd(&mut self) -> _MOD_PDW {
        _MOD_PDW { w: self }
    }
    #[doc = "Bits 24:28 - Clock source selection. All other values are reserved."]
    #[inline]
    pub fn clk_sel(&mut self) -> _CLK_SELW {
        _CLK_SELW { w: self }
    }
}
