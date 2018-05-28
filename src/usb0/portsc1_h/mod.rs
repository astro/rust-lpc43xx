#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PORTSC1_H {
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
#[doc = "Possible values of the field `CCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCSR {
    #[doc = "No device is present."]
    NO_DEVICE_IS_PRESENT,
    #[doc = "Device is present on the port."]
    DEVICE_IS_PRESENT_ON,
}
impl CCSR {
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
            CCSR::NO_DEVICE_IS_PRESENT => false,
            CCSR::DEVICE_IS_PRESENT_ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCSR {
        match value {
            false => CCSR::NO_DEVICE_IS_PRESENT,
            true => CCSR::DEVICE_IS_PRESENT_ON,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DEVICE_IS_PRESENT`"]
    #[inline]
    pub fn is_no_device_is_present(&self) -> bool {
        *self == CCSR::NO_DEVICE_IS_PRESENT
    }
    #[doc = "Checks if the value of the field is `DEVICE_IS_PRESENT_ON`"]
    #[inline]
    pub fn is_device_is_present_on(&self) -> bool {
        *self == CCSR::DEVICE_IS_PRESENT_ON
    }
}
#[doc = "Possible values of the field `CSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCR {
    #[doc = "No change in current status."]
    NO_CHANGE_IN_CURRENT,
    #[doc = "Change in current status."]
    CHANGE_IN_CURRENT_ST,
}
impl CSCR {
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
            CSCR::NO_CHANGE_IN_CURRENT => false,
            CSCR::CHANGE_IN_CURRENT_ST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSCR {
        match value {
            false => CSCR::NO_CHANGE_IN_CURRENT,
            true => CSCR::CHANGE_IN_CURRENT_ST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE_IN_CURRENT`"]
    #[inline]
    pub fn is_no_change_in_current(&self) -> bool {
        *self == CSCR::NO_CHANGE_IN_CURRENT
    }
    #[doc = "Checks if the value of the field is `CHANGE_IN_CURRENT_ST`"]
    #[inline]
    pub fn is_change_in_current_st(&self) -> bool {
        *self == CSCR::CHANGE_IN_CURRENT_ST
    }
}
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "Port disabled."]
    PORT_DISABLED_,
    #[doc = "Port enabled."]
    PORT_ENABLED_,
}
impl PER {
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
            PER::PORT_DISABLED_ => false,
            PER::PORT_ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER {
        match value {
            false => PER::PORT_DISABLED_,
            true => PER::PORT_ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_DISABLED_`"]
    #[inline]
    pub fn is_port_disabled_(&self) -> bool {
        *self == PER::PORT_DISABLED_
    }
    #[doc = "Checks if the value of the field is `PORT_ENABLED_`"]
    #[inline]
    pub fn is_port_enabled_(&self) -> bool {
        *self == PER::PORT_ENABLED_
    }
}
#[doc = "Possible values of the field `PEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECR {
    #[doc = "No change."]
    NO_CHANGE_,
    #[doc = "Port enabled/disabled status has changed."]
    CHANGED,
}
impl PECR {
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
            PECR::NO_CHANGE_ => false,
            PECR::CHANGED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PECR {
        match value {
            false => PECR::NO_CHANGE_,
            true => PECR::CHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE_`"]
    #[inline]
    pub fn is_no_change_(&self) -> bool {
        *self == PECR::NO_CHANGE_
    }
    #[doc = "Checks if the value of the field is `CHANGED`"]
    #[inline]
    pub fn is_changed(&self) -> bool {
        *self == PECR::CHANGED
    }
}
#[doc = "Possible values of the field `OCA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCAR {
    #[doc = "The port does not have an over-current condition."]
    THE_PORT_DOES_NOT_HA,
    #[doc = "The port has currently an over-current condition."]
    THE_PORT_HAS_CURRENT,
}
impl OCAR {
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
            OCAR::THE_PORT_DOES_NOT_HA => false,
            OCAR::THE_PORT_HAS_CURRENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCAR {
        match value {
            false => OCAR::THE_PORT_DOES_NOT_HA,
            true => OCAR::THE_PORT_HAS_CURRENT,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PORT_DOES_NOT_HA`"]
    #[inline]
    pub fn is_the_port_does_not_ha(&self) -> bool {
        *self == OCAR::THE_PORT_DOES_NOT_HA
    }
    #[doc = "Checks if the value of the field is `THE_PORT_HAS_CURRENT`"]
    #[inline]
    pub fn is_the_port_has_current(&self) -> bool {
        *self == OCAR::THE_PORT_HAS_CURRENT
    }
}
#[doc = r" Value of the field"]
pub struct OCCR {
    bits: bool,
}
impl OCCR {
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
#[doc = "Possible values of the field `FPR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRR {
    #[doc = "No resume (K-state) detected/driven on port."]
    NO_RESUME,
    #[doc = "Resume detected/driven on port."]
    RESUME_DETECTED,
}
impl FPRR {
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
            FPRR::NO_RESUME => false,
            FPRR::RESUME_DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FPRR {
        match value {
            false => FPRR::NO_RESUME,
            true => FPRR::RESUME_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESUME`"]
    #[inline]
    pub fn is_no_resume(&self) -> bool {
        *self == FPRR::NO_RESUME
    }
    #[doc = "Checks if the value of the field is `RESUME_DETECTED`"]
    #[inline]
    pub fn is_resume_detected(&self) -> bool {
        *self == FPRR::RESUME_DETECTED
    }
}
#[doc = "Possible values of the field `SUSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPR {
    #[doc = "Port not in suspend state"]
    PORT_NOT_IN_SUSPEND_,
    #[doc = "Port in suspend state When in suspend state, downstream propagation of data is blocked on this port, except for port reset. The blocking occurs at the end of the current transaction if a transaction was in progress when this bit was written to 1. In the suspend state, the port is sensitive to resume detection. Note that the bit status does not change until the port is suspended and that there may be a delay in suspending a port if there is a transaction currently in progress on the USB."]
    PORT_IN_SUSPEND_STAT,
}
impl SUSPR {
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
            SUSPR::PORT_NOT_IN_SUSPEND_ => false,
            SUSPR::PORT_IN_SUSPEND_STAT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUSPR {
        match value {
            false => SUSPR::PORT_NOT_IN_SUSPEND_,
            true => SUSPR::PORT_IN_SUSPEND_STAT,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_NOT_IN_SUSPEND_`"]
    #[inline]
    pub fn is_port_not_in_suspend_(&self) -> bool {
        *self == SUSPR::PORT_NOT_IN_SUSPEND_
    }
    #[doc = "Checks if the value of the field is `PORT_IN_SUSPEND_STAT`"]
    #[inline]
    pub fn is_port_in_suspend_stat(&self) -> bool {
        *self == SUSPR::PORT_IN_SUSPEND_STAT
    }
}
#[doc = "Possible values of the field `PR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRR {
    #[doc = "Port is not in the reset state."]
    PORT_IS_NOT_IN_THE_R,
    #[doc = "Port is in the reset state."]
    PORT_IS_IN_THE_RESET,
}
impl PRR {
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
            PRR::PORT_IS_NOT_IN_THE_R => false,
            PRR::PORT_IS_IN_THE_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRR {
        match value {
            false => PRR::PORT_IS_NOT_IN_THE_R,
            true => PRR::PORT_IS_IN_THE_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_IS_NOT_IN_THE_R`"]
    #[inline]
    pub fn is_port_is_not_in_the_r(&self) -> bool {
        *self == PRR::PORT_IS_NOT_IN_THE_R
    }
    #[doc = "Checks if the value of the field is `PORT_IS_IN_THE_RESET`"]
    #[inline]
    pub fn is_port_is_in_the_reset(&self) -> bool {
        *self == PRR::PORT_IS_IN_THE_RESET
    }
}
#[doc = "Possible values of the field `HSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSPR {
    #[doc = "Host/device connected to the port is not in High-speed mode."]
    NO_HISPEED,
    #[doc = "Host/device connected to the port is in High-speed mode."]
    HISPEED,
}
impl HSPR {
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
            HSPR::NO_HISPEED => false,
            HSPR::HISPEED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSPR {
        match value {
            false => HSPR::NO_HISPEED,
            true => HSPR::HISPEED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HISPEED`"]
    #[inline]
    pub fn is_no_hispeed(&self) -> bool {
        *self == HSPR::NO_HISPEED
    }
    #[doc = "Checks if the value of the field is `HISPEED`"]
    #[inline]
    pub fn is_hispeed(&self) -> bool {
        *self == HSPR::HISPEED
    }
}
#[doc = "Possible values of the field `LS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSR {
    #[doc = "SE0 (USB_DP and USB_DM LOW)"]
    SE0,
    #[doc = "J-state (USB_DP HIGH and USB_DM LOW)"]
    J_STATE,
    #[doc = "K-state (USB_DP LOW and USB_DM HIGH)"]
    K_STATE,
    #[doc = "Undefined"]
    UNDEFINED,
}
impl LSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LSR::SE0 => 0,
            LSR::J_STATE => 1,
            LSR::K_STATE => 2,
            LSR::UNDEFINED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LSR {
        match value {
            0 => LSR::SE0,
            1 => LSR::J_STATE,
            2 => LSR::K_STATE,
            3 => LSR::UNDEFINED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE0`"]
    #[inline]
    pub fn is_se0(&self) -> bool {
        *self == LSR::SE0
    }
    #[doc = "Checks if the value of the field is `J_STATE`"]
    #[inline]
    pub fn is_j_state(&self) -> bool {
        *self == LSR::J_STATE
    }
    #[doc = "Checks if the value of the field is `K_STATE`"]
    #[inline]
    pub fn is_k_state(&self) -> bool {
        *self == LSR::K_STATE
    }
    #[doc = "Checks if the value of the field is `UNDEFINED`"]
    #[inline]
    pub fn is_undefined(&self) -> bool {
        *self == LSR::UNDEFINED
    }
}
#[doc = "Possible values of the field `PP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPR {
    #[doc = "Port power off."]
    PORT_POWER_OFF_,
    #[doc = "Port power on."]
    PORT_POWER_ON_,
}
impl PPR {
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
            PPR::PORT_POWER_OFF_ => false,
            PPR::PORT_POWER_ON_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPR {
        match value {
            false => PPR::PORT_POWER_OFF_,
            true => PPR::PORT_POWER_ON_,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_POWER_OFF_`"]
    #[inline]
    pub fn is_port_power_off_(&self) -> bool {
        *self == PPR::PORT_POWER_OFF_
    }
    #[doc = "Checks if the value of the field is `PORT_POWER_ON_`"]
    #[inline]
    pub fn is_port_power_on_(&self) -> bool {
        *self == PPR::PORT_POWER_ON_
    }
}
#[doc = "Possible values of the field `PIC1_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIC1_0R {
    #[doc = "Port indicators are off."]
    PORT_INDICATORS_ARE_,
    #[doc = "Amber"]
    AMBER,
    #[doc = "Green"]
    GREEN,
    #[doc = "Undefined"]
    UNDEFINED,
}
impl PIC1_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIC1_0R::PORT_INDICATORS_ARE_ => 0,
            PIC1_0R::AMBER => 1,
            PIC1_0R::GREEN => 2,
            PIC1_0R::UNDEFINED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PIC1_0R {
        match value {
            0 => PIC1_0R::PORT_INDICATORS_ARE_,
            1 => PIC1_0R::AMBER,
            2 => PIC1_0R::GREEN,
            3 => PIC1_0R::UNDEFINED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PORT_INDICATORS_ARE_`"]
    #[inline]
    pub fn is_port_indicators_are_(&self) -> bool {
        *self == PIC1_0R::PORT_INDICATORS_ARE_
    }
    #[doc = "Checks if the value of the field is `AMBER`"]
    #[inline]
    pub fn is_amber(&self) -> bool {
        *self == PIC1_0R::AMBER
    }
    #[doc = "Checks if the value of the field is `GREEN`"]
    #[inline]
    pub fn is_green(&self) -> bool {
        *self == PIC1_0R::GREEN
    }
    #[doc = "Checks if the value of the field is `UNDEFINED`"]
    #[inline]
    pub fn is_undefined(&self) -> bool {
        *self == PIC1_0R::UNDEFINED
    }
}
#[doc = "Possible values of the field `PTC3_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTC3_0R {
    #[doc = "TEST_MODE_DISABLE"]
    TEST_MODE_DISABLE,
    #[doc = "J_STATE"]
    J_STATE,
    #[doc = "K_STATE"]
    K_STATE,
    #[doc = "SE0 (host)/NAK (device)"]
    SE0_NAK,
    #[doc = "Packet"]
    PACKET,
    #[doc = "FORCE_ENABLE_HS"]
    FORCE_ENABLE_HS,
    #[doc = "FORCE_ENABLE_FS"]
    FORCE_ENABLE_FS,
    #[doc = "FORCE_ENABLE_LS"]
    FORCE_ENABLE_LS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PTC3_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PTC3_0R::TEST_MODE_DISABLE => 0,
            PTC3_0R::J_STATE => 1,
            PTC3_0R::K_STATE => 2,
            PTC3_0R::SE0_NAK => 3,
            PTC3_0R::PACKET => 4,
            PTC3_0R::FORCE_ENABLE_HS => 5,
            PTC3_0R::FORCE_ENABLE_FS => 6,
            PTC3_0R::FORCE_ENABLE_LS => 7,
            PTC3_0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PTC3_0R {
        match value {
            0 => PTC3_0R::TEST_MODE_DISABLE,
            1 => PTC3_0R::J_STATE,
            2 => PTC3_0R::K_STATE,
            3 => PTC3_0R::SE0_NAK,
            4 => PTC3_0R::PACKET,
            5 => PTC3_0R::FORCE_ENABLE_HS,
            6 => PTC3_0R::FORCE_ENABLE_FS,
            7 => PTC3_0R::FORCE_ENABLE_LS,
            i => PTC3_0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TEST_MODE_DISABLE`"]
    #[inline]
    pub fn is_test_mode_disable(&self) -> bool {
        *self == PTC3_0R::TEST_MODE_DISABLE
    }
    #[doc = "Checks if the value of the field is `J_STATE`"]
    #[inline]
    pub fn is_j_state(&self) -> bool {
        *self == PTC3_0R::J_STATE
    }
    #[doc = "Checks if the value of the field is `K_STATE`"]
    #[inline]
    pub fn is_k_state(&self) -> bool {
        *self == PTC3_0R::K_STATE
    }
    #[doc = "Checks if the value of the field is `SE0_NAK`"]
    #[inline]
    pub fn is_se0_nak(&self) -> bool {
        *self == PTC3_0R::SE0_NAK
    }
    #[doc = "Checks if the value of the field is `PACKET`"]
    #[inline]
    pub fn is_packet(&self) -> bool {
        *self == PTC3_0R::PACKET
    }
    #[doc = "Checks if the value of the field is `FORCE_ENABLE_HS`"]
    #[inline]
    pub fn is_force_enable_hs(&self) -> bool {
        *self == PTC3_0R::FORCE_ENABLE_HS
    }
    #[doc = "Checks if the value of the field is `FORCE_ENABLE_FS`"]
    #[inline]
    pub fn is_force_enable_fs(&self) -> bool {
        *self == PTC3_0R::FORCE_ENABLE_FS
    }
    #[doc = "Checks if the value of the field is `FORCE_ENABLE_LS`"]
    #[inline]
    pub fn is_force_enable_ls(&self) -> bool {
        *self == PTC3_0R::FORCE_ENABLE_LS
    }
}
#[doc = "Possible values of the field `WKCN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKCNR {
    #[doc = "Disables the port to wake up on device connects."]
    DISABLES_THE_PORT_TO,
    #[doc = "Writing this bit to a one enables the port to be sensitive to device connects as wake-up events."]
    WRITING_THIS_BIT_TO_,
}
impl WKCNR {
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
            WKCNR::DISABLES_THE_PORT_TO => false,
            WKCNR::WRITING_THIS_BIT_TO_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKCNR {
        match value {
            false => WKCNR::DISABLES_THE_PORT_TO,
            true => WKCNR::WRITING_THIS_BIT_TO_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLES_THE_PORT_TO`"]
    #[inline]
    pub fn is_disables_the_port_to(&self) -> bool {
        *self == WKCNR::DISABLES_THE_PORT_TO
    }
    #[doc = "Checks if the value of the field is `WRITING_THIS_BIT_TO_`"]
    #[inline]
    pub fn is_writing_this_bit_to_(&self) -> bool {
        *self == WKCNR::WRITING_THIS_BIT_TO_
    }
}
#[doc = "Possible values of the field `WKDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKDCR {
    #[doc = "Disables the port to wake up on device disconnects."]
    DISABLES_THE_PORT_TO,
    #[doc = "Writing this bit to a one enables the port to be sensitive to device disconnects as wake-up events."]
    WRITING_THIS_BIT_TO_,
}
impl WKDCR {
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
            WKDCR::DISABLES_THE_PORT_TO => false,
            WKDCR::WRITING_THIS_BIT_TO_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKDCR {
        match value {
            false => WKDCR::DISABLES_THE_PORT_TO,
            true => WKDCR::WRITING_THIS_BIT_TO_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLES_THE_PORT_TO`"]
    #[inline]
    pub fn is_disables_the_port_to(&self) -> bool {
        *self == WKDCR::DISABLES_THE_PORT_TO
    }
    #[doc = "Checks if the value of the field is `WRITING_THIS_BIT_TO_`"]
    #[inline]
    pub fn is_writing_this_bit_to_(&self) -> bool {
        *self == WKDCR::WRITING_THIS_BIT_TO_
    }
}
#[doc = "Possible values of the field `WKOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKOCR {
    #[doc = "Disables the port to wake up on over-current events."]
    DISABLES_THE_PORT_TO,
    #[doc = "Writing a one to this bit enabled the port to be sensitive to over-current conditions as wake-up events."]
    WRITING_A_ONE_TO_THI,
}
impl WKOCR {
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
            WKOCR::DISABLES_THE_PORT_TO => false,
            WKOCR::WRITING_A_ONE_TO_THI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKOCR {
        match value {
            false => WKOCR::DISABLES_THE_PORT_TO,
            true => WKOCR::WRITING_A_ONE_TO_THI,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLES_THE_PORT_TO`"]
    #[inline]
    pub fn is_disables_the_port_to(&self) -> bool {
        *self == WKOCR::DISABLES_THE_PORT_TO
    }
    #[doc = "Checks if the value of the field is `WRITING_A_ONE_TO_THI`"]
    #[inline]
    pub fn is_writing_a_one_to_thi(&self) -> bool {
        *self == WKOCR::WRITING_A_ONE_TO_THI
    }
}
#[doc = "Possible values of the field `PHCD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHCDR {
    #[doc = "Writing a 0 enables the PHY clock. Reading a 0 indicates the status of the PHY clock (enabled)."]
    WRITING_A_0_ENABLES_,
    #[doc = "Writing a 1 disables the PHY clock. Reading a 1 indicates the status of the PHY clock (disabled)."]
    WRITING_A_1_DISABLES,
}
impl PHCDR {
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
            PHCDR::WRITING_A_0_ENABLES_ => false,
            PHCDR::WRITING_A_1_DISABLES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PHCDR {
        match value {
            false => PHCDR::WRITING_A_0_ENABLES_,
            true => PHCDR::WRITING_A_1_DISABLES,
        }
    }
    #[doc = "Checks if the value of the field is `WRITING_A_0_ENABLES_`"]
    #[inline]
    pub fn is_writing_a_0_enables_(&self) -> bool {
        *self == PHCDR::WRITING_A_0_ENABLES_
    }
    #[doc = "Checks if the value of the field is `WRITING_A_1_DISABLES`"]
    #[inline]
    pub fn is_writing_a_1_disables(&self) -> bool {
        *self == PHCDR::WRITING_A_1_DISABLES
    }
}
#[doc = "Possible values of the field `PFSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFSCR {
    #[doc = "Port connects at any speed."]
    PORT_CONNECTS_AT_ANY,
    #[doc = "Writing this bit to a 1 will force the port to only connect at Full Speed. It disables the chirp sequence that allows the port to identify itself as High Speed. This is useful for testing FS configurations with a HS host, hub or device."]
    WRITING_THIS_BIT_TO_,
}
impl PFSCR {
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
            PFSCR::PORT_CONNECTS_AT_ANY => false,
            PFSCR::WRITING_THIS_BIT_TO_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFSCR {
        match value {
            false => PFSCR::PORT_CONNECTS_AT_ANY,
            true => PFSCR::WRITING_THIS_BIT_TO_,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_CONNECTS_AT_ANY`"]
    #[inline]
    pub fn is_port_connects_at_any(&self) -> bool {
        *self == PFSCR::PORT_CONNECTS_AT_ANY
    }
    #[doc = "Checks if the value of the field is `WRITING_THIS_BIT_TO_`"]
    #[inline]
    pub fn is_writing_this_bit_to_(&self) -> bool {
        *self == PFSCR::WRITING_THIS_BIT_TO_
    }
}
#[doc = "Possible values of the field `PSPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSPDR {
    #[doc = "Full-speed"]
    FULL_SPEED,
    #[doc = "Low-speed"]
    LOW_SPEED,
    #[doc = "High-speed"]
    HIGH_SPEED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PSPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSPDR::FULL_SPEED => 0,
            PSPDR::LOW_SPEED => 1,
            PSPDR::HIGH_SPEED => 2,
            PSPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSPDR {
        match value {
            0 => PSPDR::FULL_SPEED,
            1 => PSPDR::LOW_SPEED,
            2 => PSPDR::HIGH_SPEED,
            i => PSPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FULL_SPEED`"]
    #[inline]
    pub fn is_full_speed(&self) -> bool {
        *self == PSPDR::FULL_SPEED
    }
    #[doc = "Checks if the value of the field is `LOW_SPEED`"]
    #[inline]
    pub fn is_low_speed(&self) -> bool {
        *self == PSPDR::LOW_SPEED
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline]
    pub fn is_high_speed(&self) -> bool {
        *self == PSPDR::HIGH_SPEED
    }
}
#[doc = "Values that can be written to the field `CCS`"]
pub enum CCSW {
    #[doc = "No device is present."]
    NO_DEVICE_IS_PRESENT,
    #[doc = "Device is present on the port."]
    DEVICE_IS_PRESENT_ON,
}
impl CCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCSW::NO_DEVICE_IS_PRESENT => false,
            CCSW::DEVICE_IS_PRESENT_ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No device is present."]
    #[inline]
    pub fn no_device_is_present(self) -> &'a mut W {
        self.variant(CCSW::NO_DEVICE_IS_PRESENT)
    }
    #[doc = "Device is present on the port."]
    #[inline]
    pub fn device_is_present_on(self) -> &'a mut W {
        self.variant(CCSW::DEVICE_IS_PRESENT_ON)
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
#[doc = "Values that can be written to the field `CSC`"]
pub enum CSCW {
    #[doc = "No change in current status."]
    NO_CHANGE_IN_CURRENT,
    #[doc = "Change in current status."]
    CHANGE_IN_CURRENT_ST,
}
impl CSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSCW::NO_CHANGE_IN_CURRENT => false,
            CSCW::CHANGE_IN_CURRENT_ST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change in current status."]
    #[inline]
    pub fn no_change_in_current(self) -> &'a mut W {
        self.variant(CSCW::NO_CHANGE_IN_CURRENT)
    }
    #[doc = "Change in current status."]
    #[inline]
    pub fn change_in_current_st(self) -> &'a mut W {
        self.variant(CSCW::CHANGE_IN_CURRENT_ST)
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
#[doc = "Values that can be written to the field `PE`"]
pub enum PEW {
    #[doc = "Port disabled."]
    PORT_DISABLED_,
    #[doc = "Port enabled."]
    PORT_ENABLED_,
}
impl PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEW::PORT_DISABLED_ => false,
            PEW::PORT_ENABLED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port disabled."]
    #[inline]
    pub fn port_disabled_(self) -> &'a mut W {
        self.variant(PEW::PORT_DISABLED_)
    }
    #[doc = "Port enabled."]
    #[inline]
    pub fn port_enabled_(self) -> &'a mut W {
        self.variant(PEW::PORT_ENABLED_)
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
#[doc = "Values that can be written to the field `PEC`"]
pub enum PECW {
    #[doc = "No change."]
    NO_CHANGE_,
    #[doc = "Port enabled/disabled status has changed."]
    CHANGED,
}
impl PECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PECW::NO_CHANGE_ => false,
            PECW::CHANGED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PECW<'a> {
    w: &'a mut W,
}
impl<'a> _PECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PECW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change_(self) -> &'a mut W {
        self.variant(PECW::NO_CHANGE_)
    }
    #[doc = "Port enabled/disabled status has changed."]
    #[inline]
    pub fn changed(self) -> &'a mut W {
        self.variant(PECW::CHANGED)
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
#[doc = "Values that can be written to the field `OCA`"]
pub enum OCAW {
    #[doc = "The port does not have an over-current condition."]
    THE_PORT_DOES_NOT_HA,
    #[doc = "The port has currently an over-current condition."]
    THE_PORT_HAS_CURRENT,
}
impl OCAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCAW::THE_PORT_DOES_NOT_HA => false,
            OCAW::THE_PORT_HAS_CURRENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCAW<'a> {
    w: &'a mut W,
}
impl<'a> _OCAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The port does not have an over-current condition."]
    #[inline]
    pub fn the_port_does_not_ha(self) -> &'a mut W {
        self.variant(OCAW::THE_PORT_DOES_NOT_HA)
    }
    #[doc = "The port has currently an over-current condition."]
    #[inline]
    pub fn the_port_has_current(self) -> &'a mut W {
        self.variant(OCAW::THE_PORT_HAS_CURRENT)
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
#[doc = r" Proxy"]
pub struct _OCCW<'a> {
    w: &'a mut W,
}
impl<'a> _OCCW<'a> {
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
#[doc = "Values that can be written to the field `FPR`"]
pub enum FPRW {
    #[doc = "No resume (K-state) detected/driven on port."]
    NO_RESUME,
    #[doc = "Resume detected/driven on port."]
    RESUME_DETECTED,
}
impl FPRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FPRW::NO_RESUME => false,
            FPRW::RESUME_DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FPRW<'a> {
    w: &'a mut W,
}
impl<'a> _FPRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FPRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No resume (K-state) detected/driven on port."]
    #[inline]
    pub fn no_resume(self) -> &'a mut W {
        self.variant(FPRW::NO_RESUME)
    }
    #[doc = "Resume detected/driven on port."]
    #[inline]
    pub fn resume_detected(self) -> &'a mut W {
        self.variant(FPRW::RESUME_DETECTED)
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
#[doc = "Values that can be written to the field `SUSP`"]
pub enum SUSPW {
    #[doc = "Port not in suspend state"]
    PORT_NOT_IN_SUSPEND_,
    #[doc = "Port in suspend state When in suspend state, downstream propagation of data is blocked on this port, except for port reset. The blocking occurs at the end of the current transaction if a transaction was in progress when this bit was written to 1. In the suspend state, the port is sensitive to resume detection. Note that the bit status does not change until the port is suspended and that there may be a delay in suspending a port if there is a transaction currently in progress on the USB."]
    PORT_IN_SUSPEND_STAT,
}
impl SUSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUSPW::PORT_NOT_IN_SUSPEND_ => false,
            SUSPW::PORT_IN_SUSPEND_STAT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port not in suspend state"]
    #[inline]
    pub fn port_not_in_suspend_(self) -> &'a mut W {
        self.variant(SUSPW::PORT_NOT_IN_SUSPEND_)
    }
    #[doc = "Port in suspend state When in suspend state, downstream propagation of data is blocked on this port, except for port reset. The blocking occurs at the end of the current transaction if a transaction was in progress when this bit was written to 1. In the suspend state, the port is sensitive to resume detection. Note that the bit status does not change until the port is suspended and that there may be a delay in suspending a port if there is a transaction currently in progress on the USB."]
    #[inline]
    pub fn port_in_suspend_stat(self) -> &'a mut W {
        self.variant(SUSPW::PORT_IN_SUSPEND_STAT)
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
#[doc = "Values that can be written to the field `PR`"]
pub enum PRW {
    #[doc = "Port is not in the reset state."]
    PORT_IS_NOT_IN_THE_R,
    #[doc = "Port is in the reset state."]
    PORT_IS_IN_THE_RESET,
}
impl PRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRW::PORT_IS_NOT_IN_THE_R => false,
            PRW::PORT_IS_IN_THE_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRW<'a> {
    w: &'a mut W,
}
impl<'a> _PRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port is not in the reset state."]
    #[inline]
    pub fn port_is_not_in_the_r(self) -> &'a mut W {
        self.variant(PRW::PORT_IS_NOT_IN_THE_R)
    }
    #[doc = "Port is in the reset state."]
    #[inline]
    pub fn port_is_in_the_reset(self) -> &'a mut W {
        self.variant(PRW::PORT_IS_IN_THE_RESET)
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
#[doc = "Values that can be written to the field `HSP`"]
pub enum HSPW {
    #[doc = "Host/device connected to the port is not in High-speed mode."]
    NO_HISPEED,
    #[doc = "Host/device connected to the port is in High-speed mode."]
    HISPEED,
}
impl HSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSPW::NO_HISPEED => false,
            HSPW::HISPEED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSPW<'a> {
    w: &'a mut W,
}
impl<'a> _HSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Host/device connected to the port is not in High-speed mode."]
    #[inline]
    pub fn no_hispeed(self) -> &'a mut W {
        self.variant(HSPW::NO_HISPEED)
    }
    #[doc = "Host/device connected to the port is in High-speed mode."]
    #[inline]
    pub fn hispeed(self) -> &'a mut W {
        self.variant(HSPW::HISPEED)
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
#[doc = "Values that can be written to the field `LS`"]
pub enum LSW {
    #[doc = "SE0 (USB_DP and USB_DM LOW)"]
    SE0,
    #[doc = "J-state (USB_DP HIGH and USB_DM LOW)"]
    J_STATE,
    #[doc = "K-state (USB_DP LOW and USB_DM HIGH)"]
    K_STATE,
    #[doc = "Undefined"]
    UNDEFINED,
}
impl LSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LSW::SE0 => 0,
            LSW::J_STATE => 1,
            LSW::K_STATE => 2,
            LSW::UNDEFINED => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSW<'a> {
    w: &'a mut W,
}
impl<'a> _LSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SE0 (USB_DP and USB_DM LOW)"]
    #[inline]
    pub fn se0(self) -> &'a mut W {
        self.variant(LSW::SE0)
    }
    #[doc = "J-state (USB_DP HIGH and USB_DM LOW)"]
    #[inline]
    pub fn j_state(self) -> &'a mut W {
        self.variant(LSW::J_STATE)
    }
    #[doc = "K-state (USB_DP LOW and USB_DM HIGH)"]
    #[inline]
    pub fn k_state(self) -> &'a mut W {
        self.variant(LSW::K_STATE)
    }
    #[doc = "Undefined"]
    #[inline]
    pub fn undefined(self) -> &'a mut W {
        self.variant(LSW::UNDEFINED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PP`"]
pub enum PPW {
    #[doc = "Port power off."]
    PORT_POWER_OFF_,
    #[doc = "Port power on."]
    PORT_POWER_ON_,
}
impl PPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPW::PORT_POWER_OFF_ => false,
            PPW::PORT_POWER_ON_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPW<'a> {
    w: &'a mut W,
}
impl<'a> _PPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port power off."]
    #[inline]
    pub fn port_power_off_(self) -> &'a mut W {
        self.variant(PPW::PORT_POWER_OFF_)
    }
    #[doc = "Port power on."]
    #[inline]
    pub fn port_power_on_(self) -> &'a mut W {
        self.variant(PPW::PORT_POWER_ON_)
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
#[doc = "Values that can be written to the field `PIC1_0`"]
pub enum PIC1_0W {
    #[doc = "Port indicators are off."]
    PORT_INDICATORS_ARE_,
    #[doc = "Amber"]
    AMBER,
    #[doc = "Green"]
    GREEN,
    #[doc = "Undefined"]
    UNDEFINED,
}
impl PIC1_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PIC1_0W::PORT_INDICATORS_ARE_ => 0,
            PIC1_0W::AMBER => 1,
            PIC1_0W::GREEN => 2,
            PIC1_0W::UNDEFINED => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIC1_0W<'a> {
    w: &'a mut W,
}
impl<'a> _PIC1_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIC1_0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Port indicators are off."]
    #[inline]
    pub fn port_indicators_are_(self) -> &'a mut W {
        self.variant(PIC1_0W::PORT_INDICATORS_ARE_)
    }
    #[doc = "Amber"]
    #[inline]
    pub fn amber(self) -> &'a mut W {
        self.variant(PIC1_0W::AMBER)
    }
    #[doc = "Green"]
    #[inline]
    pub fn green(self) -> &'a mut W {
        self.variant(PIC1_0W::GREEN)
    }
    #[doc = "Undefined"]
    #[inline]
    pub fn undefined(self) -> &'a mut W {
        self.variant(PIC1_0W::UNDEFINED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PTC3_0`"]
pub enum PTC3_0W {
    #[doc = "TEST_MODE_DISABLE"]
    TEST_MODE_DISABLE,
    #[doc = "J_STATE"]
    J_STATE,
    #[doc = "K_STATE"]
    K_STATE,
    #[doc = "SE0 (host)/NAK (device)"]
    SE0_NAK,
    #[doc = "Packet"]
    PACKET,
    #[doc = "FORCE_ENABLE_HS"]
    FORCE_ENABLE_HS,
    #[doc = "FORCE_ENABLE_FS"]
    FORCE_ENABLE_FS,
    #[doc = "FORCE_ENABLE_LS"]
    FORCE_ENABLE_LS,
}
impl PTC3_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PTC3_0W::TEST_MODE_DISABLE => 0,
            PTC3_0W::J_STATE => 1,
            PTC3_0W::K_STATE => 2,
            PTC3_0W::SE0_NAK => 3,
            PTC3_0W::PACKET => 4,
            PTC3_0W::FORCE_ENABLE_HS => 5,
            PTC3_0W::FORCE_ENABLE_FS => 6,
            PTC3_0W::FORCE_ENABLE_LS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTC3_0W<'a> {
    w: &'a mut W,
}
impl<'a> _PTC3_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTC3_0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TEST_MODE_DISABLE"]
    #[inline]
    pub fn test_mode_disable(self) -> &'a mut W {
        self.variant(PTC3_0W::TEST_MODE_DISABLE)
    }
    #[doc = "J_STATE"]
    #[inline]
    pub fn j_state(self) -> &'a mut W {
        self.variant(PTC3_0W::J_STATE)
    }
    #[doc = "K_STATE"]
    #[inline]
    pub fn k_state(self) -> &'a mut W {
        self.variant(PTC3_0W::K_STATE)
    }
    #[doc = "SE0 (host)/NAK (device)"]
    #[inline]
    pub fn se0_nak(self) -> &'a mut W {
        self.variant(PTC3_0W::SE0_NAK)
    }
    #[doc = "Packet"]
    #[inline]
    pub fn packet(self) -> &'a mut W {
        self.variant(PTC3_0W::PACKET)
    }
    #[doc = "FORCE_ENABLE_HS"]
    #[inline]
    pub fn force_enable_hs(self) -> &'a mut W {
        self.variant(PTC3_0W::FORCE_ENABLE_HS)
    }
    #[doc = "FORCE_ENABLE_FS"]
    #[inline]
    pub fn force_enable_fs(self) -> &'a mut W {
        self.variant(PTC3_0W::FORCE_ENABLE_FS)
    }
    #[doc = "FORCE_ENABLE_LS"]
    #[inline]
    pub fn force_enable_ls(self) -> &'a mut W {
        self.variant(PTC3_0W::FORCE_ENABLE_LS)
    }
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
#[doc = "Values that can be written to the field `WKCN`"]
pub enum WKCNW {
    #[doc = "Disables the port to wake up on device connects."]
    DISABLES_THE_PORT_TO,
    #[doc = "Writing this bit to a one enables the port to be sensitive to device connects as wake-up events."]
    WRITING_THIS_BIT_TO_,
}
impl WKCNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKCNW::DISABLES_THE_PORT_TO => false,
            WKCNW::WRITING_THIS_BIT_TO_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKCNW<'a> {
    w: &'a mut W,
}
impl<'a> _WKCNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKCNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the port to wake up on device connects."]
    #[inline]
    pub fn disables_the_port_to(self) -> &'a mut W {
        self.variant(WKCNW::DISABLES_THE_PORT_TO)
    }
    #[doc = "Writing this bit to a one enables the port to be sensitive to device connects as wake-up events."]
    #[inline]
    pub fn writing_this_bit_to_(self) -> &'a mut W {
        self.variant(WKCNW::WRITING_THIS_BIT_TO_)
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
#[doc = "Values that can be written to the field `WKDC`"]
pub enum WKDCW {
    #[doc = "Disables the port to wake up on device disconnects."]
    DISABLES_THE_PORT_TO,
    #[doc = "Writing this bit to a one enables the port to be sensitive to device disconnects as wake-up events."]
    WRITING_THIS_BIT_TO_,
}
impl WKDCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKDCW::DISABLES_THE_PORT_TO => false,
            WKDCW::WRITING_THIS_BIT_TO_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKDCW<'a> {
    w: &'a mut W,
}
impl<'a> _WKDCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKDCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the port to wake up on device disconnects."]
    #[inline]
    pub fn disables_the_port_to(self) -> &'a mut W {
        self.variant(WKDCW::DISABLES_THE_PORT_TO)
    }
    #[doc = "Writing this bit to a one enables the port to be sensitive to device disconnects as wake-up events."]
    #[inline]
    pub fn writing_this_bit_to_(self) -> &'a mut W {
        self.variant(WKDCW::WRITING_THIS_BIT_TO_)
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
#[doc = "Values that can be written to the field `WKOC`"]
pub enum WKOCW {
    #[doc = "Disables the port to wake up on over-current events."]
    DISABLES_THE_PORT_TO,
    #[doc = "Writing a one to this bit enabled the port to be sensitive to over-current conditions as wake-up events."]
    WRITING_A_ONE_TO_THI,
}
impl WKOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKOCW::DISABLES_THE_PORT_TO => false,
            WKOCW::WRITING_A_ONE_TO_THI => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKOCW<'a> {
    w: &'a mut W,
}
impl<'a> _WKOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKOCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the port to wake up on over-current events."]
    #[inline]
    pub fn disables_the_port_to(self) -> &'a mut W {
        self.variant(WKOCW::DISABLES_THE_PORT_TO)
    }
    #[doc = "Writing a one to this bit enabled the port to be sensitive to over-current conditions as wake-up events."]
    #[inline]
    pub fn writing_a_one_to_thi(self) -> &'a mut W {
        self.variant(WKOCW::WRITING_A_ONE_TO_THI)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PHCD`"]
pub enum PHCDW {
    #[doc = "Writing a 0 enables the PHY clock. Reading a 0 indicates the status of the PHY clock (enabled)."]
    WRITING_A_0_ENABLES_,
    #[doc = "Writing a 1 disables the PHY clock. Reading a 1 indicates the status of the PHY clock (disabled)."]
    WRITING_A_1_DISABLES,
}
impl PHCDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PHCDW::WRITING_A_0_ENABLES_ => false,
            PHCDW::WRITING_A_1_DISABLES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PHCDW<'a> {
    w: &'a mut W,
}
impl<'a> _PHCDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PHCDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writing a 0 enables the PHY clock. Reading a 0 indicates the status of the PHY clock (enabled)."]
    #[inline]
    pub fn writing_a_0_enables_(self) -> &'a mut W {
        self.variant(PHCDW::WRITING_A_0_ENABLES_)
    }
    #[doc = "Writing a 1 disables the PHY clock. Reading a 1 indicates the status of the PHY clock (disabled)."]
    #[inline]
    pub fn writing_a_1_disables(self) -> &'a mut W {
        self.variant(PHCDW::WRITING_A_1_DISABLES)
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
#[doc = "Values that can be written to the field `PFSC`"]
pub enum PFSCW {
    #[doc = "Port connects at any speed."]
    PORT_CONNECTS_AT_ANY,
    #[doc = "Writing this bit to a 1 will force the port to only connect at Full Speed. It disables the chirp sequence that allows the port to identify itself as High Speed. This is useful for testing FS configurations with a HS host, hub or device."]
    WRITING_THIS_BIT_TO_,
}
impl PFSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PFSCW::PORT_CONNECTS_AT_ANY => false,
            PFSCW::WRITING_THIS_BIT_TO_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PFSCW<'a> {
    w: &'a mut W,
}
impl<'a> _PFSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PFSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port connects at any speed."]
    #[inline]
    pub fn port_connects_at_any(self) -> &'a mut W {
        self.variant(PFSCW::PORT_CONNECTS_AT_ANY)
    }
    #[doc = "Writing this bit to a 1 will force the port to only connect at Full Speed. It disables the chirp sequence that allows the port to identify itself as High Speed. This is useful for testing FS configurations with a HS host, hub or device."]
    #[inline]
    pub fn writing_this_bit_to_(self) -> &'a mut W {
        self.variant(PFSCW::WRITING_THIS_BIT_TO_)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSPD`"]
pub enum PSPDW {
    #[doc = "Full-speed"]
    FULL_SPEED,
    #[doc = "Low-speed"]
    LOW_SPEED,
    #[doc = "High-speed"]
    HIGH_SPEED,
}
impl PSPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSPDW::FULL_SPEED => 0,
            PSPDW::LOW_SPEED => 1,
            PSPDW::HIGH_SPEED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSPDW<'a> {
    w: &'a mut W,
}
impl<'a> _PSPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSPDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Full-speed"]
    #[inline]
    pub fn full_speed(self) -> &'a mut W {
        self.variant(PSPDW::FULL_SPEED)
    }
    #[doc = "Low-speed"]
    #[inline]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(PSPDW::LOW_SPEED)
    }
    #[doc = "High-speed"]
    #[inline]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(PSPDW::HIGH_SPEED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - Current connect status This value reflects the current state of the port and may not correspond directly to the event that caused the CSC bit to be set. This bit is 0 if PP (Port Power bit) is 0. Software clears this bit by writing a 1 to it."]
    #[inline]
    pub fn ccs(&self) -> CCSR {
        CCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Connect status change Indicates a change has occurred in the port's Current Connect Status. The host/device controller sets this bit for all changes to the port device connect status, even if system software has not cleared an existing connect status change. For example, the insertion status changes twice before system software has cleared the changed condition, hub hardware will be setting an already-set bit (i.e., the bit will remain set). Software clears this bit by writing a one to it. This bit is 0 if PP (Port Power bit) is 0"]
    #[inline]
    pub fn csc(&self) -> CSCR {
        CSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port enable. Ports can only be enabled by the host controller as a part of the reset and enable. Software cannot enable a port by writing a one to this field. Ports can be disabled by either a fault condition (disconnect event or other fault condition) or by the host software. Note that the bit status does not change until the port state actually changes. There may be a delay in disabling or enabling a port due to other host controller and bus events. When the port is disabled. downstream propagation of data is blocked except for reset. This bit is 0 if PP (Port Power bit) is 0."]
    #[inline]
    pub fn pe(&self) -> PER {
        PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port disable/enable change For the root hub, this bit gets set to a one only when a port is disabled due to disconnect on the port or due to the appropriate conditions existing at the EOF2 point (See Chapter 11 of the USB Specification). Software clears this by writing a one to it. This bit is 0 if PP (Port Power bit) is 0,"]
    #[inline]
    pub fn pec(&self) -> PECR {
        PECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Over-current active This bit will automatically transition from 1 to 0 when the over-current condition is removed."]
    #[inline]
    pub fn oca(&self) -> OCAR {
        OCAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Over-current change This bit gets set to one when there is a change to Over-current Active. Software clears this bit by writing a one to this bit position."]
    #[inline]
    pub fn occ(&self) -> OCCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OCCR { bits }
    }
    #[doc = "Bit 6 - Force port resume Software sets this bit to one to drive resume signaling. The Host Controller sets this bit to one if a J-to-K transition is detected while the port is in the Suspend state. When this bit transitions to a one because a J-to-K transition is detected, the Port Change Detect bit in the USBSTS register is also set to one. This bit will automatically change to zero after the resume sequence is complete. This behavior is different from EHCI where the host controller driver is required to set this bit to a zero after the resume duration is timed in the driver. Note that when the Host controller owns the port, the resume sequence follows the defined sequence documented in the USB Specification Revision 2.0. The resume signaling (Full-speed K) is driven on the port as long as this bit remains a one. This bit will remain a one until the port has switched to the high-speed idle. Writing a zero has no affect because the port controller will time the resume operation clear the bit the port control state switches to HS or FS idle. This bit is 0 if PP (Port Power bit) is 0."]
    #[inline]
    pub fn fpr(&self) -> FPRR {
        FPRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Suspend Together with the PE (Port enabled bit), this bit describes the port states, see Table 240. The host controller will unconditionally set this bit to zero when software sets the Force Port Resume bit to zero. The host controller ignores a write of zero to this bit. If host software sets this bit to a one when the port is not enabled (i.e. Port enabled bit is a zero) the results are undefined. This bit is 0 if PP (Port Power bit) is 0."]
    #[inline]
    pub fn susp(&self) -> SUSPR {
        SUSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port reset When software writes a one to this bit the bus-reset sequence as defined in the USB Specification Revision 2.0 is started. This bit will automatically change to zero after the reset sequence is complete. This behavior is different from EHCI where the host controller driver is required to set this bit to a zero after the reset duration is timed in the driver. This bit is 0 if PP (Port Power bit) is 0."]
    #[inline]
    pub fn pr(&self) -> PRR {
        PRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - High-speed status"]
    #[inline]
    pub fn hsp(&self) -> HSPR {
        HSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:11 - Line status These bits reflect the current logical levels of the USB_DP and USB_DM signal lines. USB_DP corresponds to bit 11 and USB_DM to bit 10. In host mode, the use of linestate by the host controller driver is not necessary for this controller (unlike EHCI) because the controller hardware manages the connection of LS and FS."]
    #[inline]
    pub fn ls(&self) -> LSR {
        LSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Port power control Host/OTG controller requires port power control switches. This bit represents the current setting of the switch (0=off, 1=on). When power is not available on a port (i.e. PP equals a 0), the port is non-functional and will not report attaches, detaches, etc. When an over-current condition is detected on a powered port and PPC is a one, the PP bit in each affected port may be transitioned by the host controller driver from a one to a zero (removing power from the port)."]
    #[inline]
    pub fn pp(&self) -> PPR {
        PPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 14:15 - Port indicator control Writing to this field effects the value of the pins USB0_IND1 and USB0_IND0."]
    #[inline]
    pub fn pic1_0(&self) -> PIC1_0R {
        PIC1_0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Port test control Any value other than 0000 indicates that the port is operating in test mode. The FORCE_ENABLE_FS and FORCE ENABLE_LS are extensions to the test mode support specified in the EHCI specification. Writing the PTC field to any of the FORCE_ENABLE_{HS/FS/LS} values will force the port into the connected and enabled state at the selected speed. Writing the PTC field back to TEST_MODE_DISABLE will allow the port state machines to progress normally from that point. Values 0x8 to 0xF are reserved."]
    #[inline]
    pub fn ptc3_0(&self) -> PTC3_0R {
        PTC3_0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Wake on connect enable (WKCNNT_E) This bit is 0 if PP (Port Power bit) is 0"]
    #[inline]
    pub fn wkcn(&self) -> WKCNR {
        WKCNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Wake on disconnect enable (WKDSCNNT_E) This bit is 0 if PP (Port Power bit) is 0."]
    #[inline]
    pub fn wkdc(&self) -> WKDCR {
        WKDCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Wake on over-current enable (WKOC_E)"]
    #[inline]
    pub fn wkoc(&self) -> WKOCR {
        WKOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - PHY low power suspend - clock disable (PLPSCD) In host mode, the PHY can be put into Low Power Suspend - Clock Disable when the downstream device has been put into suspend mode or when no downstream device is connected. Low power suspend is completely under the control of software."]
    #[inline]
    pub fn phcd(&self) -> PHCDR {
        PHCDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Port force full speed connect"]
    #[inline]
    pub fn pfsc(&self) -> PFSCR {
        PFSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 26:27 - Port speed This register field indicates the speed at which the port is operating. For HS mode operation in the host controller and HS/FS operation in the device controller the port routing steers data to the Protocol engine. For FS and LS mode operation in the host controller, the port routing steers data to the Protocol Engine w/ Embedded Transaction Translator."]
    #[inline]
    pub fn pspd(&self) -> PSPDR {
        PSPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 0 - Current connect status This value reflects the current state of the port and may not correspond directly to the event that caused the CSC bit to be set. This bit is 0 if PP (Port Power bit) is 0. Software clears this bit by writing a 1 to it."]
    #[inline]
    pub fn ccs(&mut self) -> _CCSW {
        _CCSW { w: self }
    }
    #[doc = "Bit 1 - Connect status change Indicates a change has occurred in the port's Current Connect Status. The host/device controller sets this bit for all changes to the port device connect status, even if system software has not cleared an existing connect status change. For example, the insertion status changes twice before system software has cleared the changed condition, hub hardware will be setting an already-set bit (i.e., the bit will remain set). Software clears this bit by writing a one to it. This bit is 0 if PP (Port Power bit) is 0"]
    #[inline]
    pub fn csc(&mut self) -> _CSCW {
        _CSCW { w: self }
    }
    #[doc = "Bit 2 - Port enable. Ports can only be enabled by the host controller as a part of the reset and enable. Software cannot enable a port by writing a one to this field. Ports can be disabled by either a fault condition (disconnect event or other fault condition) or by the host software. Note that the bit status does not change until the port state actually changes. There may be a delay in disabling or enabling a port due to other host controller and bus events. When the port is disabled. downstream propagation of data is blocked except for reset. This bit is 0 if PP (Port Power bit) is 0."]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bit 3 - Port disable/enable change For the root hub, this bit gets set to a one only when a port is disabled due to disconnect on the port or due to the appropriate conditions existing at the EOF2 point (See Chapter 11 of the USB Specification). Software clears this by writing a one to it. This bit is 0 if PP (Port Power bit) is 0,"]
    #[inline]
    pub fn pec(&mut self) -> _PECW {
        _PECW { w: self }
    }
    #[doc = "Bit 4 - Over-current active This bit will automatically transition from 1 to 0 when the over-current condition is removed."]
    #[inline]
    pub fn oca(&mut self) -> _OCAW {
        _OCAW { w: self }
    }
    #[doc = "Bit 5 - Over-current change This bit gets set to one when there is a change to Over-current Active. Software clears this bit by writing a one to this bit position."]
    #[inline]
    pub fn occ(&mut self) -> _OCCW {
        _OCCW { w: self }
    }
    #[doc = "Bit 6 - Force port resume Software sets this bit to one to drive resume signaling. The Host Controller sets this bit to one if a J-to-K transition is detected while the port is in the Suspend state. When this bit transitions to a one because a J-to-K transition is detected, the Port Change Detect bit in the USBSTS register is also set to one. This bit will automatically change to zero after the resume sequence is complete. This behavior is different from EHCI where the host controller driver is required to set this bit to a zero after the resume duration is timed in the driver. Note that when the Host controller owns the port, the resume sequence follows the defined sequence documented in the USB Specification Revision 2.0. The resume signaling (Full-speed K) is driven on the port as long as this bit remains a one. This bit will remain a one until the port has switched to the high-speed idle. Writing a zero has no affect because the port controller will time the resume operation clear the bit the port control state switches to HS or FS idle. This bit is 0 if PP (Port Power bit) is 0."]
    #[inline]
    pub fn fpr(&mut self) -> _FPRW {
        _FPRW { w: self }
    }
    #[doc = "Bit 7 - Suspend Together with the PE (Port enabled bit), this bit describes the port states, see Table 240. The host controller will unconditionally set this bit to zero when software sets the Force Port Resume bit to zero. The host controller ignores a write of zero to this bit. If host software sets this bit to a one when the port is not enabled (i.e. Port enabled bit is a zero) the results are undefined. This bit is 0 if PP (Port Power bit) is 0."]
    #[inline]
    pub fn susp(&mut self) -> _SUSPW {
        _SUSPW { w: self }
    }
    #[doc = "Bit 8 - Port reset When software writes a one to this bit the bus-reset sequence as defined in the USB Specification Revision 2.0 is started. This bit will automatically change to zero after the reset sequence is complete. This behavior is different from EHCI where the host controller driver is required to set this bit to a zero after the reset duration is timed in the driver. This bit is 0 if PP (Port Power bit) is 0."]
    #[inline]
    pub fn pr(&mut self) -> _PRW {
        _PRW { w: self }
    }
    #[doc = "Bit 9 - High-speed status"]
    #[inline]
    pub fn hsp(&mut self) -> _HSPW {
        _HSPW { w: self }
    }
    #[doc = "Bits 10:11 - Line status These bits reflect the current logical levels of the USB_DP and USB_DM signal lines. USB_DP corresponds to bit 11 and USB_DM to bit 10. In host mode, the use of linestate by the host controller driver is not necessary for this controller (unlike EHCI) because the controller hardware manages the connection of LS and FS."]
    #[inline]
    pub fn ls(&mut self) -> _LSW {
        _LSW { w: self }
    }
    #[doc = "Bit 12 - Port power control Host/OTG controller requires port power control switches. This bit represents the current setting of the switch (0=off, 1=on). When power is not available on a port (i.e. PP equals a 0), the port is non-functional and will not report attaches, detaches, etc. When an over-current condition is detected on a powered port and PPC is a one, the PP bit in each affected port may be transitioned by the host controller driver from a one to a zero (removing power from the port)."]
    #[inline]
    pub fn pp(&mut self) -> _PPW {
        _PPW { w: self }
    }
    #[doc = "Bits 14:15 - Port indicator control Writing to this field effects the value of the pins USB0_IND1 and USB0_IND0."]
    #[inline]
    pub fn pic1_0(&mut self) -> _PIC1_0W {
        _PIC1_0W { w: self }
    }
    #[doc = "Bits 16:19 - Port test control Any value other than 0000 indicates that the port is operating in test mode. The FORCE_ENABLE_FS and FORCE ENABLE_LS are extensions to the test mode support specified in the EHCI specification. Writing the PTC field to any of the FORCE_ENABLE_{HS/FS/LS} values will force the port into the connected and enabled state at the selected speed. Writing the PTC field back to TEST_MODE_DISABLE will allow the port state machines to progress normally from that point. Values 0x8 to 0xF are reserved."]
    #[inline]
    pub fn ptc3_0(&mut self) -> _PTC3_0W {
        _PTC3_0W { w: self }
    }
    #[doc = "Bit 20 - Wake on connect enable (WKCNNT_E) This bit is 0 if PP (Port Power bit) is 0"]
    #[inline]
    pub fn wkcn(&mut self) -> _WKCNW {
        _WKCNW { w: self }
    }
    #[doc = "Bit 21 - Wake on disconnect enable (WKDSCNNT_E) This bit is 0 if PP (Port Power bit) is 0."]
    #[inline]
    pub fn wkdc(&mut self) -> _WKDCW {
        _WKDCW { w: self }
    }
    #[doc = "Bit 22 - Wake on over-current enable (WKOC_E)"]
    #[inline]
    pub fn wkoc(&mut self) -> _WKOCW {
        _WKOCW { w: self }
    }
    #[doc = "Bit 23 - PHY low power suspend - clock disable (PLPSCD) In host mode, the PHY can be put into Low Power Suspend - Clock Disable when the downstream device has been put into suspend mode or when no downstream device is connected. Low power suspend is completely under the control of software."]
    #[inline]
    pub fn phcd(&mut self) -> _PHCDW {
        _PHCDW { w: self }
    }
    #[doc = "Bit 24 - Port force full speed connect"]
    #[inline]
    pub fn pfsc(&mut self) -> _PFSCW {
        _PFSCW { w: self }
    }
    #[doc = "Bits 26:27 - Port speed This register field indicates the speed at which the port is operating. For HS mode operation in the host controller and HS/FS operation in the device controller the port routing steers data to the Protocol engine. For FS and LS mode operation in the host controller, the port routing steers data to the Protocol Engine w/ Embedded Transaction Translator."]
    #[inline]
    pub fn pspd(&mut self) -> _PSPDW {
        _PSPDW { w: self }
    }
}
