#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct TIMEOUTR {
    bits: u16,
}
impl TIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CSHIGHR {
    bits: u8,
}
impl CSHIGHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct D_PRFTCH_DISR {
    bits: bool,
}
impl D_PRFTCH_DISR {
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
pub struct INTENR {
    bits: bool,
}
impl INTENR {
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
#[doc = "Possible values of the field `MODE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE3R {
    #[doc = "SCK LOW. The SPIFI drives SCK low after the rising edge at which the last bit of each command is captured, and keeps it low while CS is HIGH."]
    SCK_LOW,
    #[doc = "SCK HIGH. the SPIFI keeps SCK high after the rising edge for the last bit of each command and while CS is HIGH, and drives it low after it drives  CS LOW. (Known serial flash devices can handle either mode, but some devices may require a particular mode for proper operation.)  MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final rising edge on SCK on which to sample the last data bit of the frame."]
    SCK_HIGH,
}
impl MODE3R {
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
            MODE3R::SCK_LOW => false,
            MODE3R::SCK_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODE3R {
        match value {
            false => MODE3R::SCK_LOW,
            true => MODE3R::SCK_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `SCK_LOW`"]
    #[inline]
    pub fn is_sck_low(&self) -> bool {
        *self == MODE3R::SCK_LOW
    }
    #[doc = "Checks if the value of the field is `SCK_HIGH`"]
    #[inline]
    pub fn is_sck_high(&self) -> bool {
        *self == MODE3R::SCK_HIGH
    }
}
#[doc = "Possible values of the field `PRFTCH_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTCH_DISR {
    #[doc = "Enable. Cache prefetching enabled."]
    ENABLE,
    #[doc = "Disable. Disables prefetching of cache lines."]
    DISABLE,
}
impl PRFTCH_DISR {
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
            PRFTCH_DISR::ENABLE => false,
            PRFTCH_DISR::DISABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRFTCH_DISR {
        match value {
            false => PRFTCH_DISR::ENABLE,
            true => PRFTCH_DISR::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PRFTCH_DISR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PRFTCH_DISR::DISABLE
    }
}
#[doc = "Possible values of the field `DUAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUALR {
    #[doc = "Quad protocol. This protocol uses IO3:0."]
    QUAD_PROTOCOL,
    #[doc = "Dual protocol. This protocol uses IO1:0."]
    DUAL_PROTOCOL,
}
impl DUALR {
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
            DUALR::QUAD_PROTOCOL => false,
            DUALR::DUAL_PROTOCOL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DUALR {
        match value {
            false => DUALR::QUAD_PROTOCOL,
            true => DUALR::DUAL_PROTOCOL,
        }
    }
    #[doc = "Checks if the value of the field is `QUAD_PROTOCOL`"]
    #[inline]
    pub fn is_quad_protocol(&self) -> bool {
        *self == DUALR::QUAD_PROTOCOL
    }
    #[doc = "Checks if the value of the field is `DUAL_PROTOCOL`"]
    #[inline]
    pub fn is_dual_protocol(&self) -> bool {
        *self == DUALR::DUAL_PROTOCOL
    }
}
#[doc = "Possible values of the field `RFCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCLKR {
    #[doc = "Rising edge. Read data is sampled on rising edges on the clock, as in classic SPI operation."]
    RISING_EDGE,
    #[doc = "Falling edge. Read data is sampled on falling edges of the clock, allowing a full serial clock of of time in order to maximize the serial clock frequency. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final rising edge on SCK on which to sample the last data bit of the frame."]
    FALLING_EDGE,
}
impl RFCLKR {
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
            RFCLKR::RISING_EDGE => false,
            RFCLKR::FALLING_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFCLKR {
        match value {
            false => RFCLKR::RISING_EDGE,
            true => RFCLKR::FALLING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == RFCLKR::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == RFCLKR::FALLING_EDGE
    }
}
#[doc = "Possible values of the field `FBCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBCLKR {
    #[doc = "Internal clock. The SPIFI samples read data using an internal clock."]
    INTERNAL_CLOCK,
    #[doc = "Feedback clock. Read data is sampled using a feedback clock from the SCK pin. This allows slightly more time for each received bit.  MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final rising edge on SCK on which to sample the last data bit of the frame."]
    FEEDBACK_CLOCK,
}
impl FBCLKR {
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
            FBCLKR::INTERNAL_CLOCK => false,
            FBCLKR::FEEDBACK_CLOCK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FBCLKR {
        match value {
            false => FBCLKR::INTERNAL_CLOCK,
            true => FBCLKR::FEEDBACK_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL_CLOCK`"]
    #[inline]
    pub fn is_internal_clock(&self) -> bool {
        *self == FBCLKR::INTERNAL_CLOCK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK_CLOCK`"]
    #[inline]
    pub fn is_feedback_clock(&self) -> bool {
        *self == FBCLKR::FEEDBACK_CLOCK
    }
}
#[doc = r" Value of the field"]
pub struct DMAENR {
    bits: bool,
}
impl DMAENR {
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
pub struct _TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _CSHIGHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _D_PRFTCH_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _D_PRFTCH_DISW<'a> {
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
pub struct _INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTENW<'a> {
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
#[doc = "Values that can be written to the field `MODE3`"]
pub enum MODE3W {
    #[doc = "SCK LOW. The SPIFI drives SCK low after the rising edge at which the last bit of each command is captured, and keeps it low while CS is HIGH."]
    SCK_LOW,
    #[doc = "SCK HIGH. the SPIFI keeps SCK high after the rising edge for the last bit of each command and while CS is HIGH, and drives it low after it drives  CS LOW. (Known serial flash devices can handle either mode, but some devices may require a particular mode for proper operation.)  MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final rising edge on SCK on which to sample the last data bit of the frame."]
    SCK_HIGH,
}
impl MODE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODE3W::SCK_LOW => false,
            MODE3W::SCK_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE3W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SCK LOW. The SPIFI drives SCK low after the rising edge at which the last bit of each command is captured, and keeps it low while CS is HIGH."]
    #[inline]
    pub fn sck_low(self) -> &'a mut W {
        self.variant(MODE3W::SCK_LOW)
    }
    #[doc = "SCK HIGH. the SPIFI keeps SCK high after the rising edge for the last bit of each command and while CS is HIGH, and drives it low after it drives CS LOW. (Known serial flash devices can handle either mode, but some devices may require a particular mode for proper operation.) MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final rising edge on SCK on which to sample the last data bit of the frame."]
    #[inline]
    pub fn sck_high(self) -> &'a mut W {
        self.variant(MODE3W::SCK_HIGH)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRFTCH_DIS`"]
pub enum PRFTCH_DISW {
    #[doc = "Enable. Cache prefetching enabled."]
    ENABLE,
    #[doc = "Disable. Disables prefetching of cache lines."]
    DISABLE,
}
impl PRFTCH_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRFTCH_DISW::ENABLE => false,
            PRFTCH_DISW::DISABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRFTCH_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _PRFTCH_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRFTCH_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable. Cache prefetching enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRFTCH_DISW::ENABLE)
    }
    #[doc = "Disable. Disables prefetching of cache lines."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRFTCH_DISW::DISABLE)
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
#[doc = "Values that can be written to the field `DUAL`"]
pub enum DUALW {
    #[doc = "Quad protocol. This protocol uses IO3:0."]
    QUAD_PROTOCOL,
    #[doc = "Dual protocol. This protocol uses IO1:0."]
    DUAL_PROTOCOL,
}
impl DUALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DUALW::QUAD_PROTOCOL => false,
            DUALW::DUAL_PROTOCOL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DUALW<'a> {
    w: &'a mut W,
}
impl<'a> _DUALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DUALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Quad protocol. This protocol uses IO3:0."]
    #[inline]
    pub fn quad_protocol(self) -> &'a mut W {
        self.variant(DUALW::QUAD_PROTOCOL)
    }
    #[doc = "Dual protocol. This protocol uses IO1:0."]
    #[inline]
    pub fn dual_protocol(self) -> &'a mut W {
        self.variant(DUALW::DUAL_PROTOCOL)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RFCLK`"]
pub enum RFCLKW {
    #[doc = "Rising edge. Read data is sampled on rising edges on the clock, as in classic SPI operation."]
    RISING_EDGE,
    #[doc = "Falling edge. Read data is sampled on falling edges of the clock, allowing a full serial clock of of time in order to maximize the serial clock frequency. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final rising edge on SCK on which to sample the last data bit of the frame."]
    FALLING_EDGE,
}
impl RFCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFCLKW::RISING_EDGE => false,
            RFCLKW::FALLING_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _RFCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rising edge. Read data is sampled on rising edges on the clock, as in classic SPI operation."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(RFCLKW::RISING_EDGE)
    }
    #[doc = "Falling edge. Read data is sampled on falling edges of the clock, allowing a full serial clock of of time in order to maximize the serial clock frequency. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final rising edge on SCK on which to sample the last data bit of the frame."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(RFCLKW::FALLING_EDGE)
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
#[doc = "Values that can be written to the field `FBCLK`"]
pub enum FBCLKW {
    #[doc = "Internal clock. The SPIFI samples read data using an internal clock."]
    INTERNAL_CLOCK,
    #[doc = "Feedback clock. Read data is sampled using a feedback clock from the SCK pin. This allows slightly more time for each received bit.  MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final rising edge on SCK on which to sample the last data bit of the frame."]
    FEEDBACK_CLOCK,
}
impl FBCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FBCLKW::INTERNAL_CLOCK => false,
            FBCLKW::FEEDBACK_CLOCK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FBCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _FBCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FBCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal clock. The SPIFI samples read data using an internal clock."]
    #[inline]
    pub fn internal_clock(self) -> &'a mut W {
        self.variant(FBCLKW::INTERNAL_CLOCK)
    }
    #[doc = "Feedback clock. Read data is sampled using a feedback clock from the SCK pin. This allows slightly more time for each received bit. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final rising edge on SCK on which to sample the last data bit of the frame."]
    #[inline]
    pub fn feedback_clock(self) -> &'a mut W {
        self.variant(FBCLKW::FEEDBACK_CLOCK)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
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
    #[doc = "Bits 0:15 - This field contains the number of serial clock periods without the processor reading data in memory mode, which will cause the SPIFI hardware to terminate the command by driving the CS pin high and negating the CMD bit in the Status register. (This allows the flash memory to enter a lower-power state.) If the processor reads data from the flash region after a time-out, the command in the Memory Command Register is issued again."]
    #[inline]
    pub fn timeout(&self) -> TIMEOUTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TIMEOUTR { bits }
    }
    #[doc = "Bits 16:19 - This field controls the minimum CS high time, expressed as a number of serial clock periods minus one."]
    #[inline]
    pub fn cshigh(&self) -> CSHIGHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CSHIGHR { bits }
    }
    #[doc = "Bit 21 - This bit allows conditioning of memory mode prefetches based on the AHB HPROT (instruction/data) access information. A 1 in this register means that the SPIFI will not attempt a speculative prefetch when it encounters data accesses."]
    #[inline]
    pub fn d_prftch_dis(&self) -> D_PRFTCH_DISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        D_PRFTCH_DISR { bits }
    }
    #[doc = "Bit 22 - If this bit is 1 when a command ends, the SPIFI will assert its interrupt request output. See INTRQ in the status register for further details."]
    #[inline]
    pub fn inten(&self) -> INTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTENR { bits }
    }
    #[doc = "Bit 23 - SPI Mode 3 select."]
    #[inline]
    pub fn mode3(&self) -> MODE3R {
        MODE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines."]
    #[inline]
    pub fn prftch_dis(&self) -> PRFTCH_DISR {
        PRFTCH_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Select dual protocol."]
    #[inline]
    pub fn dual(&self) -> DUALR {
        DUALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Select active clock edge for input data."]
    #[inline]
    pub fn rfclk(&self) -> RFCLKR {
        RFCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Feedback clock select."]
    #[inline]
    pub fn fbclk(&self) -> FBCLKR {
        FBCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - A 1 in this bit enables the DMA Request output from the SPIFI. Set this bit only when a DMA channel is used to transfer data in peripheral mode. Do not set this bit when a DMA channel is used for memory-to-memory transfers from the SPIFI memory area. DRQEN should only be used in Command mode."]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1074790399 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - This field contains the number of serial clock periods without the processor reading data in memory mode, which will cause the SPIFI hardware to terminate the command by driving the CS pin high and negating the CMD bit in the Status register. (This allows the flash memory to enter a lower-power state.) If the processor reads data from the flash region after a time-out, the command in the Memory Command Register is issued again."]
    #[inline]
    pub fn timeout(&mut self) -> _TIMEOUTW {
        _TIMEOUTW { w: self }
    }
    #[doc = "Bits 16:19 - This field controls the minimum CS high time, expressed as a number of serial clock periods minus one."]
    #[inline]
    pub fn cshigh(&mut self) -> _CSHIGHW {
        _CSHIGHW { w: self }
    }
    #[doc = "Bit 21 - This bit allows conditioning of memory mode prefetches based on the AHB HPROT (instruction/data) access information. A 1 in this register means that the SPIFI will not attempt a speculative prefetch when it encounters data accesses."]
    #[inline]
    pub fn d_prftch_dis(&mut self) -> _D_PRFTCH_DISW {
        _D_PRFTCH_DISW { w: self }
    }
    #[doc = "Bit 22 - If this bit is 1 when a command ends, the SPIFI will assert its interrupt request output. See INTRQ in the status register for further details."]
    #[inline]
    pub fn inten(&mut self) -> _INTENW {
        _INTENW { w: self }
    }
    #[doc = "Bit 23 - SPI Mode 3 select."]
    #[inline]
    pub fn mode3(&mut self) -> _MODE3W {
        _MODE3W { w: self }
    }
    #[doc = "Bit 27 - Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines."]
    #[inline]
    pub fn prftch_dis(&mut self) -> _PRFTCH_DISW {
        _PRFTCH_DISW { w: self }
    }
    #[doc = "Bit 28 - Select dual protocol."]
    #[inline]
    pub fn dual(&mut self) -> _DUALW {
        _DUALW { w: self }
    }
    #[doc = "Bit 29 - Select active clock edge for input data."]
    #[inline]
    pub fn rfclk(&mut self) -> _RFCLKW {
        _RFCLKW { w: self }
    }
    #[doc = "Bit 30 - Feedback clock select."]
    #[inline]
    pub fn fbclk(&mut self) -> _FBCLKW {
        _FBCLKW { w: self }
    }
    #[doc = "Bit 31 - A 1 in this bit enables the DMA Request output from the SPIFI. Set this bit only when a DMA channel is used to transfer data in peripheral mode. Do not set this bit when a DMA channel is used for memory-to-memory transfers from the SPIFI memory area. DRQEN should only be used in Command mode."]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
}
