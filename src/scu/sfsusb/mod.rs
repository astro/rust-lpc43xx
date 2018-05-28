#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SFSUSB {
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
#[doc = "Possible values of the field `USB_AIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_AIMR {
    #[doc = "Going LOW with full speed edge rate"]
    GOING_LOW_WITH_FULL,
    #[doc = "Going HIGH with full speed edge rate"]
    GOING_HIGH_WITH_FULL,
}
impl USB_AIMR {
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
            USB_AIMR::GOING_LOW_WITH_FULL => false,
            USB_AIMR::GOING_HIGH_WITH_FULL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB_AIMR {
        match value {
            false => USB_AIMR::GOING_LOW_WITH_FULL,
            true => USB_AIMR::GOING_HIGH_WITH_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `GOING_LOW_WITH_FULL`"]
    #[inline]
    pub fn is_going_low_with_full(&self) -> bool {
        *self == USB_AIMR::GOING_LOW_WITH_FULL
    }
    #[doc = "Checks if the value of the field is `GOING_HIGH_WITH_FULL`"]
    #[inline]
    pub fn is_going_high_with_full(&self) -> bool {
        *self == USB_AIMR::GOING_HIGH_WITH_FULL
    }
}
#[doc = "Possible values of the field `USB_ESEA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_ESEAR {
    #[doc = "Single input. Enables USB1. Use with the on-chip full-speed PHY."]
    SINGLE_INPUT,
}
impl USB_ESEAR {
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
            USB_ESEAR::SINGLE_INPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB_ESEAR {
        match value {
            true => USB_ESEAR::SINGLE_INPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_INPUT`"]
    #[inline]
    pub fn is_single_input(&self) -> bool {
        *self == USB_ESEAR::SINGLE_INPUT
    }
}
#[doc = "Possible values of the field `USB_EPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_EPDR {
    #[doc = "Pull-down disconnected"]
    PULL_DOWN_DISCONNECT,
    #[doc = "Pull-down connected"]
    PULL_DOWN_CONNECTED,
}
impl USB_EPDR {
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
            USB_EPDR::PULL_DOWN_DISCONNECT => false,
            USB_EPDR::PULL_DOWN_CONNECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB_EPDR {
        match value {
            false => USB_EPDR::PULL_DOWN_DISCONNECT,
            true => USB_EPDR::PULL_DOWN_CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN_DISCONNECT`"]
    #[inline]
    pub fn is_pull_down_disconnect(&self) -> bool {
        *self == USB_EPDR::PULL_DOWN_DISCONNECT
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN_CONNECTED`"]
    #[inline]
    pub fn is_pull_down_connected(&self) -> bool {
        *self == USB_EPDR::PULL_DOWN_CONNECTED
    }
}
#[doc = "Possible values of the field `USB_EPWR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_EPWRR {
    #[doc = "Power saving mode (Suspend mode)"]
    POWER_SAVING_MODE_S,
    #[doc = "Normal mode"]
    NORMAL_MODE,
}
impl USB_EPWRR {
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
            USB_EPWRR::POWER_SAVING_MODE_S => false,
            USB_EPWRR::NORMAL_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB_EPWRR {
        match value {
            false => USB_EPWRR::POWER_SAVING_MODE_S,
            true => USB_EPWRR::NORMAL_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_SAVING_MODE_S`"]
    #[inline]
    pub fn is_power_saving_mode_s(&self) -> bool {
        *self == USB_EPWRR::POWER_SAVING_MODE_S
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE`"]
    #[inline]
    pub fn is_normal_mode(&self) -> bool {
        *self == USB_EPWRR::NORMAL_MODE
    }
}
#[doc = "Possible values of the field `USB_VBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_VBUSR {
    #[doc = "VBUS signal LOW or inactive"]
    VBUS_SIGNAL_LOW_OR_I,
    #[doc = "VBUS signal HIGH or active"]
    VBUS_SIGNAL_HIGH_OR,
}
impl USB_VBUSR {
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
            USB_VBUSR::VBUS_SIGNAL_LOW_OR_I => false,
            USB_VBUSR::VBUS_SIGNAL_HIGH_OR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB_VBUSR {
        match value {
            false => USB_VBUSR::VBUS_SIGNAL_LOW_OR_I,
            true => USB_VBUSR::VBUS_SIGNAL_HIGH_OR,
        }
    }
    #[doc = "Checks if the value of the field is `VBUS_SIGNAL_LOW_OR_I`"]
    #[inline]
    pub fn is_vbus_signal_low_or_i(&self) -> bool {
        *self == USB_VBUSR::VBUS_SIGNAL_LOW_OR_I
    }
    #[doc = "Checks if the value of the field is `VBUS_SIGNAL_HIGH_OR`"]
    #[inline]
    pub fn is_vbus_signal_high_or(&self) -> bool {
        *self == USB_VBUSR::VBUS_SIGNAL_HIGH_OR
    }
}
#[doc = "Values that can be written to the field `USB_AIM`"]
pub enum USB_AIMW {
    #[doc = "Going LOW with full speed edge rate"]
    GOING_LOW_WITH_FULL,
    #[doc = "Going HIGH with full speed edge rate"]
    GOING_HIGH_WITH_FULL,
}
impl USB_AIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB_AIMW::GOING_LOW_WITH_FULL => false,
            USB_AIMW::GOING_HIGH_WITH_FULL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB_AIMW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_AIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB_AIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Going LOW with full speed edge rate"]
    #[inline]
    pub fn going_low_with_full(self) -> &'a mut W {
        self.variant(USB_AIMW::GOING_LOW_WITH_FULL)
    }
    #[doc = "Going HIGH with full speed edge rate"]
    #[inline]
    pub fn going_high_with_full(self) -> &'a mut W {
        self.variant(USB_AIMW::GOING_HIGH_WITH_FULL)
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
#[doc = "Values that can be written to the field `USB_ESEA`"]
pub enum USB_ESEAW {
    #[doc = "Single input. Enables USB1. Use with the on-chip full-speed PHY."]
    SINGLE_INPUT,
}
impl USB_ESEAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB_ESEAW::SINGLE_INPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB_ESEAW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_ESEAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB_ESEAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single input. Enables USB1. Use with the on-chip full-speed PHY."]
    #[inline]
    pub fn single_input(self) -> &'a mut W {
        self.variant(USB_ESEAW::SINGLE_INPUT)
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
#[doc = "Values that can be written to the field `USB_EPD`"]
pub enum USB_EPDW {
    #[doc = "Pull-down disconnected"]
    PULL_DOWN_DISCONNECT,
    #[doc = "Pull-down connected"]
    PULL_DOWN_CONNECTED,
}
impl USB_EPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB_EPDW::PULL_DOWN_DISCONNECT => false,
            USB_EPDW::PULL_DOWN_CONNECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB_EPDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_EPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB_EPDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pull-down disconnected"]
    #[inline]
    pub fn pull_down_disconnect(self) -> &'a mut W {
        self.variant(USB_EPDW::PULL_DOWN_DISCONNECT)
    }
    #[doc = "Pull-down connected"]
    #[inline]
    pub fn pull_down_connected(self) -> &'a mut W {
        self.variant(USB_EPDW::PULL_DOWN_CONNECTED)
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
#[doc = "Values that can be written to the field `USB_EPWR`"]
pub enum USB_EPWRW {
    #[doc = "Power saving mode (Suspend mode)"]
    POWER_SAVING_MODE_S,
    #[doc = "Normal mode"]
    NORMAL_MODE,
}
impl USB_EPWRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB_EPWRW::POWER_SAVING_MODE_S => false,
            USB_EPWRW::NORMAL_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB_EPWRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_EPWRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB_EPWRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power saving mode (Suspend mode)"]
    #[inline]
    pub fn power_saving_mode_s(self) -> &'a mut W {
        self.variant(USB_EPWRW::POWER_SAVING_MODE_S)
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(USB_EPWRW::NORMAL_MODE)
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
#[doc = "Values that can be written to the field `USB_VBUS`"]
pub enum USB_VBUSW {
    #[doc = "VBUS signal LOW or inactive"]
    VBUS_SIGNAL_LOW_OR_I,
    #[doc = "VBUS signal HIGH or active"]
    VBUS_SIGNAL_HIGH_OR,
}
impl USB_VBUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB_VBUSW::VBUS_SIGNAL_LOW_OR_I => false,
            USB_VBUSW::VBUS_SIGNAL_HIGH_OR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB_VBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_VBUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB_VBUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "VBUS signal LOW or inactive"]
    #[inline]
    pub fn vbus_signal_low_or_i(self) -> &'a mut W {
        self.variant(USB_VBUSW::VBUS_SIGNAL_LOW_OR_I)
    }
    #[doc = "VBUS signal HIGH or active"]
    #[inline]
    pub fn vbus_signal_high_or(self) -> &'a mut W {
        self.variant(USB_VBUSW::VBUS_SIGNAL_HIGH_OR)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Differential data input AIP/AIM."]
    #[inline]
    pub fn usb_aim(&self) -> USB_AIMR {
        USB_AIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Control signal for differential input or single input."]
    #[inline]
    pub fn usb_esea(&self) -> USB_ESEAR {
        USB_ESEAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable pull-down connect."]
    #[inline]
    pub fn usb_epd(&self) -> USB_EPDR {
        USB_EPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Power mode."]
    #[inline]
    pub fn usb_epwr(&self) -> USB_EPWRR {
        USB_EPWRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable the vbus_valid signal. This signal is monitored by the USB1 block. Use this bit for software de-bouncing of the VBUS sense signal or to indicate the VBUS state to the USB1 controller when the VBUS signal is present but the USB1_VBUS function is not connected in the SFSP2_5 register. The setting of this bit has no effect if the USB1_VBUS function of pin P2_5 is enabled through the SFSP2_5 register."]
    #[inline]
    pub fn usb_vbus(&self) -> USB_VBUSR {
        USB_VBUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Differential data input AIP/AIM."]
    #[inline]
    pub fn usb_aim(&mut self) -> _USB_AIMW {
        _USB_AIMW { w: self }
    }
    #[doc = "Bit 1 - Control signal for differential input or single input."]
    #[inline]
    pub fn usb_esea(&mut self) -> _USB_ESEAW {
        _USB_ESEAW { w: self }
    }
    #[doc = "Bit 2 - Enable pull-down connect."]
    #[inline]
    pub fn usb_epd(&mut self) -> _USB_EPDW {
        _USB_EPDW { w: self }
    }
    #[doc = "Bit 4 - Power mode."]
    #[inline]
    pub fn usb_epwr(&mut self) -> _USB_EPWRW {
        _USB_EPWRW { w: self }
    }
    #[doc = "Bit 5 - Enable the vbus_valid signal. This signal is monitored by the USB1 block. Use this bit for software de-bouncing of the VBUS sense signal or to indicate the VBUS state to the USB1 controller when the VBUS signal is present but the USB1_VBUS function is not connected in the SFSP2_5 register. The setting of this bit has no effect if the USB1_VBUS function of pin P2_5 is enabled through the SFSP2_5 register."]
    #[inline]
    pub fn usb_vbus(&mut self) -> _USB_VBUSW {
        _USB_VBUSW { w: self }
    }
}
