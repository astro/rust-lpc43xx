#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOTH {
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
pub struct TX_WMARKR {
    bits: u16,
}
impl TX_WMARKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RX_WMARKR {
    bits: u16,
}
impl RX_WMARKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DMA_MTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_MTSR {
    #[doc = "1 transfer"]
    _1_TRANSFER,
    #[doc = "4 transfers"]
    _4_TRANSFERS,
    #[doc = "8 transfers"]
    _8_TRANSFERS,
    #[doc = "16 transfers"]
    _16_TRANSFERS,
    #[doc = "32 transfers"]
    _32_TRANSFERS,
    #[doc = "64 transfers"]
    _64_TRANSFERS,
    #[doc = "128 transfers"]
    _128_TRANSFERS,
    #[doc = "256 transfers"]
    _256_TRANSFERS,
}
impl DMA_MTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMA_MTSR::_1_TRANSFER => 0,
            DMA_MTSR::_4_TRANSFERS => 1,
            DMA_MTSR::_8_TRANSFERS => 2,
            DMA_MTSR::_16_TRANSFERS => 3,
            DMA_MTSR::_32_TRANSFERS => 4,
            DMA_MTSR::_64_TRANSFERS => 5,
            DMA_MTSR::_128_TRANSFERS => 6,
            DMA_MTSR::_256_TRANSFERS => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMA_MTSR {
        match value {
            0 => DMA_MTSR::_1_TRANSFER,
            1 => DMA_MTSR::_4_TRANSFERS,
            2 => DMA_MTSR::_8_TRANSFERS,
            3 => DMA_MTSR::_16_TRANSFERS,
            4 => DMA_MTSR::_32_TRANSFERS,
            5 => DMA_MTSR::_64_TRANSFERS,
            6 => DMA_MTSR::_128_TRANSFERS,
            7 => DMA_MTSR::_256_TRANSFERS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_TRANSFER`"]
    #[inline]
    pub fn is_1_transfer(&self) -> bool {
        *self == DMA_MTSR::_1_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_4_TRANSFERS`"]
    #[inline]
    pub fn is_4_transfers(&self) -> bool {
        *self == DMA_MTSR::_4_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_8_TRANSFERS`"]
    #[inline]
    pub fn is_8_transfers(&self) -> bool {
        *self == DMA_MTSR::_8_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_16_TRANSFERS`"]
    #[inline]
    pub fn is_16_transfers(&self) -> bool {
        *self == DMA_MTSR::_16_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_32_TRANSFERS`"]
    #[inline]
    pub fn is_32_transfers(&self) -> bool {
        *self == DMA_MTSR::_32_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_64_TRANSFERS`"]
    #[inline]
    pub fn is_64_transfers(&self) -> bool {
        *self == DMA_MTSR::_64_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_128_TRANSFERS`"]
    #[inline]
    pub fn is_128_transfers(&self) -> bool {
        *self == DMA_MTSR::_128_TRANSFERS
    }
    #[doc = "Checks if the value of the field is `_256_TRANSFERS`"]
    #[inline]
    pub fn is_256_transfers(&self) -> bool {
        *self == DMA_MTSR::_256_TRANSFERS
    }
}
#[doc = r" Proxy"]
pub struct _TX_WMARKW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_WMARKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_WMARKW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_WMARKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA_MTS`"]
pub enum DMA_MTSW {
    #[doc = "1 transfer"]
    _1_TRANSFER,
    #[doc = "4 transfers"]
    _4_TRANSFERS,
    #[doc = "8 transfers"]
    _8_TRANSFERS,
    #[doc = "16 transfers"]
    _16_TRANSFERS,
    #[doc = "32 transfers"]
    _32_TRANSFERS,
    #[doc = "64 transfers"]
    _64_TRANSFERS,
    #[doc = "128 transfers"]
    _128_TRANSFERS,
    #[doc = "256 transfers"]
    _256_TRANSFERS,
}
impl DMA_MTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMA_MTSW::_1_TRANSFER => 0,
            DMA_MTSW::_4_TRANSFERS => 1,
            DMA_MTSW::_8_TRANSFERS => 2,
            DMA_MTSW::_16_TRANSFERS => 3,
            DMA_MTSW::_32_TRANSFERS => 4,
            DMA_MTSW::_64_TRANSFERS => 5,
            DMA_MTSW::_128_TRANSFERS => 6,
            DMA_MTSW::_256_TRANSFERS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_MTSW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_MTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_MTSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 transfer"]
    #[inline]
    pub fn _1_transfer(self) -> &'a mut W {
        self.variant(DMA_MTSW::_1_TRANSFER)
    }
    #[doc = "4 transfers"]
    #[inline]
    pub fn _4_transfers(self) -> &'a mut W {
        self.variant(DMA_MTSW::_4_TRANSFERS)
    }
    #[doc = "8 transfers"]
    #[inline]
    pub fn _8_transfers(self) -> &'a mut W {
        self.variant(DMA_MTSW::_8_TRANSFERS)
    }
    #[doc = "16 transfers"]
    #[inline]
    pub fn _16_transfers(self) -> &'a mut W {
        self.variant(DMA_MTSW::_16_TRANSFERS)
    }
    #[doc = "32 transfers"]
    #[inline]
    pub fn _32_transfers(self) -> &'a mut W {
        self.variant(DMA_MTSW::_32_TRANSFERS)
    }
    #[doc = "64 transfers"]
    #[inline]
    pub fn _64_transfers(self) -> &'a mut W {
        self.variant(DMA_MTSW::_64_TRANSFERS)
    }
    #[doc = "128 transfers"]
    #[inline]
    pub fn _128_transfers(self) -> &'a mut W {
        self.variant(DMA_MTSW::_128_TRANSFERS)
    }
    #[doc = "256 transfers"]
    #[inline]
    pub fn _256_transfers(self) -> &'a mut W {
        self.variant(DMA_MTSW::_256_TRANSFERS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card. When FIFO data count is less than or equal to this number, DMA/FIFO request is raised. If Interrupt is enabled, then interrupt occurs. During end of packet, request or interrupt is generated, regardless of threshold programming. In non-DMA mode, when transmit FIFO threshold (TXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, on last interrupt, host is responsible for filling FIFO with only required remaining bytes (not before FIFO is full or after CIU completes data transfers, because FIFO may not be empty). In DMA mode, at end of packet, if last transfer is less than burst size, DMA controller does single cycles until required bytes are transferred. 12 bits - 1 bit less than FIFO-count of status register, which is 13 bits. Limitation: TX_WMark >= 1; Recommended value: TX_WMARK = 16; (means less than or equal to FIFO_DEPTH/2)."]
    #[inline]
    pub fn tx_wmark(&self) -> TX_WMARKR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TX_WMARKR { bits }
    }
    #[doc = "Bits 16:27 - FIFO threshold watermark level when receiving data to card. When FIFO data count reaches greater than this number, DMA/FIFO request is raised. During end of packet, request is generated regardless of threshold programming in order to complete any remaining data. In non-DMA mode, when receiver FIFO threshold (RXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, interrupt is not generated if threshold programming is larger than any remaining data. It is responsibility of host to read remaining bytes on seeing Data Transfer Done interrupt. In DMA mode, at end of packet, even if remaining bytes are less than threshold, DMA request does single transfers to flush out any remaining bytes before Data Transfer Done interrupt is set. 12 bits - 1 bit less than FIFO-count of status register, which is 13 bits. Limitation: RX_WMark less than FIFO_DEPTH-2 Recommended: RX_WMARK = 15; (means greater than (FIFO_DEPTH/2) - 1) NOTE: In DMA mode during CCS time-out, the DMA does not generate the request at the end of packet, even if remaining bytes are less than threshold. In this case, there will be some data left in the FIFO. It is the responsibility of the application to reset the FIFO after the CCS time-out."]
    #[inline]
    pub fn rx_wmark(&self) -> RX_WMARKR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RX_WMARKR { bits }
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE.The units for transfers is the H_DATA_WIDTH parameter. A single transfer (dw_dma_single assertion in case of Non DW DMA interface) would be signalled based on this value. Value should be sub-multiple of (RX_WMark + 1) and (32 - TX_WMark). For example, if FIFO_DEPTH = 16, FDATA_WIDTH = H_DATA_WIDTH Allowed combinations for MSize and TX_WMark are: MSize = 1, TX_WMARK = 1-15 MSize = 4, TX_WMark = 8 MSize = 4, TX_WMark = 4 MSize = 4, TX_WMark = 12 MSize = 8, TX_WMark = 8 MSize = 8, TX_WMark = 4. Allowed combinations for MSize and RX_WMark are: MSize = 1, RX_WMARK = 0-14 MSize = 4, RX_WMark = 3 MSize = 4, RX_WMark = 7 MSize = 4, RX_WMark = 11 MSize = 8, RX_WMark = 7 MSize = 8, RX_WMark = 11 Recommended: MSize = 8, TX_WMark = 8, RX_WMark = 7"]
    #[inline]
    pub fn dma_mts(&self) -> DMA_MTSR {
        DMA_MTSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 260046848 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card. When FIFO data count is less than or equal to this number, DMA/FIFO request is raised. If Interrupt is enabled, then interrupt occurs. During end of packet, request or interrupt is generated, regardless of threshold programming. In non-DMA mode, when transmit FIFO threshold (TXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, on last interrupt, host is responsible for filling FIFO with only required remaining bytes (not before FIFO is full or after CIU completes data transfers, because FIFO may not be empty). In DMA mode, at end of packet, if last transfer is less than burst size, DMA controller does single cycles until required bytes are transferred. 12 bits - 1 bit less than FIFO-count of status register, which is 13 bits. Limitation: TX_WMark >= 1; Recommended value: TX_WMARK = 16; (means less than or equal to FIFO_DEPTH/2)."]
    #[inline]
    pub fn tx_wmark(&mut self) -> _TX_WMARKW {
        _TX_WMARKW { w: self }
    }
    #[doc = "Bits 16:27 - FIFO threshold watermark level when receiving data to card. When FIFO data count reaches greater than this number, DMA/FIFO request is raised. During end of packet, request is generated regardless of threshold programming in order to complete any remaining data. In non-DMA mode, when receiver FIFO threshold (RXDR) interrupt is enabled, then interrupt is generated instead of DMA request. During end of packet, interrupt is not generated if threshold programming is larger than any remaining data. It is responsibility of host to read remaining bytes on seeing Data Transfer Done interrupt. In DMA mode, at end of packet, even if remaining bytes are less than threshold, DMA request does single transfers to flush out any remaining bytes before Data Transfer Done interrupt is set. 12 bits - 1 bit less than FIFO-count of status register, which is 13 bits. Limitation: RX_WMark less than FIFO_DEPTH-2 Recommended: RX_WMARK = 15; (means greater than (FIFO_DEPTH/2) - 1) NOTE: In DMA mode during CCS time-out, the DMA does not generate the request at the end of packet, even if remaining bytes are less than threshold. In this case, there will be some data left in the FIFO. It is the responsibility of the application to reset the FIFO after the CCS time-out."]
    #[inline]
    pub fn rx_wmark(&mut self) -> _RX_WMARKW {
        _RX_WMARKW { w: self }
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE.The units for transfers is the H_DATA_WIDTH parameter. A single transfer (dw_dma_single assertion in case of Non DW DMA interface) would be signalled based on this value. Value should be sub-multiple of (RX_WMark + 1) and (32 - TX_WMark). For example, if FIFO_DEPTH = 16, FDATA_WIDTH = H_DATA_WIDTH Allowed combinations for MSize and TX_WMark are: MSize = 1, TX_WMARK = 1-15 MSize = 4, TX_WMark = 8 MSize = 4, TX_WMark = 4 MSize = 4, TX_WMark = 12 MSize = 8, TX_WMark = 8 MSize = 8, TX_WMark = 4. Allowed combinations for MSize and RX_WMark are: MSize = 1, RX_WMARK = 0-14 MSize = 4, RX_WMark = 3 MSize = 4, RX_WMark = 7 MSize = 4, RX_WMark = 11 MSize = 8, RX_WMark = 7 MSize = 8, RX_WMark = 11 Recommended: MSize = 8, TX_WMark = 8, RX_WMark = 7"]
    #[inline]
    pub fn dma_mts(&mut self) -> _DMA_MTSW {
        _DMA_MTSW { w: self }
    }
}
