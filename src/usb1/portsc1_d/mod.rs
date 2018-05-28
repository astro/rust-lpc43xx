#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PORTSC1_D {
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
    #[doc = "Device not attached A zero indicates that the device did not attach successfully or was forcibly disconnected by the software writing a zero to the Run bit in the USBCMD register. It does not state the device being disconnected or suspended."]
    DEVICE_NOT_ATTACHED,
    #[doc = "Device attached.  A one indicates that the device successfully attached and is operating in either high-speed mode or full-speed mode as indicated by the High Speed Port bit in this register."]
    DEVICE_ATTACHED_A,
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
            CCSR::DEVICE_NOT_ATTACHED => false,
            CCSR::DEVICE_ATTACHED_A => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCSR {
        match value {
            false => CCSR::DEVICE_NOT_ATTACHED,
            true => CCSR::DEVICE_ATTACHED_A,
        }
    }
    #[doc = "Checks if the value of the field is `DEVICE_NOT_ATTACHED`"]
    #[inline]
    pub fn is_device_not_attached(&self) -> bool {
        *self == CCSR::DEVICE_NOT_ATTACHED
    }
    #[doc = "Checks if the value of the field is `DEVICE_ATTACHED_A`"]
    #[inline]
    pub fn is_device_attached_a(&self) -> bool {
        *self == CCSR::DEVICE_ATTACHED_A
    }
}
#[doc = r" Value of the field"]
pub struct CSCR {
    bits: bool,
}
impl CSCR {
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
pub struct PER {
    bits: bool,
}
impl PER {
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
pub struct PECR {
    bits: bool,
}
impl PECR {
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
    PORT_NOT_IN_SUSPEND,
    #[doc = "Port in suspend state"]
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
            SUSPR::PORT_NOT_IN_SUSPEND => false,
            SUSPR::PORT_IN_SUSPEND_STAT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUSPR {
        match value {
            false => SUSPR::PORT_NOT_IN_SUSPEND,
            true => SUSPR::PORT_IN_SUSPEND_STAT,
        }
    }
    #[doc = "Checks if the value of the field is `PORT_NOT_IN_SUSPEND`"]
    #[inline]
    pub fn is_port_not_in_suspend(&self) -> bool {
        *self == SUSPR::PORT_NOT_IN_SUSPEND
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
    NOHISPEED,
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
            HSPR::NOHISPEED => false,
            HSPR::HISPEED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSPR {
        match value {
            false => HSPR::NOHISPEED,
            true => HSPR::HISPEED,
        }
    }
    #[doc = "Checks if the value of the field is `NOHISPEED`"]
    #[inline]
    pub fn is_nohispeed(&self) -> bool {
        *self == HSPR::NOHISPEED
    }
    #[doc = "Checks if the value of the field is `HISPEED`"]
    #[inline]
    pub fn is_hispeed(&self) -> bool {
        *self == HSPR::HISPEED
    }
}
#[doc = r" Value of the field"]
pub struct LSR {
    bits: u8,
}
impl LSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PPR {
    bits: bool,
}
impl PPR {
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
#[doc = "Possible values of the field `PIC1_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIC1_0R {
    #[doc = "Port indicators are off."]
    OFF,
    #[doc = "amber"]
    AMBER,
    #[doc = "green"]
    GREEN,
    #[doc = "undefined"]
    UNDEFINED,
}
impl PIC1_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIC1_0R::OFF => 0,
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
            0 => PIC1_0R::OFF,
            1 => PIC1_0R::AMBER,
            2 => PIC1_0R::GREEN,
            3 => PIC1_0R::UNDEFINED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == PIC1_0R::OFF
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
    SE0,
    #[doc = "Packet"]
    PACKET,
    #[doc = "FORCE_ENABLE_HS"]
    FORCE_ENABLE_HS,
    #[doc = "FORCE_ENABLE_FS"]
    FORCE_ENABLE_FS,
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
            PTC3_0R::SE0 => 3,
            PTC3_0R::PACKET => 4,
            PTC3_0R::FORCE_ENABLE_HS => 5,
            PTC3_0R::FORCE_ENABLE_FS => 6,
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
            3 => PTC3_0R::SE0,
            4 => PTC3_0R::PACKET,
            5 => PTC3_0R::FORCE_ENABLE_HS,
            6 => PTC3_0R::FORCE_ENABLE_FS,
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
    #[doc = "Checks if the value of the field is `SE0`"]
    #[inline]
    pub fn is_se0(&self) -> bool {
        *self == PTC3_0R::SE0
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
}
#[doc = "Possible values of the field `PHCD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHCDR {
    #[doc = "Writing a 0 enables the PHY clock. Reading a 0 indicates the status of the PHY clock (enabled)."]
    ENABLED,
    #[doc = "Writing a 1 disables the PHY clock. Reading a 1 indicates the status of the PHY clock (disabled)."]
    DISABLED,
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
            PHCDR::ENABLED => false,
            PHCDR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PHCDR {
        match value {
            false => PHCDR::ENABLED,
            true => PHCDR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PHCDR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PHCDR::DISABLED
    }
}
#[doc = "Possible values of the field `PFSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFSCR {
    #[doc = "Port connects at any speed."]
    ANYSPEED,
    #[doc = "Writing this bit to a 1 will force the port to only connect at full speed. It disables the chirp sequence that allows the port to identify itself as High-speed. This is useful for testing FS configurations with a HS host, hub or device."]
    FULLSPEED,
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
            PFSCR::ANYSPEED => false,
            PFSCR::FULLSPEED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFSCR {
        match value {
            false => PFSCR::ANYSPEED,
            true => PFSCR::FULLSPEED,
        }
    }
    #[doc = "Checks if the value of the field is `ANYSPEED`"]
    #[inline]
    pub fn is_anyspeed(&self) -> bool {
        *self == PFSCR::ANYSPEED
    }
    #[doc = "Checks if the value of the field is `FULLSPEED`"]
    #[inline]
    pub fn is_fullspeed(&self) -> bool {
        *self == PFSCR::FULLSPEED
    }
}
#[doc = "Possible values of the field `PSPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSPDR {
    #[doc = "Full-speed"]
    FULL_SPEED,
    #[doc = "invalid in device mode"]
    INVALID_IN_DEVICE_MO,
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
            PSPDR::FULL_SPEED => 1,
            PSPDR::INVALID_IN_DEVICE_MO => 2,
            PSPDR::HIGH_SPEED => 3,
            PSPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSPDR {
        match value {
            1 => PSPDR::FULL_SPEED,
            2 => PSPDR::INVALID_IN_DEVICE_MO,
            3 => PSPDR::HIGH_SPEED,
            i => PSPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FULL_SPEED`"]
    #[inline]
    pub fn is_full_speed(&self) -> bool {
        *self == PSPDR::FULL_SPEED
    }
    #[doc = "Checks if the value of the field is `INVALID_IN_DEVICE_MO`"]
    #[inline]
    pub fn is_invalid_in_device_mo(&self) -> bool {
        *self == PSPDR::INVALID_IN_DEVICE_MO
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline]
    pub fn is_high_speed(&self) -> bool {
        *self == PSPDR::HIGH_SPEED
    }
}
#[doc = "Possible values of the field `PTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSR {
    #[doc = "ULPI"]
    ULPI,
    #[doc = "Serial/ 1.1 PHY (Full-speed only)"]
    SERIAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PTSR::ULPI => 2,
            PTSR::SERIAL => 3,
            PTSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PTSR {
        match value {
            2 => PTSR::ULPI,
            3 => PTSR::SERIAL,
            i => PTSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ULPI`"]
    #[inline]
    pub fn is_ulpi(&self) -> bool {
        *self == PTSR::ULPI
    }
    #[doc = "Checks if the value of the field is `SERIAL`"]
    #[inline]
    pub fn is_serial(&self) -> bool {
        *self == PTSR::SERIAL
    }
}
#[doc = "Values that can be written to the field `CCS`"]
pub enum CCSW {
    #[doc = "Device not attached A zero indicates that the device did not attach successfully or was forcibly disconnected by the software writing a zero to the Run bit in the USBCMD register. It does not state the device being disconnected or suspended."]
    DEVICE_NOT_ATTACHED,
    #[doc = "Device attached.  A one indicates that the device successfully attached and is operating in either high-speed mode or full-speed mode as indicated by the High Speed Port bit in this register."]
    DEVICE_ATTACHED_A,
}
impl CCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCSW::DEVICE_NOT_ATTACHED => false,
            CCSW::DEVICE_ATTACHED_A => true,
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
    #[doc = "Device not attached A zero indicates that the device did not attach successfully or was forcibly disconnected by the software writing a zero to the Run bit in the USBCMD register. It does not state the device being disconnected or suspended."]
    #[inline]
    pub fn device_not_attached(self) -> &'a mut W {
        self.variant(CCSW::DEVICE_NOT_ATTACHED)
    }
    #[doc = "Device attached. A one indicates that the device successfully attached and is operating in either high-speed mode or full-speed mode as indicated by the High Speed Port bit in this register."]
    #[inline]
    pub fn device_attached_a(self) -> &'a mut W {
        self.variant(CCSW::DEVICE_ATTACHED_A)
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
#[doc = r" Proxy"]
pub struct _CSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSCW<'a> {
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
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
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
#[doc = r" Proxy"]
pub struct _PECW<'a> {
    w: &'a mut W,
}
impl<'a> _PECW<'a> {
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
    PORT_NOT_IN_SUSPEND,
    #[doc = "Port in suspend state"]
    PORT_IN_SUSPEND_STAT,
}
impl SUSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUSPW::PORT_NOT_IN_SUSPEND => false,
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
    pub fn port_not_in_suspend(self) -> &'a mut W {
        self.variant(SUSPW::PORT_NOT_IN_SUSPEND)
    }
    #[doc = "Port in suspend state"]
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
    NOHISPEED,
    #[doc = "Host/device connected to the port is in High-speed mode."]
    HISPEED,
}
impl HSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSPW::NOHISPEED => false,
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
    pub fn nohispeed(self) -> &'a mut W {
        self.variant(HSPW::NOHISPEED)
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
#[doc = r" Proxy"]
pub struct _LSW<'a> {
    w: &'a mut W,
}
impl<'a> _LSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PPW<'a> {
    w: &'a mut W,
}
impl<'a> _PPW<'a> {
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
    OFF,
    #[doc = "amber"]
    AMBER,
    #[doc = "green"]
    GREEN,
    #[doc = "undefined"]
    UNDEFINED,
}
impl PIC1_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PIC1_0W::OFF => 0,
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
    pub fn off(self) -> &'a mut W {
        self.variant(PIC1_0W::OFF)
    }
    #[doc = "amber"]
    #[inline]
    pub fn amber(self) -> &'a mut W {
        self.variant(PIC1_0W::AMBER)
    }
    #[doc = "green"]
    #[inline]
    pub fn green(self) -> &'a mut W {
        self.variant(PIC1_0W::GREEN)
    }
    #[doc = "undefined"]
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
    SE0,
    #[doc = "Packet"]
    PACKET,
    #[doc = "FORCE_ENABLE_HS"]
    FORCE_ENABLE_HS,
    #[doc = "FORCE_ENABLE_FS"]
    FORCE_ENABLE_FS,
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
            PTC3_0W::SE0 => 3,
            PTC3_0W::PACKET => 4,
            PTC3_0W::FORCE_ENABLE_HS => 5,
            PTC3_0W::FORCE_ENABLE_FS => 6,
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
    pub fn se0(self) -> &'a mut W {
        self.variant(PTC3_0W::SE0)
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
#[doc = "Values that can be written to the field `PHCD`"]
pub enum PHCDW {
    #[doc = "Writing a 0 enables the PHY clock. Reading a 0 indicates the status of the PHY clock (enabled)."]
    ENABLED,
    #[doc = "Writing a 1 disables the PHY clock. Reading a 1 indicates the status of the PHY clock (disabled)."]
    DISABLED,
}
impl PHCDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PHCDW::ENABLED => false,
            PHCDW::DISABLED => true,
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
    pub fn enabled(self) -> &'a mut W {
        self.variant(PHCDW::ENABLED)
    }
    #[doc = "Writing a 1 disables the PHY clock. Reading a 1 indicates the status of the PHY clock (disabled)."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PHCDW::DISABLED)
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
    ANYSPEED,
    #[doc = "Writing this bit to a 1 will force the port to only connect at full speed. It disables the chirp sequence that allows the port to identify itself as High-speed. This is useful for testing FS configurations with a HS host, hub or device."]
    FULLSPEED,
}
impl PFSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PFSCW::ANYSPEED => false,
            PFSCW::FULLSPEED => true,
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
    pub fn anyspeed(self) -> &'a mut W {
        self.variant(PFSCW::ANYSPEED)
    }
    #[doc = "Writing this bit to a 1 will force the port to only connect at full speed. It disables the chirp sequence that allows the port to identify itself as High-speed. This is useful for testing FS configurations with a HS host, hub or device."]
    #[inline]
    pub fn fullspeed(self) -> &'a mut W {
        self.variant(PFSCW::FULLSPEED)
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
    #[doc = "invalid in device mode"]
    INVALID_IN_DEVICE_MO,
    #[doc = "High-speed"]
    HIGH_SPEED,
}
impl PSPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSPDW::FULL_SPEED => 1,
            PSPDW::INVALID_IN_DEVICE_MO => 2,
            PSPDW::HIGH_SPEED => 3,
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
    #[doc = "invalid in device mode"]
    #[inline]
    pub fn invalid_in_device_mo(self) -> &'a mut W {
        self.variant(PSPDW::INVALID_IN_DEVICE_MO)
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
#[doc = "Values that can be written to the field `PTS`"]
pub enum PTSW {
    #[doc = "ULPI"]
    ULPI,
    #[doc = "Serial/ 1.1 PHY (Full-speed only)"]
    SERIAL,
}
impl PTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PTSW::ULPI => 2,
            PTSW::SERIAL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSW<'a> {
    w: &'a mut W,
}
impl<'a> _PTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ULPI"]
    #[inline]
    pub fn ulpi(self) -> &'a mut W {
        self.variant(PTSW::ULPI)
    }
    #[doc = "Serial/ 1.1 PHY (Full-speed only)"]
    #[inline]
    pub fn serial(self) -> &'a mut W {
        self.variant(PTSW::SERIAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - Current connect status"]
    #[inline]
    pub fn ccs(&self) -> CCSR {
        CCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Not used in device mode"]
    #[inline]
    pub fn csc(&self) -> CSCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSCR { bits }
    }
    #[doc = "Bit 2 - Port enable. This bit is always 1. The device port is always enabled."]
    #[inline]
    pub fn pe(&self) -> PER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PER { bits }
    }
    #[doc = "Bit 3 - Port enable/disable change This bit is always 0. The device port is always enabled."]
    #[inline]
    pub fn pec(&self) -> PECR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PECR { bits }
    }
    #[doc = "Bit 6 - Force port resume After the device has been in Suspend State for 5 ms or more, software must set this bit to one to drive resume signaling before clearing. The Device Controller will set this bit to one if a J-to-K transition is detected while the port is in the Suspend state. The bit will be cleared when the device returns to normal operation. When this bit transitions to a one because a J-to-K transition detected, the Port Change Detect bit in the USBSTS register is set to one as well."]
    #[inline]
    pub fn fpr(&self) -> FPRR {
        FPRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Suspend In device mode, this is a read-only status bit ."]
    #[inline]
    pub fn susp(&self) -> SUSPR {
        SUSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port reset In device mode, this is a read-only status bit. A device reset from the USB bus is also indicated in the USBSTS register."]
    #[inline]
    pub fn pr(&self) -> PRR {
        PRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - High-speed status This bit is redundant with bits 27:26 (PSPD) in this register. It is implemented for compatibility reasons."]
    #[inline]
    pub fn hsp(&self) -> HSPR {
        HSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:11 - Not used in device mode."]
    #[inline]
    pub fn ls(&self) -> LSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LSR { bits }
    }
    #[doc = "Bit 12 - Not used in device mode."]
    #[inline]
    pub fn pp(&self) -> PPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PPR { bits }
    }
    #[doc = "Bits 14:15 - Port indicator control Writing to this field effects the value of the USB1_IND1:0 pins."]
    #[inline]
    pub fn pic1_0(&self) -> PIC1_0R {
        PIC1_0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Port test control Any value other than 0000 indicates that the port is operating in test mode. The FORCE_ENABLE_FS and FORCE ENABLE_LS are extensions to the test mode support specified in the EHCI specification. Writing the PTC field to any of the FORCE_ENABLE_HS/FS/LS values will force the port into the connected and enabled state at the selected speed. Writing the PTC field back to TEST_MODE_DISABLE will allow the port state machines to progress normally from that point. Values 0x7 to 0xF are reserved."]
    #[inline]
    pub fn ptc3_0(&self) -> PTC3_0R {
        PTC3_0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - PHY low power suspend - clock disable (PLPSCD) In device mode, The PHY can be put into Low Power Suspend - Clock Disable when the device is not running (USBCMD Run/Stop = 0) or the host has signaled suspend (PORTSC SUSPEND = 1). Low power suspend will be cleared automatically when the host has signaled resume. Before forcing a resume from the device, the device controller driver must clear this bit."]
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
    #[doc = "Bits 26:27 - Port speed This register field indicates the speed at which the port is operating."]
    #[inline]
    pub fn pspd(&self) -> PSPDR {
        PSPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Parallel transceiver select. All other values are reserved."]
    #[inline]
    pub fn pts(&self) -> PTSR {
        PTSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - Current connect status"]
    #[inline]
    pub fn ccs(&mut self) -> _CCSW {
        _CCSW { w: self }
    }
    #[doc = "Bit 1 - Not used in device mode"]
    #[inline]
    pub fn csc(&mut self) -> _CSCW {
        _CSCW { w: self }
    }
    #[doc = "Bit 2 - Port enable. This bit is always 1. The device port is always enabled."]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bit 3 - Port enable/disable change This bit is always 0. The device port is always enabled."]
    #[inline]
    pub fn pec(&mut self) -> _PECW {
        _PECW { w: self }
    }
    #[doc = "Bit 6 - Force port resume After the device has been in Suspend State for 5 ms or more, software must set this bit to one to drive resume signaling before clearing. The Device Controller will set this bit to one if a J-to-K transition is detected while the port is in the Suspend state. The bit will be cleared when the device returns to normal operation. When this bit transitions to a one because a J-to-K transition detected, the Port Change Detect bit in the USBSTS register is set to one as well."]
    #[inline]
    pub fn fpr(&mut self) -> _FPRW {
        _FPRW { w: self }
    }
    #[doc = "Bit 7 - Suspend In device mode, this is a read-only status bit ."]
    #[inline]
    pub fn susp(&mut self) -> _SUSPW {
        _SUSPW { w: self }
    }
    #[doc = "Bit 8 - Port reset In device mode, this is a read-only status bit. A device reset from the USB bus is also indicated in the USBSTS register."]
    #[inline]
    pub fn pr(&mut self) -> _PRW {
        _PRW { w: self }
    }
    #[doc = "Bit 9 - High-speed status This bit is redundant with bits 27:26 (PSPD) in this register. It is implemented for compatibility reasons."]
    #[inline]
    pub fn hsp(&mut self) -> _HSPW {
        _HSPW { w: self }
    }
    #[doc = "Bits 10:11 - Not used in device mode."]
    #[inline]
    pub fn ls(&mut self) -> _LSW {
        _LSW { w: self }
    }
    #[doc = "Bit 12 - Not used in device mode."]
    #[inline]
    pub fn pp(&mut self) -> _PPW {
        _PPW { w: self }
    }
    #[doc = "Bits 14:15 - Port indicator control Writing to this field effects the value of the USB1_IND1:0 pins."]
    #[inline]
    pub fn pic1_0(&mut self) -> _PIC1_0W {
        _PIC1_0W { w: self }
    }
    #[doc = "Bits 16:19 - Port test control Any value other than 0000 indicates that the port is operating in test mode. The FORCE_ENABLE_FS and FORCE ENABLE_LS are extensions to the test mode support specified in the EHCI specification. Writing the PTC field to any of the FORCE_ENABLE_HS/FS/LS values will force the port into the connected and enabled state at the selected speed. Writing the PTC field back to TEST_MODE_DISABLE will allow the port state machines to progress normally from that point. Values 0x7 to 0xF are reserved."]
    #[inline]
    pub fn ptc3_0(&mut self) -> _PTC3_0W {
        _PTC3_0W { w: self }
    }
    #[doc = "Bit 23 - PHY low power suspend - clock disable (PLPSCD) In device mode, The PHY can be put into Low Power Suspend - Clock Disable when the device is not running (USBCMD Run/Stop = 0) or the host has signaled suspend (PORTSC SUSPEND = 1). Low power suspend will be cleared automatically when the host has signaled resume. Before forcing a resume from the device, the device controller driver must clear this bit."]
    #[inline]
    pub fn phcd(&mut self) -> _PHCDW {
        _PHCDW { w: self }
    }
    #[doc = "Bit 24 - Port force full speed connect"]
    #[inline]
    pub fn pfsc(&mut self) -> _PFSCW {
        _PFSCW { w: self }
    }
    #[doc = "Bits 26:27 - Port speed This register field indicates the speed at which the port is operating."]
    #[inline]
    pub fn pspd(&mut self) -> _PSPDW {
        _PSPDW { w: self }
    }
    #[doc = "Bits 30:31 - Parallel transceiver select. All other values are reserved."]
    #[inline]
    pub fn pts(&mut self) -> _PTSW {
        _PTSW { w: self }
    }
}
