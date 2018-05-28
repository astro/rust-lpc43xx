#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HILO {
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
#[doc = "Possible values of the field `WAKEUP0_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP0_LR {
    #[doc = "Detect LOW level on the WAKEUP0 pin if bit 0 in the EDGE register is 0. Detect falling edge if bit 0 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level on the WAKEUP0 pin if bit 0 in the EDGE register is 0. Detect rising edge if bit 0 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl WAKEUP0_LR {
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
            WAKEUP0_LR::DETECT_LOW_LEVEL => false,
            WAKEUP0_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP0_LR {
        match value {
            false => WAKEUP0_LR::DETECT_LOW_LEVEL,
            true => WAKEUP0_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == WAKEUP0_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == WAKEUP0_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `WAKEUP1_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP1_LR {
    #[doc = "Detect LOW level on the WAKEUP1 pin if bit 1 in the EDGE register is 0."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level on the WAKEUP1 pin if bit 1 in the EDGE register is 0. Detect rising edge if bit 1 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl WAKEUP1_LR {
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
            WAKEUP1_LR::DETECT_LOW_LEVEL => false,
            WAKEUP1_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP1_LR {
        match value {
            false => WAKEUP1_LR::DETECT_LOW_LEVEL,
            true => WAKEUP1_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == WAKEUP1_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == WAKEUP1_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `WAKEUP2_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP2_LR {
    #[doc = "Detect LOW level on the WAKEUP2 pin if bit 2 in the EDGE register is 0. Detect falling edge if bit 2 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level on the WAKEUP2 pin if bit 2 in the EDGE register is 0. Detect rising edge if bit 2 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl WAKEUP2_LR {
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
            WAKEUP2_LR::DETECT_LOW_LEVEL => false,
            WAKEUP2_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP2_LR {
        match value {
            false => WAKEUP2_LR::DETECT_LOW_LEVEL,
            true => WAKEUP2_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == WAKEUP2_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == WAKEUP2_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `WAKEUP3_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP3_LR {
    #[doc = "Detect LOW level on the WAKEUP3 pin if bit 3 in the EDGE register is 0. Detect falling edge if bit 3 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level on the WAKEUP3 pin if bit 3 in the EDGE register is 0. Detect rising edge if bit 3 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl WAKEUP3_LR {
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
            WAKEUP3_LR::DETECT_LOW_LEVEL => false,
            WAKEUP3_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP3_LR {
        match value {
            false => WAKEUP3_LR::DETECT_LOW_LEVEL,
            true => WAKEUP3_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == WAKEUP3_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == WAKEUP3_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `ATIMER_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATIMER_LR {
    #[doc = "Detect LOW level of the alarm timer interrupt if bit 4 in the EDGE register is 0. Detect falling edge if bit 4 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the alarm timer interrupt if bit 4 in the EDGE register is 0. Detect rising edge if bit 4 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl ATIMER_LR {
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
            ATIMER_LR::DETECT_LOW_LEVEL => false,
            ATIMER_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATIMER_LR {
        match value {
            false => ATIMER_LR::DETECT_LOW_LEVEL,
            true => ATIMER_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == ATIMER_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == ATIMER_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `RTC_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_LR {
    #[doc = "Detect LOW level of the RTC interrupt if bit 5 in the EDGE register is 0. Detect falling edge if bit 5 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the RTC interrupt if bit 5 in the EDGE register is 0. Detect rising edge if bit 5 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl RTC_LR {
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
            RTC_LR::DETECT_LOW_LEVEL => false,
            RTC_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_LR {
        match value {
            false => RTC_LR::DETECT_LOW_LEVEL,
            true => RTC_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == RTC_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == RTC_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `BOD_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_LR {
    #[doc = "Detect LOW level of the BOD interrupt if bit 6 in the EDGE register is 0. Detect falling edge if bit 6 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the BOD interrupt if bit 6 in the EDGE register is 0. Detect rising edge if bit 6 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl BOD_LR {
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
            BOD_LR::DETECT_LOW_LEVEL => false,
            BOD_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOD_LR {
        match value {
            false => BOD_LR::DETECT_LOW_LEVEL,
            true => BOD_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == BOD_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == BOD_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `WWDT_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_LR {
    #[doc = "Detect LOW level of the WWDT interrupt if bit 7 in the EDGE register is 0. Detect falling edge if bit 7 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the WWDT interrupt if bit 7 in the EDGE register is 0. Detect rising edge if bit 7 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl WWDT_LR {
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
            WWDT_LR::DETECT_LOW_LEVEL => false,
            WWDT_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WWDT_LR {
        match value {
            false => WWDT_LR::DETECT_LOW_LEVEL,
            true => WWDT_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == WWDT_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == WWDT_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `ETH_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH_LR {
    #[doc = "Detect LOW level of the Ethernet interrupt if bit 8 in the EDGE register is 0. Detect falling edge if bit 8 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the Ethernet interrupt if bit 8 in the EDGE register is 0. Detect rising edge if bit 8 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl ETH_LR {
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
            ETH_LR::DETECT_LOW_LEVEL => false,
            ETH_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETH_LR {
        match value {
            false => ETH_LR::DETECT_LOW_LEVEL,
            true => ETH_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == ETH_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == ETH_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `USB0_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_LR {
    #[doc = "Detect LOW level of the USB0 interrupt if bit 9 in the EDGE register is 0. Detect falling edge if bit 9 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the USB0 interrupt if bit 9 in the EDGE register is 0. Detect rising edge if bit 9 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl USB0_LR {
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
            USB0_LR::DETECT_LOW_LEVEL => false,
            USB0_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0_LR {
        match value {
            false => USB0_LR::DETECT_LOW_LEVEL,
            true => USB0_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == USB0_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == USB0_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `USB1_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_LR {
    #[doc = "Detect LOW level of the USB1 interrupt if bit 10 in the EDGE register is 0. Detect falling edge if bit 10 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the USB1 interrupt if bit 10 in the EDGE register is 0. Detect rising edge if bit 10 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl USB1_LR {
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
            USB1_LR::DETECT_LOW_LEVEL => false,
            USB1_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_LR {
        match value {
            false => USB1_LR::DETECT_LOW_LEVEL,
            true => USB1_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == USB1_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == USB1_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `SDMMC_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMC_LR {
    #[doc = "Detect LOW level of the SD/MMC interrupt if bit 11 in the EDGE register is 0. Detect falling edge if bit 11 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the SD/MMC interrupt if bit 11 in the EDGE register is 0. Detect rising edge if bit 11 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl SDMMC_LR {
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
            SDMMC_LR::DETECT_LOW_LEVEL => false,
            SDMMC_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDMMC_LR {
        match value {
            false => SDMMC_LR::DETECT_LOW_LEVEL,
            true => SDMMC_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == SDMMC_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == SDMMC_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `CAN_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN_LR {
    #[doc = "Detect LOW level of the combined C_CAN interrupt if bit 12 in the EDGE register is 0. Detect falling edge if bit 12 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the combined C_CAN interrupt if bit 12 in the EDGE register is 0. Detect rising edge if bit 12 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl CAN_LR {
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
            CAN_LR::DETECT_LOW_LEVEL => false,
            CAN_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAN_LR {
        match value {
            false => CAN_LR::DETECT_LOW_LEVEL,
            true => CAN_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == CAN_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == CAN_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `TIM2_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2_LR {
    #[doc = "Detect LOW level GIMA output 25 if bit 13 in the EDGE register is 0. Detect falling edge if bit 13 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level GIMA output 25 if bit 13 in the EDGE register is 0. Detect rising edge if bit 13 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl TIM2_LR {
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
            TIM2_LR::DETECT_LOW_LEVEL => false,
            TIM2_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM2_LR {
        match value {
            false => TIM2_LR::DETECT_LOW_LEVEL,
            true => TIM2_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == TIM2_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == TIM2_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `TIM6_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM6_LR {
    #[doc = "Detect LOW level of GIMA output 26 if bit 14 in the EDGE register is 0. Detect falling edge if bit 14 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of GIMA output 26 if bit 14 in the EDGE register is 0. Detect rising edge if bit 14 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl TIM6_LR {
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
            TIM6_LR::DETECT_LOW_LEVEL => false,
            TIM6_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM6_LR {
        match value {
            false => TIM6_LR::DETECT_LOW_LEVEL,
            true => TIM6_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == TIM6_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == TIM6_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `QEI_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QEI_LR {
    #[doc = "Detect LOW level of the QEI interrupt if bit 15 in the EDGE register is 0. Detect falling edge if bit 15 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the QEI interrupt if bit 15 in the EDGE register is 0. Detect rising edge if bit 15 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl QEI_LR {
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
            QEI_LR::DETECT_LOW_LEVEL => false,
            QEI_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QEI_LR {
        match value {
            false => QEI_LR::DETECT_LOW_LEVEL,
            true => QEI_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == QEI_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == QEI_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `TIM14_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM14_LR {
    #[doc = "Detect LOW level of GIMA output 27 if bit 16 in the EDGE register is 0. Detect falling edge if bit 16 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of GIMA output 27 if bit 16 in the EDGE register is 0. Detect rising edge if bit 16 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl TIM14_LR {
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
            TIM14_LR::DETECT_LOW_LEVEL => false,
            TIM14_LR::DETECT_HIGH_LEVEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM14_LR {
        match value {
            false => TIM14_LR::DETECT_LOW_LEVEL,
            true => TIM14_LR::DETECT_HIGH_LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL`"]
    #[inline]
    pub fn is_detect_low_level(&self) -> bool {
        *self == TIM14_LR::DETECT_LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL`"]
    #[inline]
    pub fn is_detect_high_level(&self) -> bool {
        *self == TIM14_LR::DETECT_HIGH_LEVEL
    }
}
#[doc = "Possible values of the field `RESET_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_LR {
    #[doc = "Detect LOW level if bit 17 in the EDGE register is 0. Detect falling edge if bit 17 in the EDGE register is 1."]
    DETECT_LOW_LEVEL_IF,
    #[doc = "Detect HIGH level if bit 17 in the EDGE register is 0. Detect rising edge if bit 17 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL_IF,
}
impl RESET_LR {
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
            RESET_LR::DETECT_LOW_LEVEL_IF => false,
            RESET_LR::DETECT_HIGH_LEVEL_IF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESET_LR {
        match value {
            false => RESET_LR::DETECT_LOW_LEVEL_IF,
            true => RESET_LR::DETECT_HIGH_LEVEL_IF,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL_IF`"]
    #[inline]
    pub fn is_detect_low_level_if(&self) -> bool {
        *self == RESET_LR::DETECT_LOW_LEVEL_IF
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL_IF`"]
    #[inline]
    pub fn is_detect_high_level_if(&self) -> bool {
        *self == RESET_LR::DETECT_HIGH_LEVEL_IF
    }
}
#[doc = "Possible values of the field `BODRESET_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRESET_LR {
    #[doc = "Detect LOW level if bit 20 in the EDGE register is 0. Detect falling edge if bit 20 in the EDGE register is 1."]
    DETECT_LOW_LEVEL_IF,
    #[doc = "Detect HIGH level if bit 20 in the EDGE register is 0. Detect rising edge if bit 20 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL_IF,
}
impl BODRESET_LR {
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
            BODRESET_LR::DETECT_LOW_LEVEL_IF => false,
            BODRESET_LR::DETECT_HIGH_LEVEL_IF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODRESET_LR {
        match value {
            false => BODRESET_LR::DETECT_LOW_LEVEL_IF,
            true => BODRESET_LR::DETECT_HIGH_LEVEL_IF,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL_IF`"]
    #[inline]
    pub fn is_detect_low_level_if(&self) -> bool {
        *self == BODRESET_LR::DETECT_LOW_LEVEL_IF
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL_IF`"]
    #[inline]
    pub fn is_detect_high_level_if(&self) -> bool {
        *self == BODRESET_LR::DETECT_HIGH_LEVEL_IF
    }
}
#[doc = "Possible values of the field `DPDRESET_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDRESET_LR {
    #[doc = "Detect LOW level if bit 21 in the EDGE register is 0. Detect falling edge if bit 21 in the EDGE register is 1."]
    DETECT_LOW_LEVEL_IF,
    #[doc = "Detect HIGH level if bit 21 in the EDGE register is 0. Detect rising edge if bit 21 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL_IF,
}
impl DPDRESET_LR {
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
            DPDRESET_LR::DETECT_LOW_LEVEL_IF => false,
            DPDRESET_LR::DETECT_HIGH_LEVEL_IF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPDRESET_LR {
        match value {
            false => DPDRESET_LR::DETECT_LOW_LEVEL_IF,
            true => DPDRESET_LR::DETECT_HIGH_LEVEL_IF,
        }
    }
    #[doc = "Checks if the value of the field is `DETECT_LOW_LEVEL_IF`"]
    #[inline]
    pub fn is_detect_low_level_if(&self) -> bool {
        *self == DPDRESET_LR::DETECT_LOW_LEVEL_IF
    }
    #[doc = "Checks if the value of the field is `DETECT_HIGH_LEVEL_IF`"]
    #[inline]
    pub fn is_detect_high_level_if(&self) -> bool {
        *self == DPDRESET_LR::DETECT_HIGH_LEVEL_IF
    }
}
#[doc = "Values that can be written to the field `WAKEUP0_L`"]
pub enum WAKEUP0_LW {
    #[doc = "Detect LOW level on the WAKEUP0 pin if bit 0 in the EDGE register is 0. Detect falling edge if bit 0 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level on the WAKEUP0 pin if bit 0 in the EDGE register is 0. Detect rising edge if bit 0 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl WAKEUP0_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP0_LW::DETECT_LOW_LEVEL => false,
            WAKEUP0_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP0_LW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP0_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP0_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level on the WAKEUP0 pin if bit 0 in the EDGE register is 0. Detect falling edge if bit 0 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(WAKEUP0_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level on the WAKEUP0 pin if bit 0 in the EDGE register is 0. Detect rising edge if bit 0 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(WAKEUP0_LW::DETECT_HIGH_LEVEL)
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
#[doc = "Values that can be written to the field `WAKEUP1_L`"]
pub enum WAKEUP1_LW {
    #[doc = "Detect LOW level on the WAKEUP1 pin if bit 1 in the EDGE register is 0."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level on the WAKEUP1 pin if bit 1 in the EDGE register is 0. Detect rising edge if bit 1 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl WAKEUP1_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP1_LW::DETECT_LOW_LEVEL => false,
            WAKEUP1_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP1_LW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP1_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP1_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level on the WAKEUP1 pin if bit 1 in the EDGE register is 0."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(WAKEUP1_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level on the WAKEUP1 pin if bit 1 in the EDGE register is 0. Detect rising edge if bit 1 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(WAKEUP1_LW::DETECT_HIGH_LEVEL)
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
#[doc = "Values that can be written to the field `WAKEUP2_L`"]
pub enum WAKEUP2_LW {
    #[doc = "Detect LOW level on the WAKEUP2 pin if bit 2 in the EDGE register is 0. Detect falling edge if bit 2 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level on the WAKEUP2 pin if bit 2 in the EDGE register is 0. Detect rising edge if bit 2 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl WAKEUP2_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP2_LW::DETECT_LOW_LEVEL => false,
            WAKEUP2_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP2_LW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP2_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP2_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level on the WAKEUP2 pin if bit 2 in the EDGE register is 0. Detect falling edge if bit 2 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(WAKEUP2_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level on the WAKEUP2 pin if bit 2 in the EDGE register is 0. Detect rising edge if bit 2 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(WAKEUP2_LW::DETECT_HIGH_LEVEL)
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
#[doc = "Values that can be written to the field `WAKEUP3_L`"]
pub enum WAKEUP3_LW {
    #[doc = "Detect LOW level on the WAKEUP3 pin if bit 3 in the EDGE register is 0. Detect falling edge if bit 3 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level on the WAKEUP3 pin if bit 3 in the EDGE register is 0. Detect rising edge if bit 3 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl WAKEUP3_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP3_LW::DETECT_LOW_LEVEL => false,
            WAKEUP3_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP3_LW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP3_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP3_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level on the WAKEUP3 pin if bit 3 in the EDGE register is 0. Detect falling edge if bit 3 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(WAKEUP3_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level on the WAKEUP3 pin if bit 3 in the EDGE register is 0. Detect rising edge if bit 3 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(WAKEUP3_LW::DETECT_HIGH_LEVEL)
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
#[doc = "Values that can be written to the field `ATIMER_L`"]
pub enum ATIMER_LW {
    #[doc = "Detect LOW level of the alarm timer interrupt if bit 4 in the EDGE register is 0. Detect falling edge if bit 4 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the alarm timer interrupt if bit 4 in the EDGE register is 0. Detect rising edge if bit 4 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl ATIMER_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATIMER_LW::DETECT_LOW_LEVEL => false,
            ATIMER_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATIMER_LW<'a> {
    w: &'a mut W,
}
impl<'a> _ATIMER_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATIMER_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level of the alarm timer interrupt if bit 4 in the EDGE register is 0. Detect falling edge if bit 4 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(ATIMER_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level of the alarm timer interrupt if bit 4 in the EDGE register is 0. Detect rising edge if bit 4 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(ATIMER_LW::DETECT_HIGH_LEVEL)
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
#[doc = "Values that can be written to the field `RTC_L`"]
pub enum RTC_LW {
    #[doc = "Detect LOW level of the RTC interrupt if bit 5 in the EDGE register is 0. Detect falling edge if bit 5 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the RTC interrupt if bit 5 in the EDGE register is 0. Detect rising edge if bit 5 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl RTC_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_LW::DETECT_LOW_LEVEL => false,
            RTC_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_LW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level of the RTC interrupt if bit 5 in the EDGE register is 0. Detect falling edge if bit 5 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(RTC_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level of the RTC interrupt if bit 5 in the EDGE register is 0. Detect rising edge if bit 5 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(RTC_LW::DETECT_HIGH_LEVEL)
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
#[doc = "Values that can be written to the field `BOD_L`"]
pub enum BOD_LW {
    #[doc = "Detect LOW level of the BOD interrupt if bit 6 in the EDGE register is 0. Detect falling edge if bit 6 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the BOD interrupt if bit 6 in the EDGE register is 0. Detect rising edge if bit 6 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl BOD_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOD_LW::DETECT_LOW_LEVEL => false,
            BOD_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOD_LW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOD_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level of the BOD interrupt if bit 6 in the EDGE register is 0. Detect falling edge if bit 6 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(BOD_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level of the BOD interrupt if bit 6 in the EDGE register is 0. Detect rising edge if bit 6 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(BOD_LW::DETECT_HIGH_LEVEL)
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
#[doc = "Values that can be written to the field `WWDT_L`"]
pub enum WWDT_LW {
    #[doc = "Detect LOW level of the WWDT interrupt if bit 7 in the EDGE register is 0. Detect falling edge if bit 7 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the WWDT interrupt if bit 7 in the EDGE register is 0. Detect rising edge if bit 7 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl WWDT_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WWDT_LW::DETECT_LOW_LEVEL => false,
            WWDT_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WWDT_LW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDT_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WWDT_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level of the WWDT interrupt if bit 7 in the EDGE register is 0. Detect falling edge if bit 7 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(WWDT_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level of the WWDT interrupt if bit 7 in the EDGE register is 0. Detect rising edge if bit 7 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(WWDT_LW::DETECT_HIGH_LEVEL)
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
#[doc = "Values that can be written to the field `ETH_L`"]
pub enum ETH_LW {
    #[doc = "Detect LOW level of the Ethernet interrupt if bit 8 in the EDGE register is 0. Detect falling edge if bit 8 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the Ethernet interrupt if bit 8 in the EDGE register is 0. Detect rising edge if bit 8 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl ETH_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETH_LW::DETECT_LOW_LEVEL => false,
            ETH_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETH_LW<'a> {
    w: &'a mut W,
}
impl<'a> _ETH_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETH_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level of the Ethernet interrupt if bit 8 in the EDGE register is 0. Detect falling edge if bit 8 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(ETH_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level of the Ethernet interrupt if bit 8 in the EDGE register is 0. Detect rising edge if bit 8 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(ETH_LW::DETECT_HIGH_LEVEL)
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
#[doc = "Values that can be written to the field `USB0_L`"]
pub enum USB0_LW {
    #[doc = "Detect LOW level of the USB0 interrupt if bit 9 in the EDGE register is 0. Detect falling edge if bit 9 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the USB0 interrupt if bit 9 in the EDGE register is 0. Detect rising edge if bit 9 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl USB0_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0_LW::DETECT_LOW_LEVEL => false,
            USB0_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0_LW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level of the USB0 interrupt if bit 9 in the EDGE register is 0. Detect falling edge if bit 9 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(USB0_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level of the USB0 interrupt if bit 9 in the EDGE register is 0. Detect rising edge if bit 9 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(USB0_LW::DETECT_HIGH_LEVEL)
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
#[doc = "Values that can be written to the field `USB1_L`"]
pub enum USB1_LW {
    #[doc = "Detect LOW level of the USB1 interrupt if bit 10 in the EDGE register is 0. Detect falling edge if bit 10 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the USB1 interrupt if bit 10 in the EDGE register is 0. Detect rising edge if bit 10 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl USB1_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_LW::DETECT_LOW_LEVEL => false,
            USB1_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_LW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level of the USB1 interrupt if bit 10 in the EDGE register is 0. Detect falling edge if bit 10 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(USB1_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level of the USB1 interrupt if bit 10 in the EDGE register is 0. Detect rising edge if bit 10 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(USB1_LW::DETECT_HIGH_LEVEL)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDMMC_L`"]
pub enum SDMMC_LW {
    #[doc = "Detect LOW level of the SD/MMC interrupt if bit 11 in the EDGE register is 0. Detect falling edge if bit 11 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the SD/MMC interrupt if bit 11 in the EDGE register is 0. Detect rising edge if bit 11 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl SDMMC_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDMMC_LW::DETECT_LOW_LEVEL => false,
            SDMMC_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDMMC_LW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMMC_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDMMC_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level of the SD/MMC interrupt if bit 11 in the EDGE register is 0. Detect falling edge if bit 11 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(SDMMC_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level of the SD/MMC interrupt if bit 11 in the EDGE register is 0. Detect rising edge if bit 11 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(SDMMC_LW::DETECT_HIGH_LEVEL)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAN_L`"]
pub enum CAN_LW {
    #[doc = "Detect LOW level of the combined C_CAN interrupt if bit 12 in the EDGE register is 0. Detect falling edge if bit 12 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the combined C_CAN interrupt if bit 12 in the EDGE register is 0. Detect rising edge if bit 12 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl CAN_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAN_LW::DETECT_LOW_LEVEL => false,
            CAN_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAN_LW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level of the combined C_CAN interrupt if bit 12 in the EDGE register is 0. Detect falling edge if bit 12 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(CAN_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level of the combined C_CAN interrupt if bit 12 in the EDGE register is 0. Detect rising edge if bit 12 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(CAN_LW::DETECT_HIGH_LEVEL)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM2_L`"]
pub enum TIM2_LW {
    #[doc = "Detect LOW level GIMA output 25 if bit 13 in the EDGE register is 0. Detect falling edge if bit 13 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level GIMA output 25 if bit 13 in the EDGE register is 0. Detect rising edge if bit 13 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl TIM2_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM2_LW::DETECT_LOW_LEVEL => false,
            TIM2_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM2_LW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM2_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM2_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level GIMA output 25 if bit 13 in the EDGE register is 0. Detect falling edge if bit 13 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(TIM2_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level GIMA output 25 if bit 13 in the EDGE register is 0. Detect rising edge if bit 13 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(TIM2_LW::DETECT_HIGH_LEVEL)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM6_L`"]
pub enum TIM6_LW {
    #[doc = "Detect LOW level of GIMA output 26 if bit 14 in the EDGE register is 0. Detect falling edge if bit 14 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of GIMA output 26 if bit 14 in the EDGE register is 0. Detect rising edge if bit 14 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl TIM6_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM6_LW::DETECT_LOW_LEVEL => false,
            TIM6_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM6_LW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM6_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM6_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level of GIMA output 26 if bit 14 in the EDGE register is 0. Detect falling edge if bit 14 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(TIM6_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level of GIMA output 26 if bit 14 in the EDGE register is 0. Detect rising edge if bit 14 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(TIM6_LW::DETECT_HIGH_LEVEL)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `QEI_L`"]
pub enum QEI_LW {
    #[doc = "Detect LOW level of the QEI interrupt if bit 15 in the EDGE register is 0. Detect falling edge if bit 15 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of the QEI interrupt if bit 15 in the EDGE register is 0. Detect rising edge if bit 15 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl QEI_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QEI_LW::DETECT_LOW_LEVEL => false,
            QEI_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QEI_LW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QEI_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level of the QEI interrupt if bit 15 in the EDGE register is 0. Detect falling edge if bit 15 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(QEI_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level of the QEI interrupt if bit 15 in the EDGE register is 0. Detect rising edge if bit 15 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(QEI_LW::DETECT_HIGH_LEVEL)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM14_L`"]
pub enum TIM14_LW {
    #[doc = "Detect LOW level of GIMA output 27 if bit 16 in the EDGE register is 0. Detect falling edge if bit 16 in the EDGE register is 1."]
    DETECT_LOW_LEVEL,
    #[doc = "Detect HIGH level of GIMA output 27 if bit 16 in the EDGE register is 0. Detect rising edge if bit 16 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL,
}
impl TIM14_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM14_LW::DETECT_LOW_LEVEL => false,
            TIM14_LW::DETECT_HIGH_LEVEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM14_LW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM14_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM14_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level of GIMA output 27 if bit 16 in the EDGE register is 0. Detect falling edge if bit 16 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level(self) -> &'a mut W {
        self.variant(TIM14_LW::DETECT_LOW_LEVEL)
    }
    #[doc = "Detect HIGH level of GIMA output 27 if bit 16 in the EDGE register is 0. Detect rising edge if bit 16 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level(self) -> &'a mut W {
        self.variant(TIM14_LW::DETECT_HIGH_LEVEL)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESET_L`"]
pub enum RESET_LW {
    #[doc = "Detect LOW level if bit 17 in the EDGE register is 0. Detect falling edge if bit 17 in the EDGE register is 1."]
    DETECT_LOW_LEVEL_IF,
    #[doc = "Detect HIGH level if bit 17 in the EDGE register is 0. Detect rising edge if bit 17 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL_IF,
}
impl RESET_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESET_LW::DETECT_LOW_LEVEL_IF => false,
            RESET_LW::DETECT_HIGH_LEVEL_IF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESET_LW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESET_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level if bit 17 in the EDGE register is 0. Detect falling edge if bit 17 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level_if(self) -> &'a mut W {
        self.variant(RESET_LW::DETECT_LOW_LEVEL_IF)
    }
    #[doc = "Detect HIGH level if bit 17 in the EDGE register is 0. Detect rising edge if bit 17 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level_if(self) -> &'a mut W {
        self.variant(RESET_LW::DETECT_HIGH_LEVEL_IF)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BODRESET_L`"]
pub enum BODRESET_LW {
    #[doc = "Detect LOW level if bit 20 in the EDGE register is 0. Detect falling edge if bit 20 in the EDGE register is 1."]
    DETECT_LOW_LEVEL_IF,
    #[doc = "Detect HIGH level if bit 20 in the EDGE register is 0. Detect rising edge if bit 20 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL_IF,
}
impl BODRESET_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BODRESET_LW::DETECT_LOW_LEVEL_IF => false,
            BODRESET_LW::DETECT_HIGH_LEVEL_IF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODRESET_LW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRESET_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODRESET_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level if bit 20 in the EDGE register is 0. Detect falling edge if bit 20 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level_if(self) -> &'a mut W {
        self.variant(BODRESET_LW::DETECT_LOW_LEVEL_IF)
    }
    #[doc = "Detect HIGH level if bit 20 in the EDGE register is 0. Detect rising edge if bit 20 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level_if(self) -> &'a mut W {
        self.variant(BODRESET_LW::DETECT_HIGH_LEVEL_IF)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DPDRESET_L`"]
pub enum DPDRESET_LW {
    #[doc = "Detect LOW level if bit 21 in the EDGE register is 0. Detect falling edge if bit 21 in the EDGE register is 1."]
    DETECT_LOW_LEVEL_IF,
    #[doc = "Detect HIGH level if bit 21 in the EDGE register is 0. Detect rising edge if bit 21 in the EDGE register is 1."]
    DETECT_HIGH_LEVEL_IF,
}
impl DPDRESET_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPDRESET_LW::DETECT_LOW_LEVEL_IF => false,
            DPDRESET_LW::DETECT_HIGH_LEVEL_IF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPDRESET_LW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDRESET_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPDRESET_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Detect LOW level if bit 21 in the EDGE register is 0. Detect falling edge if bit 21 in the EDGE register is 1."]
    #[inline]
    pub fn detect_low_level_if(self) -> &'a mut W {
        self.variant(DPDRESET_LW::DETECT_LOW_LEVEL_IF)
    }
    #[doc = "Detect HIGH level if bit 21 in the EDGE register is 0. Detect rising edge if bit 21 in the EDGE register is 1."]
    #[inline]
    pub fn detect_high_level_if(self) -> &'a mut W {
        self.variant(DPDRESET_LW::DETECT_HIGH_LEVEL_IF)
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
        const OFFSET: u8 = 21;
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
    #[doc = "Bit 0 - Level detect mode for WAKEUP0 event."]
    #[inline]
    pub fn wakeup0_l(&self) -> WAKEUP0_LR {
        WAKEUP0_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Level detect mode for WAKEUP1 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn wakeup1_l(&self) -> WAKEUP1_LR {
        WAKEUP1_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Level detect mode for WAKEUP2 event."]
    #[inline]
    pub fn wakeup2_l(&self) -> WAKEUP2_LR {
        WAKEUP2_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Level detect mode for WAKEUP3 event."]
    #[inline]
    pub fn wakeup3_l(&self) -> WAKEUP3_LR {
        WAKEUP3_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Level detect mode for alarm timer event."]
    #[inline]
    pub fn atimer_l(&self) -> ATIMER_LR {
        ATIMER_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Level detect mode for RTC event."]
    #[inline]
    pub fn rtc_l(&self) -> RTC_LR {
        RTC_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Level detect mode for BOD event."]
    #[inline]
    pub fn bod_l(&self) -> BOD_LR {
        BOD_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Level detect mode for WWDT event."]
    #[inline]
    pub fn wwdt_l(&self) -> WWDT_LR {
        WWDT_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Level detect mode for Ethernet event"]
    #[inline]
    pub fn eth_l(&self) -> ETH_LR {
        ETH_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Level detect mode for USB0 event"]
    #[inline]
    pub fn usb0_l(&self) -> USB0_LR {
        USB0_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Level detect mode for USB1 event"]
    #[inline]
    pub fn usb1_l(&self) -> USB1_LR {
        USB1_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Level detect mode for SD/MMC event"]
    #[inline]
    pub fn sdmmc_l(&self) -> SDMMC_LR {
        SDMMC_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Level detect mode for C_CAN event."]
    #[inline]
    pub fn can_l(&self) -> CAN_LR {
        CAN_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Level detect mode for combined timer output 2 event."]
    #[inline]
    pub fn tim2_l(&self) -> TIM2_LR {
        TIM2_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Level detect mode for combined timer output 6 event."]
    #[inline]
    pub fn tim6_l(&self) -> TIM6_LR {
        TIM6_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Level detect mode for QEI event."]
    #[inline]
    pub fn qei_l(&self) -> QEI_LR {
        QEI_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Level detect mode for combined timer output 14 event."]
    #[inline]
    pub fn tim14_l(&self) -> TIM14_LR {
        TIM14_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Level detect mode for Reset"]
    #[inline]
    pub fn reset_l(&self) -> RESET_LR {
        RESET_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Level detect mode for BOD Reset"]
    #[inline]
    pub fn bodreset_l(&self) -> BODRESET_LR {
        BODRESET_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Level detect mode for Deep power-down Reset"]
    #[inline]
    pub fn dpdreset_l(&self) -> DPDRESET_LR {
        DPDRESET_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
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
    #[doc = "Bit 0 - Level detect mode for WAKEUP0 event."]
    #[inline]
    pub fn wakeup0_l(&mut self) -> _WAKEUP0_LW {
        _WAKEUP0_LW { w: self }
    }
    #[doc = "Bit 1 - Level detect mode for WAKEUP1 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn wakeup1_l(&mut self) -> _WAKEUP1_LW {
        _WAKEUP1_LW { w: self }
    }
    #[doc = "Bit 2 - Level detect mode for WAKEUP2 event."]
    #[inline]
    pub fn wakeup2_l(&mut self) -> _WAKEUP2_LW {
        _WAKEUP2_LW { w: self }
    }
    #[doc = "Bit 3 - Level detect mode for WAKEUP3 event."]
    #[inline]
    pub fn wakeup3_l(&mut self) -> _WAKEUP3_LW {
        _WAKEUP3_LW { w: self }
    }
    #[doc = "Bit 4 - Level detect mode for alarm timer event."]
    #[inline]
    pub fn atimer_l(&mut self) -> _ATIMER_LW {
        _ATIMER_LW { w: self }
    }
    #[doc = "Bit 5 - Level detect mode for RTC event."]
    #[inline]
    pub fn rtc_l(&mut self) -> _RTC_LW {
        _RTC_LW { w: self }
    }
    #[doc = "Bit 6 - Level detect mode for BOD event."]
    #[inline]
    pub fn bod_l(&mut self) -> _BOD_LW {
        _BOD_LW { w: self }
    }
    #[doc = "Bit 7 - Level detect mode for WWDT event."]
    #[inline]
    pub fn wwdt_l(&mut self) -> _WWDT_LW {
        _WWDT_LW { w: self }
    }
    #[doc = "Bit 8 - Level detect mode for Ethernet event"]
    #[inline]
    pub fn eth_l(&mut self) -> _ETH_LW {
        _ETH_LW { w: self }
    }
    #[doc = "Bit 9 - Level detect mode for USB0 event"]
    #[inline]
    pub fn usb0_l(&mut self) -> _USB0_LW {
        _USB0_LW { w: self }
    }
    #[doc = "Bit 10 - Level detect mode for USB1 event"]
    #[inline]
    pub fn usb1_l(&mut self) -> _USB1_LW {
        _USB1_LW { w: self }
    }
    #[doc = "Bit 11 - Level detect mode for SD/MMC event"]
    #[inline]
    pub fn sdmmc_l(&mut self) -> _SDMMC_LW {
        _SDMMC_LW { w: self }
    }
    #[doc = "Bit 12 - Level detect mode for C_CAN event."]
    #[inline]
    pub fn can_l(&mut self) -> _CAN_LW {
        _CAN_LW { w: self }
    }
    #[doc = "Bit 13 - Level detect mode for combined timer output 2 event."]
    #[inline]
    pub fn tim2_l(&mut self) -> _TIM2_LW {
        _TIM2_LW { w: self }
    }
    #[doc = "Bit 14 - Level detect mode for combined timer output 6 event."]
    #[inline]
    pub fn tim6_l(&mut self) -> _TIM6_LW {
        _TIM6_LW { w: self }
    }
    #[doc = "Bit 15 - Level detect mode for QEI event."]
    #[inline]
    pub fn qei_l(&mut self) -> _QEI_LW {
        _QEI_LW { w: self }
    }
    #[doc = "Bit 16 - Level detect mode for combined timer output 14 event."]
    #[inline]
    pub fn tim14_l(&mut self) -> _TIM14_LW {
        _TIM14_LW { w: self }
    }
    #[doc = "Bit 19 - Level detect mode for Reset"]
    #[inline]
    pub fn reset_l(&mut self) -> _RESET_LW {
        _RESET_LW { w: self }
    }
    #[doc = "Bit 20 - Level detect mode for BOD Reset"]
    #[inline]
    pub fn bodreset_l(&mut self) -> _BODRESET_LW {
        _BODRESET_LW { w: self }
    }
    #[doc = "Bit 21 - Level detect mode for Deep power-down Reset"]
    #[inline]
    pub fn dpdreset_l(&mut self) -> _DPDRESET_LW {
        _DPDRESET_LW { w: self }
    }
}
