#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EDGE {
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
#[doc = "Possible values of the field `WAKEUP0_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP0_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of WAKEUP0 pin. Detect falling edge if bit 0 in the HILO register is 0. Detect rising edge if bit 0 in the HILO register is 1."]
    EDGE_DETECT_OF_WAKEU,
}
impl WAKEUP0_ER {
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
            WAKEUP0_ER::LEVEL_DETECT => false,
            WAKEUP0_ER::EDGE_DETECT_OF_WAKEU => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP0_ER {
        match value {
            false => WAKEUP0_ER::LEVEL_DETECT,
            true => WAKEUP0_ER::EDGE_DETECT_OF_WAKEU,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == WAKEUP0_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_WAKEU`"]
    #[inline]
    pub fn is_edge_detect_of_wakeu(&self) -> bool {
        *self == WAKEUP0_ER::EDGE_DETECT_OF_WAKEU
    }
}
#[doc = "Possible values of the field `WAKEUP1_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP1_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of WAKEUP1 pin. Detect falling edge if bit 1 in the HILO register is 0. Detect rising edge if bit 1 in the HILO register is 1."]
    EDGE_DETECT_OF_WAKEU,
}
impl WAKEUP1_ER {
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
            WAKEUP1_ER::LEVEL_DETECT => false,
            WAKEUP1_ER::EDGE_DETECT_OF_WAKEU => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP1_ER {
        match value {
            false => WAKEUP1_ER::LEVEL_DETECT,
            true => WAKEUP1_ER::EDGE_DETECT_OF_WAKEU,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == WAKEUP1_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_WAKEU`"]
    #[inline]
    pub fn is_edge_detect_of_wakeu(&self) -> bool {
        *self == WAKEUP1_ER::EDGE_DETECT_OF_WAKEU
    }
}
#[doc = "Possible values of the field `WAKEUP2_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP2_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of WAKEUP2 pin. Detect falling edge if bit 2 in the HILO register is 0. Detect rising edge if bit 2 in the HILO register is 1."]
    EDGE_DETECT_OF_WAKEU,
}
impl WAKEUP2_ER {
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
            WAKEUP2_ER::LEVEL_DETECT => false,
            WAKEUP2_ER::EDGE_DETECT_OF_WAKEU => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP2_ER {
        match value {
            false => WAKEUP2_ER::LEVEL_DETECT,
            true => WAKEUP2_ER::EDGE_DETECT_OF_WAKEU,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == WAKEUP2_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_WAKEU`"]
    #[inline]
    pub fn is_edge_detect_of_wakeu(&self) -> bool {
        *self == WAKEUP2_ER::EDGE_DETECT_OF_WAKEU
    }
}
#[doc = "Possible values of the field `WAKEUP3_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP3_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of WAKEUP3 pin. Detect falling edge if bit 30 in the HILO register is 0. Detect rising edge if bit 3 in the HILO register is 1."]
    EDGE_DETECT_OF_WAKEU,
}
impl WAKEUP3_ER {
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
            WAKEUP3_ER::LEVEL_DETECT => false,
            WAKEUP3_ER::EDGE_DETECT_OF_WAKEU => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP3_ER {
        match value {
            false => WAKEUP3_ER::LEVEL_DETECT,
            true => WAKEUP3_ER::EDGE_DETECT_OF_WAKEU,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == WAKEUP3_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_WAKEU`"]
    #[inline]
    pub fn is_edge_detect_of_wakeu(&self) -> bool {
        *self == WAKEUP3_ER::EDGE_DETECT_OF_WAKEU
    }
}
#[doc = "Possible values of the field `ATIMER_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATIMER_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the alarm timer interrupt. Detect falling edge if bit 4 in the HILO register is 0. Detect rising edge if bit 4 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_A,
}
impl ATIMER_ER {
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
            ATIMER_ER::LEVEL_DETECT => false,
            ATIMER_ER::EDGE_DETECT_OF_THE_A => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATIMER_ER {
        match value {
            false => ATIMER_ER::LEVEL_DETECT,
            true => ATIMER_ER::EDGE_DETECT_OF_THE_A,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == ATIMER_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_THE_A`"]
    #[inline]
    pub fn is_edge_detect_of_the_a(&self) -> bool {
        *self == ATIMER_ER::EDGE_DETECT_OF_THE_A
    }
}
#[doc = "Possible values of the field `RTC_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the RTC interrupt. Detect falling edge if bit 5 in the HILO register is 0. Detect rising edge if bit 5 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_R,
}
impl RTC_ER {
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
            RTC_ER::LEVEL_DETECT => false,
            RTC_ER::EDGE_DETECT_OF_THE_R => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_ER {
        match value {
            false => RTC_ER::LEVEL_DETECT,
            true => RTC_ER::EDGE_DETECT_OF_THE_R,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == RTC_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_THE_R`"]
    #[inline]
    pub fn is_edge_detect_of_the_r(&self) -> bool {
        *self == RTC_ER::EDGE_DETECT_OF_THE_R
    }
}
#[doc = "Possible values of the field `BOD_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the BOD interrupt. Detect falling edge if bit 6 in the HILO register is 0. Detect rising edge if bit 6 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_B,
}
impl BOD_ER {
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
            BOD_ER::LEVEL_DETECT => false,
            BOD_ER::EDGE_DETECT_OF_THE_B => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOD_ER {
        match value {
            false => BOD_ER::LEVEL_DETECT,
            true => BOD_ER::EDGE_DETECT_OF_THE_B,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == BOD_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_THE_B`"]
    #[inline]
    pub fn is_edge_detect_of_the_b(&self) -> bool {
        *self == BOD_ER::EDGE_DETECT_OF_THE_B
    }
}
#[doc = "Possible values of the field `WWDT_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the WWDT interrupt. Detect falling edge if bit 7 in the HILO register is 0. Detect rising edge if bit 7 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_W,
}
impl WWDT_ER {
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
            WWDT_ER::LEVEL_DETECT => false,
            WWDT_ER::EDGE_DETECT_OF_THE_W => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WWDT_ER {
        match value {
            false => WWDT_ER::LEVEL_DETECT,
            true => WWDT_ER::EDGE_DETECT_OF_THE_W,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == WWDT_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_THE_W`"]
    #[inline]
    pub fn is_edge_detect_of_the_w(&self) -> bool {
        *self == WWDT_ER::EDGE_DETECT_OF_THE_W
    }
}
#[doc = "Possible values of the field `ETH_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETH_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the Ethernet interrupt. Detect falling edge if bit 8 in the HILO register is 0. Detect rising edge if bit 8 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_E,
}
impl ETH_ER {
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
            ETH_ER::LEVEL_DETECT => false,
            ETH_ER::EDGE_DETECT_OF_THE_E => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETH_ER {
        match value {
            false => ETH_ER::LEVEL_DETECT,
            true => ETH_ER::EDGE_DETECT_OF_THE_E,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == ETH_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_THE_E`"]
    #[inline]
    pub fn is_edge_detect_of_the_e(&self) -> bool {
        *self == ETH_ER::EDGE_DETECT_OF_THE_E
    }
}
#[doc = "Possible values of the field `USB0_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the USB0 event. Detect falling edge if bit 9 in the HILO register is 0. Detect rising edge if bit 9 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_U,
}
impl USB0_ER {
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
            USB0_ER::LEVEL_DETECT => false,
            USB0_ER::EDGE_DETECT_OF_THE_U => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0_ER {
        match value {
            false => USB0_ER::LEVEL_DETECT,
            true => USB0_ER::EDGE_DETECT_OF_THE_U,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == USB0_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_THE_U`"]
    #[inline]
    pub fn is_edge_detect_of_the_u(&self) -> bool {
        *self == USB0_ER::EDGE_DETECT_OF_THE_U
    }
}
#[doc = "Possible values of the field `USB1_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the USB1 interrupt. Detect falling edge if bit 10 in the HILO register is 0. Detect rising edge if bit 10 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_U,
}
impl USB1_ER {
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
            USB1_ER::LEVEL_DETECT => false,
            USB1_ER::EDGE_DETECT_OF_THE_U => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_ER {
        match value {
            false => USB1_ER::LEVEL_DETECT,
            true => USB1_ER::EDGE_DETECT_OF_THE_U,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == USB1_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_THE_U`"]
    #[inline]
    pub fn is_edge_detect_of_the_u(&self) -> bool {
        *self == USB1_ER::EDGE_DETECT_OF_THE_U
    }
}
#[doc = "Possible values of the field `SDMMC_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMC_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the SD/MMC interrupt. Detect falling edge if bit 10 in the HILO register is 0. Detect rising edge if bit 10 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_S,
}
impl SDMMC_ER {
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
            SDMMC_ER::LEVEL_DETECT => false,
            SDMMC_ER::EDGE_DETECT_OF_THE_S => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDMMC_ER {
        match value {
            false => SDMMC_ER::LEVEL_DETECT,
            true => SDMMC_ER::EDGE_DETECT_OF_THE_S,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == SDMMC_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_THE_S`"]
    #[inline]
    pub fn is_edge_detect_of_the_s(&self) -> bool {
        *self == SDMMC_ER::EDGE_DETECT_OF_THE_S
    }
}
#[doc = "Possible values of the field `CAN_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the combined C_CAN interrupt. Detect falling edge if bit 12 in the HILO register is 0. Detect rising edge if bit 12 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_C,
}
impl CAN_ER {
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
            CAN_ER::LEVEL_DETECT => false,
            CAN_ER::EDGE_DETECT_OF_THE_C => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAN_ER {
        match value {
            false => CAN_ER::LEVEL_DETECT,
            true => CAN_ER::EDGE_DETECT_OF_THE_C,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == CAN_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_THE_C`"]
    #[inline]
    pub fn is_edge_detect_of_the_c(&self) -> bool {
        *self == CAN_ER::EDGE_DETECT_OF_THE_C
    }
}
#[doc = "Possible values of the field `TIM2_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of GIMA output 25. Detect falling edge if bit 13 in the HILO register is 0. Detect rising edge if bit 13 in the HILO register is 1."]
    EDGE_DETECT_OF_GIMA,
}
impl TIM2_ER {
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
            TIM2_ER::LEVEL_DETECT => false,
            TIM2_ER::EDGE_DETECT_OF_GIMA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM2_ER {
        match value {
            false => TIM2_ER::LEVEL_DETECT,
            true => TIM2_ER::EDGE_DETECT_OF_GIMA,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == TIM2_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_GIMA`"]
    #[inline]
    pub fn is_edge_detect_of_gima(&self) -> bool {
        *self == TIM2_ER::EDGE_DETECT_OF_GIMA
    }
}
#[doc = "Possible values of the field `TIM6_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM6_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of GIMA output 26. Detect falling edge if bit 14 in the HILO register is 0. Detect rising edge if bit 14 in the HILO register is 1."]
    EDGE_DETECT_OF_GIMA,
}
impl TIM6_ER {
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
            TIM6_ER::LEVEL_DETECT => false,
            TIM6_ER::EDGE_DETECT_OF_GIMA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM6_ER {
        match value {
            false => TIM6_ER::LEVEL_DETECT,
            true => TIM6_ER::EDGE_DETECT_OF_GIMA,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == TIM6_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_GIMA`"]
    #[inline]
    pub fn is_edge_detect_of_gima(&self) -> bool {
        *self == TIM6_ER::EDGE_DETECT_OF_GIMA
    }
}
#[doc = "Possible values of the field `QEI_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QEI_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of QEI interrupt. Detect falling edge if bit 15 in the HILO register is 0. Detect rising edge if bit 15 in the HILO register is 1."]
    EDGE_DETECT_OF_QEI_I,
}
impl QEI_ER {
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
            QEI_ER::LEVEL_DETECT => false,
            QEI_ER::EDGE_DETECT_OF_QEI_I => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QEI_ER {
        match value {
            false => QEI_ER::LEVEL_DETECT,
            true => QEI_ER::EDGE_DETECT_OF_QEI_I,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == QEI_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_QEI_I`"]
    #[inline]
    pub fn is_edge_detect_of_qei_i(&self) -> bool {
        *self == QEI_ER::EDGE_DETECT_OF_QEI_I
    }
}
#[doc = "Possible values of the field `TIM14_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM14_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of GIMA output 27. Detect falling edge if bit 16 in the HILO register is 0. Detect rising edge if bit 16 in the HILO register is 1."]
    EDGE_DETECT_OF_GIMA,
}
impl TIM14_ER {
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
            TIM14_ER::LEVEL_DETECT => false,
            TIM14_ER::EDGE_DETECT_OF_GIMA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM14_ER {
        match value {
            false => TIM14_ER::LEVEL_DETECT,
            true => TIM14_ER::EDGE_DETECT_OF_GIMA,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == TIM14_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_GIMA`"]
    #[inline]
    pub fn is_edge_detect_of_gima(&self) -> bool {
        *self == TIM14_ER::EDGE_DETECT_OF_GIMA
    }
}
#[doc = "Possible values of the field `RESET_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the reset signal. Detect falling edge if bit 19 in the HILO register is 0. Detect rising edge if bit 19 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_R,
}
impl RESET_ER {
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
            RESET_ER::LEVEL_DETECT => false,
            RESET_ER::EDGE_DETECT_OF_THE_R => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESET_ER {
        match value {
            false => RESET_ER::LEVEL_DETECT,
            true => RESET_ER::EDGE_DETECT_OF_THE_R,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == RESET_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_THE_R`"]
    #[inline]
    pub fn is_edge_detect_of_the_r(&self) -> bool {
        *self == RESET_ER::EDGE_DETECT_OF_THE_R
    }
}
#[doc = "Possible values of the field `BODRESET_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRESET_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the reset signal. Detect falling edge if bit 20 in the HILO register is 0. Detect rising edge if bit 19 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_R,
}
impl BODRESET_ER {
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
            BODRESET_ER::LEVEL_DETECT => false,
            BODRESET_ER::EDGE_DETECT_OF_THE_R => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODRESET_ER {
        match value {
            false => BODRESET_ER::LEVEL_DETECT,
            true => BODRESET_ER::EDGE_DETECT_OF_THE_R,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == BODRESET_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_THE_R`"]
    #[inline]
    pub fn is_edge_detect_of_the_r(&self) -> bool {
        *self == BODRESET_ER::EDGE_DETECT_OF_THE_R
    }
}
#[doc = "Possible values of the field `DPDRESET_E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDRESET_ER {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the reset signal. Detect falling edge if bit 21 in the HILO register is 0. Detect rising edge if bit 21 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_R,
}
impl DPDRESET_ER {
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
            DPDRESET_ER::LEVEL_DETECT => false,
            DPDRESET_ER::EDGE_DETECT_OF_THE_R => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPDRESET_ER {
        match value {
            false => DPDRESET_ER::LEVEL_DETECT,
            true => DPDRESET_ER::EDGE_DETECT_OF_THE_R,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_DETECT`"]
    #[inline]
    pub fn is_level_detect(&self) -> bool {
        *self == DPDRESET_ER::LEVEL_DETECT
    }
    #[doc = "Checks if the value of the field is `EDGE_DETECT_OF_THE_R`"]
    #[inline]
    pub fn is_edge_detect_of_the_r(&self) -> bool {
        *self == DPDRESET_ER::EDGE_DETECT_OF_THE_R
    }
}
#[doc = "Values that can be written to the field `WAKEUP0_E`"]
pub enum WAKEUP0_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of WAKEUP0 pin. Detect falling edge if bit 0 in the HILO register is 0. Detect rising edge if bit 0 in the HILO register is 1."]
    EDGE_DETECT_OF_WAKEU,
}
impl WAKEUP0_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP0_EW::LEVEL_DETECT => false,
            WAKEUP0_EW::EDGE_DETECT_OF_WAKEU => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP0_EW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP0_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP0_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(WAKEUP0_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of WAKEUP0 pin. Detect falling edge if bit 0 in the HILO register is 0. Detect rising edge if bit 0 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_wakeu(self) -> &'a mut W {
        self.variant(WAKEUP0_EW::EDGE_DETECT_OF_WAKEU)
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
#[doc = "Values that can be written to the field `WAKEUP1_E`"]
pub enum WAKEUP1_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of WAKEUP1 pin. Detect falling edge if bit 1 in the HILO register is 0. Detect rising edge if bit 1 in the HILO register is 1."]
    EDGE_DETECT_OF_WAKEU,
}
impl WAKEUP1_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP1_EW::LEVEL_DETECT => false,
            WAKEUP1_EW::EDGE_DETECT_OF_WAKEU => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP1_EW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP1_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP1_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(WAKEUP1_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of WAKEUP1 pin. Detect falling edge if bit 1 in the HILO register is 0. Detect rising edge if bit 1 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_wakeu(self) -> &'a mut W {
        self.variant(WAKEUP1_EW::EDGE_DETECT_OF_WAKEU)
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
#[doc = "Values that can be written to the field `WAKEUP2_E`"]
pub enum WAKEUP2_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of WAKEUP2 pin. Detect falling edge if bit 2 in the HILO register is 0. Detect rising edge if bit 2 in the HILO register is 1."]
    EDGE_DETECT_OF_WAKEU,
}
impl WAKEUP2_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP2_EW::LEVEL_DETECT => false,
            WAKEUP2_EW::EDGE_DETECT_OF_WAKEU => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP2_EW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP2_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP2_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(WAKEUP2_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of WAKEUP2 pin. Detect falling edge if bit 2 in the HILO register is 0. Detect rising edge if bit 2 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_wakeu(self) -> &'a mut W {
        self.variant(WAKEUP2_EW::EDGE_DETECT_OF_WAKEU)
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
#[doc = "Values that can be written to the field `WAKEUP3_E`"]
pub enum WAKEUP3_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of WAKEUP3 pin. Detect falling edge if bit 30 in the HILO register is 0. Detect rising edge if bit 3 in the HILO register is 1."]
    EDGE_DETECT_OF_WAKEU,
}
impl WAKEUP3_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP3_EW::LEVEL_DETECT => false,
            WAKEUP3_EW::EDGE_DETECT_OF_WAKEU => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP3_EW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP3_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP3_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(WAKEUP3_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of WAKEUP3 pin. Detect falling edge if bit 30 in the HILO register is 0. Detect rising edge if bit 3 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_wakeu(self) -> &'a mut W {
        self.variant(WAKEUP3_EW::EDGE_DETECT_OF_WAKEU)
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
#[doc = "Values that can be written to the field `ATIMER_E`"]
pub enum ATIMER_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the alarm timer interrupt. Detect falling edge if bit 4 in the HILO register is 0. Detect rising edge if bit 4 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_A,
}
impl ATIMER_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATIMER_EW::LEVEL_DETECT => false,
            ATIMER_EW::EDGE_DETECT_OF_THE_A => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATIMER_EW<'a> {
    w: &'a mut W,
}
impl<'a> _ATIMER_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATIMER_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(ATIMER_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of the alarm timer interrupt. Detect falling edge if bit 4 in the HILO register is 0. Detect rising edge if bit 4 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_the_a(self) -> &'a mut W {
        self.variant(ATIMER_EW::EDGE_DETECT_OF_THE_A)
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
#[doc = "Values that can be written to the field `RTC_E`"]
pub enum RTC_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the RTC interrupt. Detect falling edge if bit 5 in the HILO register is 0. Detect rising edge if bit 5 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_R,
}
impl RTC_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_EW::LEVEL_DETECT => false,
            RTC_EW::EDGE_DETECT_OF_THE_R => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_EW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(RTC_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of the RTC interrupt. Detect falling edge if bit 5 in the HILO register is 0. Detect rising edge if bit 5 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_the_r(self) -> &'a mut W {
        self.variant(RTC_EW::EDGE_DETECT_OF_THE_R)
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
#[doc = "Values that can be written to the field `BOD_E`"]
pub enum BOD_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the BOD interrupt. Detect falling edge if bit 6 in the HILO register is 0. Detect rising edge if bit 6 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_B,
}
impl BOD_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOD_EW::LEVEL_DETECT => false,
            BOD_EW::EDGE_DETECT_OF_THE_B => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOD_EW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOD_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(BOD_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of the BOD interrupt. Detect falling edge if bit 6 in the HILO register is 0. Detect rising edge if bit 6 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_the_b(self) -> &'a mut W {
        self.variant(BOD_EW::EDGE_DETECT_OF_THE_B)
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
#[doc = "Values that can be written to the field `WWDT_E`"]
pub enum WWDT_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the WWDT interrupt. Detect falling edge if bit 7 in the HILO register is 0. Detect rising edge if bit 7 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_W,
}
impl WWDT_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WWDT_EW::LEVEL_DETECT => false,
            WWDT_EW::EDGE_DETECT_OF_THE_W => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WWDT_EW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDT_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WWDT_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(WWDT_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of the WWDT interrupt. Detect falling edge if bit 7 in the HILO register is 0. Detect rising edge if bit 7 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_the_w(self) -> &'a mut W {
        self.variant(WWDT_EW::EDGE_DETECT_OF_THE_W)
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
#[doc = "Values that can be written to the field `ETH_E`"]
pub enum ETH_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the Ethernet interrupt. Detect falling edge if bit 8 in the HILO register is 0. Detect rising edge if bit 8 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_E,
}
impl ETH_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETH_EW::LEVEL_DETECT => false,
            ETH_EW::EDGE_DETECT_OF_THE_E => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETH_EW<'a> {
    w: &'a mut W,
}
impl<'a> _ETH_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETH_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(ETH_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of the Ethernet interrupt. Detect falling edge if bit 8 in the HILO register is 0. Detect rising edge if bit 8 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_the_e(self) -> &'a mut W {
        self.variant(ETH_EW::EDGE_DETECT_OF_THE_E)
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
#[doc = "Values that can be written to the field `USB0_E`"]
pub enum USB0_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the USB0 event. Detect falling edge if bit 9 in the HILO register is 0. Detect rising edge if bit 9 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_U,
}
impl USB0_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0_EW::LEVEL_DETECT => false,
            USB0_EW::EDGE_DETECT_OF_THE_U => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0_EW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(USB0_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of the USB0 event. Detect falling edge if bit 9 in the HILO register is 0. Detect rising edge if bit 9 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_the_u(self) -> &'a mut W {
        self.variant(USB0_EW::EDGE_DETECT_OF_THE_U)
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
#[doc = "Values that can be written to the field `USB1_E`"]
pub enum USB1_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the USB1 interrupt. Detect falling edge if bit 10 in the HILO register is 0. Detect rising edge if bit 10 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_U,
}
impl USB1_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_EW::LEVEL_DETECT => false,
            USB1_EW::EDGE_DETECT_OF_THE_U => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_EW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(USB1_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of the USB1 interrupt. Detect falling edge if bit 10 in the HILO register is 0. Detect rising edge if bit 10 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_the_u(self) -> &'a mut W {
        self.variant(USB1_EW::EDGE_DETECT_OF_THE_U)
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
#[doc = "Values that can be written to the field `SDMMC_E`"]
pub enum SDMMC_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the SD/MMC interrupt. Detect falling edge if bit 10 in the HILO register is 0. Detect rising edge if bit 10 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_S,
}
impl SDMMC_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDMMC_EW::LEVEL_DETECT => false,
            SDMMC_EW::EDGE_DETECT_OF_THE_S => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDMMC_EW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMMC_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDMMC_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(SDMMC_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of the SD/MMC interrupt. Detect falling edge if bit 10 in the HILO register is 0. Detect rising edge if bit 10 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_the_s(self) -> &'a mut W {
        self.variant(SDMMC_EW::EDGE_DETECT_OF_THE_S)
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
#[doc = "Values that can be written to the field `CAN_E`"]
pub enum CAN_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the combined C_CAN interrupt. Detect falling edge if bit 12 in the HILO register is 0. Detect rising edge if bit 12 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_C,
}
impl CAN_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAN_EW::LEVEL_DETECT => false,
            CAN_EW::EDGE_DETECT_OF_THE_C => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAN_EW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(CAN_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of the combined C_CAN interrupt. Detect falling edge if bit 12 in the HILO register is 0. Detect rising edge if bit 12 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_the_c(self) -> &'a mut W {
        self.variant(CAN_EW::EDGE_DETECT_OF_THE_C)
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
#[doc = "Values that can be written to the field `TIM2_E`"]
pub enum TIM2_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of GIMA output 25. Detect falling edge if bit 13 in the HILO register is 0. Detect rising edge if bit 13 in the HILO register is 1."]
    EDGE_DETECT_OF_GIMA,
}
impl TIM2_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM2_EW::LEVEL_DETECT => false,
            TIM2_EW::EDGE_DETECT_OF_GIMA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM2_EW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM2_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM2_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(TIM2_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of GIMA output 25. Detect falling edge if bit 13 in the HILO register is 0. Detect rising edge if bit 13 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_gima(self) -> &'a mut W {
        self.variant(TIM2_EW::EDGE_DETECT_OF_GIMA)
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
#[doc = "Values that can be written to the field `TIM6_E`"]
pub enum TIM6_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of GIMA output 26. Detect falling edge if bit 14 in the HILO register is 0. Detect rising edge if bit 14 in the HILO register is 1."]
    EDGE_DETECT_OF_GIMA,
}
impl TIM6_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM6_EW::LEVEL_DETECT => false,
            TIM6_EW::EDGE_DETECT_OF_GIMA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM6_EW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM6_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM6_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(TIM6_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of GIMA output 26. Detect falling edge if bit 14 in the HILO register is 0. Detect rising edge if bit 14 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_gima(self) -> &'a mut W {
        self.variant(TIM6_EW::EDGE_DETECT_OF_GIMA)
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
#[doc = "Values that can be written to the field `QEI_E`"]
pub enum QEI_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of QEI interrupt. Detect falling edge if bit 15 in the HILO register is 0. Detect rising edge if bit 15 in the HILO register is 1."]
    EDGE_DETECT_OF_QEI_I,
}
impl QEI_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QEI_EW::LEVEL_DETECT => false,
            QEI_EW::EDGE_DETECT_OF_QEI_I => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QEI_EW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QEI_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(QEI_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of QEI interrupt. Detect falling edge if bit 15 in the HILO register is 0. Detect rising edge if bit 15 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_qei_i(self) -> &'a mut W {
        self.variant(QEI_EW::EDGE_DETECT_OF_QEI_I)
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
#[doc = "Values that can be written to the field `TIM14_E`"]
pub enum TIM14_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of GIMA output 27. Detect falling edge if bit 16 in the HILO register is 0. Detect rising edge if bit 16 in the HILO register is 1."]
    EDGE_DETECT_OF_GIMA,
}
impl TIM14_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM14_EW::LEVEL_DETECT => false,
            TIM14_EW::EDGE_DETECT_OF_GIMA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM14_EW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM14_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM14_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(TIM14_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of GIMA output 27. Detect falling edge if bit 16 in the HILO register is 0. Detect rising edge if bit 16 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_gima(self) -> &'a mut W {
        self.variant(TIM14_EW::EDGE_DETECT_OF_GIMA)
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
#[doc = "Values that can be written to the field `RESET_E`"]
pub enum RESET_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the reset signal. Detect falling edge if bit 19 in the HILO register is 0. Detect rising edge if bit 19 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_R,
}
impl RESET_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESET_EW::LEVEL_DETECT => false,
            RESET_EW::EDGE_DETECT_OF_THE_R => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESET_EW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESET_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(RESET_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of the reset signal. Detect falling edge if bit 19 in the HILO register is 0. Detect rising edge if bit 19 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_the_r(self) -> &'a mut W {
        self.variant(RESET_EW::EDGE_DETECT_OF_THE_R)
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
#[doc = "Values that can be written to the field `BODRESET_E`"]
pub enum BODRESET_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the reset signal. Detect falling edge if bit 20 in the HILO register is 0. Detect rising edge if bit 19 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_R,
}
impl BODRESET_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BODRESET_EW::LEVEL_DETECT => false,
            BODRESET_EW::EDGE_DETECT_OF_THE_R => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODRESET_EW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRESET_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODRESET_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(BODRESET_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of the reset signal. Detect falling edge if bit 20 in the HILO register is 0. Detect rising edge if bit 19 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_the_r(self) -> &'a mut W {
        self.variant(BODRESET_EW::EDGE_DETECT_OF_THE_R)
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
#[doc = "Values that can be written to the field `DPDRESET_E`"]
pub enum DPDRESET_EW {
    #[doc = "Level detect."]
    LEVEL_DETECT,
    #[doc = "Edge detect of the reset signal. Detect falling edge if bit 21 in the HILO register is 0. Detect rising edge if bit 21 in the HILO register is 1."]
    EDGE_DETECT_OF_THE_R,
}
impl DPDRESET_EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPDRESET_EW::LEVEL_DETECT => false,
            DPDRESET_EW::EDGE_DETECT_OF_THE_R => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPDRESET_EW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDRESET_EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPDRESET_EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level detect."]
    #[inline]
    pub fn level_detect(self) -> &'a mut W {
        self.variant(DPDRESET_EW::LEVEL_DETECT)
    }
    #[doc = "Edge detect of the reset signal. Detect falling edge if bit 21 in the HILO register is 0. Detect rising edge if bit 21 in the HILO register is 1."]
    #[inline]
    pub fn edge_detect_of_the_r(self) -> &'a mut W {
        self.variant(DPDRESET_EW::EDGE_DETECT_OF_THE_R)
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
    #[doc = "Bit 0 - Edge detect mode for WAKEUP0 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn wakeup0_e(&self) -> WAKEUP0_ER {
        WAKEUP0_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Edge/level detect mode for WAKEUP1 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn wakeup1_e(&self) -> WAKEUP1_ER {
        WAKEUP1_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Edge/level detect mode for WAKEUP2 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn wakeup2_e(&self) -> WAKEUP2_ER {
        WAKEUP2_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Edge/level detect mode for WAKEUP3 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn wakeup3_e(&self) -> WAKEUP3_ER {
        WAKEUP3_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Edge/level detect mode for alarm timer event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn atimer_e(&self) -> ATIMER_ER {
        ATIMER_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Edge/level detect mode for RTC event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn rtc_e(&self) -> RTC_ER {
        RTC_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Edge/level detect mode for BOD event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn bod_e(&self) -> BOD_ER {
        BOD_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Edge/level detect mode for WWDTD event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn wwdt_e(&self) -> WWDT_ER {
        WWDT_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Edge/level detect mode for ethernet event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn eth_e(&self) -> ETH_ER {
        ETH_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Edge/level detect mode for USB0 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn usb0_e(&self) -> USB0_ER {
        USB0_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Edge/level detect mode for USB1 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn usb1_e(&self) -> USB1_ER {
        USB1_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Edge/level detect mode for SD/MMC event.The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn sdmmc_e(&self) -> SDMMC_ER {
        SDMMC_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Edge/level detect mode for C_CAN event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn can_e(&self) -> CAN_ER {
        CAN_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Edge/level detect mode for combined timer output 2 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn tim2_e(&self) -> TIM2_ER {
        TIM2_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Edge/level detect mode for combined timer output 6 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn tim6_e(&self) -> TIM6_ER {
        TIM6_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Edge/level detect mode for QEI interrupt signal. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn qei_e(&self) -> QEI_ER {
        QEI_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Edge/level detect mode for combined timer output 14 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn tim14_e(&self) -> TIM14_ER {
        TIM14_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Edge/level detect mode for Reset. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn reset_e(&self) -> RESET_ER {
        RESET_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Edge detect of the BOD reset signal. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn bodreset_e(&self) -> BODRESET_ER {
        BODRESET_ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Edge detect of the deep power-down reset signal. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn dpdreset_e(&self) -> DPDRESET_ER {
        DPDRESET_ER::_from({
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
    #[doc = "Bit 0 - Edge detect mode for WAKEUP0 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn wakeup0_e(&mut self) -> _WAKEUP0_EW {
        _WAKEUP0_EW { w: self }
    }
    #[doc = "Bit 1 - Edge/level detect mode for WAKEUP1 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn wakeup1_e(&mut self) -> _WAKEUP1_EW {
        _WAKEUP1_EW { w: self }
    }
    #[doc = "Bit 2 - Edge/level detect mode for WAKEUP2 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn wakeup2_e(&mut self) -> _WAKEUP2_EW {
        _WAKEUP2_EW { w: self }
    }
    #[doc = "Bit 3 - Edge/level detect mode for WAKEUP3 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn wakeup3_e(&mut self) -> _WAKEUP3_EW {
        _WAKEUP3_EW { w: self }
    }
    #[doc = "Bit 4 - Edge/level detect mode for alarm timer event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn atimer_e(&mut self) -> _ATIMER_EW {
        _ATIMER_EW { w: self }
    }
    #[doc = "Bit 5 - Edge/level detect mode for RTC event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn rtc_e(&mut self) -> _RTC_EW {
        _RTC_EW { w: self }
    }
    #[doc = "Bit 6 - Edge/level detect mode for BOD event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn bod_e(&mut self) -> _BOD_EW {
        _BOD_EW { w: self }
    }
    #[doc = "Bit 7 - Edge/level detect mode for WWDTD event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn wwdt_e(&mut self) -> _WWDT_EW {
        _WWDT_EW { w: self }
    }
    #[doc = "Bit 8 - Edge/level detect mode for ethernet event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn eth_e(&mut self) -> _ETH_EW {
        _ETH_EW { w: self }
    }
    #[doc = "Bit 9 - Edge/level detect mode for USB0 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn usb0_e(&mut self) -> _USB0_EW {
        _USB0_EW { w: self }
    }
    #[doc = "Bit 10 - Edge/level detect mode for USB1 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn usb1_e(&mut self) -> _USB1_EW {
        _USB1_EW { w: self }
    }
    #[doc = "Bit 11 - Edge/level detect mode for SD/MMC event.The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn sdmmc_e(&mut self) -> _SDMMC_EW {
        _SDMMC_EW { w: self }
    }
    #[doc = "Bit 12 - Edge/level detect mode for C_CAN event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn can_e(&mut self) -> _CAN_EW {
        _CAN_EW { w: self }
    }
    #[doc = "Bit 13 - Edge/level detect mode for combined timer output 2 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn tim2_e(&mut self) -> _TIM2_EW {
        _TIM2_EW { w: self }
    }
    #[doc = "Bit 14 - Edge/level detect mode for combined timer output 6 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn tim6_e(&mut self) -> _TIM6_EW {
        _TIM6_EW { w: self }
    }
    #[doc = "Bit 15 - Edge/level detect mode for QEI interrupt signal. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn qei_e(&mut self) -> _QEI_EW {
        _QEI_EW { w: self }
    }
    #[doc = "Bit 16 - Edge/level detect mode for combined timer output 14 event. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn tim14_e(&mut self) -> _TIM14_EW {
        _TIM14_EW { w: self }
    }
    #[doc = "Bit 19 - Edge/level detect mode for Reset. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn reset_e(&mut self) -> _RESET_EW {
        _RESET_EW { w: self }
    }
    #[doc = "Bit 20 - Edge detect of the BOD reset signal. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn bodreset_e(&mut self) -> _BODRESET_EW {
        _BODRESET_EW { w: self }
    }
    #[doc = "Bit 21 - Edge detect of the deep power-down reset signal. The corresponding bit in the EDGE register must be 0."]
    #[inline]
    pub fn dpdreset_e(&mut self) -> _DPDRESET_EW {
        _DPDRESET_EW { w: self }
    }
}
