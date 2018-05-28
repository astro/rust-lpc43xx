#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_TIMESTP_CTRL {
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
pub struct TSENAR {
    bits: bool,
}
impl TSENAR {
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
pub struct TSCFUPDTR {
    bits: bool,
}
impl TSCFUPDTR {
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
pub struct TSINITR {
    bits: bool,
}
impl TSINITR {
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
pub struct TSUPDTR {
    bits: bool,
}
impl TSUPDTR {
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
pub struct TSTRIGR {
    bits: bool,
}
impl TSTRIGR {
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
pub struct TSADDREGR {
    bits: bool,
}
impl TSADDREGR {
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
pub struct TSENALLR {
    bits: bool,
}
impl TSENALLR {
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
pub struct TSCTRLSSRR {
    bits: bool,
}
impl TSCTRLSSRR {
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
pub struct TSVER2ENAR {
    bits: bool,
}
impl TSVER2ENAR {
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
pub struct TSIPENAR {
    bits: bool,
}
impl TSIPENAR {
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
pub struct TSIPV6ENAR {
    bits: bool,
}
impl TSIPV6ENAR {
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
pub struct TSIPV4ENAR {
    bits: bool,
}
impl TSIPV4ENAR {
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
pub struct TSEVNTENAR {
    bits: bool,
}
impl TSEVNTENAR {
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
pub struct TSMSTRENAR {
    bits: bool,
}
impl TSMSTRENAR {
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
pub struct TSCLKTYPER {
    bits: u8,
}
impl TSCLKTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TSENMACADDRR {
    bits: bool,
}
impl TSENMACADDRR {
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
pub struct _TSENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSENAW<'a> {
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
pub struct _TSCFUPDTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSCFUPDTW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSINITW<'a> {
    w: &'a mut W,
}
impl<'a> _TSINITW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSUPDTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSUPDTW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTRIGW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSADDREGW<'a> {
    w: &'a mut W,
}
impl<'a> _TSADDREGW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSENALLW<'a> {
    w: &'a mut W,
}
impl<'a> _TSENALLW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSCTRLSSRW<'a> {
    w: &'a mut W,
}
impl<'a> _TSCTRLSSRW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSVER2ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSVER2ENAW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSIPENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIPENAW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSIPV6ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIPV6ENAW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSIPV4ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIPV4ENAW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSEVNTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSEVNTENAW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSMSTRENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSMSTRENAW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSCLKTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _TSCLKTYPEW<'a> {
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
#[doc = r" Proxy"]
pub struct _TSENMACADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _TSENMACADDRW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Time stamp Enable When this bit, is set the timestamping is enabled for transmit and receive frames. When disabled timestamp is not added for transmit and receive frames and the TimeStamp Generator is also suspended. User has to always initialize the TimeStamp (system time) after enabling this mode."]
    #[inline]
    pub fn tsena(&self) -> TSENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSENAR { bits }
    }
    #[doc = "Bit 1 - Time stamp Fine or Coarse Update When set, indicates that the system times update to be done using fine update method. When reset it indicates the system time stamp update to be done using Coarse method. This bit is reserved if the fine correction option is not enabled."]
    #[inline]
    pub fn tscfupdt(&self) -> TSCFUPDTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSCFUPDTR { bits }
    }
    #[doc = "Bit 2 - Time stamp Initialize This register field can be read and written by the application (Read and Write), and is cleared to 0 by the Ethernet core (Self Clear). When set, the system time is initialized (over-written) with the value specified in the Time stamp High Update and Time stamp Low Update registers. This register bit should be read zero before updating it. This bit is reset once the initialize is complete."]
    #[inline]
    pub fn tsinit(&self) -> TSINITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSINITR { bits }
    }
    #[doc = "Bit 3 - Time stamp Update This register field can be read and written by the application (Read and Write), and is cleared to 0 by the Ethernet core (Self Clear). When set, the system time is updated (added/subtracted) with the value specified in the Time stamp High Update and Time stamp Low Update registers. This register bit should be read zero before updating it. This bit is reset once the update is completed in hardware."]
    #[inline]
    pub fn tsupdt(&self) -> TSUPDTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSUPDTR { bits }
    }
    #[doc = "Bit 4 - Time stamp Interrupt Trigger Enable This register field can be read and written by the application (Read and Write), and is cleared to 0 by the Ethernet core (Self Clear). When set, the Time stamp interrupt is generated when the System Time becomes greater than the value written in Target Time register. This bit is reset after the generation of Time stamp Trigger Interrupt."]
    #[inline]
    pub fn tstrig(&self) -> TSTRIGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSTRIGR { bits }
    }
    #[doc = "Bit 5 - Addend Reg Update When set, the contents of the Time stamp Addend register is updated in the PTP block for fine correction. This is cleared when the update is completed. This register bit should be zero before setting it. This is a reserved bit when only coarse correction option is selected."]
    #[inline]
    pub fn tsaddreg(&self) -> TSADDREGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSADDREGR { bits }
    }
    #[doc = "Bit 8 - Enable Time stamp for All Frames When set, the Time stamp snapshot is enabled for all frames received by the core."]
    #[inline]
    pub fn tsenall(&self) -> TSENALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSENALLR { bits }
    }
    #[doc = "Bit 9 - Time stamp Digital or Binary rollover control When set, the Time stamp Low register rolls over after 0x3B9A_C9FF value (i.e., 1 nanosecond accuracy) and increments the Time stamp (High) seconds. When reset, the rollover value of sub-second register is 0x7FFF_FFFF. The sub-second increment has to be programmed correctly depending on the PTP reference clock frequency and this bit value."]
    #[inline]
    pub fn tsctrlssr(&self) -> TSCTRLSSRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSCTRLSSRR { bits }
    }
    #[doc = "Bit 10 - Enable PTP packet snooping for version 2 format When set, the PTP packets are snooped using the 1588 version 2 format else snooped using the version 1 format."]
    #[inline]
    pub fn tsver2ena(&self) -> TSVER2ENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSVER2ENAR { bits }
    }
    #[doc = "Bit 11 - Enable Time stamp Snapshot for PTP over Ethernet frames When set, the Time stamp snapshot is taken for frames which have PTP messages in Ethernet frames (PTP over Ethernet) also. By default snapshots are taken for UDP-IP-Ethernet PTP packets."]
    #[inline]
    pub fn tsipena(&self) -> TSIPENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSIPENAR { bits }
    }
    #[doc = "Bit 12 - Enable Time stamp Snapshot for IPv6 frames When set, the Time stamp snapshot is taken for IPv6 frames."]
    #[inline]
    pub fn tsipv6ena(&self) -> TSIPV6ENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSIPV6ENAR { bits }
    }
    #[doc = "Bit 13 - Enable Time stamp Snapshot for IPv4 frames When set, the Time stamp snapshot is taken for IPv4 frames."]
    #[inline]
    pub fn tsipv4ena(&self) -> TSIPV4ENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSIPV4ENAR { bits }
    }
    #[doc = "Bit 14 - Enable Time stamp Snapshot for Event Messages When set, the Time stamp snapshot is taken for event messages only. When reset snapshot is taken for all other messages except Announce, Management and Signaling."]
    #[inline]
    pub fn tsevntena(&self) -> TSEVNTENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSEVNTENAR { bits }
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master When set, the snapshot is taken for messages relevant to master node only else snapshot is taken for messages relevant to slave node. This is valid only for ordinary clock and boundary clock node."]
    #[inline]
    pub fn tsmstrena(&self) -> TSMSTRENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSMSTRENAR { bits }
    }
    #[doc = "Bits 16:17 - Select the type of clock node The following are the options to select the type of clock node: 00 = ordinary clock 01 = boundary clock 10 = end-to-end transparent clock 11 = peer-to-peer transparent clock"]
    #[inline]
    pub fn tsclktype(&self) -> TSCLKTYPER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TSCLKTYPER { bits }
    }
    #[doc = "Bit 18 - Enable MAC address for PTP frame filtering When set, uses the DA MAC address (that matches any MAC Address register except the default MAC address 0) to filter the PTP frames when PTP is sent directly over Ethernet."]
    #[inline]
    pub fn tsenmacaddr(&self) -> TSENMACADDRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSENMACADDRR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8192 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Time stamp Enable When this bit, is set the timestamping is enabled for transmit and receive frames. When disabled timestamp is not added for transmit and receive frames and the TimeStamp Generator is also suspended. User has to always initialize the TimeStamp (system time) after enabling this mode."]
    #[inline]
    pub fn tsena(&mut self) -> _TSENAW {
        _TSENAW { w: self }
    }
    #[doc = "Bit 1 - Time stamp Fine or Coarse Update When set, indicates that the system times update to be done using fine update method. When reset it indicates the system time stamp update to be done using Coarse method. This bit is reserved if the fine correction option is not enabled."]
    #[inline]
    pub fn tscfupdt(&mut self) -> _TSCFUPDTW {
        _TSCFUPDTW { w: self }
    }
    #[doc = "Bit 2 - Time stamp Initialize This register field can be read and written by the application (Read and Write), and is cleared to 0 by the Ethernet core (Self Clear). When set, the system time is initialized (over-written) with the value specified in the Time stamp High Update and Time stamp Low Update registers. This register bit should be read zero before updating it. This bit is reset once the initialize is complete."]
    #[inline]
    pub fn tsinit(&mut self) -> _TSINITW {
        _TSINITW { w: self }
    }
    #[doc = "Bit 3 - Time stamp Update This register field can be read and written by the application (Read and Write), and is cleared to 0 by the Ethernet core (Self Clear). When set, the system time is updated (added/subtracted) with the value specified in the Time stamp High Update and Time stamp Low Update registers. This register bit should be read zero before updating it. This bit is reset once the update is completed in hardware."]
    #[inline]
    pub fn tsupdt(&mut self) -> _TSUPDTW {
        _TSUPDTW { w: self }
    }
    #[doc = "Bit 4 - Time stamp Interrupt Trigger Enable This register field can be read and written by the application (Read and Write), and is cleared to 0 by the Ethernet core (Self Clear). When set, the Time stamp interrupt is generated when the System Time becomes greater than the value written in Target Time register. This bit is reset after the generation of Time stamp Trigger Interrupt."]
    #[inline]
    pub fn tstrig(&mut self) -> _TSTRIGW {
        _TSTRIGW { w: self }
    }
    #[doc = "Bit 5 - Addend Reg Update When set, the contents of the Time stamp Addend register is updated in the PTP block for fine correction. This is cleared when the update is completed. This register bit should be zero before setting it. This is a reserved bit when only coarse correction option is selected."]
    #[inline]
    pub fn tsaddreg(&mut self) -> _TSADDREGW {
        _TSADDREGW { w: self }
    }
    #[doc = "Bit 8 - Enable Time stamp for All Frames When set, the Time stamp snapshot is enabled for all frames received by the core."]
    #[inline]
    pub fn tsenall(&mut self) -> _TSENALLW {
        _TSENALLW { w: self }
    }
    #[doc = "Bit 9 - Time stamp Digital or Binary rollover control When set, the Time stamp Low register rolls over after 0x3B9A_C9FF value (i.e., 1 nanosecond accuracy) and increments the Time stamp (High) seconds. When reset, the rollover value of sub-second register is 0x7FFF_FFFF. The sub-second increment has to be programmed correctly depending on the PTP reference clock frequency and this bit value."]
    #[inline]
    pub fn tsctrlssr(&mut self) -> _TSCTRLSSRW {
        _TSCTRLSSRW { w: self }
    }
    #[doc = "Bit 10 - Enable PTP packet snooping for version 2 format When set, the PTP packets are snooped using the 1588 version 2 format else snooped using the version 1 format."]
    #[inline]
    pub fn tsver2ena(&mut self) -> _TSVER2ENAW {
        _TSVER2ENAW { w: self }
    }
    #[doc = "Bit 11 - Enable Time stamp Snapshot for PTP over Ethernet frames When set, the Time stamp snapshot is taken for frames which have PTP messages in Ethernet frames (PTP over Ethernet) also. By default snapshots are taken for UDP-IP-Ethernet PTP packets."]
    #[inline]
    pub fn tsipena(&mut self) -> _TSIPENAW {
        _TSIPENAW { w: self }
    }
    #[doc = "Bit 12 - Enable Time stamp Snapshot for IPv6 frames When set, the Time stamp snapshot is taken for IPv6 frames."]
    #[inline]
    pub fn tsipv6ena(&mut self) -> _TSIPV6ENAW {
        _TSIPV6ENAW { w: self }
    }
    #[doc = "Bit 13 - Enable Time stamp Snapshot for IPv4 frames When set, the Time stamp snapshot is taken for IPv4 frames."]
    #[inline]
    pub fn tsipv4ena(&mut self) -> _TSIPV4ENAW {
        _TSIPV4ENAW { w: self }
    }
    #[doc = "Bit 14 - Enable Time stamp Snapshot for Event Messages When set, the Time stamp snapshot is taken for event messages only. When reset snapshot is taken for all other messages except Announce, Management and Signaling."]
    #[inline]
    pub fn tsevntena(&mut self) -> _TSEVNTENAW {
        _TSEVNTENAW { w: self }
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master When set, the snapshot is taken for messages relevant to master node only else snapshot is taken for messages relevant to slave node. This is valid only for ordinary clock and boundary clock node."]
    #[inline]
    pub fn tsmstrena(&mut self) -> _TSMSTRENAW {
        _TSMSTRENAW { w: self }
    }
    #[doc = "Bits 16:17 - Select the type of clock node The following are the options to select the type of clock node: 00 = ordinary clock 01 = boundary clock 10 = end-to-end transparent clock 11 = peer-to-peer transparent clock"]
    #[inline]
    pub fn tsclktype(&mut self) -> _TSCLKTYPEW {
        _TSCLKTYPEW { w: self }
    }
    #[doc = "Bit 18 - Enable MAC address for PTP frame filtering When set, uses the DA MAC address (that matches any MAC Address register except the default MAC address 0) to filter the PTP frames when PTP is sent directly over Ethernet."]
    #[inline]
    pub fn tsenmacaddr(&mut self) -> _TSENMACADDRW {
        _TSENMACADDRW { w: self }
    }
}
