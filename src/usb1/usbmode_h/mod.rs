#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBMODE_H {
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
#[doc = "Possible values of the field `CM1_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM1_0R {
    #[doc = "Idle"]
    IDLE,
    #[doc = "Device controller"]
    DEVICE_CONTROLLER,
    #[doc = "Host controller"]
    HOST_CONTROLLER,
}
impl CM1_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CM1_0R::IDLE => 0,
            CM1_0R::DEVICE_CONTROLLER => 2,
            CM1_0R::HOST_CONTROLLER => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CM1_0R {
        match value {
            0 => CM1_0R::IDLE,
            2 => CM1_0R::DEVICE_CONTROLLER,
            3 => CM1_0R::HOST_CONTROLLER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == CM1_0R::IDLE
    }
    #[doc = "Checks if the value of the field is `DEVICE_CONTROLLER`"]
    #[inline]
    pub fn is_device_controller(&self) -> bool {
        *self == CM1_0R::DEVICE_CONTROLLER
    }
    #[doc = "Checks if the value of the field is `HOST_CONTROLLER`"]
    #[inline]
    pub fn is_host_controller(&self) -> bool {
        *self == CM1_0R::HOST_CONTROLLER
    }
}
#[doc = "Possible values of the field `ES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESR {
    #[doc = "Little endian: first byte referenced in least significant byte of 32-bit word."]
    LITTLE_ENDIAN_FIRST,
    #[doc = "Big endian: first byte referenced in most significant byte of 32-bit word."]
    BIG_ENDIAN_FIRST_BY,
}
impl ESR {
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
            ESR::LITTLE_ENDIAN_FIRST => false,
            ESR::BIG_ENDIAN_FIRST_BY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESR {
        match value {
            false => ESR::LITTLE_ENDIAN_FIRST,
            true => ESR::BIG_ENDIAN_FIRST_BY,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN_FIRST`"]
    #[inline]
    pub fn is_little_endian_first(&self) -> bool {
        *self == ESR::LITTLE_ENDIAN_FIRST
    }
    #[doc = "Checks if the value of the field is `BIG_ENDIAN_FIRST_BY`"]
    #[inline]
    pub fn is_big_endian_first_by(&self) -> bool {
        *self == ESR::BIG_ENDIAN_FIRST_BY
    }
}
#[doc = "Possible values of the field `SDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDISR {
    #[doc = "Not disabled"]
    NOT_DISABLED,
    #[doc = "Disabled. Setting to a 1 ensures that overruns/underruns of the latency FIFO are eliminated for low bandwidth systems where the RX and TX buffers are sufficient to contain the entire packet. Enabling stream disable also has the effect of ensuring the the TX latency is filled to capacity before the packet is launched onto the USB. Note: Time duration to pre-fill the FIFO becomes significant when stream disable is active. See TXFILLTUNING to characterize the adjustments needed for the scheduler when using this feature."]
    DISABLED_SETTING_TO,
}
impl SDISR {
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
            SDISR::NOT_DISABLED => false,
            SDISR::DISABLED_SETTING_TO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDISR {
        match value {
            false => SDISR::NOT_DISABLED,
            true => SDISR::DISABLED_SETTING_TO,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DISABLED`"]
    #[inline]
    pub fn is_not_disabled(&self) -> bool {
        *self == SDISR::NOT_DISABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED_SETTING_TO`"]
    #[inline]
    pub fn is_disabled_setting_to(&self) -> bool {
        *self == SDISR::DISABLED_SETTING_TO
    }
}
#[doc = "Possible values of the field `VBPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBPSR {
    #[doc = "vbus_pwr_select is set LOW."]
    LOW,
    #[doc = "vbus_pwr_select is set HIGH"]
    HIGH,
}
impl VBPSR {
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
            VBPSR::LOW => false,
            VBPSR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBPSR {
        match value {
            false => VBPSR::LOW,
            true => VBPSR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == VBPSR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == VBPSR::HIGH
    }
}
#[doc = "Values that can be written to the field `CM1_0`"]
pub enum CM1_0W {
    #[doc = "Idle"]
    IDLE,
    #[doc = "Device controller"]
    DEVICE_CONTROLLER,
    #[doc = "Host controller"]
    HOST_CONTROLLER,
}
impl CM1_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CM1_0W::IDLE => 0,
            CM1_0W::DEVICE_CONTROLLER => 2,
            CM1_0W::HOST_CONTROLLER => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CM1_0W<'a> {
    w: &'a mut W,
}
impl<'a> _CM1_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CM1_0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Idle"]
    #[inline]
    pub fn idle(self) -> &'a mut W {
        self.variant(CM1_0W::IDLE)
    }
    #[doc = "Device controller"]
    #[inline]
    pub fn device_controller(self) -> &'a mut W {
        self.variant(CM1_0W::DEVICE_CONTROLLER)
    }
    #[doc = "Host controller"]
    #[inline]
    pub fn host_controller(self) -> &'a mut W {
        self.variant(CM1_0W::HOST_CONTROLLER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ES`"]
pub enum ESW {
    #[doc = "Little endian: first byte referenced in least significant byte of 32-bit word."]
    LITTLE_ENDIAN_FIRST,
    #[doc = "Big endian: first byte referenced in most significant byte of 32-bit word."]
    BIG_ENDIAN_FIRST_BY,
}
impl ESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESW::LITTLE_ENDIAN_FIRST => false,
            ESW::BIG_ENDIAN_FIRST_BY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESW<'a> {
    w: &'a mut W,
}
impl<'a> _ESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Little endian: first byte referenced in least significant byte of 32-bit word."]
    #[inline]
    pub fn little_endian_first(self) -> &'a mut W {
        self.variant(ESW::LITTLE_ENDIAN_FIRST)
    }
    #[doc = "Big endian: first byte referenced in most significant byte of 32-bit word."]
    #[inline]
    pub fn big_endian_first_by(self) -> &'a mut W {
        self.variant(ESW::BIG_ENDIAN_FIRST_BY)
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
#[doc = "Values that can be written to the field `SDIS`"]
pub enum SDISW {
    #[doc = "Not disabled"]
    NOT_DISABLED,
    #[doc = "Disabled. Setting to a 1 ensures that overruns/underruns of the latency FIFO are eliminated for low bandwidth systems where the RX and TX buffers are sufficient to contain the entire packet. Enabling stream disable also has the effect of ensuring the the TX latency is filled to capacity before the packet is launched onto the USB. Note: Time duration to pre-fill the FIFO becomes significant when stream disable is active. See TXFILLTUNING to characterize the adjustments needed for the scheduler when using this feature."]
    DISABLED_SETTING_TO,
}
impl SDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDISW::NOT_DISABLED => false,
            SDISW::DISABLED_SETTING_TO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not disabled"]
    #[inline]
    pub fn not_disabled(self) -> &'a mut W {
        self.variant(SDISW::NOT_DISABLED)
    }
    #[doc = "Disabled. Setting to a 1 ensures that overruns/underruns of the latency FIFO are eliminated for low bandwidth systems where the RX and TX buffers are sufficient to contain the entire packet. Enabling stream disable also has the effect of ensuring the the TX latency is filled to capacity before the packet is launched onto the USB. Note: Time duration to pre-fill the FIFO becomes significant when stream disable is active. See TXFILLTUNING to characterize the adjustments needed for the scheduler when using this feature."]
    #[inline]
    pub fn disabled_setting_to(self) -> &'a mut W {
        self.variant(SDISW::DISABLED_SETTING_TO)
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
#[doc = "Values that can be written to the field `VBPS`"]
pub enum VBPSW {
    #[doc = "vbus_pwr_select is set LOW."]
    LOW,
    #[doc = "vbus_pwr_select is set HIGH"]
    HIGH,
}
impl VBPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VBPSW::LOW => false,
            VBPSW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBPSW<'a> {
    w: &'a mut W,
}
impl<'a> _VBPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "vbus_pwr_select is set LOW."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(VBPSW::LOW)
    }
    #[doc = "vbus_pwr_select is set HIGH"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(VBPSW::HIGH)
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
    #[doc = "Bits 0:1 - Controller mode The controller defaults to an idle state and needs to be initialized to the desired operating mode after reset. This register can only be written once after reset. If it is necessary to switch modes, software must reset the controller by writing to the RESET bit in the USBCMD register before reprogramming this register."]
    #[inline]
    pub fn cm1_0(&self) -> CM1_0R {
        CM1_0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Endian select This bit can change the byte ordering of the transfer buffers. The bit fields in the microprocessor interface and the DMA data structures (including the setup buffer within the device QH) are unaffected by the value of this bit, because they are based upon 32-bit words."]
    #[inline]
    pub fn es(&self) -> ESR {
        ESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Stream disable mode The use of this feature substantially limits the overall USB performance that can be achieved."]
    #[inline]
    pub fn sdis(&self) -> SDISR {
        SDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - VBUS power select"]
    #[inline]
    pub fn vbps(&self) -> VBPSR {
        VBPSR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Controller mode The controller defaults to an idle state and needs to be initialized to the desired operating mode after reset. This register can only be written once after reset. If it is necessary to switch modes, software must reset the controller by writing to the RESET bit in the USBCMD register before reprogramming this register."]
    #[inline]
    pub fn cm1_0(&mut self) -> _CM1_0W {
        _CM1_0W { w: self }
    }
    #[doc = "Bit 2 - Endian select This bit can change the byte ordering of the transfer buffers. The bit fields in the microprocessor interface and the DMA data structures (including the setup buffer within the device QH) are unaffected by the value of this bit, because they are based upon 32-bit words."]
    #[inline]
    pub fn es(&mut self) -> _ESW {
        _ESW { w: self }
    }
    #[doc = "Bit 4 - Stream disable mode The use of this feature substantially limits the overall USB performance that can be achieved."]
    #[inline]
    pub fn sdis(&mut self) -> _SDISW {
        _SDISW { w: self }
    }
    #[doc = "Bit 5 - VBUS power select"]
    #[inline]
    pub fn vbps(&mut self) -> _VBPSW {
        _VBPSW { w: self }
    }
}
