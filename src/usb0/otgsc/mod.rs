#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OTGSC {
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
pub struct VDR {
    bits: bool,
}
impl VDR {
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
pub struct VCR {
    bits: bool,
}
impl VCR {
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
#[doc = "Possible values of the field `HAAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HAARR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enable automatic reset after connect on host port."]
    ENABLE_AUTOMATIC_RES,
}
impl HAARR {
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
            HAARR::DISABLED => false,
            HAARR::ENABLE_AUTOMATIC_RES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HAARR {
        match value {
            false => HAARR::DISABLED,
            true => HAARR::ENABLE_AUTOMATIC_RES,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HAARR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTOMATIC_RES`"]
    #[inline]
    pub fn is_enable_automatic_res(&self) -> bool {
        *self == HAARR::ENABLE_AUTOMATIC_RES
    }
}
#[doc = r" Value of the field"]
pub struct OTR {
    bits: bool,
}
impl OTR {
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
pub struct DPR {
    bits: bool,
}
impl DPR {
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
#[doc = "Possible values of the field `IDPU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDPUR {
    #[doc = "Pull-up off. The ID bit will not be sampled."]
    PULL_UP_OFF_THE_ID,
    #[doc = "Pull-up on."]
    PULL_UP_ON,
}
impl IDPUR {
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
            IDPUR::PULL_UP_OFF_THE_ID => false,
            IDPUR::PULL_UP_ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDPUR {
        match value {
            false => IDPUR::PULL_UP_OFF_THE_ID,
            true => IDPUR::PULL_UP_ON,
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP_OFF_THE_ID`"]
    #[inline]
    pub fn is_pull_up_off_the_id(&self) -> bool {
        *self == IDPUR::PULL_UP_OFF_THE_ID
    }
    #[doc = "Checks if the value of the field is `PULL_UP_ON`"]
    #[inline]
    pub fn is_pull_up_on(&self) -> bool {
        *self == IDPUR::PULL_UP_ON
    }
}
#[doc = r" Value of the field"]
pub struct HADPR {
    bits: bool,
}
impl HADPR {
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
#[doc = "Possible values of the field `HABA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HABAR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enable automatic B-disconnect to A-connect sequence."]
    ENABLE_AUTOMATIC_B_D,
}
impl HABAR {
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
            HABAR::DISABLED => false,
            HABAR::ENABLE_AUTOMATIC_B_D => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HABAR {
        match value {
            false => HABAR::DISABLED,
            true => HABAR::ENABLE_AUTOMATIC_B_D,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HABAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTOMATIC_B_D`"]
    #[inline]
    pub fn is_enable_automatic_b_d(&self) -> bool {
        *self == HABAR::ENABLE_AUTOMATIC_B_D
    }
}
#[doc = "Possible values of the field `ID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDR {
    #[doc = "A-device"]
    A_DEVICE,
    #[doc = "B-device"]
    B_DEVICE,
}
impl IDR {
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
            IDR::A_DEVICE => false,
            IDR::B_DEVICE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDR {
        match value {
            false => IDR::A_DEVICE,
            true => IDR::B_DEVICE,
        }
    }
    #[doc = "Checks if the value of the field is `A_DEVICE`"]
    #[inline]
    pub fn is_a_device(&self) -> bool {
        *self == IDR::A_DEVICE
    }
    #[doc = "Checks if the value of the field is `B_DEVICE`"]
    #[inline]
    pub fn is_b_device(&self) -> bool {
        *self == IDR::B_DEVICE
    }
}
#[doc = r" Value of the field"]
pub struct AVVR {
    bits: bool,
}
impl AVVR {
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
pub struct ASVR {
    bits: bool,
}
impl ASVR {
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
pub struct BSVR {
    bits: bool,
}
impl BSVR {
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
pub struct BSER {
    bits: bool,
}
impl BSER {
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
pub struct MS1TR {
    bits: bool,
}
impl MS1TR {
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
pub struct DPSR {
    bits: bool,
}
impl DPSR {
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
pub struct IDISR {
    bits: bool,
}
impl IDISR {
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
pub struct AVVISR {
    bits: bool,
}
impl AVVISR {
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
pub struct ASVISR {
    bits: bool,
}
impl ASVISR {
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
pub struct BSVISR {
    bits: bool,
}
impl BSVISR {
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
pub struct BSEISR {
    bits: bool,
}
impl BSEISR {
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
pub struct MS1SR {
    bits: bool,
}
impl MS1SR {
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
pub struct DPISR {
    bits: bool,
}
impl DPISR {
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
pub struct IDIER {
    bits: bool,
}
impl IDIER {
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
pub struct AVVIER {
    bits: bool,
}
impl AVVIER {
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
pub struct ASVIER {
    bits: bool,
}
impl ASVIER {
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
pub struct BSVIER {
    bits: bool,
}
impl BSVIER {
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
pub struct BSEIER {
    bits: bool,
}
impl BSEIER {
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
pub struct MS1ER {
    bits: bool,
}
impl MS1ER {
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
pub struct DPIER {
    bits: bool,
}
impl DPIER {
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
pub struct _VDW<'a> {
    w: &'a mut W,
}
impl<'a> _VDW<'a> {
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
pub struct _VCW<'a> {
    w: &'a mut W,
}
impl<'a> _VCW<'a> {
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
#[doc = "Values that can be written to the field `HAAR`"]
pub enum HAARW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enable automatic reset after connect on host port."]
    ENABLE_AUTOMATIC_RES,
}
impl HAARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HAARW::DISABLED => false,
            HAARW::ENABLE_AUTOMATIC_RES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HAARW<'a> {
    w: &'a mut W,
}
impl<'a> _HAARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HAARW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HAARW::DISABLED)
    }
    #[doc = "Enable automatic reset after connect on host port."]
    #[inline]
    pub fn enable_automatic_res(self) -> &'a mut W {
        self.variant(HAARW::ENABLE_AUTOMATIC_RES)
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
#[doc = r" Proxy"]
pub struct _OTW<'a> {
    w: &'a mut W,
}
impl<'a> _OTW<'a> {
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
pub struct _DPW<'a> {
    w: &'a mut W,
}
impl<'a> _DPW<'a> {
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
#[doc = "Values that can be written to the field `IDPU`"]
pub enum IDPUW {
    #[doc = "Pull-up off. The ID bit will not be sampled."]
    PULL_UP_OFF_THE_ID,
    #[doc = "Pull-up on."]
    PULL_UP_ON,
}
impl IDPUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDPUW::PULL_UP_OFF_THE_ID => false,
            IDPUW::PULL_UP_ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDPUW<'a> {
    w: &'a mut W,
}
impl<'a> _IDPUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDPUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pull-up off. The ID bit will not be sampled."]
    #[inline]
    pub fn pull_up_off_the_id(self) -> &'a mut W {
        self.variant(IDPUW::PULL_UP_OFF_THE_ID)
    }
    #[doc = "Pull-up on."]
    #[inline]
    pub fn pull_up_on(self) -> &'a mut W {
        self.variant(IDPUW::PULL_UP_ON)
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
#[doc = r" Proxy"]
pub struct _HADPW<'a> {
    w: &'a mut W,
}
impl<'a> _HADPW<'a> {
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
#[doc = "Values that can be written to the field `HABA`"]
pub enum HABAW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enable automatic B-disconnect to A-connect sequence."]
    ENABLE_AUTOMATIC_B_D,
}
impl HABAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HABAW::DISABLED => false,
            HABAW::ENABLE_AUTOMATIC_B_D => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HABAW<'a> {
    w: &'a mut W,
}
impl<'a> _HABAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HABAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HABAW::DISABLED)
    }
    #[doc = "Enable automatic B-disconnect to A-connect sequence."]
    #[inline]
    pub fn enable_automatic_b_d(self) -> &'a mut W {
        self.variant(HABAW::ENABLE_AUTOMATIC_B_D)
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
#[doc = "Values that can be written to the field `ID`"]
pub enum IDW {
    #[doc = "A-device"]
    A_DEVICE,
    #[doc = "B-device"]
    B_DEVICE,
}
impl IDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDW::A_DEVICE => false,
            IDW::B_DEVICE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDW<'a> {
    w: &'a mut W,
}
impl<'a> _IDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A-device"]
    #[inline]
    pub fn a_device(self) -> &'a mut W {
        self.variant(IDW::A_DEVICE)
    }
    #[doc = "B-device"]
    #[inline]
    pub fn b_device(self) -> &'a mut W {
        self.variant(IDW::B_DEVICE)
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
#[doc = r" Proxy"]
pub struct _AVVW<'a> {
    w: &'a mut W,
}
impl<'a> _AVVW<'a> {
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
pub struct _ASVW<'a> {
    w: &'a mut W,
}
impl<'a> _ASVW<'a> {
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
pub struct _BSVW<'a> {
    w: &'a mut W,
}
impl<'a> _BSVW<'a> {
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
pub struct _BSEW<'a> {
    w: &'a mut W,
}
impl<'a> _BSEW<'a> {
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
pub struct _MS1TW<'a> {
    w: &'a mut W,
}
impl<'a> _MS1TW<'a> {
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
pub struct _DPSW<'a> {
    w: &'a mut W,
}
impl<'a> _DPSW<'a> {
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
pub struct _IDISW<'a> {
    w: &'a mut W,
}
impl<'a> _IDISW<'a> {
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
#[doc = r" Proxy"]
pub struct _AVVISW<'a> {
    w: &'a mut W,
}
impl<'a> _AVVISW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ASVISW<'a> {
    w: &'a mut W,
}
impl<'a> _ASVISW<'a> {
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
#[doc = r" Proxy"]
pub struct _BSVISW<'a> {
    w: &'a mut W,
}
impl<'a> _BSVISW<'a> {
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
#[doc = r" Proxy"]
pub struct _BSEISW<'a> {
    w: &'a mut W,
}
impl<'a> _BSEISW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MS1SW<'a> {
    w: &'a mut W,
}
impl<'a> _MS1SW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPISW<'a> {
    w: &'a mut W,
}
impl<'a> _DPISW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _IDIEW<'a> {
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
pub struct _AVVIEW<'a> {
    w: &'a mut W,
}
impl<'a> _AVVIEW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ASVIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ASVIEW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BSVIEW<'a> {
    w: &'a mut W,
}
impl<'a> _BSVIEW<'a> {
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
#[doc = r" Proxy"]
pub struct _BSEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _BSEIEW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MS1EW<'a> {
    w: &'a mut W,
}
impl<'a> _MS1EW<'a> {
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
pub struct _DPIEW<'a> {
    w: &'a mut W,
}
impl<'a> _DPIEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - VBUS_Discharge Setting this bit to 1 causes VBUS to discharge through a resistor."]
    #[inline]
    pub fn vd(&self) -> VDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDR { bits }
    }
    #[doc = "Bit 1 - VBUS_Charge Setting this bit to 1 causes the VBUS line to be charged. This is used for VBUS pulsing during SRP."]
    #[inline]
    pub fn vc(&self) -> VCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VCR { bits }
    }
    #[doc = "Bit 2 - Hardware assist auto_reset"]
    #[inline]
    pub fn haar(&self) -> HAARR {
        HAARR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - OTG termination This bit must be set to 1 when the OTG controller is in device mode. This controls the pull-down on USB_DM."]
    #[inline]
    pub fn ot(&self) -> OTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTR { bits }
    }
    #[doc = "Bit 4 - Data pulsing Setting this bit to 1 causes the pull-up on USB_DP to be asserted for data pulsing during SRP."]
    #[inline]
    pub fn dp(&self) -> DPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DPR { bits }
    }
    #[doc = "Bit 5 - ID pull-up. This bit provides control over the pull-up resistor."]
    #[inline]
    pub fn idpu(&self) -> IDPUR {
        IDPUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Hardware assist data pulse Write a 1 to start data pulse sequence."]
    #[inline]
    pub fn hadp(&self) -> HADPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HADPR { bits }
    }
    #[doc = "Bit 7 - Hardware assist B-disconnect to A-connect"]
    #[inline]
    pub fn haba(&self) -> HABAR {
        HABAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - USB ID"]
    #[inline]
    pub fn id(&self) -> IDR {
        IDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - A-VBUS valid Reading 1 indicates that VBUS is above the A-VBUS valid threshold."]
    #[inline]
    pub fn avv(&self) -> AVVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AVVR { bits }
    }
    #[doc = "Bit 10 - A-session valid Reading 1 indicates that VBUS is above the A-session valid threshold."]
    #[inline]
    pub fn asv(&self) -> ASVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASVR { bits }
    }
    #[doc = "Bit 11 - B-session valid Reading 1 indicates that VBUS is above the B-session valid threshold."]
    #[inline]
    pub fn bsv(&self) -> BSVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BSVR { bits }
    }
    #[doc = "Bit 12 - B-session end Reading 1 indicates that VBUS is below the B-session end threshold."]
    #[inline]
    pub fn bse(&self) -> BSER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BSER { bits }
    }
    #[doc = "Bit 13 - 1 millisecond timer toggle This bit toggles once per millisecond."]
    #[inline]
    pub fn ms1t(&self) -> MS1TR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MS1TR { bits }
    }
    #[doc = "Bit 14 - Data bus pulsing status Reading a 1 indicates that data bus pulsing is detected on the port."]
    #[inline]
    pub fn dps(&self) -> DPSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DPSR { bits }
    }
    #[doc = "Bit 16 - USB ID interrupt status This bit is set when a change on the ID input has been detected. Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn idis(&self) -> IDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDISR { bits }
    }
    #[doc = "Bit 17 - A-VBUS valid interrupt status This bit is set then VBUS has either risen above or fallen below the A-VBUS valid threshold (4.4 V on an A-device). Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn avvis(&self) -> AVVISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AVVISR { bits }
    }
    #[doc = "Bit 18 - A-Session valid interrupt status This bit is set then VBUS has either risen above or fallen below the A-session valid threshold (0.8 V). Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn asvis(&self) -> ASVISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASVISR { bits }
    }
    #[doc = "Bit 19 - B-Session valid interrupt status This bit is set then VBUS has either risen above or fallen below the B-session valid threshold (0.8 V). Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn bsvis(&self) -> BSVISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BSVISR { bits }
    }
    #[doc = "Bit 20 - B-Session end interrupt status This bit is set then VBUS has fallen below the B-session end threshold. Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn bseis(&self) -> BSEISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BSEISR { bits }
    }
    #[doc = "Bit 21 - 1 millisecond timer interrupt status This bit is set once every millisecond. Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn ms1s(&self) -> MS1SR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MS1SR { bits }
    }
    #[doc = "Bit 22 - Data pulse interrupt status This bit is set when data bus pulsing occurs on DP or DM. Data bus pulsing is only detected when the CM bit in USBMODE = Host (11) and the PortPower bit in PORTSC = Off (0). Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn dpis(&self) -> DPISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DPISR { bits }
    }
    #[doc = "Bit 24 - USB ID interrupt enable Setting this bit enables the interrupt. Writing a 0 disables the interrupt."]
    #[inline]
    pub fn idie(&self) -> IDIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDIER { bits }
    }
    #[doc = "Bit 25 - A-VBUS valid interrupt enable Setting this bit enables the A-VBUS valid interrupt. Writing a 0 disables the interrupt."]
    #[inline]
    pub fn avvie(&self) -> AVVIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AVVIER { bits }
    }
    #[doc = "Bit 26 - A-session valid interrupt enable Setting this bit enables the A-session valid interrupt. Writing a 0 disables the interrupt"]
    #[inline]
    pub fn asvie(&self) -> ASVIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASVIER { bits }
    }
    #[doc = "Bit 27 - B-session valid interrupt enable Setting this bit enables the B-session valid interrupt. Writing a 0 disables the interrupt."]
    #[inline]
    pub fn bsvie(&self) -> BSVIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BSVIER { bits }
    }
    #[doc = "Bit 28 - B-session end interrupt enable Setting this bit enables the B-session end interrupt. Writing a 0 disables the interrupt."]
    #[inline]
    pub fn bseie(&self) -> BSEIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BSEIER { bits }
    }
    #[doc = "Bit 29 - 1 millisecond timer interrupt enable Setting this bit enables the 1 millisecond timer interrupt. Writing a 0 disables the interrupt."]
    #[inline]
    pub fn ms1e(&self) -> MS1ER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MS1ER { bits }
    }
    #[doc = "Bit 30 - Data pulse interrupt enable Setting this bit enables the data pulse interrupt. Writing a 0 disables the interrupt"]
    #[inline]
    pub fn dpie(&self) -> DPIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DPIER { bits }
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
    #[doc = "Bit 0 - VBUS_Discharge Setting this bit to 1 causes VBUS to discharge through a resistor."]
    #[inline]
    pub fn vd(&mut self) -> _VDW {
        _VDW { w: self }
    }
    #[doc = "Bit 1 - VBUS_Charge Setting this bit to 1 causes the VBUS line to be charged. This is used for VBUS pulsing during SRP."]
    #[inline]
    pub fn vc(&mut self) -> _VCW {
        _VCW { w: self }
    }
    #[doc = "Bit 2 - Hardware assist auto_reset"]
    #[inline]
    pub fn haar(&mut self) -> _HAARW {
        _HAARW { w: self }
    }
    #[doc = "Bit 3 - OTG termination This bit must be set to 1 when the OTG controller is in device mode. This controls the pull-down on USB_DM."]
    #[inline]
    pub fn ot(&mut self) -> _OTW {
        _OTW { w: self }
    }
    #[doc = "Bit 4 - Data pulsing Setting this bit to 1 causes the pull-up on USB_DP to be asserted for data pulsing during SRP."]
    #[inline]
    pub fn dp(&mut self) -> _DPW {
        _DPW { w: self }
    }
    #[doc = "Bit 5 - ID pull-up. This bit provides control over the pull-up resistor."]
    #[inline]
    pub fn idpu(&mut self) -> _IDPUW {
        _IDPUW { w: self }
    }
    #[doc = "Bit 6 - Hardware assist data pulse Write a 1 to start data pulse sequence."]
    #[inline]
    pub fn hadp(&mut self) -> _HADPW {
        _HADPW { w: self }
    }
    #[doc = "Bit 7 - Hardware assist B-disconnect to A-connect"]
    #[inline]
    pub fn haba(&mut self) -> _HABAW {
        _HABAW { w: self }
    }
    #[doc = "Bit 8 - USB ID"]
    #[inline]
    pub fn id(&mut self) -> _IDW {
        _IDW { w: self }
    }
    #[doc = "Bit 9 - A-VBUS valid Reading 1 indicates that VBUS is above the A-VBUS valid threshold."]
    #[inline]
    pub fn avv(&mut self) -> _AVVW {
        _AVVW { w: self }
    }
    #[doc = "Bit 10 - A-session valid Reading 1 indicates that VBUS is above the A-session valid threshold."]
    #[inline]
    pub fn asv(&mut self) -> _ASVW {
        _ASVW { w: self }
    }
    #[doc = "Bit 11 - B-session valid Reading 1 indicates that VBUS is above the B-session valid threshold."]
    #[inline]
    pub fn bsv(&mut self) -> _BSVW {
        _BSVW { w: self }
    }
    #[doc = "Bit 12 - B-session end Reading 1 indicates that VBUS is below the B-session end threshold."]
    #[inline]
    pub fn bse(&mut self) -> _BSEW {
        _BSEW { w: self }
    }
    #[doc = "Bit 13 - 1 millisecond timer toggle This bit toggles once per millisecond."]
    #[inline]
    pub fn ms1t(&mut self) -> _MS1TW {
        _MS1TW { w: self }
    }
    #[doc = "Bit 14 - Data bus pulsing status Reading a 1 indicates that data bus pulsing is detected on the port."]
    #[inline]
    pub fn dps(&mut self) -> _DPSW {
        _DPSW { w: self }
    }
    #[doc = "Bit 16 - USB ID interrupt status This bit is set when a change on the ID input has been detected. Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn idis(&mut self) -> _IDISW {
        _IDISW { w: self }
    }
    #[doc = "Bit 17 - A-VBUS valid interrupt status This bit is set then VBUS has either risen above or fallen below the A-VBUS valid threshold (4.4 V on an A-device). Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn avvis(&mut self) -> _AVVISW {
        _AVVISW { w: self }
    }
    #[doc = "Bit 18 - A-Session valid interrupt status This bit is set then VBUS has either risen above or fallen below the A-session valid threshold (0.8 V). Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn asvis(&mut self) -> _ASVISW {
        _ASVISW { w: self }
    }
    #[doc = "Bit 19 - B-Session valid interrupt status This bit is set then VBUS has either risen above or fallen below the B-session valid threshold (0.8 V). Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn bsvis(&mut self) -> _BSVISW {
        _BSVISW { w: self }
    }
    #[doc = "Bit 20 - B-Session end interrupt status This bit is set then VBUS has fallen below the B-session end threshold. Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn bseis(&mut self) -> _BSEISW {
        _BSEISW { w: self }
    }
    #[doc = "Bit 21 - 1 millisecond timer interrupt status This bit is set once every millisecond. Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn ms1s(&mut self) -> _MS1SW {
        _MS1SW { w: self }
    }
    #[doc = "Bit 22 - Data pulse interrupt status This bit is set when data bus pulsing occurs on DP or DM. Data bus pulsing is only detected when the CM bit in USBMODE = Host (11) and the PortPower bit in PORTSC = Off (0). Software must write a 1 to this bit to clear it."]
    #[inline]
    pub fn dpis(&mut self) -> _DPISW {
        _DPISW { w: self }
    }
    #[doc = "Bit 24 - USB ID interrupt enable Setting this bit enables the interrupt. Writing a 0 disables the interrupt."]
    #[inline]
    pub fn idie(&mut self) -> _IDIEW {
        _IDIEW { w: self }
    }
    #[doc = "Bit 25 - A-VBUS valid interrupt enable Setting this bit enables the A-VBUS valid interrupt. Writing a 0 disables the interrupt."]
    #[inline]
    pub fn avvie(&mut self) -> _AVVIEW {
        _AVVIEW { w: self }
    }
    #[doc = "Bit 26 - A-session valid interrupt enable Setting this bit enables the A-session valid interrupt. Writing a 0 disables the interrupt"]
    #[inline]
    pub fn asvie(&mut self) -> _ASVIEW {
        _ASVIEW { w: self }
    }
    #[doc = "Bit 27 - B-session valid interrupt enable Setting this bit enables the B-session valid interrupt. Writing a 0 disables the interrupt."]
    #[inline]
    pub fn bsvie(&mut self) -> _BSVIEW {
        _BSVIEW { w: self }
    }
    #[doc = "Bit 28 - B-session end interrupt enable Setting this bit enables the B-session end interrupt. Writing a 0 disables the interrupt."]
    #[inline]
    pub fn bseie(&mut self) -> _BSEIEW {
        _BSEIEW { w: self }
    }
    #[doc = "Bit 29 - 1 millisecond timer interrupt enable Setting this bit enables the 1 millisecond timer interrupt. Writing a 0 disables the interrupt."]
    #[inline]
    pub fn ms1e(&mut self) -> _MS1EW {
        _MS1EW { w: self }
    }
    #[doc = "Bit 30 - Data pulse interrupt enable Setting this bit enables the data pulse interrupt. Writing a 0 disables the interrupt"]
    #[inline]
    pub fn dpie(&mut self) -> _DPIEW {
        _DPIEW { w: self }
    }
}
