#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IER {
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
#[doc = "Possible values of the field `RBRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBRIER {
    #[doc = "Disable. Disable the RDA interrupts."]
    DISABLE,
    #[doc = "Enable. Enable the RDA interrupts."]
    ENABLE,
}
impl RBRIER {
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
            RBRIER::DISABLE => false,
            RBRIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RBRIER {
        match value {
            false => RBRIER::DISABLE,
            true => RBRIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RBRIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RBRIER::ENABLE
    }
}
#[doc = "Possible values of the field `THREIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREIER {
    #[doc = "Disable. Disable the THRE interrupts."]
    DISABLE,
    #[doc = "Enable. Enable the THRE interrupts."]
    ENABLE,
}
impl THREIER {
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
            THREIER::DISABLE => false,
            THREIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> THREIER {
        match value {
            false => THREIER::DISABLE,
            true => THREIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == THREIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == THREIER::ENABLE
    }
}
#[doc = "Possible values of the field `RXIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIER {
    #[doc = "Disable. Disable the RX line status interrupts."]
    DISABLE,
    #[doc = "Enable. Enable the RX line status interrupts."]
    ENABLE,
}
impl RXIER {
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
            RXIER::DISABLE => false,
            RXIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXIER {
        match value {
            false => RXIER::DISABLE,
            true => RXIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RXIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RXIER::ENABLE
    }
}
#[doc = "Possible values of the field `MSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSIER {
    #[doc = "Disable. Disable the modem interrupt."]
    DISABLE,
    #[doc = "Enable. Enable the modem interrupt."]
    ENABLE,
}
impl MSIER {
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
            MSIER::DISABLE => false,
            MSIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSIER {
        match value {
            false => MSIER::DISABLE,
            true => MSIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MSIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MSIER::ENABLE
    }
}
#[doc = "Possible values of the field `CTSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIER {
    #[doc = "Disable. Disable the CTS interrupt."]
    DISABLE,
    #[doc = "Enable. Enable the CTS interrupt."]
    ENABLE,
}
impl CTSIER {
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
            CTSIER::DISABLE => false,
            CTSIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSIER {
        match value {
            false => CTSIER::DISABLE,
            true => CTSIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CTSIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CTSIER::ENABLE
    }
}
#[doc = "Possible values of the field `ABEOIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOIER {
    #[doc = "Disable. Disable end of auto-baud Interrupt."]
    DISABLE,
    #[doc = "Enable. Enable end of auto-baud Interrupt."]
    ENABLE,
}
impl ABEOIER {
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
            ABEOIER::DISABLE => false,
            ABEOIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABEOIER {
        match value {
            false => ABEOIER::DISABLE,
            true => ABEOIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ABEOIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ABEOIER::ENABLE
    }
}
#[doc = "Possible values of the field `ABTOIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOIER {
    #[doc = "Disable. Disable auto-baud time-out Interrupt."]
    DISABLE,
    #[doc = "Enable. Enable auto-baud time-out Interrupt."]
    ENABLE,
}
impl ABTOIER {
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
            ABTOIER::DISABLE => false,
            ABTOIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABTOIER {
        match value {
            false => ABTOIER::DISABLE,
            true => ABTOIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ABTOIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ABTOIER::ENABLE
    }
}
#[doc = "Values that can be written to the field `RBRIE`"]
pub enum RBRIEW {
    #[doc = "Disable. Disable the RDA interrupts."]
    DISABLE,
    #[doc = "Enable. Enable the RDA interrupts."]
    ENABLE,
}
impl RBRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RBRIEW::DISABLE => false,
            RBRIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RBRIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RBRIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RBRIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. Disable the RDA interrupts."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RBRIEW::DISABLE)
    }
    #[doc = "Enable. Enable the RDA interrupts."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RBRIEW::ENABLE)
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
#[doc = "Values that can be written to the field `THREIE`"]
pub enum THREIEW {
    #[doc = "Disable. Disable the THRE interrupts."]
    DISABLE,
    #[doc = "Enable. Enable the THRE interrupts."]
    ENABLE,
}
impl THREIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            THREIEW::DISABLE => false,
            THREIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _THREIEW<'a> {
    w: &'a mut W,
}
impl<'a> _THREIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: THREIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. Disable the THRE interrupts."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(THREIEW::DISABLE)
    }
    #[doc = "Enable. Enable the THRE interrupts."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(THREIEW::ENABLE)
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
#[doc = "Values that can be written to the field `RXIE`"]
pub enum RXIEW {
    #[doc = "Disable. Disable the RX line status interrupts."]
    DISABLE,
    #[doc = "Enable. Enable the RX line status interrupts."]
    ENABLE,
}
impl RXIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXIEW::DISABLE => false,
            RXIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. Disable the RX line status interrupts."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXIEW::DISABLE)
    }
    #[doc = "Enable. Enable the RX line status interrupts."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXIEW::ENABLE)
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
#[doc = "Values that can be written to the field `MSIE`"]
pub enum MSIEW {
    #[doc = "Disable. Disable the modem interrupt."]
    DISABLE,
    #[doc = "Enable. Enable the modem interrupt."]
    ENABLE,
}
impl MSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSIEW::DISABLE => false,
            MSIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _MSIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. Disable the modem interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MSIEW::DISABLE)
    }
    #[doc = "Enable. Enable the modem interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MSIEW::ENABLE)
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
#[doc = "Values that can be written to the field `CTSIE`"]
pub enum CTSIEW {
    #[doc = "Disable. Disable the CTS interrupt."]
    DISABLE,
    #[doc = "Enable. Enable the CTS interrupt."]
    ENABLE,
}
impl CTSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSIEW::DISABLE => false,
            CTSIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. Disable the CTS interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTSIEW::DISABLE)
    }
    #[doc = "Enable. Enable the CTS interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTSIEW::ENABLE)
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
#[doc = "Values that can be written to the field `ABEOIE`"]
pub enum ABEOIEW {
    #[doc = "Disable. Disable end of auto-baud Interrupt."]
    DISABLE,
    #[doc = "Enable. Enable end of auto-baud Interrupt."]
    ENABLE,
}
impl ABEOIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABEOIEW::DISABLE => false,
            ABEOIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABEOIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ABEOIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABEOIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. Disable end of auto-baud Interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ABEOIEW::DISABLE)
    }
    #[doc = "Enable. Enable end of auto-baud Interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ABEOIEW::ENABLE)
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
#[doc = "Values that can be written to the field `ABTOIE`"]
pub enum ABTOIEW {
    #[doc = "Disable. Disable auto-baud time-out Interrupt."]
    DISABLE,
    #[doc = "Enable. Enable auto-baud time-out Interrupt."]
    ENABLE,
}
impl ABTOIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABTOIEW::DISABLE => false,
            ABTOIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABTOIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ABTOIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABTOIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. Disable auto-baud time-out Interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ABTOIEW::DISABLE)
    }
    #[doc = "Enable. Enable auto-baud time-out Interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ABTOIEW::ENABLE)
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt."]
    #[inline]
    pub fn rbrie(&self) -> RBRIER {
        RBRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR[5]."]
    #[inline]
    pub fn threie(&self) -> THREIER {
        THREIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR[4:1]."]
    #[inline]
    pub fn rxie(&self) -> RXIER {
        RXIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR[3:0]."]
    #[inline]
    pub fn msie(&self) -> MSIER {
        MSIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER[3]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER[3] bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER[3] and IER[7] bits are set."]
    #[inline]
    pub fn ctsie(&self) -> CTSIER {
        CTSIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline]
    pub fn abeoie(&self) -> ABEOIER {
        ABEOIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline]
    pub fn abtoie(&self) -> ABTOIER {
        ABTOIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt."]
    #[inline]
    pub fn rbrie(&mut self) -> _RBRIEW {
        _RBRIEW { w: self }
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR[5]."]
    #[inline]
    pub fn threie(&mut self) -> _THREIEW {
        _THREIEW { w: self }
    }
    #[doc = "Bit 2 - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR[4:1]."]
    #[inline]
    pub fn rxie(&mut self) -> _RXIEW {
        _RXIEW { w: self }
    }
    #[doc = "Bit 3 - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR[3:0]."]
    #[inline]
    pub fn msie(&mut self) -> _MSIEW {
        _MSIEW { w: self }
    }
    #[doc = "Bit 7 - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER[3]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER[3] bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER[3] and IER[7] bits are set."]
    #[inline]
    pub fn ctsie(&mut self) -> _CTSIEW {
        _CTSIEW { w: self }
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline]
    pub fn abeoie(&mut self) -> _ABEOIEW {
        _ABEOIEW { w: self }
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline]
    pub fn abtoie(&mut self) -> _ABTOIEW {
        _ABTOIEW { w: self }
    }
}
