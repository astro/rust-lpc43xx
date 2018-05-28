#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CREG0 {
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
#[doc = "Possible values of the field `EN1KHZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1KHZR {
    #[doc = "1 kHz output disabled."]
    _1_KHZ_OUTPUT_DISABLE,
    #[doc = "1 kHz output enabled."]
    _1_KHZ_OUTPUT_ENABLED,
}
impl EN1KHZR {
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
            EN1KHZR::_1_KHZ_OUTPUT_DISABLE => false,
            EN1KHZR::_1_KHZ_OUTPUT_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN1KHZR {
        match value {
            false => EN1KHZR::_1_KHZ_OUTPUT_DISABLE,
            true => EN1KHZR::_1_KHZ_OUTPUT_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `_1_KHZ_OUTPUT_DISABLE`"]
    #[inline]
    pub fn is_1_khz_output_disable(&self) -> bool {
        *self == EN1KHZR::_1_KHZ_OUTPUT_DISABLE
    }
    #[doc = "Checks if the value of the field is `_1_KHZ_OUTPUT_ENABLED`"]
    #[inline]
    pub fn is_1_khz_output_enabled(&self) -> bool {
        *self == EN1KHZR::_1_KHZ_OUTPUT_ENABLED
    }
}
#[doc = "Possible values of the field `EN32KHZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN32KHZR {
    #[doc = "32 kHz output disabled."]
    _32_KHZ_OUTPUT_DISABL,
    #[doc = "32 kHz output enabled."]
    _32_KHZ_OUTPUT_ENABLE,
}
impl EN32KHZR {
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
            EN32KHZR::_32_KHZ_OUTPUT_DISABL => false,
            EN32KHZR::_32_KHZ_OUTPUT_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN32KHZR {
        match value {
            false => EN32KHZR::_32_KHZ_OUTPUT_DISABL,
            true => EN32KHZR::_32_KHZ_OUTPUT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `_32_KHZ_OUTPUT_DISABL`"]
    #[inline]
    pub fn is_32_khz_output_disabl(&self) -> bool {
        *self == EN32KHZR::_32_KHZ_OUTPUT_DISABL
    }
    #[doc = "Checks if the value of the field is `_32_KHZ_OUTPUT_ENABLE`"]
    #[inline]
    pub fn is_32_khz_output_enable(&self) -> bool {
        *self == EN32KHZR::_32_KHZ_OUTPUT_ENABLE
    }
}
#[doc = "Possible values of the field `RESET32KHZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET32KHZR {
    #[doc = "Clear reset."]
    CLEAR_RESET,
    #[doc = "Reset active."]
    RESET_ACTIVE,
}
impl RESET32KHZR {
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
            RESET32KHZR::CLEAR_RESET => false,
            RESET32KHZR::RESET_ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESET32KHZR {
        match value {
            false => RESET32KHZR::CLEAR_RESET,
            true => RESET32KHZR::RESET_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline]
    pub fn is_clear_reset(&self) -> bool {
        *self == RESET32KHZR::CLEAR_RESET
    }
    #[doc = "Checks if the value of the field is `RESET_ACTIVE`"]
    #[inline]
    pub fn is_reset_active(&self) -> bool {
        *self == RESET32KHZR::RESET_ACTIVE
    }
}
#[doc = "Possible values of the field `PD32KHZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD32KHZR {
    #[doc = "Powered."]
    POWERED,
    #[doc = "Powered-down."]
    POWERED_DOWN,
}
impl PD32KHZR {
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
            PD32KHZR::POWERED => false,
            PD32KHZR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PD32KHZR {
        match value {
            false => PD32KHZR::POWERED,
            true => PD32KHZR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == PD32KHZR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == PD32KHZR::POWERED_DOWN
    }
}
#[doc = "Possible values of the field `USB0PHY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0PHYR {
    #[doc = "Enable USB0 PHY power."]
    ENABLE_USB0_PHY_POWE,
    #[doc = "Disable USB0 PHY. PHY powered down."]
    DISABLE_USB0_PHY,
}
impl USB0PHYR {
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
            USB0PHYR::ENABLE_USB0_PHY_POWE => false,
            USB0PHYR::DISABLE_USB0_PHY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0PHYR {
        match value {
            false => USB0PHYR::ENABLE_USB0_PHY_POWE,
            true => USB0PHYR::DISABLE_USB0_PHY,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_USB0_PHY_POWE`"]
    #[inline]
    pub fn is_enable_usb0_phy_powe(&self) -> bool {
        *self == USB0PHYR::ENABLE_USB0_PHY_POWE
    }
    #[doc = "Checks if the value of the field is `DISABLE_USB0_PHY`"]
    #[inline]
    pub fn is_disable_usb0_phy(&self) -> bool {
        *self == USB0PHYR::DISABLE_USB0_PHY
    }
}
#[doc = "Possible values of the field `ALARMCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARMCTRLR {
    #[doc = "RTC alarm."]
    RTC_ALARM,
    #[doc = "Event router event."]
    EVENT_ROUTER_EVENT,
    #[doc = "Inactive."]
    INACTIVE,
}
impl ALARMCTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALARMCTRLR::RTC_ALARM => 0,
            ALARMCTRLR::EVENT_ROUTER_EVENT => 1,
            ALARMCTRLR::INACTIVE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALARMCTRLR {
        match value {
            0 => ALARMCTRLR::RTC_ALARM,
            1 => ALARMCTRLR::EVENT_ROUTER_EVENT,
            3 => ALARMCTRLR::INACTIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM`"]
    #[inline]
    pub fn is_rtc_alarm(&self) -> bool {
        *self == ALARMCTRLR::RTC_ALARM
    }
    #[doc = "Checks if the value of the field is `EVENT_ROUTER_EVENT`"]
    #[inline]
    pub fn is_event_router_event(&self) -> bool {
        *self == ALARMCTRLR::EVENT_ROUTER_EVENT
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == ALARMCTRLR::INACTIVE
    }
}
#[doc = "Possible values of the field `BODLVL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODLVL1R {
    #[doc = "Level 0 interrupt"]
    LEVEL_0_INTERRUPT,
    #[doc = "Level 1 interrupt"]
    LEVEL_1_INTERRUPT,
    #[doc = "Level 2 interrupt"]
    LEVEL_2_INTERRUPT,
    #[doc = "Level 3 interrupt"]
    LEVEL_3_INTERRUPT,
}
impl BODLVL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BODLVL1R::LEVEL_0_INTERRUPT => 0,
            BODLVL1R::LEVEL_1_INTERRUPT => 1,
            BODLVL1R::LEVEL_2_INTERRUPT => 2,
            BODLVL1R::LEVEL_3_INTERRUPT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BODLVL1R {
        match value {
            0 => BODLVL1R::LEVEL_0_INTERRUPT,
            1 => BODLVL1R::LEVEL_1_INTERRUPT,
            2 => BODLVL1R::LEVEL_2_INTERRUPT,
            3 => BODLVL1R::LEVEL_3_INTERRUPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_0_INTERRUPT`"]
    #[inline]
    pub fn is_level_0_interrupt(&self) -> bool {
        *self == BODLVL1R::LEVEL_0_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `LEVEL_1_INTERRUPT`"]
    #[inline]
    pub fn is_level_1_interrupt(&self) -> bool {
        *self == BODLVL1R::LEVEL_1_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `LEVEL_2_INTERRUPT`"]
    #[inline]
    pub fn is_level_2_interrupt(&self) -> bool {
        *self == BODLVL1R::LEVEL_2_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `LEVEL_3_INTERRUPT`"]
    #[inline]
    pub fn is_level_3_interrupt(&self) -> bool {
        *self == BODLVL1R::LEVEL_3_INTERRUPT
    }
}
#[doc = "Possible values of the field `BODLVL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODLVL2R {
    #[doc = "Level 0 reset"]
    LEVEL_0_RESET,
    #[doc = "Level 1 reset"]
    LEVEL_1_RESET,
    #[doc = "Level 2 reset"]
    LEVEL_2_RESET,
    #[doc = "Level 3 reset"]
    LEVEL_3_RESET,
}
impl BODLVL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BODLVL2R::LEVEL_0_RESET => 0,
            BODLVL2R::LEVEL_1_RESET => 1,
            BODLVL2R::LEVEL_2_RESET => 2,
            BODLVL2R::LEVEL_3_RESET => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BODLVL2R {
        match value {
            0 => BODLVL2R::LEVEL_0_RESET,
            1 => BODLVL2R::LEVEL_1_RESET,
            2 => BODLVL2R::LEVEL_2_RESET,
            3 => BODLVL2R::LEVEL_3_RESET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_0_RESET`"]
    #[inline]
    pub fn is_level_0_reset(&self) -> bool {
        *self == BODLVL2R::LEVEL_0_RESET
    }
    #[doc = "Checks if the value of the field is `LEVEL_1_RESET`"]
    #[inline]
    pub fn is_level_1_reset(&self) -> bool {
        *self == BODLVL2R::LEVEL_1_RESET
    }
    #[doc = "Checks if the value of the field is `LEVEL_2_RESET`"]
    #[inline]
    pub fn is_level_2_reset(&self) -> bool {
        *self == BODLVL2R::LEVEL_2_RESET
    }
    #[doc = "Checks if the value of the field is `LEVEL_3_RESET`"]
    #[inline]
    pub fn is_level_3_reset(&self) -> bool {
        *self == BODLVL2R::LEVEL_3_RESET
    }
}
#[doc = "Possible values of the field `SAMPLECTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLECTRLR {
    #[doc = "Sample output from the event monitor/recorder."]
    SAMPLE_OUTPUT_FROM_T,
    #[doc = "Output from the event router."]
    OUTPUT_FROM_THE_EVEN,
}
impl SAMPLECTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAMPLECTRLR::SAMPLE_OUTPUT_FROM_T => 1,
            SAMPLECTRLR::OUTPUT_FROM_THE_EVEN => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAMPLECTRLR {
        match value {
            1 => SAMPLECTRLR::SAMPLE_OUTPUT_FROM_T,
            2 => SAMPLECTRLR::OUTPUT_FROM_THE_EVEN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE_OUTPUT_FROM_T`"]
    #[inline]
    pub fn is_sample_output_from_t(&self) -> bool {
        *self == SAMPLECTRLR::SAMPLE_OUTPUT_FROM_T
    }
    #[doc = "Checks if the value of the field is `OUTPUT_FROM_THE_EVEN`"]
    #[inline]
    pub fn is_output_from_the_even(&self) -> bool {
        *self == SAMPLECTRLR::OUTPUT_FROM_THE_EVEN
    }
}
#[doc = "Possible values of the field `WAKEUP0CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP0CTRLR {
    #[doc = "Input to the event router."]
    INPUT_TO_THE_EVENT_R_1,
    #[doc = "Output from the event router."]
    OUTPUT_FROM_THE_EVEN,
    #[doc = "Input to the event router."]
    INPUT_TO_THE_EVENT_R_2,
}
impl WAKEUP0CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAKEUP0CTRLR::INPUT_TO_THE_EVENT_R_1 => 0,
            WAKEUP0CTRLR::OUTPUT_FROM_THE_EVEN => 1,
            WAKEUP0CTRLR::INPUT_TO_THE_EVENT_R_2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAKEUP0CTRLR {
        match value {
            0 => WAKEUP0CTRLR::INPUT_TO_THE_EVENT_R_1,
            1 => WAKEUP0CTRLR::OUTPUT_FROM_THE_EVEN,
            3 => WAKEUP0CTRLR::INPUT_TO_THE_EVENT_R_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_TO_THE_EVENT_R_1`"]
    #[inline]
    pub fn is_input_to_the_event_r_1(&self) -> bool {
        *self == WAKEUP0CTRLR::INPUT_TO_THE_EVENT_R_1
    }
    #[doc = "Checks if the value of the field is `OUTPUT_FROM_THE_EVEN`"]
    #[inline]
    pub fn is_output_from_the_even(&self) -> bool {
        *self == WAKEUP0CTRLR::OUTPUT_FROM_THE_EVEN
    }
    #[doc = "Checks if the value of the field is `INPUT_TO_THE_EVENT_R_2`"]
    #[inline]
    pub fn is_input_to_the_event_r_2(&self) -> bool {
        *self == WAKEUP0CTRLR::INPUT_TO_THE_EVENT_R_2
    }
}
#[doc = "Possible values of the field `WAKEUP1CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP1CTRLR {
    #[doc = "Input to event router."]
    INPUT_TO_EVENT_ROUTE_1,
    #[doc = "Output from the event router."]
    OUTPUT_FROM_THE_EVEN,
    #[doc = "Input to event router."]
    INPUT_TO_EVENT_ROUTE_2,
}
impl WAKEUP1CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAKEUP1CTRLR::INPUT_TO_EVENT_ROUTE_1 => 0,
            WAKEUP1CTRLR::OUTPUT_FROM_THE_EVEN => 1,
            WAKEUP1CTRLR::INPUT_TO_EVENT_ROUTE_2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAKEUP1CTRLR {
        match value {
            0 => WAKEUP1CTRLR::INPUT_TO_EVENT_ROUTE_1,
            1 => WAKEUP1CTRLR::OUTPUT_FROM_THE_EVEN,
            3 => WAKEUP1CTRLR::INPUT_TO_EVENT_ROUTE_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_TO_EVENT_ROUTE_1`"]
    #[inline]
    pub fn is_input_to_event_route_1(&self) -> bool {
        *self == WAKEUP1CTRLR::INPUT_TO_EVENT_ROUTE_1
    }
    #[doc = "Checks if the value of the field is `OUTPUT_FROM_THE_EVEN`"]
    #[inline]
    pub fn is_output_from_the_even(&self) -> bool {
        *self == WAKEUP1CTRLR::OUTPUT_FROM_THE_EVEN
    }
    #[doc = "Checks if the value of the field is `INPUT_TO_EVENT_ROUTE_2`"]
    #[inline]
    pub fn is_input_to_event_route_2(&self) -> bool {
        *self == WAKEUP1CTRLR::INPUT_TO_EVENT_ROUTE_2
    }
}
#[doc = "Values that can be written to the field `EN1KHZ`"]
pub enum EN1KHZW {
    #[doc = "1 kHz output disabled."]
    _1_KHZ_OUTPUT_DISABLE,
    #[doc = "1 kHz output enabled."]
    _1_KHZ_OUTPUT_ENABLED,
}
impl EN1KHZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN1KHZW::_1_KHZ_OUTPUT_DISABLE => false,
            EN1KHZW::_1_KHZ_OUTPUT_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN1KHZW<'a> {
    w: &'a mut W,
}
impl<'a> _EN1KHZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN1KHZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1 kHz output disabled."]
    #[inline]
    pub fn _1_khz_output_disable(self) -> &'a mut W {
        self.variant(EN1KHZW::_1_KHZ_OUTPUT_DISABLE)
    }
    #[doc = "1 kHz output enabled."]
    #[inline]
    pub fn _1_khz_output_enabled(self) -> &'a mut W {
        self.variant(EN1KHZW::_1_KHZ_OUTPUT_ENABLED)
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
#[doc = "Values that can be written to the field `EN32KHZ`"]
pub enum EN32KHZW {
    #[doc = "32 kHz output disabled."]
    _32_KHZ_OUTPUT_DISABL,
    #[doc = "32 kHz output enabled."]
    _32_KHZ_OUTPUT_ENABLE,
}
impl EN32KHZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN32KHZW::_32_KHZ_OUTPUT_DISABL => false,
            EN32KHZW::_32_KHZ_OUTPUT_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN32KHZW<'a> {
    w: &'a mut W,
}
impl<'a> _EN32KHZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN32KHZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "32 kHz output disabled."]
    #[inline]
    pub fn _32_khz_output_disabl(self) -> &'a mut W {
        self.variant(EN32KHZW::_32_KHZ_OUTPUT_DISABL)
    }
    #[doc = "32 kHz output enabled."]
    #[inline]
    pub fn _32_khz_output_enable(self) -> &'a mut W {
        self.variant(EN32KHZW::_32_KHZ_OUTPUT_ENABLE)
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
#[doc = "Values that can be written to the field `RESET32KHZ`"]
pub enum RESET32KHZW {
    #[doc = "Clear reset."]
    CLEAR_RESET,
    #[doc = "Reset active."]
    RESET_ACTIVE,
}
impl RESET32KHZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESET32KHZW::CLEAR_RESET => false,
            RESET32KHZW::RESET_ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESET32KHZW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET32KHZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESET32KHZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear reset."]
    #[inline]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(RESET32KHZW::CLEAR_RESET)
    }
    #[doc = "Reset active."]
    #[inline]
    pub fn reset_active(self) -> &'a mut W {
        self.variant(RESET32KHZW::RESET_ACTIVE)
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
#[doc = "Values that can be written to the field `PD32KHZ`"]
pub enum PD32KHZW {
    #[doc = "Powered."]
    POWERED,
    #[doc = "Powered-down."]
    POWERED_DOWN,
}
impl PD32KHZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PD32KHZW::POWERED => false,
            PD32KHZW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD32KHZW<'a> {
    w: &'a mut W,
}
impl<'a> _PD32KHZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD32KHZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered."]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(PD32KHZW::POWERED)
    }
    #[doc = "Powered-down."]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(PD32KHZW::POWERED_DOWN)
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
#[doc = "Values that can be written to the field `USB0PHY`"]
pub enum USB0PHYW {
    #[doc = "Enable USB0 PHY power."]
    ENABLE_USB0_PHY_POWE,
    #[doc = "Disable USB0 PHY. PHY powered down."]
    DISABLE_USB0_PHY,
}
impl USB0PHYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0PHYW::ENABLE_USB0_PHY_POWE => false,
            USB0PHYW::DISABLE_USB0_PHY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0PHYW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0PHYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0PHYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable USB0 PHY power."]
    #[inline]
    pub fn enable_usb0_phy_powe(self) -> &'a mut W {
        self.variant(USB0PHYW::ENABLE_USB0_PHY_POWE)
    }
    #[doc = "Disable USB0 PHY. PHY powered down."]
    #[inline]
    pub fn disable_usb0_phy(self) -> &'a mut W {
        self.variant(USB0PHYW::DISABLE_USB0_PHY)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALARMCTRL`"]
pub enum ALARMCTRLW {
    #[doc = "RTC alarm."]
    RTC_ALARM,
    #[doc = "Event router event."]
    EVENT_ROUTER_EVENT,
    #[doc = "Inactive."]
    INACTIVE,
}
impl ALARMCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ALARMCTRLW::RTC_ALARM => 0,
            ALARMCTRLW::EVENT_ROUTER_EVENT => 1,
            ALARMCTRLW::INACTIVE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALARMCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _ALARMCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALARMCTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "RTC alarm."]
    #[inline]
    pub fn rtc_alarm(self) -> &'a mut W {
        self.variant(ALARMCTRLW::RTC_ALARM)
    }
    #[doc = "Event router event."]
    #[inline]
    pub fn event_router_event(self) -> &'a mut W {
        self.variant(ALARMCTRLW::EVENT_ROUTER_EVENT)
    }
    #[doc = "Inactive."]
    #[inline]
    pub fn inactive(self) -> &'a mut W {
        self.variant(ALARMCTRLW::INACTIVE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BODLVL1`"]
pub enum BODLVL1W {
    #[doc = "Level 0 interrupt"]
    LEVEL_0_INTERRUPT,
    #[doc = "Level 1 interrupt"]
    LEVEL_1_INTERRUPT,
    #[doc = "Level 2 interrupt"]
    LEVEL_2_INTERRUPT,
    #[doc = "Level 3 interrupt"]
    LEVEL_3_INTERRUPT,
}
impl BODLVL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BODLVL1W::LEVEL_0_INTERRUPT => 0,
            BODLVL1W::LEVEL_1_INTERRUPT => 1,
            BODLVL1W::LEVEL_2_INTERRUPT => 2,
            BODLVL1W::LEVEL_3_INTERRUPT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODLVL1W<'a> {
    w: &'a mut W,
}
impl<'a> _BODLVL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODLVL1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level 0 interrupt"]
    #[inline]
    pub fn level_0_interrupt(self) -> &'a mut W {
        self.variant(BODLVL1W::LEVEL_0_INTERRUPT)
    }
    #[doc = "Level 1 interrupt"]
    #[inline]
    pub fn level_1_interrupt(self) -> &'a mut W {
        self.variant(BODLVL1W::LEVEL_1_INTERRUPT)
    }
    #[doc = "Level 2 interrupt"]
    #[inline]
    pub fn level_2_interrupt(self) -> &'a mut W {
        self.variant(BODLVL1W::LEVEL_2_INTERRUPT)
    }
    #[doc = "Level 3 interrupt"]
    #[inline]
    pub fn level_3_interrupt(self) -> &'a mut W {
        self.variant(BODLVL1W::LEVEL_3_INTERRUPT)
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
#[doc = "Values that can be written to the field `BODLVL2`"]
pub enum BODLVL2W {
    #[doc = "Level 0 reset"]
    LEVEL_0_RESET,
    #[doc = "Level 1 reset"]
    LEVEL_1_RESET,
    #[doc = "Level 2 reset"]
    LEVEL_2_RESET,
    #[doc = "Level 3 reset"]
    LEVEL_3_RESET,
}
impl BODLVL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BODLVL2W::LEVEL_0_RESET => 0,
            BODLVL2W::LEVEL_1_RESET => 1,
            BODLVL2W::LEVEL_2_RESET => 2,
            BODLVL2W::LEVEL_3_RESET => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODLVL2W<'a> {
    w: &'a mut W,
}
impl<'a> _BODLVL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODLVL2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level 0 reset"]
    #[inline]
    pub fn level_0_reset(self) -> &'a mut W {
        self.variant(BODLVL2W::LEVEL_0_RESET)
    }
    #[doc = "Level 1 reset"]
    #[inline]
    pub fn level_1_reset(self) -> &'a mut W {
        self.variant(BODLVL2W::LEVEL_1_RESET)
    }
    #[doc = "Level 2 reset"]
    #[inline]
    pub fn level_2_reset(self) -> &'a mut W {
        self.variant(BODLVL2W::LEVEL_2_RESET)
    }
    #[doc = "Level 3 reset"]
    #[inline]
    pub fn level_3_reset(self) -> &'a mut W {
        self.variant(BODLVL2W::LEVEL_3_RESET)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SAMPLECTRL`"]
pub enum SAMPLECTRLW {
    #[doc = "Sample output from the event monitor/recorder."]
    SAMPLE_OUTPUT_FROM_T,
    #[doc = "Output from the event router."]
    OUTPUT_FROM_THE_EVEN,
}
impl SAMPLECTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAMPLECTRLW::SAMPLE_OUTPUT_FROM_T => 1,
            SAMPLECTRLW::OUTPUT_FROM_THE_EVEN => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMPLECTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLECTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMPLECTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Sample output from the event monitor/recorder."]
    #[inline]
    pub fn sample_output_from_t(self) -> &'a mut W {
        self.variant(SAMPLECTRLW::SAMPLE_OUTPUT_FROM_T)
    }
    #[doc = "Output from the event router."]
    #[inline]
    pub fn output_from_the_even(self) -> &'a mut W {
        self.variant(SAMPLECTRLW::OUTPUT_FROM_THE_EVEN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAKEUP0CTRL`"]
pub enum WAKEUP0CTRLW {
    #[doc = "Input to the event router."]
    INPUT_TO_THE_EVENT_R_1,
    #[doc = "Output from the event router."]
    OUTPUT_FROM_THE_EVEN,
    #[doc = "Input to the event router."]
    INPUT_TO_THE_EVENT_R_2,
}
impl WAKEUP0CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAKEUP0CTRLW::INPUT_TO_THE_EVENT_R_1 => 0,
            WAKEUP0CTRLW::OUTPUT_FROM_THE_EVEN => 1,
            WAKEUP0CTRLW::INPUT_TO_THE_EVENT_R_2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP0CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP0CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP0CTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input to the event router."]
    #[inline]
    pub fn input_to_the_event_r_1(self) -> &'a mut W {
        self.variant(WAKEUP0CTRLW::INPUT_TO_THE_EVENT_R_1)
    }
    #[doc = "Output from the event router."]
    #[inline]
    pub fn output_from_the_even(self) -> &'a mut W {
        self.variant(WAKEUP0CTRLW::OUTPUT_FROM_THE_EVEN)
    }
    #[doc = "Input to the event router."]
    #[inline]
    pub fn input_to_the_event_r_2(self) -> &'a mut W {
        self.variant(WAKEUP0CTRLW::INPUT_TO_THE_EVENT_R_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAKEUP1CTRL`"]
pub enum WAKEUP1CTRLW {
    #[doc = "Input to event router."]
    INPUT_TO_EVENT_ROUTE_1,
    #[doc = "Output from the event router."]
    OUTPUT_FROM_THE_EVEN,
    #[doc = "Input to event router."]
    INPUT_TO_EVENT_ROUTE_2,
}
impl WAKEUP1CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAKEUP1CTRLW::INPUT_TO_EVENT_ROUTE_1 => 0,
            WAKEUP1CTRLW::OUTPUT_FROM_THE_EVEN => 1,
            WAKEUP1CTRLW::INPUT_TO_EVENT_ROUTE_2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP1CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP1CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP1CTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input to event router."]
    #[inline]
    pub fn input_to_event_route_1(self) -> &'a mut W {
        self.variant(WAKEUP1CTRLW::INPUT_TO_EVENT_ROUTE_1)
    }
    #[doc = "Output from the event router."]
    #[inline]
    pub fn output_from_the_even(self) -> &'a mut W {
        self.variant(WAKEUP1CTRLW::OUTPUT_FROM_THE_EVEN)
    }
    #[doc = "Input to event router."]
    #[inline]
    pub fn input_to_event_route_2(self) -> &'a mut W {
        self.variant(WAKEUP1CTRLW::INPUT_TO_EVENT_ROUTE_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Enable 1 kHz output."]
    #[inline]
    pub fn en1khz(&self) -> EN1KHZR {
        EN1KHZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable 32 kHz output"]
    #[inline]
    pub fn en32khz(&self) -> EN32KHZR {
        EN32KHZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - 32 kHz oscillator reset"]
    #[inline]
    pub fn reset32khz(&self) -> RESET32KHZR {
        RESET32KHZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - 32 kHz power control."]
    #[inline]
    pub fn pd32khz(&self) -> PD32KHZR {
        PD32KHZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - USB0 PHY power control."]
    #[inline]
    pub fn usb0phy(&self) -> USB0PHYR {
        USB0PHYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - RTC_ALARM pin output control"]
    #[inline]
    pub fn alarmctrl(&self) -> ALARMCTRLR {
        ALARMCTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - BOD trip level to generate an interrupt. See the LPC43xx data sheets for the trip values."]
    #[inline]
    pub fn bodlvl1(&self) -> BODLVL1R {
        BODLVL1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - BOD trip level to generate a reset. See the LPC43xx data sheets for the trip values."]
    #[inline]
    pub fn bodlvl2(&self) -> BODLVL2R {
        BODLVL2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - SAMPLE pin input/output control"]
    #[inline]
    pub fn samplectrl(&self) -> SAMPLECTRLR {
        SAMPLECTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - WAKEUP0 pin input/output control"]
    #[inline]
    pub fn wakeup0ctrl(&self) -> WAKEUP0CTRLR {
        WAKEUP0CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - WAKEUP1 pin input/output control"]
    #[inline]
    pub fn wakeup1ctrl(&self) -> WAKEUP1CTRLR {
        WAKEUP1CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Enable 1 kHz output."]
    #[inline]
    pub fn en1khz(&mut self) -> _EN1KHZW {
        _EN1KHZW { w: self }
    }
    #[doc = "Bit 1 - Enable 32 kHz output"]
    #[inline]
    pub fn en32khz(&mut self) -> _EN32KHZW {
        _EN32KHZW { w: self }
    }
    #[doc = "Bit 2 - 32 kHz oscillator reset"]
    #[inline]
    pub fn reset32khz(&mut self) -> _RESET32KHZW {
        _RESET32KHZW { w: self }
    }
    #[doc = "Bit 3 - 32 kHz power control."]
    #[inline]
    pub fn pd32khz(&mut self) -> _PD32KHZW {
        _PD32KHZW { w: self }
    }
    #[doc = "Bit 5 - USB0 PHY power control."]
    #[inline]
    pub fn usb0phy(&mut self) -> _USB0PHYW {
        _USB0PHYW { w: self }
    }
    #[doc = "Bits 6:7 - RTC_ALARM pin output control"]
    #[inline]
    pub fn alarmctrl(&mut self) -> _ALARMCTRLW {
        _ALARMCTRLW { w: self }
    }
    #[doc = "Bits 8:9 - BOD trip level to generate an interrupt. See the LPC43xx data sheets for the trip values."]
    #[inline]
    pub fn bodlvl1(&mut self) -> _BODLVL1W {
        _BODLVL1W { w: self }
    }
    #[doc = "Bits 10:11 - BOD trip level to generate a reset. See the LPC43xx data sheets for the trip values."]
    #[inline]
    pub fn bodlvl2(&mut self) -> _BODLVL2W {
        _BODLVL2W { w: self }
    }
    #[doc = "Bits 12:13 - SAMPLE pin input/output control"]
    #[inline]
    pub fn samplectrl(&mut self) -> _SAMPLECTRLW {
        _SAMPLECTRLW { w: self }
    }
    #[doc = "Bits 14:15 - WAKEUP0 pin input/output control"]
    #[inline]
    pub fn wakeup0ctrl(&mut self) -> _WAKEUP0CTRLW {
        _WAKEUP0CTRLW { w: self }
    }
    #[doc = "Bits 16:17 - WAKEUP1 pin input/output control"]
    #[inline]
    pub fn wakeup1ctrl(&mut self) -> _WAKEUP1CTRLW {
        _WAKEUP1CTRLW { w: self }
    }
}
