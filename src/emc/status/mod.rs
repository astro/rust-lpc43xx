#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BR {
    #[doc = "Idle. EMC is idle (warm reset value)."]
    IDLE,
    #[doc = "Busy. EMC is busy performing memory transactions, commands, auto-refresh cycles, or is in self-refresh mode (POR reset value)."]
    BUSY,
}
impl BR {
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
            BR::IDLE => false,
            BR::BUSY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BR {
        match value {
            false => BR::IDLE,
            true => BR::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == BR::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline]
    pub fn is_busy(&self) -> bool {
        *self == BR::BUSY
    }
}
#[doc = "Possible values of the field `S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR {
    #[doc = "Empty. Write buffers empty (POR reset value)"]
    EMPTY,
    #[doc = "Data. Write buffers contain data."]
    DATA,
}
impl SR {
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
            SR::EMPTY => false,
            SR::DATA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR {
        match value {
            false => SR::EMPTY,
            true => SR::DATA,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == SR::EMPTY
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline]
    pub fn is_data(&self) -> bool {
        *self == SR::DATA
    }
}
#[doc = "Possible values of the field `SA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAR {
    #[doc = "Normal mode."]
    NORMAL_MODE,
    #[doc = "Self-refresh mode. (POR reset value.)"]
    SELF_REFRESH_MODE,
}
impl SAR {
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
            SAR::NORMAL_MODE => false,
            SAR::SELF_REFRESH_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAR {
        match value {
            false => SAR::NORMAL_MODE,
            true => SAR::SELF_REFRESH_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE`"]
    #[inline]
    pub fn is_normal_mode(&self) -> bool {
        *self == SAR::NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `SELF_REFRESH_MODE`"]
    #[inline]
    pub fn is_self_refresh_mode(&self) -> bool {
        *self == SAR::SELF_REFRESH_MODE
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Busy indicator. This bit is used to ensure that the memory controller enters the low-power or disabled mode cleanly by determining if the memory controller is busy or not:"]
    #[inline]
    pub fn b(&self) -> BR {
        BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Write buffer status. This bit enables the EMC to enter low-power mode or disabled mode cleanly:"]
    #[inline]
    pub fn s(&self) -> SR {
        SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Self-refresh acknowledge. This bit indicates the operating mode of the EMC:"]
    #[inline]
    pub fn sa(&self) -> SAR {
        SAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
