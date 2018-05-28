#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::BASE_SAFE_CLK {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDR {
    #[doc = "Enabled. Output stage enabled (default)"]
    ENABLED,
    #[doc = "Power-down"]
    POWER_DOWN,
}
impl PDR {
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
            PDR::ENABLED => false,
            PDR::POWER_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDR {
        match value {
            false => PDR::ENABLED,
            true => PDR::POWER_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PDR::ENABLED
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline]
    pub fn is_power_down(&self) -> bool {
        *self == PDR::POWER_DOWN
    }
}
#[doc = "Possible values of the field `AUTOBLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOBLOCKR {
    #[doc = "Disabled. Autoblocking disabled"]
    DISABLED,
    #[doc = "Enabled. Autoblocking enabled"]
    ENABLED,
}
impl AUTOBLOCKR {
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
            AUTOBLOCKR::DISABLED => false,
            AUTOBLOCKR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTOBLOCKR {
        match value {
            false => AUTOBLOCKR::DISABLED,
            true => AUTOBLOCKR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOBLOCKR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOBLOCKR::ENABLED
    }
}
#[doc = "Possible values of the field `CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_SELR {
    #[doc = "IRC (default)"]
    IRC_DEFAULT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_SELR::IRC_DEFAULT => 1,
            CLK_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_SELR {
        match value {
            1 => CLK_SELR::IRC_DEFAULT,
            i => CLK_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_DEFAULT`"]
    #[inline]
    pub fn is_irc_default(&self) -> bool {
        *self == CLK_SELR::IRC_DEFAULT
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Output stage power down"]
    #[inline]
    pub fn pd(&self) -> PDR {
        PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Block clock automatically during frequency change"]
    #[inline]
    pub fn autoblock(&self) -> AUTOBLOCKR {
        AUTOBLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:28 - Clock source selection. All other values are reserved."]
    #[inline]
    pub fn clk_sel(&self) -> CLK_SELR {
        CLK_SELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
