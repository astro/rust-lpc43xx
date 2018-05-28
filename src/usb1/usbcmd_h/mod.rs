#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBCMD_H {
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
#[doc = "Possible values of the field `RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSR {
    #[doc = "When this bit is set to 0, the Host Controller completes the current transaction on the USB and then halts. The HC Halted bit in the status register indicates when the Host Controller has finished the transaction and has entered the stopped state. Software should not write a one to this field unless the host controller is in the Halted state (i.e. HCHalted in the USBSTS register is a one)."]
    HALT,
    #[doc = "When set to a 1, the Host Controller proceeds with the execution of the schedule. The Host Controller continues execution as long as this bit is set to a one."]
    PROCEED,
}
impl RSR {
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
            RSR::HALT => false,
            RSR::PROCEED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSR {
        match value {
            false => RSR::HALT,
            true => RSR::PROCEED,
        }
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline]
    pub fn is_halt(&self) -> bool {
        *self == RSR::HALT
    }
    #[doc = "Checks if the value of the field is `PROCEED`"]
    #[inline]
    pub fn is_proceed(&self) -> bool {
        *self == RSR::PROCEED
    }
}
#[doc = "Possible values of the field `RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTR {
    #[doc = "This bit is set to zero by hardware when the reset process is complete."]
    RESETCOMPLETE,
    #[doc = "When software writes a one to this bit, the Host Controller resets its internal pipelines, timers, counters, state machines etc. to their initial value. Any transaction currently in progress on USB is immediately terminated. A USB reset is not driven on downstream ports. Software should not set this bit to a one when the HCHalted bit in the USBSTS register is a zero. Attempting to reset an actively running host controller will result in undefined behavior."]
    RESET,
}
impl RSTR {
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
            RSTR::RESETCOMPLETE => false,
            RSTR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTR {
        match value {
            false => RSTR::RESETCOMPLETE,
            true => RSTR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESETCOMPLETE`"]
    #[inline]
    pub fn is_resetcomplete(&self) -> bool {
        *self == RSTR::RESETCOMPLETE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == RSTR::RESET
    }
}
#[doc = r" Value of the field"]
pub struct FS0R {
    bits: bool,
}
impl FS0R {
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
pub struct FS1R {
    bits: bool,
}
impl FS1R {
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
#[doc = "Possible values of the field `PSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSER {
    #[doc = "Do not process the periodic schedule."]
    DO_NOT_PROCESS_THE_P,
    #[doc = "Use the PERIODICLISTBASE register to access the periodic schedule."]
    USE_THE_PERIODICLIST,
}
impl PSER {
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
            PSER::DO_NOT_PROCESS_THE_P => false,
            PSER::USE_THE_PERIODICLIST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSER {
        match value {
            false => PSER::DO_NOT_PROCESS_THE_P,
            true => PSER::USE_THE_PERIODICLIST,
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOT_PROCESS_THE_P`"]
    #[inline]
    pub fn is_do_not_process_the_p(&self) -> bool {
        *self == PSER::DO_NOT_PROCESS_THE_P
    }
    #[doc = "Checks if the value of the field is `USE_THE_PERIODICLIST`"]
    #[inline]
    pub fn is_use_the_periodiclist(&self) -> bool {
        *self == PSER::USE_THE_PERIODICLIST
    }
}
#[doc = "Possible values of the field `ASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASER {
    #[doc = "Do not process the asynchronous schedule."]
    DO_NOT_PROCESS_THE_A,
    #[doc = "Use the ASYNCLISTADDR to access the asynchronous schedule."]
    USE_THE_ASYNCLISTADD,
}
impl ASER {
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
            ASER::DO_NOT_PROCESS_THE_A => false,
            ASER::USE_THE_ASYNCLISTADD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASER {
        match value {
            false => ASER::DO_NOT_PROCESS_THE_A,
            true => ASER::USE_THE_ASYNCLISTADD,
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOT_PROCESS_THE_A`"]
    #[inline]
    pub fn is_do_not_process_the_a(&self) -> bool {
        *self == ASER::DO_NOT_PROCESS_THE_A
    }
    #[doc = "Checks if the value of the field is `USE_THE_ASYNCLISTADD`"]
    #[inline]
    pub fn is_use_the_asynclistadd(&self) -> bool {
        *self == ASER::USE_THE_ASYNCLISTADD
    }
}
#[doc = "Possible values of the field `IAA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IAAR {
    #[doc = "The host controller sets this bit to zero after it has set the Interrupt on Sync Advance status bit in the USBSTS register to one."]
    ST,
    #[doc = "Software must write a 1 to this bit to ring the doorbell. When the host controller has evicted all appropriate cached schedule states, it sets the Interrupt on Async Advance status bit in the USBSTS register. If the Interrupt on Sync Advance Enable bit in the USBINTR register is one, then the host controller will assert an interrupt at the next interrupt threshold. Software should not write a one to this bit when the asynchronous schedule is inactive. Doing so will yield undefined results."]
    DOORBELL,
}
impl IAAR {
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
            IAAR::ST => false,
            IAAR::DOORBELL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IAAR {
        match value {
            false => IAAR::ST,
            true => IAAR::DOORBELL,
        }
    }
    #[doc = "Checks if the value of the field is `ST`"]
    #[inline]
    pub fn is_st(&self) -> bool {
        *self == IAAR::ST
    }
    #[doc = "Checks if the value of the field is `DOORBELL`"]
    #[inline]
    pub fn is_doorbell(&self) -> bool {
        *self == IAAR::DOORBELL
    }
}
#[doc = r" Value of the field"]
pub struct ASP1_0R {
    bits: u8,
}
impl ASP1_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ASPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASPER {
    #[doc = "Park mode is disabled."]
    PARK_MODE_IS_DISABLE,
    #[doc = "Park mode is enabled."]
    PARK_MODE_IS_ENABLED,
}
impl ASPER {
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
            ASPER::PARK_MODE_IS_DISABLE => false,
            ASPER::PARK_MODE_IS_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASPER {
        match value {
            false => ASPER::PARK_MODE_IS_DISABLE,
            true => ASPER::PARK_MODE_IS_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `PARK_MODE_IS_DISABLE`"]
    #[inline]
    pub fn is_park_mode_is_disable(&self) -> bool {
        *self == ASPER::PARK_MODE_IS_DISABLE
    }
    #[doc = "Checks if the value of the field is `PARK_MODE_IS_ENABLED`"]
    #[inline]
    pub fn is_park_mode_is_enabled(&self) -> bool {
        *self == ASPER::PARK_MODE_IS_ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct FS2R {
    bits: bool,
}
impl FS2R {
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
pub struct ITCR {
    bits: u8,
}
impl ITCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `RS`"]
pub enum RSW {
    #[doc = "When this bit is set to 0, the Host Controller completes the current transaction on the USB and then halts. The HC Halted bit in the status register indicates when the Host Controller has finished the transaction and has entered the stopped state. Software should not write a one to this field unless the host controller is in the Halted state (i.e. HCHalted in the USBSTS register is a one)."]
    HALT,
    #[doc = "When set to a 1, the Host Controller proceeds with the execution of the schedule. The Host Controller continues execution as long as this bit is set to a one."]
    PROCEED,
}
impl RSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSW::HALT => false,
            RSW::PROCEED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSW<'a> {
    w: &'a mut W,
}
impl<'a> _RSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When this bit is set to 0, the Host Controller completes the current transaction on the USB and then halts. The HC Halted bit in the status register indicates when the Host Controller has finished the transaction and has entered the stopped state. Software should not write a one to this field unless the host controller is in the Halted state (i.e. HCHalted in the USBSTS register is a one)."]
    #[inline]
    pub fn halt(self) -> &'a mut W {
        self.variant(RSW::HALT)
    }
    #[doc = "When set to a 1, the Host Controller proceeds with the execution of the schedule. The Host Controller continues execution as long as this bit is set to a one."]
    #[inline]
    pub fn proceed(self) -> &'a mut W {
        self.variant(RSW::PROCEED)
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
#[doc = "Values that can be written to the field `RST`"]
pub enum RSTW {
    #[doc = "This bit is set to zero by hardware when the reset process is complete."]
    RESETCOMPLETE,
    #[doc = "When software writes a one to this bit, the Host Controller resets its internal pipelines, timers, counters, state machines etc. to their initial value. Any transaction currently in progress on USB is immediately terminated. A USB reset is not driven on downstream ports. Software should not set this bit to a one when the HCHalted bit in the USBSTS register is a zero. Attempting to reset an actively running host controller will result in undefined behavior."]
    RESET,
}
impl RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTW::RESETCOMPLETE => false,
            RSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is set to zero by hardware when the reset process is complete."]
    #[inline]
    pub fn resetcomplete(self) -> &'a mut W {
        self.variant(RSTW::RESETCOMPLETE)
    }
    #[doc = "When software writes a one to this bit, the Host Controller resets its internal pipelines, timers, counters, state machines etc. to their initial value. Any transaction currently in progress on USB is immediately terminated. A USB reset is not driven on downstream ports. Software should not set this bit to a one when the HCHalted bit in the USBSTS register is a zero. Attempting to reset an actively running host controller will result in undefined behavior."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(RSTW::RESET)
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
#[doc = r" Proxy"]
pub struct _FS0W<'a> {
    w: &'a mut W,
}
impl<'a> _FS0W<'a> {
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
pub struct _FS1W<'a> {
    w: &'a mut W,
}
impl<'a> _FS1W<'a> {
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
#[doc = "Values that can be written to the field `PSE`"]
pub enum PSEW {
    #[doc = "Do not process the periodic schedule."]
    DO_NOT_PROCESS_THE_P,
    #[doc = "Use the PERIODICLISTBASE register to access the periodic schedule."]
    USE_THE_PERIODICLIST,
}
impl PSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSEW::DO_NOT_PROCESS_THE_P => false,
            PSEW::USE_THE_PERIODICLIST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSEW<'a> {
    w: &'a mut W,
}
impl<'a> _PSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not process the periodic schedule."]
    #[inline]
    pub fn do_not_process_the_p(self) -> &'a mut W {
        self.variant(PSEW::DO_NOT_PROCESS_THE_P)
    }
    #[doc = "Use the PERIODICLISTBASE register to access the periodic schedule."]
    #[inline]
    pub fn use_the_periodiclist(self) -> &'a mut W {
        self.variant(PSEW::USE_THE_PERIODICLIST)
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
#[doc = "Values that can be written to the field `ASE`"]
pub enum ASEW {
    #[doc = "Do not process the asynchronous schedule."]
    DO_NOT_PROCESS_THE_A,
    #[doc = "Use the ASYNCLISTADDR to access the asynchronous schedule."]
    USE_THE_ASYNCLISTADD,
}
impl ASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASEW::DO_NOT_PROCESS_THE_A => false,
            ASEW::USE_THE_ASYNCLISTADD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASEW<'a> {
    w: &'a mut W,
}
impl<'a> _ASEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not process the asynchronous schedule."]
    #[inline]
    pub fn do_not_process_the_a(self) -> &'a mut W {
        self.variant(ASEW::DO_NOT_PROCESS_THE_A)
    }
    #[doc = "Use the ASYNCLISTADDR to access the asynchronous schedule."]
    #[inline]
    pub fn use_the_asynclistadd(self) -> &'a mut W {
        self.variant(ASEW::USE_THE_ASYNCLISTADD)
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
#[doc = "Values that can be written to the field `IAA`"]
pub enum IAAW {
    #[doc = "The host controller sets this bit to zero after it has set the Interrupt on Sync Advance status bit in the USBSTS register to one."]
    ST,
    #[doc = "Software must write a 1 to this bit to ring the doorbell. When the host controller has evicted all appropriate cached schedule states, it sets the Interrupt on Async Advance status bit in the USBSTS register. If the Interrupt on Sync Advance Enable bit in the USBINTR register is one, then the host controller will assert an interrupt at the next interrupt threshold. Software should not write a one to this bit when the asynchronous schedule is inactive. Doing so will yield undefined results."]
    DOORBELL,
}
impl IAAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IAAW::ST => false,
            IAAW::DOORBELL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IAAW<'a> {
    w: &'a mut W,
}
impl<'a> _IAAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IAAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The host controller sets this bit to zero after it has set the Interrupt on Sync Advance status bit in the USBSTS register to one."]
    #[inline]
    pub fn st(self) -> &'a mut W {
        self.variant(IAAW::ST)
    }
    #[doc = "Software must write a 1 to this bit to ring the doorbell. When the host controller has evicted all appropriate cached schedule states, it sets the Interrupt on Async Advance status bit in the USBSTS register. If the Interrupt on Sync Advance Enable bit in the USBINTR register is one, then the host controller will assert an interrupt at the next interrupt threshold. Software should not write a one to this bit when the asynchronous schedule is inactive. Doing so will yield undefined results."]
    #[inline]
    pub fn doorbell(self) -> &'a mut W {
        self.variant(IAAW::DOORBELL)
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
#[doc = r" Proxy"]
pub struct _ASP1_0W<'a> {
    w: &'a mut W,
}
impl<'a> _ASP1_0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASPE`"]
pub enum ASPEW {
    #[doc = "Park mode is disabled."]
    PARK_MODE_IS_DISABLE,
    #[doc = "Park mode is enabled."]
    PARK_MODE_IS_ENABLED,
}
impl ASPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASPEW::PARK_MODE_IS_DISABLE => false,
            ASPEW::PARK_MODE_IS_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASPEW<'a> {
    w: &'a mut W,
}
impl<'a> _ASPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Park mode is disabled."]
    #[inline]
    pub fn park_mode_is_disable(self) -> &'a mut W {
        self.variant(ASPEW::PARK_MODE_IS_DISABLE)
    }
    #[doc = "Park mode is enabled."]
    #[inline]
    pub fn park_mode_is_enabled(self) -> &'a mut W {
        self.variant(ASPEW::PARK_MODE_IS_ENABLED)
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
#[doc = r" Proxy"]
pub struct _FS2W<'a> {
    w: &'a mut W,
}
impl<'a> _FS2W<'a> {
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
#[doc = r" Proxy"]
pub struct _ITCW<'a> {
    w: &'a mut W,
}
impl<'a> _ITCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Run/Stop"]
    #[inline]
    pub fn rs(&self) -> RSR {
        RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Controller reset. Software uses this bit to reset the controller. This bit is set to zero by the Host/Device Controller when the reset process is complete. Software cannot terminate the reset process early by writing a zero to this register."]
    #[inline]
    pub fn rst(&self) -> RSTR {
        RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Bit 0 of the Frame List Size bits. See Table 281. This field specifies the size of the frame list that controls which bits in the Frame Index Register should be used for the Frame List Current index. Note that this field is made up from USBCMD bits 15, 3, and 2."]
    #[inline]
    pub fn fs0(&self) -> FS0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FS0R { bits }
    }
    #[doc = "Bit 3 - Bit 1 of the Frame List Size bits. See Table 281"]
    #[inline]
    pub fn fs1(&self) -> FS1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FS1R { bits }
    }
    #[doc = "Bit 4 - This bit controls whether the host controller skips processing the periodic schedule."]
    #[inline]
    pub fn pse(&self) -> PSER {
        PSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - This bit controls whether the host controller skips processing the asynchronous schedule."]
    #[inline]
    pub fn ase(&self) -> ASER {
        ASER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - This bit is used as a doorbell by software to tell the host controller to issue an interrupt the next time it advances asynchronous schedule."]
    #[inline]
    pub fn iaa(&self) -> IAAR {
        IAAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Asynchronous schedule park mode. Contains a count of the number of successive transactions the host controller is allowed to execute from a high-speed queue head on the Asynchronous schedule before continuing traversal of the Asynchronous schedule. Valid values are 0x1 to 0x3. Software must not write 00 to this bit when Park Mode Enable is one as this will result in undefined behavior."]
    #[inline]
    pub fn asp1_0(&self) -> ASP1_0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ASP1_0R { bits }
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park Mode Enable"]
    #[inline]
    pub fn aspe(&self) -> ASPER {
        ASPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Bit 2 of the Frame List Size bits. See Table 281."]
    #[inline]
    pub fn fs2(&self) -> FS2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FS2R { bits }
    }
    #[doc = "Bits 16:23 - Interrupt threshold control. The system software uses this field to set the maximum rate at which the host/device controller will issue interrupts. ITC contains the maximum interrupt interval measured in micro-frames. Valid values are shown below. All other values are reserved. 0x0 = Immediate (no threshold) 0x1 = 1 micro frame. 0x2 = 2 micro frames. 0x8 = 8 micro frames. 0x10 = 16 micro frames. 0x20 = 32 micro frames. 0x40 = 64 micro frames."]
    #[inline]
    pub fn itc(&self) -> ITCR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ITCR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 262320 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Run/Stop"]
    #[inline]
    pub fn rs(&mut self) -> _RSW {
        _RSW { w: self }
    }
    #[doc = "Bit 1 - Controller reset. Software uses this bit to reset the controller. This bit is set to zero by the Host/Device Controller when the reset process is complete. Software cannot terminate the reset process early by writing a zero to this register."]
    #[inline]
    pub fn rst(&mut self) -> _RSTW {
        _RSTW { w: self }
    }
    #[doc = "Bit 2 - Bit 0 of the Frame List Size bits. See Table 281. This field specifies the size of the frame list that controls which bits in the Frame Index Register should be used for the Frame List Current index. Note that this field is made up from USBCMD bits 15, 3, and 2."]
    #[inline]
    pub fn fs0(&mut self) -> _FS0W {
        _FS0W { w: self }
    }
    #[doc = "Bit 3 - Bit 1 of the Frame List Size bits. See Table 281"]
    #[inline]
    pub fn fs1(&mut self) -> _FS1W {
        _FS1W { w: self }
    }
    #[doc = "Bit 4 - This bit controls whether the host controller skips processing the periodic schedule."]
    #[inline]
    pub fn pse(&mut self) -> _PSEW {
        _PSEW { w: self }
    }
    #[doc = "Bit 5 - This bit controls whether the host controller skips processing the asynchronous schedule."]
    #[inline]
    pub fn ase(&mut self) -> _ASEW {
        _ASEW { w: self }
    }
    #[doc = "Bit 6 - This bit is used as a doorbell by software to tell the host controller to issue an interrupt the next time it advances asynchronous schedule."]
    #[inline]
    pub fn iaa(&mut self) -> _IAAW {
        _IAAW { w: self }
    }
    #[doc = "Bits 8:9 - Asynchronous schedule park mode. Contains a count of the number of successive transactions the host controller is allowed to execute from a high-speed queue head on the Asynchronous schedule before continuing traversal of the Asynchronous schedule. Valid values are 0x1 to 0x3. Software must not write 00 to this bit when Park Mode Enable is one as this will result in undefined behavior."]
    #[inline]
    pub fn asp1_0(&mut self) -> _ASP1_0W {
        _ASP1_0W { w: self }
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park Mode Enable"]
    #[inline]
    pub fn aspe(&mut self) -> _ASPEW {
        _ASPEW { w: self }
    }
    #[doc = "Bit 15 - Bit 2 of the Frame List Size bits. See Table 281."]
    #[inline]
    pub fn fs2(&mut self) -> _FS2W {
        _FS2W { w: self }
    }
    #[doc = "Bits 16:23 - Interrupt threshold control. The system software uses this field to set the maximum rate at which the host/device controller will issue interrupts. ITC contains the maximum interrupt interval measured in micro-frames. Valid values are shown below. All other values are reserved. 0x0 = Immediate (no threshold) 0x1 = 1 micro frame. 0x2 = 2 micro frames. 0x8 = 8 micro frames. 0x10 = 16 micro frames. 0x20 = 32 micro frames. 0x40 = 64 micro frames."]
    #[inline]
    pub fn itc(&mut self) -> _ITCW {
        _ITCW { w: self }
    }
}
