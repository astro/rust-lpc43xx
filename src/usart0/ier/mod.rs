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
    #[doc = "Disable. Disable the RDA interrupt."]
    DISABLE,
    #[doc = "Enable. Enable the RDA interrupt."]
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
    #[doc = "Disable. Disable the THRE interrupt."]
    DISABLE,
    #[doc = "Enable. Enable the THRE interrupt."]
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
#[doc = "Possible values of the field `ABEOINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOINTENR {
    #[doc = "Disable. Disable end of auto-baud Interrupt."]
    DISABLE,
    #[doc = "Enable. Enable end of auto-baud Interrupt."]
    ENABLE,
}
impl ABEOINTENR {
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
            ABEOINTENR::DISABLE => false,
            ABEOINTENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABEOINTENR {
        match value {
            false => ABEOINTENR::DISABLE,
            true => ABEOINTENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ABEOINTENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ABEOINTENR::ENABLE
    }
}
#[doc = "Possible values of the field `ABTOINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOINTENR {
    #[doc = "Disable. Disable auto-baud time-out Interrupt."]
    DISABLE,
    #[doc = "Enable. Enable auto-baud time-out Interrupt."]
    ENABLE,
}
impl ABTOINTENR {
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
            ABTOINTENR::DISABLE => false,
            ABTOINTENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABTOINTENR {
        match value {
            false => ABTOINTENR::DISABLE,
            true => ABTOINTENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ABTOINTENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ABTOINTENR::ENABLE
    }
}
#[doc = "Values that can be written to the field `RBRIE`"]
pub enum RBRIEW {
    #[doc = "Disable. Disable the RDA interrupt."]
    DISABLE,
    #[doc = "Enable. Enable the RDA interrupt."]
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
    #[doc = "Disable. Disable the RDA interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RBRIEW::DISABLE)
    }
    #[doc = "Enable. Enable the RDA interrupt."]
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
    #[doc = "Disable. Disable the THRE interrupt."]
    DISABLE,
    #[doc = "Enable. Enable the THRE interrupt."]
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
    #[doc = "Disable. Disable the THRE interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(THREIEW::DISABLE)
    }
    #[doc = "Enable. Enable the THRE interrupt."]
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
#[doc = "Values that can be written to the field `ABEOINTEN`"]
pub enum ABEOINTENW {
    #[doc = "Disable. Disable end of auto-baud Interrupt."]
    DISABLE,
    #[doc = "Enable. Enable end of auto-baud Interrupt."]
    ENABLE,
}
impl ABEOINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABEOINTENW::DISABLE => false,
            ABEOINTENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABEOINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ABEOINTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABEOINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. Disable end of auto-baud Interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ABEOINTENW::DISABLE)
    }
    #[doc = "Enable. Enable end of auto-baud Interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ABEOINTENW::ENABLE)
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
#[doc = "Values that can be written to the field `ABTOINTEN`"]
pub enum ABTOINTENW {
    #[doc = "Disable. Disable auto-baud time-out Interrupt."]
    DISABLE,
    #[doc = "Enable. Enable auto-baud time-out Interrupt."]
    ENABLE,
}
impl ABTOINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABTOINTENW::DISABLE => false,
            ABTOINTENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABTOINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ABTOINTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABTOINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. Disable auto-baud time-out Interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ABTOINTENW::DISABLE)
    }
    #[doc = "Enable. Enable auto-baud time-out Interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ABTOINTENW::ENABLE)
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
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for USART. It also controls the Character Receive Time-out interrupt."]
    #[inline]
    pub fn rbrie(&self) -> RBRIER {
        RBRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for USART. The status of this interrupt can be read from LSR[5]."]
    #[inline]
    pub fn threie(&self) -> THREIER {
        THREIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RX Line Interrupt Enable. Enables the USART RX line status interrupts. The status of this interrupt can be read from LSR[4:1]."]
    #[inline]
    pub fn rxie(&self) -> RXIER {
        RXIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline]
    pub fn abeointen(&self) -> ABEOINTENR {
        ABEOINTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline]
    pub fn abtointen(&self) -> ABTOINTENR {
        ABTOINTENR::_from({
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
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for USART. It also controls the Character Receive Time-out interrupt."]
    #[inline]
    pub fn rbrie(&mut self) -> _RBRIEW {
        _RBRIEW { w: self }
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for USART. The status of this interrupt can be read from LSR[5]."]
    #[inline]
    pub fn threie(&mut self) -> _THREIEW {
        _THREIEW { w: self }
    }
    #[doc = "Bit 2 - RX Line Interrupt Enable. Enables the USART RX line status interrupts. The status of this interrupt can be read from LSR[4:1]."]
    #[inline]
    pub fn rxie(&mut self) -> _RXIEW {
        _RXIEW { w: self }
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline]
    pub fn abeointen(&mut self) -> _ABEOINTENW {
        _ABEOINTENW { w: self }
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline]
    pub fn abtointen(&mut self) -> _ABTOINTENW {
        _ABTOINTENW { w: self }
    }
}
