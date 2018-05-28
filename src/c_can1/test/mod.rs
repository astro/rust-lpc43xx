#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TEST {
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
#[doc = "Possible values of the field `BASIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BASICR {
    #[doc = "IF1 registers used as TX buffer, IF2 registers used as RX buffer."]
    IF1_TX_IF2_RX,
    #[doc = "Basic mode disabled."]
    BASIC_MODE_DISABLED_,
}
impl BASICR {
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
            BASICR::IF1_TX_IF2_RX => true,
            BASICR::BASIC_MODE_DISABLED_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BASICR {
        match value {
            true => BASICR::IF1_TX_IF2_RX,
            false => BASICR::BASIC_MODE_DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `IF1_TX_IF2_RX`"]
    #[inline]
    pub fn is_if1_tx_if2_rx(&self) -> bool {
        *self == BASICR::IF1_TX_IF2_RX
    }
    #[doc = "Checks if the value of the field is `BASIC_MODE_DISABLED_`"]
    #[inline]
    pub fn is_basic_mode_disabled_(&self) -> bool {
        *self == BASICR::BASIC_MODE_DISABLED_
    }
}
#[doc = "Possible values of the field `SILENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SILENTR {
    #[doc = "The module is in silent mode."]
    SILENT,
    #[doc = "Normal operation."]
    NORMAL_OPERATION_,
}
impl SILENTR {
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
            SILENTR::SILENT => true,
            SILENTR::NORMAL_OPERATION_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SILENTR {
        match value {
            true => SILENTR::SILENT,
            false => SILENTR::NORMAL_OPERATION_,
        }
    }
    #[doc = "Checks if the value of the field is `SILENT`"]
    #[inline]
    pub fn is_silent(&self) -> bool {
        *self == SILENTR::SILENT
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION_`"]
    #[inline]
    pub fn is_normal_operation_(&self) -> bool {
        *self == SILENTR::NORMAL_OPERATION_
    }
}
#[doc = "Possible values of the field `LBACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBACKR {
    #[doc = "Loop back mode is enabled."]
    ENABLED,
    #[doc = "Loop back mode is disabled."]
    DISABLED,
}
impl LBACKR {
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
            LBACKR::ENABLED => true,
            LBACKR::DISABLED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBACKR {
        match value {
            true => LBACKR::ENABLED,
            false => LBACKR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LBACKR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LBACKR::DISABLED
    }
}
#[doc = "Possible values of the field `TX1_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX1_0R {
    #[doc = "Level at the TD pin is controlled by the CAN controller. This is the value at reset."]
    LEVEL_AT_THE_TD_PIN_,
    #[doc = "The sample point can be monitored at the TD pin."]
    THE_SAMPLE_POINT_CAN,
    #[doc = "TD pin is driven LOW/dominant."]
    TD_PIN_IS_DRIVEN_LOW,
    #[doc = "TD pin is driven HIGH/recessive."]
    TD_PIN_IS_DRIVEN_HIG,
}
impl TX1_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX1_0R::LEVEL_AT_THE_TD_PIN_ => 0,
            TX1_0R::THE_SAMPLE_POINT_CAN => 1,
            TX1_0R::TD_PIN_IS_DRIVEN_LOW => 2,
            TX1_0R::TD_PIN_IS_DRIVEN_HIG => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX1_0R {
        match value {
            0 => TX1_0R::LEVEL_AT_THE_TD_PIN_,
            1 => TX1_0R::THE_SAMPLE_POINT_CAN,
            2 => TX1_0R::TD_PIN_IS_DRIVEN_LOW,
            3 => TX1_0R::TD_PIN_IS_DRIVEN_HIG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_AT_THE_TD_PIN_`"]
    #[inline]
    pub fn is_level_at_the_td_pin_(&self) -> bool {
        *self == TX1_0R::LEVEL_AT_THE_TD_PIN_
    }
    #[doc = "Checks if the value of the field is `THE_SAMPLE_POINT_CAN`"]
    #[inline]
    pub fn is_the_sample_point_can(&self) -> bool {
        *self == TX1_0R::THE_SAMPLE_POINT_CAN
    }
    #[doc = "Checks if the value of the field is `TD_PIN_IS_DRIVEN_LOW`"]
    #[inline]
    pub fn is_td_pin_is_driven_low(&self) -> bool {
        *self == TX1_0R::TD_PIN_IS_DRIVEN_LOW
    }
    #[doc = "Checks if the value of the field is `TD_PIN_IS_DRIVEN_HIG`"]
    #[inline]
    pub fn is_td_pin_is_driven_hig(&self) -> bool {
        *self == TX1_0R::TD_PIN_IS_DRIVEN_HIG
    }
}
#[doc = "Possible values of the field `RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXR {
    #[doc = "The CAN bus is recessive (RD = 1)."]
    THE_CAN_BUS_IS_RECES,
    #[doc = "The CAN bus is dominant (RD = 0)."]
    THE_CAN_BUS_IS_DOMIN,
}
impl RXR {
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
            RXR::THE_CAN_BUS_IS_RECES => true,
            RXR::THE_CAN_BUS_IS_DOMIN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXR {
        match value {
            true => RXR::THE_CAN_BUS_IS_RECES,
            false => RXR::THE_CAN_BUS_IS_DOMIN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_CAN_BUS_IS_RECES`"]
    #[inline]
    pub fn is_the_can_bus_is_reces(&self) -> bool {
        *self == RXR::THE_CAN_BUS_IS_RECES
    }
    #[doc = "Checks if the value of the field is `THE_CAN_BUS_IS_DOMIN`"]
    #[inline]
    pub fn is_the_can_bus_is_domin(&self) -> bool {
        *self == RXR::THE_CAN_BUS_IS_DOMIN
    }
}
#[doc = "Values that can be written to the field `BASIC`"]
pub enum BASICW {
    #[doc = "IF1 registers used as TX buffer, IF2 registers used as RX buffer."]
    IF1_TX_IF2_RX,
    #[doc = "Basic mode disabled."]
    BASIC_MODE_DISABLED_,
}
impl BASICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BASICW::IF1_TX_IF2_RX => true,
            BASICW::BASIC_MODE_DISABLED_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BASICW<'a> {
    w: &'a mut W,
}
impl<'a> _BASICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BASICW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IF1 registers used as TX buffer, IF2 registers used as RX buffer."]
    #[inline]
    pub fn if1_tx_if2_rx(self) -> &'a mut W {
        self.variant(BASICW::IF1_TX_IF2_RX)
    }
    #[doc = "Basic mode disabled."]
    #[inline]
    pub fn basic_mode_disabled_(self) -> &'a mut W {
        self.variant(BASICW::BASIC_MODE_DISABLED_)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SILENT`"]
pub enum SILENTW {
    #[doc = "The module is in silent mode."]
    SILENT,
    #[doc = "Normal operation."]
    NORMAL_OPERATION_,
}
impl SILENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SILENTW::SILENT => true,
            SILENTW::NORMAL_OPERATION_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SILENTW<'a> {
    w: &'a mut W,
}
impl<'a> _SILENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SILENTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The module is in silent mode."]
    #[inline]
    pub fn silent(self) -> &'a mut W {
        self.variant(SILENTW::SILENT)
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn normal_operation_(self) -> &'a mut W {
        self.variant(SILENTW::NORMAL_OPERATION_)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LBACK`"]
pub enum LBACKW {
    #[doc = "Loop back mode is enabled."]
    ENABLED,
    #[doc = "Loop back mode is disabled."]
    DISABLED,
}
impl LBACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBACKW::ENABLED => true,
            LBACKW::DISABLED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBACKW<'a> {
    w: &'a mut W,
}
impl<'a> _LBACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Loop back mode is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBACKW::ENABLED)
    }
    #[doc = "Loop back mode is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBACKW::DISABLED)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX1_0`"]
pub enum TX1_0W {
    #[doc = "Level at the TD pin is controlled by the CAN controller. This is the value at reset."]
    LEVEL_AT_THE_TD_PIN_,
    #[doc = "The sample point can be monitored at the TD pin."]
    THE_SAMPLE_POINT_CAN,
    #[doc = "TD pin is driven LOW/dominant."]
    TD_PIN_IS_DRIVEN_LOW,
    #[doc = "TD pin is driven HIGH/recessive."]
    TD_PIN_IS_DRIVEN_HIG,
}
impl TX1_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TX1_0W::LEVEL_AT_THE_TD_PIN_ => 0,
            TX1_0W::THE_SAMPLE_POINT_CAN => 1,
            TX1_0W::TD_PIN_IS_DRIVEN_LOW => 2,
            TX1_0W::TD_PIN_IS_DRIVEN_HIG => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX1_0W<'a> {
    w: &'a mut W,
}
impl<'a> _TX1_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX1_0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level at the TD pin is controlled by the CAN controller. This is the value at reset."]
    #[inline]
    pub fn level_at_the_td_pin_(self) -> &'a mut W {
        self.variant(TX1_0W::LEVEL_AT_THE_TD_PIN_)
    }
    #[doc = "The sample point can be monitored at the TD pin."]
    #[inline]
    pub fn the_sample_point_can(self) -> &'a mut W {
        self.variant(TX1_0W::THE_SAMPLE_POINT_CAN)
    }
    #[doc = "TD pin is driven LOW/dominant."]
    #[inline]
    pub fn td_pin_is_driven_low(self) -> &'a mut W {
        self.variant(TX1_0W::TD_PIN_IS_DRIVEN_LOW)
    }
    #[doc = "TD pin is driven HIGH/recessive."]
    #[inline]
    pub fn td_pin_is_driven_hig(self) -> &'a mut W {
        self.variant(TX1_0W::TD_PIN_IS_DRIVEN_HIG)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX`"]
pub enum RXW {
    #[doc = "The CAN bus is recessive (RD = 1)."]
    THE_CAN_BUS_IS_RECES,
    #[doc = "The CAN bus is dominant (RD = 0)."]
    THE_CAN_BUS_IS_DOMIN,
}
impl RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXW::THE_CAN_BUS_IS_RECES => true,
            RXW::THE_CAN_BUS_IS_DOMIN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXW<'a> {
    w: &'a mut W,
}
impl<'a> _RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CAN bus is recessive (RD = 1)."]
    #[inline]
    pub fn the_can_bus_is_reces(self) -> &'a mut W {
        self.variant(RXW::THE_CAN_BUS_IS_RECES)
    }
    #[doc = "The CAN bus is dominant (RD = 0)."]
    #[inline]
    pub fn the_can_bus_is_domin(self) -> &'a mut W {
        self.variant(RXW::THE_CAN_BUS_IS_DOMIN)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - Basic mode"]
    #[inline]
    pub fn basic(&self) -> BASICR {
        BASICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Silent mode"]
    #[inline]
    pub fn silent(&self) -> SILENTR {
        SILENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Loop back mode"]
    #[inline]
    pub fn lback(&self) -> LBACKR {
        LBACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - Control of TD pins"]
    #[inline]
    pub fn tx1_0(&self) -> TX1_0R {
        TX1_0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Monitors the actual value of the RD Pin"]
    #[inline]
    pub fn rx(&self) -> RXR {
        RXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 2 - Basic mode"]
    #[inline]
    pub fn basic(&mut self) -> _BASICW {
        _BASICW { w: self }
    }
    #[doc = "Bit 3 - Silent mode"]
    #[inline]
    pub fn silent(&mut self) -> _SILENTW {
        _SILENTW { w: self }
    }
    #[doc = "Bit 4 - Loop back mode"]
    #[inline]
    pub fn lback(&mut self) -> _LBACKW {
        _LBACKW { w: self }
    }
    #[doc = "Bits 5:6 - Control of TD pins"]
    #[inline]
    pub fn tx1_0(&mut self) -> _TX1_0W {
        _TX1_0W { w: self }
    }
    #[doc = "Bit 7 - Monitors the actual value of the RD Pin"]
    #[inline]
    pub fn rx(&mut self) -> _RXW {
        _RXW { w: self }
    }
}
