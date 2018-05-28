#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ULPIVIEWPORT {
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
pub struct ULPIDATWRR {
    bits: u8,
}
impl ULPIDATWRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ULPIDATRDR {
    bits: u8,
}
impl ULPIDATRDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ULPIADDRR {
    bits: u8,
}
impl ULPIADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ULPIPORTR {
    bits: u8,
}
impl ULPIPORTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ULPISS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPISSR {
    #[doc = "In another state  (ie. carkit, serial, low power)"]
    IN_ANOTHER_STATE,
    #[doc = "Normal Sync. State."]
    NORMAL_SYNC_STATE,
}
impl ULPISSR {
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
            ULPISSR::IN_ANOTHER_STATE => false,
            ULPISSR::NORMAL_SYNC_STATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ULPISSR {
        match value {
            false => ULPISSR::IN_ANOTHER_STATE,
            true => ULPISSR::NORMAL_SYNC_STATE,
        }
    }
    #[doc = "Checks if the value of the field is `IN_ANOTHER_STATE`"]
    #[inline]
    pub fn is_in_another_state(&self) -> bool {
        *self == ULPISSR::IN_ANOTHER_STATE
    }
    #[doc = "Checks if the value of the field is `NORMAL_SYNC_STATE`"]
    #[inline]
    pub fn is_normal_sync_state(&self) -> bool {
        *self == ULPISSR::NORMAL_SYNC_STATE
    }
}
#[doc = "Possible values of the field `ULPIRW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPIRWR {
    #[doc = "Read"]
    READ,
    #[doc = "Write"]
    WRITE,
}
impl ULPIRWR {
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
            ULPIRWR::READ => false,
            ULPIRWR::WRITE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ULPIRWR {
        match value {
            false => ULPIRWR::READ,
            true => ULPIRWR::WRITE,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == ULPIRWR::READ
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline]
    pub fn is_write(&self) -> bool {
        *self == ULPIRWR::WRITE
    }
}
#[doc = r" Value of the field"]
pub struct ULPIRUNR {
    bits: bool,
}
impl ULPIRUNR {
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
pub struct ULPIWUR {
    bits: bool,
}
impl ULPIWUR {
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
#[doc = r" Proxy"]
pub struct _ULPIDATWRW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPIDATWRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ULPIDATRDW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPIDATRDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ULPIADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPIADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ULPIPORTW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPIPORTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ULPISS`"]
pub enum ULPISSW {
    #[doc = "In another state  (ie. carkit, serial, low power)"]
    IN_ANOTHER_STATE,
    #[doc = "Normal Sync. State."]
    NORMAL_SYNC_STATE,
}
impl ULPISSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ULPISSW::IN_ANOTHER_STATE => false,
            ULPISSW::NORMAL_SYNC_STATE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ULPISSW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPISSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ULPISSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "In another state (ie. carkit, serial, low power)"]
    #[inline]
    pub fn in_another_state(self) -> &'a mut W {
        self.variant(ULPISSW::IN_ANOTHER_STATE)
    }
    #[doc = "Normal Sync. State."]
    #[inline]
    pub fn normal_sync_state(self) -> &'a mut W {
        self.variant(ULPISSW::NORMAL_SYNC_STATE)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ULPIRW`"]
pub enum ULPIRWW {
    #[doc = "Read"]
    READ,
    #[doc = "Write"]
    WRITE,
}
impl ULPIRWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ULPIRWW::READ => false,
            ULPIRWW::WRITE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ULPIRWW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPIRWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ULPIRWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read"]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(ULPIRWW::READ)
    }
    #[doc = "Write"]
    #[inline]
    pub fn write(self) -> &'a mut W {
        self.variant(ULPIRWW::WRITE)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ULPIRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPIRUNW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ULPIWUW<'a> {
    w: &'a mut W,
}
impl<'a> _ULPIWUW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:7 - When a write operation is commanded, the data to be sent is written to this field."]
    #[inline]
    pub fn ulpidatwr(&self) -> ULPIDATWRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ULPIDATWRR { bits }
    }
    #[doc = "Bits 8:15 - After a read operation completes, the result is placed in this field."]
    #[inline]
    pub fn ulpidatrd(&self) -> ULPIDATRDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ULPIDATRDR { bits }
    }
    #[doc = "Bits 16:23 - When a read or write operation is commanded, the address of the operation is written to this field."]
    #[inline]
    pub fn ulpiaddr(&self) -> ULPIADDRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ULPIADDRR { bits }
    }
    #[doc = "Bits 24:26 - For the wakeup or read/write operation to be executed, this value must be written as 0."]
    #[inline]
    pub fn ulpiport(&self) -> ULPIPORTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ULPIPORTR { bits }
    }
    #[doc = "Bit 27 - ULPI sync state. This bit represents the state of the ULPI interface."]
    #[inline]
    pub fn ulpiss(&self) -> ULPISSR {
        ULPISSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - ULPI Read/Write control. This bit selects between running a read or write operation."]
    #[inline]
    pub fn ulpirw(&self) -> ULPIRWR {
        ULPIRWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - ULPI Read/Write Run. Writing the 1 to this bit will begin the read/write operation. The bit will automatically transition to 0 after the read/write is complete. Once this bit is set, the driver can not set it back to 0. The driver must never executue a wakeup and a read/write operation at the same time."]
    #[inline]
    pub fn ulpirun(&self) -> ULPIRUNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ULPIRUNR { bits }
    }
    #[doc = "Bit 31 - ULPI Wake-up. Writing the 1 to this bit will begin the wakeup operation. The bit will automatically transition to 0 after the wakeup is complete. Once this bit is set, the driver can not set it back to 0. The driver must never executue a wakeup and a read/write operation at the same time."]
    #[inline]
    pub fn ulpiwu(&self) -> ULPIWUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ULPIWUR { bits }
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
    #[doc = "Bits 0:7 - When a write operation is commanded, the data to be sent is written to this field."]
    #[inline]
    pub fn ulpidatwr(&mut self) -> _ULPIDATWRW {
        _ULPIDATWRW { w: self }
    }
    #[doc = "Bits 8:15 - After a read operation completes, the result is placed in this field."]
    #[inline]
    pub fn ulpidatrd(&mut self) -> _ULPIDATRDW {
        _ULPIDATRDW { w: self }
    }
    #[doc = "Bits 16:23 - When a read or write operation is commanded, the address of the operation is written to this field."]
    #[inline]
    pub fn ulpiaddr(&mut self) -> _ULPIADDRW {
        _ULPIADDRW { w: self }
    }
    #[doc = "Bits 24:26 - For the wakeup or read/write operation to be executed, this value must be written as 0."]
    #[inline]
    pub fn ulpiport(&mut self) -> _ULPIPORTW {
        _ULPIPORTW { w: self }
    }
    #[doc = "Bit 27 - ULPI sync state. This bit represents the state of the ULPI interface."]
    #[inline]
    pub fn ulpiss(&mut self) -> _ULPISSW {
        _ULPISSW { w: self }
    }
    #[doc = "Bit 29 - ULPI Read/Write control. This bit selects between running a read or write operation."]
    #[inline]
    pub fn ulpirw(&mut self) -> _ULPIRWW {
        _ULPIRWW { w: self }
    }
    #[doc = "Bit 30 - ULPI Read/Write Run. Writing the 1 to this bit will begin the read/write operation. The bit will automatically transition to 0 after the read/write is complete. Once this bit is set, the driver can not set it back to 0. The driver must never executue a wakeup and a read/write operation at the same time."]
    #[inline]
    pub fn ulpirun(&mut self) -> _ULPIRUNW {
        _ULPIRUNW { w: self }
    }
    #[doc = "Bit 31 - ULPI Wake-up. Writing the 1 to this bit will begin the wakeup operation. The bit will automatically transition to 0 after the wakeup is complete. Once this bit is set, the driver can not set it back to 0. The driver must never executue a wakeup and a read/write operation at the same time."]
    #[inline]
    pub fn ulpiwu(&mut self) -> _ULPIWUW {
        _ULPIWUW { w: self }
    }
}
