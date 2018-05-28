#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBSTS_H {
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
#[doc = "Possible values of the field `UI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIR {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "This bit is set by the Host/Device Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set. This bit is also set by the Host/Device Controller when a short packet is detected. A short packet is when the actual number of bytes received was less than the expected number of bytes."]
    CLEAR,
}
impl UIR {
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
            UIR::ST => false,
            UIR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UIR {
        match value {
            false => UIR::ST,
            true => UIR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ST`"]
    #[inline]
    pub fn is_st(&self) -> bool {
        *self == UIR::ST
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == UIR::CLEAR
    }
}
#[doc = "Possible values of the field `UEI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UEIR {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set."]
    CLEAR,
}
impl UEIR {
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
            UEIR::ST => false,
            UEIR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UEIR {
        match value {
            false => UEIR::ST,
            true => UEIR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ST`"]
    #[inline]
    pub fn is_st(&self) -> bool {
        *self == UEIR::ST
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == UEIR::CLEAR
    }
}
#[doc = "Possible values of the field `PCI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCIR {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "The Host Controller sets this bit to a one when on any port a Connect Status occurs, a Port Enable/Disable Change occurs, or the Force Port Resume bit is set as the result of a J-K transition on the suspended port."]
    CLEAR,
}
impl PCIR {
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
            PCIR::ST => false,
            PCIR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCIR {
        match value {
            false => PCIR::ST,
            true => PCIR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ST`"]
    #[inline]
    pub fn is_st(&self) -> bool {
        *self == PCIR::ST
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == PCIR::CLEAR
    }
}
#[doc = "Possible values of the field `FRI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRIR {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "The Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example, if the frame list size (as programmed in the Frame List Size field of the USBCMD register) is 1024, the Frame Index Register rolls over every time FRINDEX bit 13 toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FRINDEX bit 12 toggles (see Section 18.6.6)."]
    CLEAR,
}
impl FRIR {
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
            FRIR::ST => false,
            FRIR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRIR {
        match value {
            false => FRIR::ST,
            true => FRIR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ST`"]
    #[inline]
    pub fn is_st(&self) -> bool {
        *self == FRIR::ST
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == FRIR::CLEAR
    }
}
#[doc = "Possible values of the field `AAI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AAIR {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "System software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the USBCMD register. This status bit indicates the assertion of that interrupt source."]
    CLEAR,
}
impl AAIR {
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
            AAIR::ST => false,
            AAIR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AAIR {
        match value {
            false => AAIR::ST,
            true => AAIR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ST`"]
    #[inline]
    pub fn is_st(&self) -> bool {
        *self == AAIR::ST
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == AAIR::CLEAR
    }
}
#[doc = "Possible values of the field `SRI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRIR {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "In host mode, this bit will be set every 125 ms and can be used by host controller driver as a time base."]
    CLEAR,
}
impl SRIR {
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
            SRIR::ST => false,
            SRIR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRIR {
        match value {
            false => SRIR::ST,
            true => SRIR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ST`"]
    #[inline]
    pub fn is_st(&self) -> bool {
        *self == SRIR::ST
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == SRIR::CLEAR
    }
}
#[doc = "Possible values of the field `HCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCHR {
    #[doc = "The RS bit in USBCMD is set to zero. Set by the host controller."]
    RS,
    #[doc = "The Host Controller sets this bit to one after it has stopped executing because of the Run/Stop bit being set to 0, either by software or by the Host Controller hardware (e.g. because of an internal error)."]
    HALT,
}
impl HCHR {
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
            HCHR::RS => false,
            HCHR::HALT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HCHR {
        match value {
            false => HCHR::RS,
            true => HCHR::HALT,
        }
    }
    #[doc = "Checks if the value of the field is `RS`"]
    #[inline]
    pub fn is_rs(&self) -> bool {
        *self == HCHR::RS
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline]
    pub fn is_halt(&self) -> bool {
        *self == HCHR::HALT
    }
}
#[doc = "Possible values of the field `RCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCLR {
    #[doc = "No empty asynchronous schedule detected."]
    NO_EMPTY_ASYNCHRONOU,
    #[doc = "An empty asynchronous schedule is detected. Set by the host controller."]
    EMPTY_ASYNCHRONOU,
}
impl RCLR {
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
            RCLR::NO_EMPTY_ASYNCHRONOU => false,
            RCLR::EMPTY_ASYNCHRONOU => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCLR {
        match value {
            false => RCLR::NO_EMPTY_ASYNCHRONOU,
            true => RCLR::EMPTY_ASYNCHRONOU,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EMPTY_ASYNCHRONOU`"]
    #[inline]
    pub fn is_no_empty_asynchronou(&self) -> bool {
        *self == RCLR::NO_EMPTY_ASYNCHRONOU
    }
    #[doc = "Checks if the value of the field is `EMPTY_ASYNCHRONOU`"]
    #[inline]
    pub fn is_empty_asynchronou(&self) -> bool {
        *self == RCLR::EMPTY_ASYNCHRONOU
    }
}
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "The periodic schedule status is disabled."]
    DISABLED,
    #[doc = "The periodic schedule status is enabled."]
    DISABLED,
}
impl PSR {
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
            PSR::DISABLED => false,
            PSR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSR {
        match value {
            false => PSR::DISABLED,
            true => PSR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PSR::DISABLED
    }
}
#[doc = "Possible values of the field `AS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASR {
    #[doc = "Asynchronous schedule status is disabled."]
    DISABLED,
    #[doc = "Asynchronous schedule status is enabled."]
    DISABLED,
}
impl ASR {
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
            ASR::DISABLED => false,
            ASR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASR {
        match value {
            false => ASR::DISABLED,
            true => ASR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ASR::DISABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ASR::DISABLED
    }
}
#[doc = "Possible values of the field `UAI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UAIR {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the asynchronous schedule. This bit is also set by the Host when a short packet is detected and the packet is on the asynchronous schedule. A short packet is when the actual number of bytes received was less than the expected number of bytes."]
    CLEAR,
}
impl UAIR {
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
            UAIR::ST => false,
            UAIR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UAIR {
        match value {
            false => UAIR::ST,
            true => UAIR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ST`"]
    #[inline]
    pub fn is_st(&self) -> bool {
        *self == UAIR::ST
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == UAIR::CLEAR
    }
}
#[doc = "Possible values of the field `UPI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPIR {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the periodic schedule. This bit is also set by the Host Controller when a short packet is detected and the packet is on the periodic schedule. A short packet is when the actual number of bytes received was less than the expected number of bytes."]
    CLEAR,
}
impl UPIR {
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
            UPIR::ST => false,
            UPIR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UPIR {
        match value {
            false => UPIR::ST,
            true => UPIR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ST`"]
    #[inline]
    pub fn is_st(&self) -> bool {
        *self == UPIR::ST
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == UPIR::CLEAR
    }
}
#[doc = "Values that can be written to the field `UI`"]
pub enum UIW {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "This bit is set by the Host/Device Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set. This bit is also set by the Host/Device Controller when a short packet is detected. A short packet is when the actual number of bytes received was less than the expected number of bytes."]
    CLEAR,
}
impl UIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UIW::ST => false,
            UIW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UIW<'a> {
    w: &'a mut W,
}
impl<'a> _UIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is cleared by software writing a one to it."]
    #[inline]
    pub fn st(self) -> &'a mut W {
        self.variant(UIW::ST)
    }
    #[doc = "This bit is set by the Host/Device Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set. This bit is also set by the Host/Device Controller when a short packet is detected. A short packet is when the actual number of bytes received was less than the expected number of bytes."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(UIW::CLEAR)
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
#[doc = "Values that can be written to the field `UEI`"]
pub enum UEIW {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set."]
    CLEAR,
}
impl UEIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UEIW::ST => false,
            UEIW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UEIW<'a> {
    w: &'a mut W,
}
impl<'a> _UEIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UEIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is cleared by software writing a one to it."]
    #[inline]
    pub fn st(self) -> &'a mut W {
        self.variant(UEIW::ST)
    }
    #[doc = "When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(UEIW::CLEAR)
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
#[doc = "Values that can be written to the field `PCI`"]
pub enum PCIW {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "The Host Controller sets this bit to a one when on any port a Connect Status occurs, a Port Enable/Disable Change occurs, or the Force Port Resume bit is set as the result of a J-K transition on the suspended port."]
    CLEAR,
}
impl PCIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCIW::ST => false,
            PCIW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCIW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is cleared by software writing a one to it."]
    #[inline]
    pub fn st(self) -> &'a mut W {
        self.variant(PCIW::ST)
    }
    #[doc = "The Host Controller sets this bit to a one when on any port a Connect Status occurs, a Port Enable/Disable Change occurs, or the Force Port Resume bit is set as the result of a J-K transition on the suspended port."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(PCIW::CLEAR)
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
#[doc = "Values that can be written to the field `FRI`"]
pub enum FRIW {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "The Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example, if the frame list size (as programmed in the Frame List Size field of the USBCMD register) is 1024, the Frame Index Register rolls over every time FRINDEX bit 13 toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FRINDEX bit 12 toggles (see Section 18.6.6)."]
    CLEAR,
}
impl FRIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRIW::ST => false,
            FRIW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRIW<'a> {
    w: &'a mut W,
}
impl<'a> _FRIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is cleared by software writing a one to it."]
    #[inline]
    pub fn st(self) -> &'a mut W {
        self.variant(FRIW::ST)
    }
    #[doc = "The Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example, if the frame list size (as programmed in the Frame List Size field of the USBCMD register) is 1024, the Frame Index Register rolls over every time FRINDEX bit 13 toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FRINDEX bit 12 toggles (see Section 18.6.6)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(FRIW::CLEAR)
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
#[doc = "Values that can be written to the field `AAI`"]
pub enum AAIW {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "System software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the USBCMD register. This status bit indicates the assertion of that interrupt source."]
    CLEAR,
}
impl AAIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AAIW::ST => false,
            AAIW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AAIW<'a> {
    w: &'a mut W,
}
impl<'a> _AAIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AAIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is cleared by software writing a one to it."]
    #[inline]
    pub fn st(self) -> &'a mut W {
        self.variant(AAIW::ST)
    }
    #[doc = "System software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the USBCMD register. This status bit indicates the assertion of that interrupt source."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(AAIW::CLEAR)
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
#[doc = "Values that can be written to the field `SRI`"]
pub enum SRIW {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "In host mode, this bit will be set every 125 ms and can be used by host controller driver as a time base."]
    CLEAR,
}
impl SRIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRIW::ST => false,
            SRIW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRIW<'a> {
    w: &'a mut W,
}
impl<'a> _SRIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is cleared by software writing a one to it."]
    #[inline]
    pub fn st(self) -> &'a mut W {
        self.variant(SRIW::ST)
    }
    #[doc = "In host mode, this bit will be set every 125 ms and can be used by host controller driver as a time base."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SRIW::CLEAR)
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
#[doc = "Values that can be written to the field `HCH`"]
pub enum HCHW {
    #[doc = "The RS bit in USBCMD is set to zero. Set by the host controller."]
    RS,
    #[doc = "The Host Controller sets this bit to one after it has stopped executing because of the Run/Stop bit being set to 0, either by software or by the Host Controller hardware (e.g. because of an internal error)."]
    HALT,
}
impl HCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HCHW::RS => false,
            HCHW::HALT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HCHW<'a> {
    w: &'a mut W,
}
impl<'a> _HCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HCHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The RS bit in USBCMD is set to zero. Set by the host controller."]
    #[inline]
    pub fn rs(self) -> &'a mut W {
        self.variant(HCHW::RS)
    }
    #[doc = "The Host Controller sets this bit to one after it has stopped executing because of the Run/Stop bit being set to 0, either by software or by the Host Controller hardware (e.g. because of an internal error)."]
    #[inline]
    pub fn halt(self) -> &'a mut W {
        self.variant(HCHW::HALT)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCL`"]
pub enum RCLW {
    #[doc = "No empty asynchronous schedule detected."]
    NO_EMPTY_ASYNCHRONOU,
    #[doc = "An empty asynchronous schedule is detected. Set by the host controller."]
    EMPTY_ASYNCHRONOU,
}
impl RCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCLW::NO_EMPTY_ASYNCHRONOU => false,
            RCLW::EMPTY_ASYNCHRONOU => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCLW<'a> {
    w: &'a mut W,
}
impl<'a> _RCLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No empty asynchronous schedule detected."]
    #[inline]
    pub fn no_empty_asynchronou(self) -> &'a mut W {
        self.variant(RCLW::NO_EMPTY_ASYNCHRONOU)
    }
    #[doc = "An empty asynchronous schedule is detected. Set by the host controller."]
    #[inline]
    pub fn empty_asynchronou(self) -> &'a mut W {
        self.variant(RCLW::EMPTY_ASYNCHRONOU)
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
#[doc = "Values that can be written to the field `PS`"]
pub enum PSW {
    #[doc = "The periodic schedule status is disabled."]
    DISABLED,
    #[doc = "The periodic schedule status is enabled."]
    DISABLED,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSW::DISABLED => false,
            PSW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The periodic schedule status is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PSW::DISABLED)
    }
    #[doc = "The periodic schedule status is enabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PSW::DISABLED)
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
#[doc = "Values that can be written to the field `AS`"]
pub enum ASW {
    #[doc = "Asynchronous schedule status is disabled."]
    DISABLED,
    #[doc = "Asynchronous schedule status is enabled."]
    DISABLED,
}
impl ASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASW::DISABLED => false,
            ASW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASW<'a> {
    w: &'a mut W,
}
impl<'a> _ASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Asynchronous schedule status is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ASW::DISABLED)
    }
    #[doc = "Asynchronous schedule status is enabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ASW::DISABLED)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UAI`"]
pub enum UAIW {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the asynchronous schedule. This bit is also set by the Host when a short packet is detected and the packet is on the asynchronous schedule. A short packet is when the actual number of bytes received was less than the expected number of bytes."]
    CLEAR,
}
impl UAIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UAIW::ST => false,
            UAIW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UAIW<'a> {
    w: &'a mut W,
}
impl<'a> _UAIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UAIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is cleared by software writing a one to it."]
    #[inline]
    pub fn st(self) -> &'a mut W {
        self.variant(UAIW::ST)
    }
    #[doc = "This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the asynchronous schedule. This bit is also set by the Host when a short packet is detected and the packet is on the asynchronous schedule. A short packet is when the actual number of bytes received was less than the expected number of bytes."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(UAIW::CLEAR)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UPI`"]
pub enum UPIW {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the periodic schedule. This bit is also set by the Host Controller when a short packet is detected and the packet is on the periodic schedule. A short packet is when the actual number of bytes received was less than the expected number of bytes."]
    CLEAR,
}
impl UPIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UPIW::ST => false,
            UPIW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPIW<'a> {
    w: &'a mut W,
}
impl<'a> _UPIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is cleared by software writing a one to it."]
    #[inline]
    pub fn st(self) -> &'a mut W {
        self.variant(UPIW::ST)
    }
    #[doc = "This bit is set by the Host Controller when the cause of an interrupt is a completion of a USB transaction where the Transfer Descriptor (TD) has an interrupt on complete (IOC) bit set and the TD was from the periodic schedule. This bit is also set by the Host Controller when a short packet is detected and the packet is on the periodic schedule. A short packet is when the actual number of bytes received was less than the expected number of bytes."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(UPIW::CLEAR)
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
        const OFFSET: u8 = 19;
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
    #[doc = "Bit 0 - USB interrupt (USBINT)"]
    #[inline]
    pub fn ui(&self) -> UIR {
        UIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - USB error interrupt (USBERRINT)"]
    #[inline]
    pub fn uei(&self) -> UEIR {
        UEIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port change detect."]
    #[inline]
    pub fn pci(&self) -> PCIR {
        PCIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Frame list roll-over"]
    #[inline]
    pub fn fri(&self) -> FRIR {
        FRIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Interrupt on async advance"]
    #[inline]
    pub fn aai(&self) -> AAIR {
        AAIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - SOF received"]
    #[inline]
    pub fn sri(&self) -> SRIR {
        SRIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - HCHalted"]
    #[inline]
    pub fn hch(&self) -> HCHR {
        HCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Reclamation"]
    #[inline]
    pub fn rcl(&self) -> RCLR {
        RCLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Periodic schedule status This bit reports the current real status of the Periodic Schedule. The Host Controller is not required to immediately disable or enable the Periodic Schedule when software transitions the Periodic Schedule Enable bit in the USBCMD register. When this bit and the Periodic Schedule Enable bit are the same value, the Periodic Schedule is either enabled (if both are 1) or disabled (if both are 0)."]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Asynchronous schedule status This bit reports the current real status of the Asynchronous Schedule. The Host Controller is not required to immediately disable or enable the Asynchronous Schedule when software transitions the Asynchronous Schedule Enable bit in the USBCMD register. When this bit and the Asynchronous Schedule Enable bit are the same value, the Asynchronous Schedule is either enabled (if both are 1) or disabled (if both are 0)."]
    #[inline]
    pub fn as_(&self) -> ASR {
        ASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - USB host asynchronous interrupt (USBHSTASYNCINT)"]
    #[inline]
    pub fn uai(&self) -> UAIR {
        UAIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - USB host periodic interrupt (USBHSTPERINT)"]
    #[inline]
    pub fn upi(&self) -> UPIR {
        UPIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
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
    #[doc = "Bit 0 - USB interrupt (USBINT)"]
    #[inline]
    pub fn ui(&mut self) -> _UIW {
        _UIW { w: self }
    }
    #[doc = "Bit 1 - USB error interrupt (USBERRINT)"]
    #[inline]
    pub fn uei(&mut self) -> _UEIW {
        _UEIW { w: self }
    }
    #[doc = "Bit 2 - Port change detect."]
    #[inline]
    pub fn pci(&mut self) -> _PCIW {
        _PCIW { w: self }
    }
    #[doc = "Bit 3 - Frame list roll-over"]
    #[inline]
    pub fn fri(&mut self) -> _FRIW {
        _FRIW { w: self }
    }
    #[doc = "Bit 5 - Interrupt on async advance"]
    #[inline]
    pub fn aai(&mut self) -> _AAIW {
        _AAIW { w: self }
    }
    #[doc = "Bit 7 - SOF received"]
    #[inline]
    pub fn sri(&mut self) -> _SRIW {
        _SRIW { w: self }
    }
    #[doc = "Bit 12 - HCHalted"]
    #[inline]
    pub fn hch(&mut self) -> _HCHW {
        _HCHW { w: self }
    }
    #[doc = "Bit 13 - Reclamation"]
    #[inline]
    pub fn rcl(&mut self) -> _RCLW {
        _RCLW { w: self }
    }
    #[doc = "Bit 14 - Periodic schedule status This bit reports the current real status of the Periodic Schedule. The Host Controller is not required to immediately disable or enable the Periodic Schedule when software transitions the Periodic Schedule Enable bit in the USBCMD register. When this bit and the Periodic Schedule Enable bit are the same value, the Periodic Schedule is either enabled (if both are 1) or disabled (if both are 0)."]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bit 15 - Asynchronous schedule status This bit reports the current real status of the Asynchronous Schedule. The Host Controller is not required to immediately disable or enable the Asynchronous Schedule when software transitions the Asynchronous Schedule Enable bit in the USBCMD register. When this bit and the Asynchronous Schedule Enable bit are the same value, the Asynchronous Schedule is either enabled (if both are 1) or disabled (if both are 0)."]
    #[inline]
    pub fn as_(&mut self) -> _ASW {
        _ASW { w: self }
    }
    #[doc = "Bit 18 - USB host asynchronous interrupt (USBHSTASYNCINT)"]
    #[inline]
    pub fn uai(&mut self) -> _UAIW {
        _UAIW { w: self }
    }
    #[doc = "Bit 19 - USB host periodic interrupt (USBHSTPERINT)"]
    #[inline]
    pub fn upi(&mut self) -> _UPIW {
        _UPIW { w: self }
    }
}
