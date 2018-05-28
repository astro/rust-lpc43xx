#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IDIVC_CTRL {
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
    #[doc = "Enabled. IDIV enabled (default)"]
    ENABLED,
    #[doc = "Power-down"]
    POWER_DOWN,
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
            PDR::ENABLED => false,
            PDR::POWER_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDR {
        match value {
            false => PDR::ENABLED,
            true => PDR::POWER_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PDR::ENABLED
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline]
    pub fn is_power_down(&self) -> bool {
        *self == PDR::POWER_DOWN
    }
}
#[doc = r" Value of the field"]
pub struct IDIVR {
    bits: u8,
}
impl IDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
    #[doc = "PLL0AUDIO"]
    PLL0AUDIO,
    #[doc = "PLL1"]
    PLL1,
    #[doc = "IDIVA"]
    IDIVA,
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
            CLK_SELR::PLL0AUDIO => 8,
            CLK_SELR::PLL1 => 9,
            CLK_SELR::IDIVA => 12,
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
            8 => CLK_SELR::PLL0AUDIO,
            9 => CLK_SELR::PLL1,
            12 => CLK_SELR::IDIVA,
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
    #[doc = "Checks if the value of the field is `PLL0AUDIO`"]
    #[inline]
    pub fn is_pll0audio(&self) -> bool {
        *self == CLK_SELR::PLL0AUDIO
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
}
#[doc = "Values that can be written to the field `PD`"]
pub enum PDW {
    #[doc = "Enabled. IDIV enabled (default)"]
    ENABLED,
    #[doc = "Power-down"]
    POWER_DOWN,
}
impl PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDW::ENABLED => false,
            PDW::POWER_DOWN => true,
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
    #[doc = "Enabled. IDIV enabled (default)"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PDW::ENABLED)
    }
    #[doc = "Power-down"]
    #[inline]
    pub fn power_down(self) -> &'a mut W {
        self.variant(PDW::POWER_DOWN)
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
#[doc = r" Proxy"]
pub struct _IDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _IDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 2;
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
    #[doc = "PLL0AUDIO"]
    PLL0AUDIO,
    #[doc = "PLL1"]
    PLL1,
    #[doc = "IDIVA"]
    IDIVA,
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
            CLK_SELW::PLL0AUDIO => 8,
            CLK_SELW::PLL1 => 9,
            CLK_SELW::IDIVA => 12,
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
    #[doc = "PLL0AUDIO"]
    #[inline]
    pub fn pll0audio(self) -> &'a mut W {
        self.variant(CLK_SELW::PLL0AUDIO)
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
    #[doc = "Bit 0 - Integer divider power down"]
    #[inline]
    pub fn pd(&self) -> PDR {
        PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:5 - Integer divider B, C, D divider values (1/(IDIV + 1)) 0000 = 1 (default) 0001 = 2 ... 1111 = 16"]
    #[inline]
    pub fn idiv(&self) -> IDIVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDIVR { bits }
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
    #[doc = "Bits 24:28 - Clock-source selection. All other values are reserved."]
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
        W { bits: 16777216 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Integer divider power down"]
    #[inline]
    pub fn pd(&mut self) -> _PDW {
        _PDW { w: self }
    }
    #[doc = "Bits 2:5 - Integer divider B, C, D divider values (1/(IDIV + 1)) 0000 = 1 (default) 0001 = 2 ... 1111 = 16"]
    #[inline]
    pub fn idiv(&mut self) -> _IDIVW {
        _IDIVW { w: self }
    }
    #[doc = "Bit 11 - Block clock automatically during frequency change"]
    #[inline]
    pub fn autoblock(&mut self) -> _AUTOBLOCKW {
        _AUTOBLOCKW { w: self }
    }
    #[doc = "Bits 24:28 - Clock-source selection. All other values are reserved."]
    #[inline]
    pub fn clk_sel(&mut self) -> _CLK_SELW {
        _CLK_SELW { w: self }
    }
}
