#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DYNAMICCONTROL {
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
#[doc = "Possible values of the field `CE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CER {
    #[doc = "Disabled. Clock enable of idle devices are deasserted to save power (POR reset value)."]
    DISABLED,
    #[doc = "Enabled. All clock enables are driven HIGH continuously.[1]"]
    ENABLED,
}
impl CER {
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
            CER::DISABLED => false,
            CER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CER {
        match value {
            false => CER::DISABLED,
            true => CER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CER::ENABLED
    }
}
#[doc = "Possible values of the field `CS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSR {
    #[doc = "Stop. CLKOUT stops when all SDRAMs are idle and during self-refresh mode."]
    STOP,
    #[doc = "Run. CLKOUT runs continuously (POR reset value)."]
    RUN,
}
impl CSR {
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
            CSR::STOP => false,
            CSR::RUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSR {
        match value {
            false => CSR::STOP,
            true => CSR::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == CSR::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == CSR::RUN
    }
}
#[doc = "Possible values of the field `SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRR {
    #[doc = "Normal mode."]
    NORMAL_MODE,
    #[doc = "Self-refresh. Enter self-refresh mode (POR reset value)."]
    SELF_REFRESH,
}
impl SRR {
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
            SRR::NORMAL_MODE => false,
            SRR::SELF_REFRESH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRR {
        match value {
            false => SRR::NORMAL_MODE,
            true => SRR::SELF_REFRESH,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE`"]
    #[inline]
    pub fn is_normal_mode(&self) -> bool {
        *self == SRR::NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `SELF_REFRESH`"]
    #[inline]
    pub fn is_self_refresh(&self) -> bool {
        *self == SRR::SELF_REFRESH
    }
}
#[doc = "Possible values of the field `MMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCR {
    #[doc = "Enabled. CLKOUT enabled (POR reset value)."]
    ENABLED,
    #[doc = "Disabled. CLKOUT disabled.[3]"]
    DISABLED,
}
impl MMCR {
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
            MMCR::ENABLED => false,
            MMCR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMCR {
        match value {
            false => MMCR::ENABLED,
            true => MMCR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MMCR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MMCR::DISABLED
    }
}
#[doc = "Possible values of the field `I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IR {
    #[doc = "Normal. Issue SDRAM NORMAL operation command (POR reset value)."]
    NORMAL,
    #[doc = "Mode. Issue SDRAM MODE command."]
    MODE,
    #[doc = "PALL. Issue SDRAM PALL (precharge all) command."]
    PALL,
    #[doc = "NOP. Issue SDRAM NOP (no operation) command)"]
    NOP,
}
impl IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IR::NORMAL => 0,
            IR::MODE => 1,
            IR::PALL => 2,
            IR::NOP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IR {
        match value {
            0 => IR::NORMAL,
            1 => IR::MODE,
            2 => IR::PALL,
            3 => IR::NOP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == IR::NORMAL
    }
    #[doc = "Checks if the value of the field is `MODE`"]
    #[inline]
    pub fn is_mode(&self) -> bool {
        *self == IR::MODE
    }
    #[doc = "Checks if the value of the field is `PALL`"]
    #[inline]
    pub fn is_pall(&self) -> bool {
        *self == IR::PALL
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline]
    pub fn is_nop(&self) -> bool {
        *self == IR::NOP
    }
}
#[doc = "Values that can be written to the field `CE`"]
pub enum CEW {
    #[doc = "Disabled. Clock enable of idle devices are deasserted to save power (POR reset value)."]
    DISABLED,
    #[doc = "Enabled. All clock enables are driven HIGH continuously.[1]"]
    ENABLED,
}
impl CEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEW::DISABLED => false,
            CEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEW<'a> {
    w: &'a mut W,
}
impl<'a> _CEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Clock enable of idle devices are deasserted to save power (POR reset value)."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEW::DISABLED)
    }
    #[doc = "Enabled. All clock enables are driven HIGH continuously.[1]"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEW::ENABLED)
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
#[doc = "Values that can be written to the field `CS`"]
pub enum CSW {
    #[doc = "Stop. CLKOUT stops when all SDRAMs are idle and during self-refresh mode."]
    STOP,
    #[doc = "Run. CLKOUT runs continuously (POR reset value)."]
    RUN,
}
impl CSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSW::STOP => false,
            CSW::RUN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSW<'a> {
    w: &'a mut W,
}
impl<'a> _CSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop. CLKOUT stops when all SDRAMs are idle and during self-refresh mode."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(CSW::STOP)
    }
    #[doc = "Run. CLKOUT runs continuously (POR reset value)."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(CSW::RUN)
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
#[doc = "Values that can be written to the field `SR`"]
pub enum SRW {
    #[doc = "Normal mode."]
    NORMAL_MODE,
    #[doc = "Self-refresh. Enter self-refresh mode (POR reset value)."]
    SELF_REFRESH,
}
impl SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRW::NORMAL_MODE => false,
            SRW::SELF_REFRESH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode."]
    #[inline]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(SRW::NORMAL_MODE)
    }
    #[doc = "Self-refresh. Enter self-refresh mode (POR reset value)."]
    #[inline]
    pub fn self_refresh(self) -> &'a mut W {
        self.variant(SRW::SELF_REFRESH)
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
#[doc = "Values that can be written to the field `MMC`"]
pub enum MMCW {
    #[doc = "Enabled. CLKOUT enabled (POR reset value)."]
    ENABLED,
    #[doc = "Disabled. CLKOUT disabled.[3]"]
    DISABLED,
}
impl MMCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMCW::ENABLED => false,
            MMCW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMCW<'a> {
    w: &'a mut W,
}
impl<'a> _MMCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled. CLKOUT enabled (POR reset value)."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MMCW::ENABLED)
    }
    #[doc = "Disabled. CLKOUT disabled.[3]"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MMCW::DISABLED)
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
#[doc = "Values that can be written to the field `I`"]
pub enum IW {
    #[doc = "Normal. Issue SDRAM NORMAL operation command (POR reset value)."]
    NORMAL,
    #[doc = "Mode. Issue SDRAM MODE command."]
    MODE,
    #[doc = "PALL. Issue SDRAM PALL (precharge all) command."]
    PALL,
    #[doc = "NOP. Issue SDRAM NOP (no operation) command)"]
    NOP,
}
impl IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IW::NORMAL => 0,
            IW::MODE => 1,
            IW::PALL => 2,
            IW::NOP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IW<'a> {
    w: &'a mut W,
}
impl<'a> _IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal. Issue SDRAM NORMAL operation command (POR reset value)."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(IW::NORMAL)
    }
    #[doc = "Mode. Issue SDRAM MODE command."]
    #[inline]
    pub fn mode(self) -> &'a mut W {
        self.variant(IW::MODE)
    }
    #[doc = "PALL. Issue SDRAM PALL (precharge all) command."]
    #[inline]
    pub fn pall(self) -> &'a mut W {
        self.variant(IW::PALL)
    }
    #[doc = "NOP. Issue SDRAM NOP (no operation) command)"]
    #[inline]
    pub fn nop(self) -> &'a mut W {
        self.variant(IW::NOP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Dynamic memory clock enable."]
    #[inline]
    pub fn ce(&self) -> CER {
        CER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
    #[inline]
    pub fn cs(&self) -> CSR {
        CSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Self-refresh request, EMC SREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.[2]"]
    #[inline]
    pub fn sr(&self) -> SRR {
        SRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Memory clock control."]
    #[inline]
    pub fn mmc(&self) -> MMCR {
        MMCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 7:8 - SDRAM initialization."]
    #[inline]
    pub fn i(&self) -> IR {
        IR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 6 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Dynamic memory clock enable."]
    #[inline]
    pub fn ce(&mut self) -> _CEW {
        _CEW { w: self }
    }
    #[doc = "Bit 1 - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
    #[inline]
    pub fn cs(&mut self) -> _CSW {
        _CSW { w: self }
    }
    #[doc = "Bit 2 - Self-refresh request, EMC SREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.[2]"]
    #[inline]
    pub fn sr(&mut self) -> _SRW {
        _SRW { w: self }
    }
    #[doc = "Bit 5 - Memory clock control."]
    #[inline]
    pub fn mmc(&mut self) -> _MMCW {
        _MMCW { w: self }
    }
    #[doc = "Bits 7:8 - SDRAM initialization."]
    #[inline]
    pub fn i(&mut self) -> _IW {
        _IW { w: self }
    }
}
