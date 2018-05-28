#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FREQ_MON {
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
#[doc = r" Value of the field"]
pub struct RCNTR {
    bits: u16,
}
impl RCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FCNTR {
    bits: u16,
}
impl FCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `MEAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEASR {
    #[doc = "RCNT and FCNT disabled"]
    RCNT_AND_FCNT_DISABL,
    #[doc = "Frequency counters started"]
    FREQUENCY_COUNTERS_S,
}
impl MEASR {
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
            MEASR::RCNT_AND_FCNT_DISABL => false,
            MEASR::FREQUENCY_COUNTERS_S => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEASR {
        match value {
            false => MEASR::RCNT_AND_FCNT_DISABL,
            true => MEASR::FREQUENCY_COUNTERS_S,
        }
    }
    #[doc = "Checks if the value of the field is `RCNT_AND_FCNT_DISABL`"]
    #[inline]
    pub fn is_rcnt_and_fcnt_disabl(&self) -> bool {
        *self == MEASR::RCNT_AND_FCNT_DISABL
    }
    #[doc = "Checks if the value of the field is `FREQUENCY_COUNTERS_S`"]
    #[inline]
    pub fn is_frequency_counters_s(&self) -> bool {
        *self == MEASR::FREQUENCY_COUNTERS_S
    }
}
#[doc = "Possible values of the field `CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_SELR {
    #[doc = "32 kHz oscillator (default)"]
    _32_KHZ_OSCILLATOR_D,
    #[doc = "IRC"]
    IRC,
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
            CLK_SELR::_32_KHZ_OSCILLATOR_D => 0,
            CLK_SELR::IRC => 1,
            CLK_SELR::ENET_RX_CLK => 2,
            CLK_SELR::ENET_TX_CLK => 3,
            CLK_SELR::GP_CLKIN => 4,
            CLK_SELR::CRYSTAL_OSCILLATOR => 6,
            CLK_SELR::PLL0USB => 7,
            CLK_SELR::PLL0AUDIO => 8,
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
            0 => CLK_SELR::_32_KHZ_OSCILLATOR_D,
            1 => CLK_SELR::IRC,
            2 => CLK_SELR::ENET_RX_CLK,
            3 => CLK_SELR::ENET_TX_CLK,
            4 => CLK_SELR::GP_CLKIN,
            6 => CLK_SELR::CRYSTAL_OSCILLATOR,
            7 => CLK_SELR::PLL0USB,
            8 => CLK_SELR::PLL0AUDIO,
            9 => CLK_SELR::PLL1,
            12 => CLK_SELR::IDIVA,
            13 => CLK_SELR::IDIVB,
            14 => CLK_SELR::IDIVC,
            15 => CLK_SELR::IDIVD,
            16 => CLK_SELR::IDIVE,
            i => CLK_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_32_KHZ_OSCILLATOR_D`"]
    #[inline]
    pub fn is_32_khz_oscillator_d(&self) -> bool {
        *self == CLK_SELR::_32_KHZ_OSCILLATOR_D
    }
    #[doc = "Checks if the value of the field is `IRC`"]
    #[inline]
    pub fn is_irc(&self) -> bool {
        *self == CLK_SELR::IRC
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
#[doc = r" Proxy"]
pub struct _RCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _RCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _FCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 16383;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MEAS`"]
pub enum MEASW {
    #[doc = "RCNT and FCNT disabled"]
    RCNT_AND_FCNT_DISABL,
    #[doc = "Frequency counters started"]
    FREQUENCY_COUNTERS_S,
}
impl MEASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEASW::RCNT_AND_FCNT_DISABL => false,
            MEASW::FREQUENCY_COUNTERS_S => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEASW<'a> {
    w: &'a mut W,
}
impl<'a> _MEASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEASW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RCNT and FCNT disabled"]
    #[inline]
    pub fn rcnt_and_fcnt_disabl(self) -> &'a mut W {
        self.variant(MEASW::RCNT_AND_FCNT_DISABL)
    }
    #[doc = "Frequency counters started"]
    #[inline]
    pub fn frequency_counters_s(self) -> &'a mut W {
        self.variant(MEASW::FREQUENCY_COUNTERS_S)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLK_SEL`"]
pub enum CLK_SELW {
    #[doc = "32 kHz oscillator (default)"]
    _32_KHZ_OSCILLATOR_D,
    #[doc = "IRC"]
    IRC,
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
            CLK_SELW::_32_KHZ_OSCILLATOR_D => 0,
            CLK_SELW::IRC => 1,
            CLK_SELW::ENET_RX_CLK => 2,
            CLK_SELW::ENET_TX_CLK => 3,
            CLK_SELW::GP_CLKIN => 4,
            CLK_SELW::CRYSTAL_OSCILLATOR => 6,
            CLK_SELW::PLL0USB => 7,
            CLK_SELW::PLL0AUDIO => 8,
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
    #[doc = "32 kHz oscillator (default)"]
    #[inline]
    pub fn _32_khz_oscillator_d(self) -> &'a mut W {
        self.variant(CLK_SELW::_32_KHZ_OSCILLATOR_D)
    }
    #[doc = "IRC"]
    #[inline]
    pub fn irc(self) -> &'a mut W {
        self.variant(CLK_SELW::IRC)
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
    #[doc = "Bits 0:8 - 9-bit reference clock-counter value"]
    #[inline]
    pub fn rcnt(&self) -> RCNTR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RCNTR { bits }
    }
    #[doc = "Bits 9:22 - 14-bit selected clock-counter value"]
    #[inline]
    pub fn fcnt(&self) -> FCNTR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FCNTR { bits }
    }
    #[doc = "Bit 23 - Measure frequency"]
    #[inline]
    pub fn meas(&self) -> MEASR {
        MEASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:28 - Clock-source selection for the clock to be measured. All other values are reserved."]
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:8 - 9-bit reference clock-counter value"]
    #[inline]
    pub fn rcnt(&mut self) -> _RCNTW {
        _RCNTW { w: self }
    }
    #[doc = "Bits 9:22 - 14-bit selected clock-counter value"]
    #[inline]
    pub fn fcnt(&mut self) -> _FCNTW {
        _FCNTW { w: self }
    }
    #[doc = "Bit 23 - Measure frequency"]
    #[inline]
    pub fn meas(&mut self) -> _MEASW {
        _MEASW { w: self }
    }
    #[doc = "Bits 24:28 - Clock-source selection for the clock to be measured. All other values are reserved."]
    #[inline]
    pub fn clk_sel(&mut self) -> _CLK_SELW {
        _CLK_SELW { w: self }
    }
}
