#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVICEADDR {
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
#[doc = "Possible values of the field `USBADRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBADRAR {
    #[doc = "Any write to USBADR are instantaneous."]
    INSTANTANEOUS,
    #[doc = "When the user writes a one to this bit at the same time or before USBADR is written, the write to USBADR fields is staged and held in a hidden register. After an IN occurs on endpoint 0 and is acknowledged, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: IN is ACKed to endpoint 0. USBADR is updated from the staging register. OUT/SETUP occurs on endpoint 0. USBADR is not updated. Device reset occurs. USBADR is set to 0. After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write the device address within 2 ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2 ms USB requirement."]
    DELAYED,
}
impl USBADRAR {
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
            USBADRAR::INSTANTANEOUS => false,
            USBADRAR::DELAYED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBADRAR {
        match value {
            false => USBADRAR::INSTANTANEOUS,
            true => USBADRAR::DELAYED,
        }
    }
    #[doc = "Checks if the value of the field is `INSTANTANEOUS`"]
    #[inline]
    pub fn is_instantaneous(&self) -> bool {
        *self == USBADRAR::INSTANTANEOUS
    }
    #[doc = "Checks if the value of the field is `DELAYED`"]
    #[inline]
    pub fn is_delayed(&self) -> bool {
        *self == USBADRAR::DELAYED
    }
}
#[doc = r" Value of the field"]
pub struct USBADRR {
    bits: u8,
}
impl USBADRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `USBADRA`"]
pub enum USBADRAW {
    #[doc = "Any write to USBADR are instantaneous."]
    INSTANTANEOUS,
    #[doc = "When the user writes a one to this bit at the same time or before USBADR is written, the write to USBADR fields is staged and held in a hidden register. After an IN occurs on endpoint 0 and is acknowledged, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: IN is ACKed to endpoint 0. USBADR is updated from the staging register. OUT/SETUP occurs on endpoint 0. USBADR is not updated. Device reset occurs. USBADR is set to 0. After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write the device address within 2 ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2 ms USB requirement."]
    DELAYED,
}
impl USBADRAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBADRAW::INSTANTANEOUS => false,
            USBADRAW::DELAYED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBADRAW<'a> {
    w: &'a mut W,
}
impl<'a> _USBADRAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBADRAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Any write to USBADR are instantaneous."]
    #[inline]
    pub fn instantaneous(self) -> &'a mut W {
        self.variant(USBADRAW::INSTANTANEOUS)
    }
    #[doc = "When the user writes a one to this bit at the same time or before USBADR is written, the write to USBADR fields is staged and held in a hidden register. After an IN occurs on endpoint 0 and is acknowledged, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: IN is ACKed to endpoint 0. USBADR is updated from the staging register. OUT/SETUP occurs on endpoint 0. USBADR is not updated. Device reset occurs. USBADR is set to 0. After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write the device address within 2 ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2 ms USB requirement."]
    #[inline]
    pub fn delayed(self) -> &'a mut W {
        self.variant(USBADRAW::DELAYED)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USBADRW<'a> {
    w: &'a mut W,
}
impl<'a> _USBADRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 24 - Device address advance"]
    #[inline]
    pub fn usbadra(&self) -> USBADRAR {
        USBADRAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 25:31 - USB device address"]
    #[inline]
    pub fn usbadr(&self) -> USBADRR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USBADRR { bits }
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
    #[doc = "Bit 24 - Device address advance"]
    #[inline]
    pub fn usbadra(&mut self) -> _USBADRAW {
        _USBADRAW { w: self }
    }
    #[doc = "Bits 25:31 - USB device address"]
    #[inline]
    pub fn usbadr(&mut self) -> _USBADRW {
        _USBADRW { w: self }
    }
}
