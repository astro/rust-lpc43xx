#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYNCCTRL {
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
#[doc = "Possible values of the field `SYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl SYNCR {
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
            SYNCR::DISABLED => false,
            SYNCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCR {
        match value {
            false => SYNCR::DISABLED,
            true => SYNCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SYNCR::ENABLED
    }
}
#[doc = "Possible values of the field `CSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRCR {
    #[doc = "Slave mode. Synchronous slave mode (SCLK in)"]
    SLAVE_MODE,
    #[doc = "Master mode. Synchronous master mode (SCLK out)"]
    MASTER_MODE,
}
impl CSRCR {
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
            CSRCR::SLAVE_MODE => false,
            CSRCR::MASTER_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSRCR {
        match value {
            false => CSRCR::SLAVE_MODE,
            true => CSRCR::MASTER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_MODE`"]
    #[inline]
    pub fn is_slave_mode(&self) -> bool {
        *self == CSRCR::SLAVE_MODE
    }
    #[doc = "Checks if the value of the field is `MASTER_MODE`"]
    #[inline]
    pub fn is_master_mode(&self) -> bool {
        *self == CSRCR::MASTER_MODE
    }
}
#[doc = "Possible values of the field `FES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FESR {
    #[doc = "Rising. RxD is sampled on the rising edge of SCLK."]
    RISING,
    #[doc = "Falling. RxD is sampled on the falling edge of SCLK."]
    FALLING,
}
impl FESR {
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
            FESR::RISING => false,
            FESR::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FESR {
        match value {
            false => FESR::RISING,
            true => FESR::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == FESR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == FESR::FALLING
    }
}
#[doc = "Possible values of the field `TSBYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSBYPASSR {
    #[doc = "Synchronized. The input clock is synchronized prior to being used in clock edge detection logic."]
    SYNCHRONIZED,
    #[doc = "Not synchronized. The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    NOT_SYNCHRONIZED,
}
impl TSBYPASSR {
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
            TSBYPASSR::SYNCHRONIZED => false,
            TSBYPASSR::NOT_SYNCHRONIZED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSBYPASSR {
        match value {
            false => TSBYPASSR::SYNCHRONIZED,
            true => TSBYPASSR::NOT_SYNCHRONIZED,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONIZED`"]
    #[inline]
    pub fn is_synchronized(&self) -> bool {
        *self == TSBYPASSR::SYNCHRONIZED
    }
    #[doc = "Checks if the value of the field is `NOT_SYNCHRONIZED`"]
    #[inline]
    pub fn is_not_synchronized(&self) -> bool {
        *self == TSBYPASSR::NOT_SYNCHRONIZED
    }
}
#[doc = "Possible values of the field `CSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCENR {
    #[doc = "On character. SCLK cycles only when characters are being sent on TxD."]
    ON_CHARACTER,
    #[doc = "Continuously. SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)."]
    CONTINUOUSLY,
}
impl CSCENR {
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
            CSCENR::ON_CHARACTER => false,
            CSCENR::CONTINUOUSLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSCENR {
        match value {
            false => CSCENR::ON_CHARACTER,
            true => CSCENR::CONTINUOUSLY,
        }
    }
    #[doc = "Checks if the value of the field is `ON_CHARACTER`"]
    #[inline]
    pub fn is_on_character(&self) -> bool {
        *self == CSCENR::ON_CHARACTER
    }
    #[doc = "Checks if the value of the field is `CONTINUOUSLY`"]
    #[inline]
    pub fn is_continuously(&self) -> bool {
        *self == CSCENR::CONTINUOUSLY
    }
}
#[doc = "Possible values of the field `SSSDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSSDISR {
    #[doc = "Send. Send start and stop bits as in other modes."]
    SEND,
    #[doc = "Do not send. Do not send start/stop bits."]
    DO_NOT_SEND,
}
impl SSSDISR {
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
            SSSDISR::SEND => false,
            SSSDISR::DO_NOT_SEND => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSSDISR {
        match value {
            false => SSSDISR::SEND,
            true => SSSDISR::DO_NOT_SEND,
        }
    }
    #[doc = "Checks if the value of the field is `SEND`"]
    #[inline]
    pub fn is_send(&self) -> bool {
        *self == SSSDISR::SEND
    }
    #[doc = "Checks if the value of the field is `DO_NOT_SEND`"]
    #[inline]
    pub fn is_do_not_send(&self) -> bool {
        *self == SSSDISR::DO_NOT_SEND
    }
}
#[doc = "Possible values of the field `CCCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCCLRR {
    #[doc = "Software. CSCEN is under software control."]
    SOFTWARE,
    #[doc = "Hardware. Hardware clears CSCEN after each character is received."]
    HARDWARE,
}
impl CCCLRR {
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
            CCCLRR::SOFTWARE => false,
            CCCLRR::HARDWARE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCCLRR {
        match value {
            false => CCCLRR::SOFTWARE,
            true => CCCLRR::HARDWARE,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline]
    pub fn is_software(&self) -> bool {
        *self == CCCLRR::SOFTWARE
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline]
    pub fn is_hardware(&self) -> bool {
        *self == CCCLRR::HARDWARE
    }
}
#[doc = "Values that can be written to the field `SYNC`"]
pub enum SYNCW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl SYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCW::DISABLED => false,
            SYNCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYNCW::ENABLED)
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
#[doc = "Values that can be written to the field `CSRC`"]
pub enum CSRCW {
    #[doc = "Slave mode. Synchronous slave mode (SCLK in)"]
    SLAVE_MODE,
    #[doc = "Master mode. Synchronous master mode (SCLK out)"]
    MASTER_MODE,
}
impl CSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSRCW::SLAVE_MODE => false,
            CSRCW::MASTER_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave mode. Synchronous slave mode (SCLK in)"]
    #[inline]
    pub fn slave_mode(self) -> &'a mut W {
        self.variant(CSRCW::SLAVE_MODE)
    }
    #[doc = "Master mode. Synchronous master mode (SCLK out)"]
    #[inline]
    pub fn master_mode(self) -> &'a mut W {
        self.variant(CSRCW::MASTER_MODE)
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
#[doc = "Values that can be written to the field `FES`"]
pub enum FESW {
    #[doc = "Rising. RxD is sampled on the rising edge of SCLK."]
    RISING,
    #[doc = "Falling. RxD is sampled on the falling edge of SCLK."]
    FALLING,
}
impl FESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FESW::RISING => false,
            FESW::FALLING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FESW<'a> {
    w: &'a mut W,
}
impl<'a> _FESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rising. RxD is sampled on the rising edge of SCLK."]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(FESW::RISING)
    }
    #[doc = "Falling. RxD is sampled on the falling edge of SCLK."]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(FESW::FALLING)
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
#[doc = "Values that can be written to the field `TSBYPASS`"]
pub enum TSBYPASSW {
    #[doc = "Synchronized. The input clock is synchronized prior to being used in clock edge detection logic."]
    SYNCHRONIZED,
    #[doc = "Not synchronized. The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    NOT_SYNCHRONIZED,
}
impl TSBYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSBYPASSW::SYNCHRONIZED => false,
            TSBYPASSW::NOT_SYNCHRONIZED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSBYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _TSBYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSBYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronized. The input clock is synchronized prior to being used in clock edge detection logic."]
    #[inline]
    pub fn synchronized(self) -> &'a mut W {
        self.variant(TSBYPASSW::SYNCHRONIZED)
    }
    #[doc = "Not synchronized. The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    #[inline]
    pub fn not_synchronized(self) -> &'a mut W {
        self.variant(TSBYPASSW::NOT_SYNCHRONIZED)
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
#[doc = "Values that can be written to the field `CSCEN`"]
pub enum CSCENW {
    #[doc = "On character. SCLK cycles only when characters are being sent on TxD."]
    ON_CHARACTER,
    #[doc = "Continuously. SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)."]
    CONTINUOUSLY,
}
impl CSCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSCENW::ON_CHARACTER => false,
            CSCENW::CONTINUOUSLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSCENW<'a> {
    w: &'a mut W,
}
impl<'a> _CSCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On character. SCLK cycles only when characters are being sent on TxD."]
    #[inline]
    pub fn on_character(self) -> &'a mut W {
        self.variant(CSCENW::ON_CHARACTER)
    }
    #[doc = "Continuously. SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)."]
    #[inline]
    pub fn continuously(self) -> &'a mut W {
        self.variant(CSCENW::CONTINUOUSLY)
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
#[doc = "Values that can be written to the field `SSSDIS`"]
pub enum SSSDISW {
    #[doc = "Send. Send start and stop bits as in other modes."]
    SEND,
    #[doc = "Do not send. Do not send start/stop bits."]
    DO_NOT_SEND,
}
impl SSSDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSSDISW::SEND => false,
            SSSDISW::DO_NOT_SEND => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSSDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSSDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSSDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Send. Send start and stop bits as in other modes."]
    #[inline]
    pub fn send(self) -> &'a mut W {
        self.variant(SSSDISW::SEND)
    }
    #[doc = "Do not send. Do not send start/stop bits."]
    #[inline]
    pub fn do_not_send(self) -> &'a mut W {
        self.variant(SSSDISW::DO_NOT_SEND)
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
#[doc = "Values that can be written to the field `CCCLR`"]
pub enum CCCLRW {
    #[doc = "Software. CSCEN is under software control."]
    SOFTWARE,
    #[doc = "Hardware. Hardware clears CSCEN after each character is received."]
    HARDWARE,
}
impl CCCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCCLRW::SOFTWARE => false,
            CCCLRW::HARDWARE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CCCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software. CSCEN is under software control."]
    #[inline]
    pub fn software(self) -> &'a mut W {
        self.variant(CCCLRW::SOFTWARE)
    }
    #[doc = "Hardware. Hardware clears CSCEN after each character is received."]
    #[inline]
    pub fn hardware(self) -> &'a mut W {
        self.variant(CCCLRW::HARDWARE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline]
    pub fn sync(&self) -> SYNCR {
        SYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline]
    pub fn csrc(&self) -> CSRCR {
        CSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Edge sampling."]
    #[inline]
    pub fn fes(&self) -> FESR {
        FESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline]
    pub fn tsbypass(&self) -> TSBYPASSR {
        TSBYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline]
    pub fn cscen(&self) -> CSCENR {
        CSCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline]
    pub fn sssdis(&self) -> SSSDISR {
        SSSDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline]
    pub fn ccclr(&self) -> CCCLRR {
        CCCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline]
    pub fn sync(&mut self) -> _SYNCW {
        _SYNCW { w: self }
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline]
    pub fn csrc(&mut self) -> _CSRCW {
        _CSRCW { w: self }
    }
    #[doc = "Bit 2 - Edge sampling."]
    #[inline]
    pub fn fes(&mut self) -> _FESW {
        _FESW { w: self }
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline]
    pub fn tsbypass(&mut self) -> _TSBYPASSW {
        _TSBYPASSW { w: self }
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline]
    pub fn cscen(&mut self) -> _CSCENW {
        _CSCENW { w: self }
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline]
    pub fn sssdis(&mut self) -> _SSSDISW {
        _SSSDISW { w: self }
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline]
    pub fn ccclr(&mut self) -> _CCCLRW {
        _CCCLRW { w: self }
    }
}
