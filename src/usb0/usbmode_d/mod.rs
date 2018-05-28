#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBMODE_D {
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
#[doc = "Possible values of the field `CM1_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM1_0R {
    #[doc = "Idle"]
    IDLE,
    #[doc = "Device controller"]
    DEVICE_CONTROLLER,
    #[doc = "Host controller"]
    HOST_CONTROLLER,
}
impl CM1_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CM1_0R::IDLE => 0,
            CM1_0R::DEVICE_CONTROLLER => 2,
            CM1_0R::HOST_CONTROLLER => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CM1_0R {
        match value {
            0 => CM1_0R::IDLE,
            2 => CM1_0R::DEVICE_CONTROLLER,
            3 => CM1_0R::HOST_CONTROLLER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == CM1_0R::IDLE
    }
    #[doc = "Checks if the value of the field is `DEVICE_CONTROLLER`"]
    #[inline]
    pub fn is_device_controller(&self) -> bool {
        *self == CM1_0R::DEVICE_CONTROLLER
    }
    #[doc = "Checks if the value of the field is `HOST_CONTROLLER`"]
    #[inline]
    pub fn is_host_controller(&self) -> bool {
        *self == CM1_0R::HOST_CONTROLLER
    }
}
#[doc = "Possible values of the field `ES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESR {
    #[doc = "Little endian: first byte referenced in least significant byte of 32-bit word."]
    LITTLE_ENDIAN_FIRST,
    #[doc = "Big endian: first byte referenced in most significant byte of 32-bit word."]
    BIG_ENDIAN_FIRST_BY,
}
impl ESR {
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
            ESR::LITTLE_ENDIAN_FIRST => false,
            ESR::BIG_ENDIAN_FIRST_BY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESR {
        match value {
            false => ESR::LITTLE_ENDIAN_FIRST,
            true => ESR::BIG_ENDIAN_FIRST_BY,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN_FIRST`"]
    #[inline]
    pub fn is_little_endian_first(&self) -> bool {
        *self == ESR::LITTLE_ENDIAN_FIRST
    }
    #[doc = "Checks if the value of the field is `BIG_ENDIAN_FIRST_BY`"]
    #[inline]
    pub fn is_big_endian_first_by(&self) -> bool {
        *self == ESR::BIG_ENDIAN_FIRST_BY
    }
}
#[doc = "Possible values of the field `SLOM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOMR {
    #[doc = "Setup Lockouts on"]
    SETUP_LOCKOUTS_ON,
    #[doc = "Setup Lockouts Off (DCD requires the use of Setup Buffer Tripwire in USBCMD)"]
    SETUP_LOCKOUTS_OFF,
}
impl SLOMR {
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
            SLOMR::SETUP_LOCKOUTS_ON => false,
            SLOMR::SETUP_LOCKOUTS_OFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLOMR {
        match value {
            false => SLOMR::SETUP_LOCKOUTS_ON,
            true => SLOMR::SETUP_LOCKOUTS_OFF,
        }
    }
    #[doc = "Checks if the value of the field is `SETUP_LOCKOUTS_ON`"]
    #[inline]
    pub fn is_setup_lockouts_on(&self) -> bool {
        *self == SLOMR::SETUP_LOCKOUTS_ON
    }
    #[doc = "Checks if the value of the field is `SETUP_LOCKOUTS_OFF`"]
    #[inline]
    pub fn is_setup_lockouts_off(&self) -> bool {
        *self == SLOMR::SETUP_LOCKOUTS_OFF
    }
}
#[doc = "Possible values of the field `SDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDISR {
    #[doc = "Not disabled"]
    NOT_DISABLED,
    #[doc = "Disabled. Setting this bit to one disables double priming on both RX and TX for low bandwidth systems. This mode ensures that when the RX and TX buffers are sufficient to contain an entire packet that the standard double buffering scheme is disabled to prevent overruns/underruns in bandwidth limited systems. Note: In High Speed Mode, all packets received will be responded to with a NYET handshake when stream disable is active."]
    DISABLED_SETTING_TH,
}
impl SDISR {
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
            SDISR::NOT_DISABLED => false,
            SDISR::DISABLED_SETTING_TH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDISR {
        match value {
            false => SDISR::NOT_DISABLED,
            true => SDISR::DISABLED_SETTING_TH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DISABLED`"]
    #[inline]
    pub fn is_not_disabled(&self) -> bool {
        *self == SDISR::NOT_DISABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED_SETTING_TH`"]
    #[inline]
    pub fn is_disabled_setting_th(&self) -> bool {
        *self == SDISR::DISABLED_SETTING_TH
    }
}
#[doc = "Values that can be written to the field `CM1_0`"]
pub enum CM1_0W {
    #[doc = "Idle"]
    IDLE,
    #[doc = "Device controller"]
    DEVICE_CONTROLLER,
    #[doc = "Host controller"]
    HOST_CONTROLLER,
}
impl CM1_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CM1_0W::IDLE => 0,
            CM1_0W::DEVICE_CONTROLLER => 2,
            CM1_0W::HOST_CONTROLLER => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CM1_0W<'a> {
    w: &'a mut W,
}
impl<'a> _CM1_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CM1_0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Idle"]
    #[inline]
    pub fn idle(self) -> &'a mut W {
        self.variant(CM1_0W::IDLE)
    }
    #[doc = "Device controller"]
    #[inline]
    pub fn device_controller(self) -> &'a mut W {
        self.variant(CM1_0W::DEVICE_CONTROLLER)
    }
    #[doc = "Host controller"]
    #[inline]
    pub fn host_controller(self) -> &'a mut W {
        self.variant(CM1_0W::HOST_CONTROLLER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ES`"]
pub enum ESW {
    #[doc = "Little endian: first byte referenced in least significant byte of 32-bit word."]
    LITTLE_ENDIAN_FIRST,
    #[doc = "Big endian: first byte referenced in most significant byte of 32-bit word."]
    BIG_ENDIAN_FIRST_BY,
}
impl ESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESW::LITTLE_ENDIAN_FIRST => false,
            ESW::BIG_ENDIAN_FIRST_BY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESW<'a> {
    w: &'a mut W,
}
impl<'a> _ESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Little endian: first byte referenced in least significant byte of 32-bit word."]
    #[inline]
    pub fn little_endian_first(self) -> &'a mut W {
        self.variant(ESW::LITTLE_ENDIAN_FIRST)
    }
    #[doc = "Big endian: first byte referenced in most significant byte of 32-bit word."]
    #[inline]
    pub fn big_endian_first_by(self) -> &'a mut W {
        self.variant(ESW::BIG_ENDIAN_FIRST_BY)
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
#[doc = "Values that can be written to the field `SLOM`"]
pub enum SLOMW {
    #[doc = "Setup Lockouts on"]
    SETUP_LOCKOUTS_ON,
    #[doc = "Setup Lockouts Off (DCD requires the use of Setup Buffer Tripwire in USBCMD)"]
    SETUP_LOCKOUTS_OFF,
}
impl SLOMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLOMW::SETUP_LOCKOUTS_ON => false,
            SLOMW::SETUP_LOCKOUTS_OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLOMW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLOMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Setup Lockouts on"]
    #[inline]
    pub fn setup_lockouts_on(self) -> &'a mut W {
        self.variant(SLOMW::SETUP_LOCKOUTS_ON)
    }
    #[doc = "Setup Lockouts Off (DCD requires the use of Setup Buffer Tripwire in USBCMD)"]
    #[inline]
    pub fn setup_lockouts_off(self) -> &'a mut W {
        self.variant(SLOMW::SETUP_LOCKOUTS_OFF)
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
#[doc = "Values that can be written to the field `SDIS`"]
pub enum SDISW {
    #[doc = "Not disabled"]
    NOT_DISABLED,
    #[doc = "Disabled. Setting this bit to one disables double priming on both RX and TX for low bandwidth systems. This mode ensures that when the RX and TX buffers are sufficient to contain an entire packet that the standard double buffering scheme is disabled to prevent overruns/underruns in bandwidth limited systems. Note: In High Speed Mode, all packets received will be responded to with a NYET handshake when stream disable is active."]
    DISABLED_SETTING_TH,
}
impl SDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDISW::NOT_DISABLED => false,
            SDISW::DISABLED_SETTING_TH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not disabled"]
    #[inline]
    pub fn not_disabled(self) -> &'a mut W {
        self.variant(SDISW::NOT_DISABLED)
    }
    #[doc = "Disabled. Setting this bit to one disables double priming on both RX and TX for low bandwidth systems. This mode ensures that when the RX and TX buffers are sufficient to contain an entire packet that the standard double buffering scheme is disabled to prevent overruns/underruns in bandwidth limited systems. Note: In High Speed Mode, all packets received will be responded to with a NYET handshake when stream disable is active."]
    #[inline]
    pub fn disabled_setting_th(self) -> &'a mut W {
        self.variant(SDISW::DISABLED_SETTING_TH)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Controller mode The controller defaults to an idle state and needs to be initialized to the desired operating mode after reset. This register can only be written once after reset. If it is necessary to switch modes, software must reset the controller by writing to the RESET bit in the USBCMD register before reprogramming this register."]
    #[inline]
    pub fn cm1_0(&self) -> CM1_0R {
        CM1_0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Endian select This bit can change the byte ordering of the transfer buffers to match the host microprocessor bus architecture. The bit fields in the microprocessor interface and the DMA data structures (including the setup buffer within the device QH) are unaffected by the value of this bit, because they are based upon 32-bit words."]
    #[inline]
    pub fn es(&self) -> ESR {
        ESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Setup Lockout mode In device mode, this bit controls behavior of the setup lock mechanism. See Section 18.10.8."]
    #[inline]
    pub fn slom(&self) -> SLOMR {
        SLOMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Stream disable mode The use of this feature substantially limits the overall USB performance that can be achieved."]
    #[inline]
    pub fn sdis(&self) -> SDISR {
        SDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - Controller mode The controller defaults to an idle state and needs to be initialized to the desired operating mode after reset. This register can only be written once after reset. If it is necessary to switch modes, software must reset the controller by writing to the RESET bit in the USBCMD register before reprogramming this register."]
    #[inline]
    pub fn cm1_0(&mut self) -> _CM1_0W {
        _CM1_0W { w: self }
    }
    #[doc = "Bit 2 - Endian select This bit can change the byte ordering of the transfer buffers to match the host microprocessor bus architecture. The bit fields in the microprocessor interface and the DMA data structures (including the setup buffer within the device QH) are unaffected by the value of this bit, because they are based upon 32-bit words."]
    #[inline]
    pub fn es(&mut self) -> _ESW {
        _ESW { w: self }
    }
    #[doc = "Bit 3 - Setup Lockout mode In device mode, this bit controls behavior of the setup lock mechanism. See Section 18.10.8."]
    #[inline]
    pub fn slom(&mut self) -> _SLOMW {
        _SLOMW { w: self }
    }
    #[doc = "Bit 4 - Stream disable mode The use of this feature substantially limits the overall USB performance that can be achieved."]
    #[inline]
    pub fn sdis(&mut self) -> _SDISW {
        _SDISW { w: self }
    }
}
