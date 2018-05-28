#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_MII_ADDR {
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
#[doc = r" Value of the field"]
pub struct GBR {
    bits: bool,
}
impl GBR {
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
pub struct WR {
    bits: bool,
}
impl WR {
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
pub struct CRR {
    bits: u8,
}
impl CRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GRR {
    bits: u8,
}
impl GRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PAR {
    bits: u8,
}
impl PAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _GBW<'a> {
    w: &'a mut W,
}
impl<'a> _GBW<'a> {
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
pub struct _WW<'a> {
    w: &'a mut W,
}
impl<'a> _WW<'a> {
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
pub struct _CRW<'a> {
    w: &'a mut W,
}
impl<'a> _CRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GRW<'a> {
    w: &'a mut W,
}
impl<'a> _GRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAW<'a> {
    w: &'a mut W,
}
impl<'a> _PAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - MII busy This register field can be read by the application (Read), can be set to 1 by the application with a register write of 1 (Write Set), and is cleared to 0 by the core (Self Clear). The application cannot clear this type of field, and a register write of 0 to this bit has no effect on this field. This bit should read a logic 0 before writing to this register and the MAC_MII_DATA register. This bit must also be set to 0 during a Write to this register. During a PHY register access, this bit will be set to 1 by the Application to indicate that a Read or Write access is in progress. The MAC_MII_DATA register should be kept valid until this bit is cleared by the MAC during a PHY Write operation. The MAC_MII_DATA register is invalid until this bit is cleared by the MAC during a PHY Read operation. This register should not be written to until this bit is cleared."]
    #[inline]
    pub fn gb(&self) -> GBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GBR { bits }
    }
    #[doc = "Bit 1 - MII write When set, this bit tells the PHY that this will be a Write operation using the MII Data register. If this bit is not set, this will be a Read operation, placing the data in the MII Data register."]
    #[inline]
    pub fn w(&self) -> WR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WR { bits }
    }
    #[doc = "Bits 2:5 - CSR clock range The CSR Clock Range selection determines the frequency of the MDC clock. The suggested range of CLK_M4_ETHERNET frequency applicable for each value below (when Bit[5] = 0) ensures that the MDC clock is approximately between the frequency range 1.0 MHz - 2.5 MHz. When bit 5 is set, you can achieve MDC clock of frequency higher than the IEEE 802.3 specified frequency limit of 2.5 MHz and program a clock divider of lower value. For example, when CLK_M4_ETHERNET is of frequency 100 MHz and you program these bits as 1010, then the resultant MDC clock will be of 12.5 MHz which is outside the limit of IEEE 802.3 specified range. Program the values given below only if the interfacing chips supports faster MDC clocks. See Table 554 for bit values."]
    #[inline]
    pub fn cr(&self) -> CRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CRR { bits }
    }
    #[doc = "Bits 6:10 - MII register These bits select the desired MII register in the selected PHY device."]
    #[inline]
    pub fn gr(&self) -> GRR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GRR { bits }
    }
    #[doc = "Bits 11:15 - Physical layer address This field tells which of the 32 possible PHY devices are being accessed."]
    #[inline]
    pub fn pa(&self) -> PAR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PAR { bits }
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
    #[doc = "Bit 0 - MII busy This register field can be read by the application (Read), can be set to 1 by the application with a register write of 1 (Write Set), and is cleared to 0 by the core (Self Clear). The application cannot clear this type of field, and a register write of 0 to this bit has no effect on this field. This bit should read a logic 0 before writing to this register and the MAC_MII_DATA register. This bit must also be set to 0 during a Write to this register. During a PHY register access, this bit will be set to 1 by the Application to indicate that a Read or Write access is in progress. The MAC_MII_DATA register should be kept valid until this bit is cleared by the MAC during a PHY Write operation. The MAC_MII_DATA register is invalid until this bit is cleared by the MAC during a PHY Read operation. This register should not be written to until this bit is cleared."]
    #[inline]
    pub fn gb(&mut self) -> _GBW {
        _GBW { w: self }
    }
    #[doc = "Bit 1 - MII write When set, this bit tells the PHY that this will be a Write operation using the MII Data register. If this bit is not set, this will be a Read operation, placing the data in the MII Data register."]
    #[inline]
    pub fn w(&mut self) -> _WW {
        _WW { w: self }
    }
    #[doc = "Bits 2:5 - CSR clock range The CSR Clock Range selection determines the frequency of the MDC clock. The suggested range of CLK_M4_ETHERNET frequency applicable for each value below (when Bit[5] = 0) ensures that the MDC clock is approximately between the frequency range 1.0 MHz - 2.5 MHz. When bit 5 is set, you can achieve MDC clock of frequency higher than the IEEE 802.3 specified frequency limit of 2.5 MHz and program a clock divider of lower value. For example, when CLK_M4_ETHERNET is of frequency 100 MHz and you program these bits as 1010, then the resultant MDC clock will be of 12.5 MHz which is outside the limit of IEEE 802.3 specified range. Program the values given below only if the interfacing chips supports faster MDC clocks. See Table 554 for bit values."]
    #[inline]
    pub fn cr(&mut self) -> _CRW {
        _CRW { w: self }
    }
    #[doc = "Bits 6:10 - MII register These bits select the desired MII register in the selected PHY device."]
    #[inline]
    pub fn gr(&mut self) -> _GRW {
        _GRW { w: self }
    }
    #[doc = "Bits 11:15 - Physical layer address This field tells which of the 32 possible PHY devices are being accessed."]
    #[inline]
    pub fn pa(&mut self) -> _PAW {
        _PAW { w: self }
    }
}
