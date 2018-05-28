#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TEC_7_0R {
    bits: u8,
}
impl TEC_7_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REC_6_0R {
    bits: u8,
}
impl REC_6_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPR {
    #[doc = "The receive counter has reached the error passive level as defined in the  CAN2.0 specification."]
    PASSIVE,
    #[doc = "The receive counter is below the error passive level."]
    BELOWPASSIVE,
}
impl RPR {
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
            RPR::PASSIVE => true,
            RPR::BELOWPASSIVE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RPR {
        match value {
            true => RPR::PASSIVE,
            false => RPR::BELOWPASSIVE,
        }
    }
    #[doc = "Checks if the value of the field is `PASSIVE`"]
    #[inline]
    pub fn is_passive(&self) -> bool {
        *self == RPR::PASSIVE
    }
    #[doc = "Checks if the value of the field is `BELOWPASSIVE`"]
    #[inline]
    pub fn is_belowpassive(&self) -> bool {
        *self == RPR::BELOWPASSIVE
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Transmit error counter Current value of the transmit error counter (maximum value 127)"]
    #[inline]
    pub fn tec_7_0(&self) -> TEC_7_0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TEC_7_0R { bits }
    }
    #[doc = "Bits 8:14 - Receive error counter Current value of the receive error counter (maximum value 255)."]
    #[inline]
    pub fn rec_6_0(&self) -> REC_6_0R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REC_6_0R { bits }
    }
    #[doc = "Bit 15 - Receive error passive"]
    #[inline]
    pub fn rp(&self) -> RPR {
        RPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
