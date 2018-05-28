#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBSTS_D {
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
    #[doc = "When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set. The device controller detects resume signaling only (see Section 18.10.11.6)."]
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
    #[doc = "The Device Controller sets this bit to a one when the port controller enters the full or high-speed operational state. When the port controller exits the full or high-speed operation states due to Reset or Suspend events, the notification mechanisms are the USB Reset Received bit (URI) and the DCSuspend bits (SLI) respectively."]
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
#[doc = "Possible values of the field `URI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URIR {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "When the device controller detects a USB Reset and enters the default state, this bit will be set to a one."]
    CLEAR,
}
impl URIR {
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
            URIR::ST => false,
            URIR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> URIR {
        match value {
            false => URIR::ST,
            true => URIR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ST`"]
    #[inline]
    pub fn is_st(&self) -> bool {
        *self == URIR::ST
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == URIR::CLEAR
    }
}
#[doc = "Possible values of the field `SRI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRIR {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "When the device controller detects a Start Of (micro) Frame, this bit will be set to a one. When a SOF is extremely late, the device controller will automatically set this bit to indicate that an SOF was expected. Therefore, this bit will be set roughly every 1 ms in device FS mode and every 125  ms in HS mode and will be synchronized to the actual SOF that is received. Since the device controller is initialized to FS before connect, this bit will be set at an interval of 1ms during the prelude to connect and chirp."]
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
#[doc = "Possible values of the field `SLI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLIR {
    #[doc = "The device controller clears the bit upon exiting from a suspend state. This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "When a device controller enters a suspend state from an active state, this bit will be set to a one."]
    CLEAR,
}
impl SLIR {
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
            SLIR::ST => false,
            SLIR::CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLIR {
        match value {
            false => SLIR::ST,
            true => SLIR::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ST`"]
    #[inline]
    pub fn is_st(&self) -> bool {
        *self == SLIR::ST
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == SLIR::CLEAR
    }
}
#[doc = "Possible values of the field `NAKI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NAKIR {
    #[doc = "This bit is automatically cleared by hardware when the all the enabled TX/RX Endpoint NAK bits are cleared."]
    ENDPCLEAR,
    #[doc = "It is set by hardware when for a particular endpoint both the TX/RX Endpoint NAK bit and the corresponding TX/RX Endpoint NAK Enable bit are set."]
    SET,
}
impl NAKIR {
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
            NAKIR::ENDPCLEAR => false,
            NAKIR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NAKIR {
        match value {
            false => NAKIR::ENDPCLEAR,
            true => NAKIR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `ENDPCLEAR`"]
    #[inline]
    pub fn is_endpclear(&self) -> bool {
        *self == NAKIR::ENDPCLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == NAKIR::SET
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
    #[doc = "When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set. The device controller detects resume signaling only (see Section 18.10.11.6)."]
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
    #[doc = "When completion of a USB transaction results in an error condition, this bit is set by the Host/Device Controller. This bit is set along with the USBINT bit, if the TD on which the error interrupt occurred also had its interrupt on complete (IOC) bit set. The device controller detects resume signaling only (see Section 18.10.11.6)."]
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
    #[doc = "The Device Controller sets this bit to a one when the port controller enters the full or high-speed operational state. When the port controller exits the full or high-speed operation states due to Reset or Suspend events, the notification mechanisms are the USB Reset Received bit (URI) and the DCSuspend bits (SLI) respectively."]
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
    #[doc = "The Device Controller sets this bit to a one when the port controller enters the full or high-speed operational state. When the port controller exits the full or high-speed operation states due to Reset or Suspend events, the notification mechanisms are the USB Reset Received bit (URI) and the DCSuspend bits (SLI) respectively."]
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
#[doc = "Values that can be written to the field `URI`"]
pub enum URIW {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "When the device controller detects a USB Reset and enters the default state, this bit will be set to a one."]
    CLEAR,
}
impl URIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            URIW::ST => false,
            URIW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _URIW<'a> {
    w: &'a mut W,
}
impl<'a> _URIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: URIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is cleared by software writing a one to it."]
    #[inline]
    pub fn st(self) -> &'a mut W {
        self.variant(URIW::ST)
    }
    #[doc = "When the device controller detects a USB Reset and enters the default state, this bit will be set to a one."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(URIW::CLEAR)
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
#[doc = "Values that can be written to the field `SRI`"]
pub enum SRIW {
    #[doc = "This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "When the device controller detects a Start Of (micro) Frame, this bit will be set to a one. When a SOF is extremely late, the device controller will automatically set this bit to indicate that an SOF was expected. Therefore, this bit will be set roughly every 1 ms in device FS mode and every 125  ms in HS mode and will be synchronized to the actual SOF that is received. Since the device controller is initialized to FS before connect, this bit will be set at an interval of 1ms during the prelude to connect and chirp."]
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
    #[doc = "When the device controller detects a Start Of (micro) Frame, this bit will be set to a one. When a SOF is extremely late, the device controller will automatically set this bit to indicate that an SOF was expected. Therefore, this bit will be set roughly every 1 ms in device FS mode and every 125 ms in HS mode and will be synchronized to the actual SOF that is received. Since the device controller is initialized to FS before connect, this bit will be set at an interval of 1ms during the prelude to connect and chirp."]
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
#[doc = "Values that can be written to the field `SLI`"]
pub enum SLIW {
    #[doc = "The device controller clears the bit upon exiting from a suspend state. This bit is cleared by software writing a one to it."]
    ST,
    #[doc = "When a device controller enters a suspend state from an active state, this bit will be set to a one."]
    CLEAR,
}
impl SLIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLIW::ST => false,
            SLIW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLIW<'a> {
    w: &'a mut W,
}
impl<'a> _SLIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The device controller clears the bit upon exiting from a suspend state. This bit is cleared by software writing a one to it."]
    #[inline]
    pub fn st(self) -> &'a mut W {
        self.variant(SLIW::ST)
    }
    #[doc = "When a device controller enters a suspend state from an active state, this bit will be set to a one."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SLIW::CLEAR)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NAKI`"]
pub enum NAKIW {
    #[doc = "This bit is automatically cleared by hardware when the all the enabled TX/RX Endpoint NAK bits are cleared."]
    ENDPCLEAR,
    #[doc = "It is set by hardware when for a particular endpoint both the TX/RX Endpoint NAK bit and the corresponding TX/RX Endpoint NAK Enable bit are set."]
    SET,
}
impl NAKIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NAKIW::ENDPCLEAR => false,
            NAKIW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NAKIW<'a> {
    w: &'a mut W,
}
impl<'a> _NAKIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NAKIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is automatically cleared by hardware when the all the enabled TX/RX Endpoint NAK bits are cleared."]
    #[inline]
    pub fn endpclear(self) -> &'a mut W {
        self.variant(NAKIW::ENDPCLEAR)
    }
    #[doc = "It is set by hardware when for a particular endpoint both the TX/RX Endpoint NAK bit and the corresponding TX/RX Endpoint NAK Enable bit are set."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(NAKIW::SET)
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
    #[doc = "Bit 0 - USB interrupt"]
    #[inline]
    pub fn ui(&self) -> UIR {
        UIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - USB error interrupt"]
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
    #[doc = "Bit 6 - USB reset received"]
    #[inline]
    pub fn uri(&self) -> URIR {
        URIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 8 - DCSuspend"]
    #[inline]
    pub fn sli(&self) -> SLIR {
        SLIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - NAK interrupt bit"]
    #[inline]
    pub fn naki(&self) -> NAKIR {
        NAKIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - USB interrupt"]
    #[inline]
    pub fn ui(&mut self) -> _UIW {
        _UIW { w: self }
    }
    #[doc = "Bit 1 - USB error interrupt"]
    #[inline]
    pub fn uei(&mut self) -> _UEIW {
        _UEIW { w: self }
    }
    #[doc = "Bit 2 - Port change detect."]
    #[inline]
    pub fn pci(&mut self) -> _PCIW {
        _PCIW { w: self }
    }
    #[doc = "Bit 6 - USB reset received"]
    #[inline]
    pub fn uri(&mut self) -> _URIW {
        _URIW { w: self }
    }
    #[doc = "Bit 7 - SOF received"]
    #[inline]
    pub fn sri(&mut self) -> _SRIW {
        _SRIW { w: self }
    }
    #[doc = "Bit 8 - DCSuspend"]
    #[inline]
    pub fn sli(&mut self) -> _SLIW {
        _SLIW { w: self }
    }
    #[doc = "Bit 16 - NAK interrupt bit"]
    #[inline]
    pub fn naki(&mut self) -> _NAKIW {
        _NAKIW { w: self }
    }
}
