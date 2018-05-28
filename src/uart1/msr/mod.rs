#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DCTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCTSR {
    #[doc = "No change. No change detected on modem input, CTS."]
    NO_CHANGE,
    #[doc = "State change. State change detected on modem input, CTS."]
    STATE_CHANGE,
}
impl DCTSR {
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
            DCTSR::NO_CHANGE => false,
            DCTSR::STATE_CHANGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCTSR {
        match value {
            false => DCTSR::NO_CHANGE,
            true => DCTSR::STATE_CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == DCTSR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE`"]
    #[inline]
    pub fn is_state_change(&self) -> bool {
        *self == DCTSR::STATE_CHANGE
    }
}
#[doc = "Possible values of the field `DDSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDSRR {
    #[doc = "No change. No change detected on modem input, DSR."]
    NO_CHANGE,
    #[doc = "State change. State change detected on modem input, DSR."]
    STATE_CHANGE,
}
impl DDSRR {
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
            DDSRR::NO_CHANGE => false,
            DDSRR::STATE_CHANGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DDSRR {
        match value {
            false => DDSRR::NO_CHANGE,
            true => DDSRR::STATE_CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == DDSRR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE`"]
    #[inline]
    pub fn is_state_change(&self) -> bool {
        *self == DDSRR::STATE_CHANGE
    }
}
#[doc = "Possible values of the field `TERI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERIR {
    #[doc = "No change. No change detected on modem input, RI."]
    NO_CHANGE,
    #[doc = "Rising. Low-to-high transition detected on RI."]
    RISING,
}
impl TERIR {
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
            TERIR::NO_CHANGE => false,
            TERIR::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TERIR {
        match value {
            false => TERIR::NO_CHANGE,
            true => TERIR::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == TERIR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == TERIR::RISING
    }
}
#[doc = "Possible values of the field `DDCD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDCDR {
    #[doc = "No change. No change detected on modem input, DCD."]
    NO_CHANGE,
    #[doc = "State change. State change detected on modem input, DCD."]
    STATE_CHANGE,
}
impl DDCDR {
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
            DDCDR::NO_CHANGE => false,
            DDCDR::STATE_CHANGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DDCDR {
        match value {
            false => DDCDR::NO_CHANGE,
            true => DDCDR::STATE_CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == DDCDR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE`"]
    #[inline]
    pub fn is_state_change(&self) -> bool {
        *self == DDCDR::STATE_CHANGE
    }
}
#[doc = r" Value of the field"]
pub struct CTSR {
    bits: bool,
}
impl CTSR {
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
pub struct DSRR {
    bits: bool,
}
impl DSRR {
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
pub struct RIR {
    bits: bool,
}
impl RIR {
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
pub struct DCDR {
    bits: bool,
}
impl DCDR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Delta CTS. Set upon state change of input CTS. Cleared on an MSR read."]
    #[inline]
    pub fn dcts(&self) -> DCTSR {
        DCTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Delta DSR. Set upon state change of input DSR. Cleared on an MSR read."]
    #[inline]
    pub fn ddsr(&self) -> DDSRR {
        DDSRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read."]
    #[inline]
    pub fn teri(&self) -> TERIR {
        TERIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Delta DCD. Set upon state change of input DCD. Cleared on an MSR read."]
    #[inline]
    pub fn ddcd(&self) -> DDCDR {
        DDCDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Clear To Send State. Complement of input signal CTS. This bit is connected to MCR[1] in modem loopback mode."]
    #[inline]
    pub fn cts(&self) -> CTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTSR { bits }
    }
    #[doc = "Bit 5 - Data Set Ready State. Complement of input signal DSR. This bit is connected to MCR[0] in modem loopback mode."]
    #[inline]
    pub fn dsr(&self) -> DSRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSRR { bits }
    }
    #[doc = "Bit 6 - Ring Indicator State. Complement of input RI. This bit is connected to MCR[2] in modem loopback mode."]
    #[inline]
    pub fn ri(&self) -> RIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RIR { bits }
    }
    #[doc = "Bit 7 - Data Carrier Detect State. Complement of input DCD. This bit is connected to MCR[3] in modem loopback mode."]
    #[inline]
    pub fn dcd(&self) -> DCDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDR { bits }
    }
}
