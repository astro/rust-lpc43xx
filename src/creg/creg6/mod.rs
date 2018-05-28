#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CREG6 {
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
#[doc = "Possible values of the field `ETHMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHMODER {
    #[doc = "MII"]
    MII,
    #[doc = "RMII"]
    RMII,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ETHMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ETHMODER::MII => 0,
            ETHMODER::RMII => 4,
            ETHMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ETHMODER {
        match value {
            0 => ETHMODER::MII,
            4 => ETHMODER::RMII,
            i => ETHMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MII`"]
    #[inline]
    pub fn is_mii(&self) -> bool {
        *self == ETHMODER::MII
    }
    #[doc = "Checks if the value of the field is `RMII`"]
    #[inline]
    pub fn is_rmii(&self) -> bool {
        *self == ETHMODER::RMII
    }
}
#[doc = "Possible values of the field `CTOUTCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTOUTCTRLR {
    #[doc = "Combine SCT and timer match outputs. SCT outputs are Red with timer outputs."]
    COMBINE_SCT_AND_TIME,
    #[doc = "SCT outputs only. SCT outputs are used without timer match outputs."]
    SCT_OUTPUTS_ONLY,
}
impl CTOUTCTRLR {
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
            CTOUTCTRLR::COMBINE_SCT_AND_TIME => false,
            CTOUTCTRLR::SCT_OUTPUTS_ONLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTOUTCTRLR {
        match value {
            false => CTOUTCTRLR::COMBINE_SCT_AND_TIME,
            true => CTOUTCTRLR::SCT_OUTPUTS_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `COMBINE_SCT_AND_TIME`"]
    #[inline]
    pub fn is_combine_sct_and_time(&self) -> bool {
        *self == CTOUTCTRLR::COMBINE_SCT_AND_TIME
    }
    #[doc = "Checks if the value of the field is `SCT_OUTPUTS_ONLY`"]
    #[inline]
    pub fn is_sct_outputs_only(&self) -> bool {
        *self == CTOUTCTRLR::SCT_OUTPUTS_ONLY
    }
}
#[doc = "Possible values of the field `I2S0_TX_SCK_IN_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S0_TX_SCK_IN_SELR {
    #[doc = "I2S Register. I2S clock selected as defined by the I2S transmit mode register Table 960."]
    I2S_REGISTER,
    #[doc = "BASE_AUDIO_CLK for I2S transmit clock MCLK input and MCLK output. The I2S must be configured in slave mode."]
    BASE_AUDIO_CLK_FOR_I,
}
impl I2S0_TX_SCK_IN_SELR {
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
            I2S0_TX_SCK_IN_SELR::I2S_REGISTER => false,
            I2S0_TX_SCK_IN_SELR::BASE_AUDIO_CLK_FOR_I => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2S0_TX_SCK_IN_SELR {
        match value {
            false => I2S0_TX_SCK_IN_SELR::I2S_REGISTER,
            true => I2S0_TX_SCK_IN_SELR::BASE_AUDIO_CLK_FOR_I,
        }
    }
    #[doc = "Checks if the value of the field is `I2S_REGISTER`"]
    #[inline]
    pub fn is_i2s_register(&self) -> bool {
        *self == I2S0_TX_SCK_IN_SELR::I2S_REGISTER
    }
    #[doc = "Checks if the value of the field is `BASE_AUDIO_CLK_FOR_I`"]
    #[inline]
    pub fn is_base_audio_clk_for_i(&self) -> bool {
        *self == I2S0_TX_SCK_IN_SELR::BASE_AUDIO_CLK_FOR_I
    }
}
#[doc = "Possible values of the field `I2S0_RX_SCK_IN_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S0_RX_SCK_IN_SELR {
    #[doc = "I2S Register. I2S clock selected as defined by the I2S receive mode register Table 961."]
    I2S_REGISTER,
    #[doc = "BASE_AUDIO_CLK for I2S receive clock MCLK input and MCLK output. The I2S must be configured in slave mode."]
    BASE_AUDIO_CLK_FOR_I,
}
impl I2S0_RX_SCK_IN_SELR {
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
            I2S0_RX_SCK_IN_SELR::I2S_REGISTER => false,
            I2S0_RX_SCK_IN_SELR::BASE_AUDIO_CLK_FOR_I => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2S0_RX_SCK_IN_SELR {
        match value {
            false => I2S0_RX_SCK_IN_SELR::I2S_REGISTER,
            true => I2S0_RX_SCK_IN_SELR::BASE_AUDIO_CLK_FOR_I,
        }
    }
    #[doc = "Checks if the value of the field is `I2S_REGISTER`"]
    #[inline]
    pub fn is_i2s_register(&self) -> bool {
        *self == I2S0_RX_SCK_IN_SELR::I2S_REGISTER
    }
    #[doc = "Checks if the value of the field is `BASE_AUDIO_CLK_FOR_I`"]
    #[inline]
    pub fn is_base_audio_clk_for_i(&self) -> bool {
        *self == I2S0_RX_SCK_IN_SELR::BASE_AUDIO_CLK_FOR_I
    }
}
#[doc = "Possible values of the field `I2S1_TX_SCK_IN_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S1_TX_SCK_IN_SELR {
    #[doc = "I2S register. I2S clock selected as defined by the I2S transmit mode register Table 960."]
    I2S_REGISTER,
    #[doc = "BASE_AUDIO_CLK for I2S transmit clock MCLK input and MCLK output. The I2S must be configured in slave mode."]
    BASE_AUDIO_CLK_FOR_I,
}
impl I2S1_TX_SCK_IN_SELR {
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
            I2S1_TX_SCK_IN_SELR::I2S_REGISTER => false,
            I2S1_TX_SCK_IN_SELR::BASE_AUDIO_CLK_FOR_I => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2S1_TX_SCK_IN_SELR {
        match value {
            false => I2S1_TX_SCK_IN_SELR::I2S_REGISTER,
            true => I2S1_TX_SCK_IN_SELR::BASE_AUDIO_CLK_FOR_I,
        }
    }
    #[doc = "Checks if the value of the field is `I2S_REGISTER`"]
    #[inline]
    pub fn is_i2s_register(&self) -> bool {
        *self == I2S1_TX_SCK_IN_SELR::I2S_REGISTER
    }
    #[doc = "Checks if the value of the field is `BASE_AUDIO_CLK_FOR_I`"]
    #[inline]
    pub fn is_base_audio_clk_for_i(&self) -> bool {
        *self == I2S1_TX_SCK_IN_SELR::BASE_AUDIO_CLK_FOR_I
    }
}
#[doc = "Possible values of the field `I2S1_RX_SCK_IN_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S1_RX_SCK_IN_SELR {
    #[doc = "I2S register. I2S clock selected as defined by the I2S receive mode register Table 961."]
    I2S_REGISTER,
    #[doc = "BASE_AUDIO_CLK for I2S receive clock MCLK input and MCLK output. The I2S must be configured in slave mode."]
    BASE_AUDIO_CLK_FOR_I,
}
impl I2S1_RX_SCK_IN_SELR {
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
            I2S1_RX_SCK_IN_SELR::I2S_REGISTER => false,
            I2S1_RX_SCK_IN_SELR::BASE_AUDIO_CLK_FOR_I => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2S1_RX_SCK_IN_SELR {
        match value {
            false => I2S1_RX_SCK_IN_SELR::I2S_REGISTER,
            true => I2S1_RX_SCK_IN_SELR::BASE_AUDIO_CLK_FOR_I,
        }
    }
    #[doc = "Checks if the value of the field is `I2S_REGISTER`"]
    #[inline]
    pub fn is_i2s_register(&self) -> bool {
        *self == I2S1_RX_SCK_IN_SELR::I2S_REGISTER
    }
    #[doc = "Checks if the value of the field is `BASE_AUDIO_CLK_FOR_I`"]
    #[inline]
    pub fn is_base_audio_clk_for_i(&self) -> bool {
        *self == I2S1_RX_SCK_IN_SELR::BASE_AUDIO_CLK_FOR_I
    }
}
#[doc = "Possible values of the field `EMC_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMC_CLK_SELR {
    #[doc = "Divide by 1. EMC_CLK_DIV not divided."]
    DIVIDE_BY_1,
    #[doc = "Divide by 2. EMC_CLK_DIV divided by 2."]
    DIVIDE_BY_2,
}
impl EMC_CLK_SELR {
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
            EMC_CLK_SELR::DIVIDE_BY_1 => false,
            EMC_CLK_SELR::DIVIDE_BY_2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMC_CLK_SELR {
        match value {
            false => EMC_CLK_SELR::DIVIDE_BY_1,
            true => EMC_CLK_SELR::DIVIDE_BY_2,
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_1`"]
    #[inline]
    pub fn is_divide_by_1(&self) -> bool {
        *self == EMC_CLK_SELR::DIVIDE_BY_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_2`"]
    #[inline]
    pub fn is_divide_by_2(&self) -> bool {
        *self == EMC_CLK_SELR::DIVIDE_BY_2
    }
}
#[doc = "Values that can be written to the field `ETHMODE`"]
pub enum ETHMODEW {
    #[doc = "MII"]
    MII,
    #[doc = "RMII"]
    RMII,
}
impl ETHMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ETHMODEW::MII => 0,
            ETHMODEW::RMII => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETHMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ETHMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETHMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MII"]
    #[inline]
    pub fn mii(self) -> &'a mut W {
        self.variant(ETHMODEW::MII)
    }
    #[doc = "RMII"]
    #[inline]
    pub fn rmii(self) -> &'a mut W {
        self.variant(ETHMODEW::RMII)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTOUTCTRL`"]
pub enum CTOUTCTRLW {
    #[doc = "Combine SCT and timer match outputs. SCT outputs are Red with timer outputs."]
    COMBINE_SCT_AND_TIME,
    #[doc = "SCT outputs only. SCT outputs are used without timer match outputs."]
    SCT_OUTPUTS_ONLY,
}
impl CTOUTCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTOUTCTRLW::COMBINE_SCT_AND_TIME => false,
            CTOUTCTRLW::SCT_OUTPUTS_ONLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTOUTCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _CTOUTCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTOUTCTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Combine SCT and timer match outputs. SCT outputs are Red with timer outputs."]
    #[inline]
    pub fn combine_sct_and_time(self) -> &'a mut W {
        self.variant(CTOUTCTRLW::COMBINE_SCT_AND_TIME)
    }
    #[doc = "SCT outputs only. SCT outputs are used without timer match outputs."]
    #[inline]
    pub fn sct_outputs_only(self) -> &'a mut W {
        self.variant(CTOUTCTRLW::SCT_OUTPUTS_ONLY)
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
#[doc = "Values that can be written to the field `I2S0_TX_SCK_IN_SEL`"]
pub enum I2S0_TX_SCK_IN_SELW {
    #[doc = "I2S Register. I2S clock selected as defined by the I2S transmit mode register Table 960."]
    I2S_REGISTER,
    #[doc = "BASE_AUDIO_CLK for I2S transmit clock MCLK input and MCLK output. The I2S must be configured in slave mode."]
    BASE_AUDIO_CLK_FOR_I,
}
impl I2S0_TX_SCK_IN_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2S0_TX_SCK_IN_SELW::I2S_REGISTER => false,
            I2S0_TX_SCK_IN_SELW::BASE_AUDIO_CLK_FOR_I => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2S0_TX_SCK_IN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _I2S0_TX_SCK_IN_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2S0_TX_SCK_IN_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2S Register. I2S clock selected as defined by the I2S transmit mode register Table 960."]
    #[inline]
    pub fn i2s_register(self) -> &'a mut W {
        self.variant(I2S0_TX_SCK_IN_SELW::I2S_REGISTER)
    }
    #[doc = "BASE_AUDIO_CLK for I2S transmit clock MCLK input and MCLK output. The I2S must be configured in slave mode."]
    #[inline]
    pub fn base_audio_clk_for_i(self) -> &'a mut W {
        self.variant(I2S0_TX_SCK_IN_SELW::BASE_AUDIO_CLK_FOR_I)
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
#[doc = "Values that can be written to the field `I2S0_RX_SCK_IN_SEL`"]
pub enum I2S0_RX_SCK_IN_SELW {
    #[doc = "I2S Register. I2S clock selected as defined by the I2S receive mode register Table 961."]
    I2S_REGISTER,
    #[doc = "BASE_AUDIO_CLK for I2S receive clock MCLK input and MCLK output. The I2S must be configured in slave mode."]
    BASE_AUDIO_CLK_FOR_I,
}
impl I2S0_RX_SCK_IN_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2S0_RX_SCK_IN_SELW::I2S_REGISTER => false,
            I2S0_RX_SCK_IN_SELW::BASE_AUDIO_CLK_FOR_I => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2S0_RX_SCK_IN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _I2S0_RX_SCK_IN_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2S0_RX_SCK_IN_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2S Register. I2S clock selected as defined by the I2S receive mode register Table 961."]
    #[inline]
    pub fn i2s_register(self) -> &'a mut W {
        self.variant(I2S0_RX_SCK_IN_SELW::I2S_REGISTER)
    }
    #[doc = "BASE_AUDIO_CLK for I2S receive clock MCLK input and MCLK output. The I2S must be configured in slave mode."]
    #[inline]
    pub fn base_audio_clk_for_i(self) -> &'a mut W {
        self.variant(I2S0_RX_SCK_IN_SELW::BASE_AUDIO_CLK_FOR_I)
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
#[doc = "Values that can be written to the field `I2S1_TX_SCK_IN_SEL`"]
pub enum I2S1_TX_SCK_IN_SELW {
    #[doc = "I2S register. I2S clock selected as defined by the I2S transmit mode register Table 960."]
    I2S_REGISTER,
    #[doc = "BASE_AUDIO_CLK for I2S transmit clock MCLK input and MCLK output. The I2S must be configured in slave mode."]
    BASE_AUDIO_CLK_FOR_I,
}
impl I2S1_TX_SCK_IN_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2S1_TX_SCK_IN_SELW::I2S_REGISTER => false,
            I2S1_TX_SCK_IN_SELW::BASE_AUDIO_CLK_FOR_I => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2S1_TX_SCK_IN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _I2S1_TX_SCK_IN_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2S1_TX_SCK_IN_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2S register. I2S clock selected as defined by the I2S transmit mode register Table 960."]
    #[inline]
    pub fn i2s_register(self) -> &'a mut W {
        self.variant(I2S1_TX_SCK_IN_SELW::I2S_REGISTER)
    }
    #[doc = "BASE_AUDIO_CLK for I2S transmit clock MCLK input and MCLK output. The I2S must be configured in slave mode."]
    #[inline]
    pub fn base_audio_clk_for_i(self) -> &'a mut W {
        self.variant(I2S1_TX_SCK_IN_SELW::BASE_AUDIO_CLK_FOR_I)
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
#[doc = "Values that can be written to the field `I2S1_RX_SCK_IN_SEL`"]
pub enum I2S1_RX_SCK_IN_SELW {
    #[doc = "I2S register. I2S clock selected as defined by the I2S receive mode register Table 961."]
    I2S_REGISTER,
    #[doc = "BASE_AUDIO_CLK for I2S receive clock MCLK input and MCLK output. The I2S must be configured in slave mode."]
    BASE_AUDIO_CLK_FOR_I,
}
impl I2S1_RX_SCK_IN_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2S1_RX_SCK_IN_SELW::I2S_REGISTER => false,
            I2S1_RX_SCK_IN_SELW::BASE_AUDIO_CLK_FOR_I => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2S1_RX_SCK_IN_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _I2S1_RX_SCK_IN_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2S1_RX_SCK_IN_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2S register. I2S clock selected as defined by the I2S receive mode register Table 961."]
    #[inline]
    pub fn i2s_register(self) -> &'a mut W {
        self.variant(I2S1_RX_SCK_IN_SELW::I2S_REGISTER)
    }
    #[doc = "BASE_AUDIO_CLK for I2S receive clock MCLK input and MCLK output. The I2S must be configured in slave mode."]
    #[inline]
    pub fn base_audio_clk_for_i(self) -> &'a mut W {
        self.variant(I2S1_RX_SCK_IN_SELW::BASE_AUDIO_CLK_FOR_I)
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
#[doc = "Values that can be written to the field `EMC_CLK_SEL`"]
pub enum EMC_CLK_SELW {
    #[doc = "Divide by 1. EMC_CLK_DIV not divided."]
    DIVIDE_BY_1,
    #[doc = "Divide by 2. EMC_CLK_DIV divided by 2."]
    DIVIDE_BY_2,
}
impl EMC_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMC_CLK_SELW::DIVIDE_BY_1 => false,
            EMC_CLK_SELW::DIVIDE_BY_2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMC_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _EMC_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMC_CLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Divide by 1. EMC_CLK_DIV not divided."]
    #[inline]
    pub fn divide_by_1(self) -> &'a mut W {
        self.variant(EMC_CLK_SELW::DIVIDE_BY_1)
    }
    #[doc = "Divide by 2. EMC_CLK_DIV divided by 2."]
    #[inline]
    pub fn divide_by_2(self) -> &'a mut W {
        self.variant(EMC_CLK_SELW::DIVIDE_BY_2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Selects the Ethernet mode. Reset the ethernet after changing the PHY interface. All other settings are reserved."]
    #[inline]
    pub fn ethmode(&self) -> ETHMODER {
        ETHMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Selects the functionality of the SCT outputs."]
    #[inline]
    pub fn ctoutctrl(&self) -> CTOUTCTRLR {
        CTOUTCTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - I2S0_TX_SCK input select"]
    #[inline]
    pub fn i2s0_tx_sck_in_sel(&self) -> I2S0_TX_SCK_IN_SELR {
        I2S0_TX_SCK_IN_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - I2S0_RX_SCK input select"]
    #[inline]
    pub fn i2s0_rx_sck_in_sel(&self) -> I2S0_RX_SCK_IN_SELR {
        I2S0_RX_SCK_IN_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - I2S1_TX_SCK input select"]
    #[inline]
    pub fn i2s1_tx_sck_in_sel(&self) -> I2S1_TX_SCK_IN_SELR {
        I2S1_TX_SCK_IN_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - I2S1_RX_SCK input select"]
    #[inline]
    pub fn i2s1_rx_sck_in_sel(&self) -> I2S1_RX_SCK_IN_SELR {
        I2S1_RX_SCK_IN_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - EMC_CLK divided clock select (see Section 21.1)."]
    #[inline]
    pub fn emc_clk_sel(&self) -> EMC_CLK_SELR {
        EMC_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:2 - Selects the Ethernet mode. Reset the ethernet after changing the PHY interface. All other settings are reserved."]
    #[inline]
    pub fn ethmode(&mut self) -> _ETHMODEW {
        _ETHMODEW { w: self }
    }
    #[doc = "Bit 4 - Selects the functionality of the SCT outputs."]
    #[inline]
    pub fn ctoutctrl(&mut self) -> _CTOUTCTRLW {
        _CTOUTCTRLW { w: self }
    }
    #[doc = "Bit 12 - I2S0_TX_SCK input select"]
    #[inline]
    pub fn i2s0_tx_sck_in_sel(&mut self) -> _I2S0_TX_SCK_IN_SELW {
        _I2S0_TX_SCK_IN_SELW { w: self }
    }
    #[doc = "Bit 13 - I2S0_RX_SCK input select"]
    #[inline]
    pub fn i2s0_rx_sck_in_sel(&mut self) -> _I2S0_RX_SCK_IN_SELW {
        _I2S0_RX_SCK_IN_SELW { w: self }
    }
    #[doc = "Bit 14 - I2S1_TX_SCK input select"]
    #[inline]
    pub fn i2s1_tx_sck_in_sel(&mut self) -> _I2S1_TX_SCK_IN_SELW {
        _I2S1_TX_SCK_IN_SELW { w: self }
    }
    #[doc = "Bit 15 - I2S1_RX_SCK input select"]
    #[inline]
    pub fn i2s1_rx_sck_in_sel(&mut self) -> _I2S1_RX_SCK_IN_SELW {
        _I2S1_RX_SCK_IN_SELW { w: self }
    }
    #[doc = "Bit 16 - EMC_CLK divided clock select (see Section 21.1)."]
    #[inline]
    pub fn emc_clk_sel(&mut self) -> _EMC_CLK_SELW {
        _EMC_CLK_SELW { w: self }
    }
}
