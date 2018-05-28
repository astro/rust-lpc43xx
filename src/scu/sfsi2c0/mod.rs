#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SFSI2C0 {
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
#[doc = "Possible values of the field `SCL_EFP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCL_EFPR {
    #[doc = "50 ns glitch filter"]
    _50_NS_GLITCH_FILTER,
    #[doc = "3 ns glitch filter"]
    _3_NS_GLITCH_FILTER,
}
impl SCL_EFPR {
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
            SCL_EFPR::_50_NS_GLITCH_FILTER => false,
            SCL_EFPR::_3_NS_GLITCH_FILTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCL_EFPR {
        match value {
            false => SCL_EFPR::_50_NS_GLITCH_FILTER,
            true => SCL_EFPR::_3_NS_GLITCH_FILTER,
        }
    }
    #[doc = "Checks if the value of the field is `_50_NS_GLITCH_FILTER`"]
    #[inline]
    pub fn is_50_ns_glitch_filter(&self) -> bool {
        *self == SCL_EFPR::_50_NS_GLITCH_FILTER
    }
    #[doc = "Checks if the value of the field is `_3_NS_GLITCH_FILTER`"]
    #[inline]
    pub fn is_3_ns_glitch_filter(&self) -> bool {
        *self == SCL_EFPR::_3_NS_GLITCH_FILTER
    }
}
#[doc = "Possible values of the field `SCL_EHD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCL_EHDR {
    #[doc = "Standard/Fast mode transmit"]
    STANDARDFAST_MODE,
    #[doc = "Fast-mode Plus transmit"]
    FAST_MODE_PLUS_TRANS,
}
impl SCL_EHDR {
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
            SCL_EHDR::STANDARDFAST_MODE => false,
            SCL_EHDR::FAST_MODE_PLUS_TRANS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCL_EHDR {
        match value {
            false => SCL_EHDR::STANDARDFAST_MODE,
            true => SCL_EHDR::FAST_MODE_PLUS_TRANS,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARDFAST_MODE`"]
    #[inline]
    pub fn is_standardfast_mode(&self) -> bool {
        *self == SCL_EHDR::STANDARDFAST_MODE
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS_TRANS`"]
    #[inline]
    pub fn is_fast_mode_plus_trans(&self) -> bool {
        *self == SCL_EHDR::FAST_MODE_PLUS_TRANS
    }
}
#[doc = "Possible values of the field `SCL_EZI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCL_EZIR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl SCL_EZIR {
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
            SCL_EZIR::DISABLED => false,
            SCL_EZIR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCL_EZIR {
        match value {
            false => SCL_EZIR::DISABLED,
            true => SCL_EZIR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SCL_EZIR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SCL_EZIR::ENABLED
    }
}
#[doc = "Possible values of the field `SCL_ZIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCL_ZIFR {
    #[doc = "Enable input filter"]
    ENABLE_INPUT_FILTER,
    #[doc = "Disable input filter"]
    DISABLE_INPUT_FILTER,
}
impl SCL_ZIFR {
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
            SCL_ZIFR::ENABLE_INPUT_FILTER => false,
            SCL_ZIFR::DISABLE_INPUT_FILTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCL_ZIFR {
        match value {
            false => SCL_ZIFR::ENABLE_INPUT_FILTER,
            true => SCL_ZIFR::DISABLE_INPUT_FILTER,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_INPUT_FILTER`"]
    #[inline]
    pub fn is_enable_input_filter(&self) -> bool {
        *self == SCL_ZIFR::ENABLE_INPUT_FILTER
    }
    #[doc = "Checks if the value of the field is `DISABLE_INPUT_FILTER`"]
    #[inline]
    pub fn is_disable_input_filter(&self) -> bool {
        *self == SCL_ZIFR::DISABLE_INPUT_FILTER
    }
}
#[doc = "Possible values of the field `SDA_EFP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDA_EFPR {
    #[doc = "50 ns glitch filter"]
    _50_NS_GLITCH_FILTER,
    #[doc = "3 ns glitch filter"]
    _3_NS_GLITCH_FILTER,
}
impl SDA_EFPR {
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
            SDA_EFPR::_50_NS_GLITCH_FILTER => false,
            SDA_EFPR::_3_NS_GLITCH_FILTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDA_EFPR {
        match value {
            false => SDA_EFPR::_50_NS_GLITCH_FILTER,
            true => SDA_EFPR::_3_NS_GLITCH_FILTER,
        }
    }
    #[doc = "Checks if the value of the field is `_50_NS_GLITCH_FILTER`"]
    #[inline]
    pub fn is_50_ns_glitch_filter(&self) -> bool {
        *self == SDA_EFPR::_50_NS_GLITCH_FILTER
    }
    #[doc = "Checks if the value of the field is `_3_NS_GLITCH_FILTER`"]
    #[inline]
    pub fn is_3_ns_glitch_filter(&self) -> bool {
        *self == SDA_EFPR::_3_NS_GLITCH_FILTER
    }
}
#[doc = "Possible values of the field `SDA_EHD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDA_EHDR {
    #[doc = "Standard/Fast mode transmit"]
    STANDARDFAST_MODE,
    #[doc = "Fast-mode Plus transmit"]
    FAST_MODE_PLUS_TRANS,
}
impl SDA_EHDR {
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
            SDA_EHDR::STANDARDFAST_MODE => false,
            SDA_EHDR::FAST_MODE_PLUS_TRANS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDA_EHDR {
        match value {
            false => SDA_EHDR::STANDARDFAST_MODE,
            true => SDA_EHDR::FAST_MODE_PLUS_TRANS,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARDFAST_MODE`"]
    #[inline]
    pub fn is_standardfast_mode(&self) -> bool {
        *self == SDA_EHDR::STANDARDFAST_MODE
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS_TRANS`"]
    #[inline]
    pub fn is_fast_mode_plus_trans(&self) -> bool {
        *self == SDA_EHDR::FAST_MODE_PLUS_TRANS
    }
}
#[doc = "Possible values of the field `SDA_EZI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDA_EZIR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl SDA_EZIR {
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
            SDA_EZIR::DISABLED => false,
            SDA_EZIR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDA_EZIR {
        match value {
            false => SDA_EZIR::DISABLED,
            true => SDA_EZIR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SDA_EZIR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SDA_EZIR::ENABLED
    }
}
#[doc = "Possible values of the field `SDA_ZIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDA_ZIFR {
    #[doc = "Enable input filter"]
    ENABLE_INPUT_FILTER,
    #[doc = "Disable input filter"]
    DISABLE_INPUT_FILTER,
}
impl SDA_ZIFR {
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
            SDA_ZIFR::ENABLE_INPUT_FILTER => false,
            SDA_ZIFR::DISABLE_INPUT_FILTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDA_ZIFR {
        match value {
            false => SDA_ZIFR::ENABLE_INPUT_FILTER,
            true => SDA_ZIFR::DISABLE_INPUT_FILTER,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_INPUT_FILTER`"]
    #[inline]
    pub fn is_enable_input_filter(&self) -> bool {
        *self == SDA_ZIFR::ENABLE_INPUT_FILTER
    }
    #[doc = "Checks if the value of the field is `DISABLE_INPUT_FILTER`"]
    #[inline]
    pub fn is_disable_input_filter(&self) -> bool {
        *self == SDA_ZIFR::DISABLE_INPUT_FILTER
    }
}
#[doc = "Values that can be written to the field `SCL_EFP`"]
pub enum SCL_EFPW {
    #[doc = "50 ns glitch filter"]
    _50_NS_GLITCH_FILTER,
    #[doc = "3 ns glitch filter"]
    _3_NS_GLITCH_FILTER,
}
impl SCL_EFPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCL_EFPW::_50_NS_GLITCH_FILTER => false,
            SCL_EFPW::_3_NS_GLITCH_FILTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCL_EFPW<'a> {
    w: &'a mut W,
}
impl<'a> _SCL_EFPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCL_EFPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "50 ns glitch filter"]
    #[inline]
    pub fn _50_ns_glitch_filter(self) -> &'a mut W {
        self.variant(SCL_EFPW::_50_NS_GLITCH_FILTER)
    }
    #[doc = "3 ns glitch filter"]
    #[inline]
    pub fn _3_ns_glitch_filter(self) -> &'a mut W {
        self.variant(SCL_EFPW::_3_NS_GLITCH_FILTER)
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
#[doc = "Values that can be written to the field `SCL_EHD`"]
pub enum SCL_EHDW {
    #[doc = "Standard/Fast mode transmit"]
    STANDARDFAST_MODE,
    #[doc = "Fast-mode Plus transmit"]
    FAST_MODE_PLUS_TRANS,
}
impl SCL_EHDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCL_EHDW::STANDARDFAST_MODE => false,
            SCL_EHDW::FAST_MODE_PLUS_TRANS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCL_EHDW<'a> {
    w: &'a mut W,
}
impl<'a> _SCL_EHDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCL_EHDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard/Fast mode transmit"]
    #[inline]
    pub fn standardfast_mode(self) -> &'a mut W {
        self.variant(SCL_EHDW::STANDARDFAST_MODE)
    }
    #[doc = "Fast-mode Plus transmit"]
    #[inline]
    pub fn fast_mode_plus_trans(self) -> &'a mut W {
        self.variant(SCL_EHDW::FAST_MODE_PLUS_TRANS)
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
#[doc = "Values that can be written to the field `SCL_EZI`"]
pub enum SCL_EZIW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl SCL_EZIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCL_EZIW::DISABLED => false,
            SCL_EZIW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCL_EZIW<'a> {
    w: &'a mut W,
}
impl<'a> _SCL_EZIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCL_EZIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCL_EZIW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCL_EZIW::ENABLED)
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
#[doc = "Values that can be written to the field `SCL_ZIF`"]
pub enum SCL_ZIFW {
    #[doc = "Enable input filter"]
    ENABLE_INPUT_FILTER,
    #[doc = "Disable input filter"]
    DISABLE_INPUT_FILTER,
}
impl SCL_ZIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCL_ZIFW::ENABLE_INPUT_FILTER => false,
            SCL_ZIFW::DISABLE_INPUT_FILTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCL_ZIFW<'a> {
    w: &'a mut W,
}
impl<'a> _SCL_ZIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCL_ZIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable input filter"]
    #[inline]
    pub fn enable_input_filter(self) -> &'a mut W {
        self.variant(SCL_ZIFW::ENABLE_INPUT_FILTER)
    }
    #[doc = "Disable input filter"]
    #[inline]
    pub fn disable_input_filter(self) -> &'a mut W {
        self.variant(SCL_ZIFW::DISABLE_INPUT_FILTER)
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
#[doc = "Values that can be written to the field `SDA_EFP`"]
pub enum SDA_EFPW {
    #[doc = "50 ns glitch filter"]
    _50_NS_GLITCH_FILTER,
    #[doc = "3 ns glitch filter"]
    _3_NS_GLITCH_FILTER,
}
impl SDA_EFPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDA_EFPW::_50_NS_GLITCH_FILTER => false,
            SDA_EFPW::_3_NS_GLITCH_FILTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDA_EFPW<'a> {
    w: &'a mut W,
}
impl<'a> _SDA_EFPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDA_EFPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "50 ns glitch filter"]
    #[inline]
    pub fn _50_ns_glitch_filter(self) -> &'a mut W {
        self.variant(SDA_EFPW::_50_NS_GLITCH_FILTER)
    }
    #[doc = "3 ns glitch filter"]
    #[inline]
    pub fn _3_ns_glitch_filter(self) -> &'a mut W {
        self.variant(SDA_EFPW::_3_NS_GLITCH_FILTER)
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
#[doc = "Values that can be written to the field `SDA_EHD`"]
pub enum SDA_EHDW {
    #[doc = "Standard/Fast mode transmit"]
    STANDARDFAST_MODE,
    #[doc = "Fast-mode Plus transmit"]
    FAST_MODE_PLUS_TRANS,
}
impl SDA_EHDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDA_EHDW::STANDARDFAST_MODE => false,
            SDA_EHDW::FAST_MODE_PLUS_TRANS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDA_EHDW<'a> {
    w: &'a mut W,
}
impl<'a> _SDA_EHDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDA_EHDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard/Fast mode transmit"]
    #[inline]
    pub fn standardfast_mode(self) -> &'a mut W {
        self.variant(SDA_EHDW::STANDARDFAST_MODE)
    }
    #[doc = "Fast-mode Plus transmit"]
    #[inline]
    pub fn fast_mode_plus_trans(self) -> &'a mut W {
        self.variant(SDA_EHDW::FAST_MODE_PLUS_TRANS)
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
#[doc = "Values that can be written to the field `SDA_EZI`"]
pub enum SDA_EZIW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl SDA_EZIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDA_EZIW::DISABLED => false,
            SDA_EZIW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDA_EZIW<'a> {
    w: &'a mut W,
}
impl<'a> _SDA_EZIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDA_EZIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDA_EZIW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDA_EZIW::ENABLED)
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
#[doc = "Values that can be written to the field `SDA_ZIF`"]
pub enum SDA_ZIFW {
    #[doc = "Enable input filter"]
    ENABLE_INPUT_FILTER,
    #[doc = "Disable input filter"]
    DISABLE_INPUT_FILTER,
}
impl SDA_ZIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDA_ZIFW::ENABLE_INPUT_FILTER => false,
            SDA_ZIFW::DISABLE_INPUT_FILTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDA_ZIFW<'a> {
    w: &'a mut W,
}
impl<'a> _SDA_ZIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDA_ZIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable input filter"]
    #[inline]
    pub fn enable_input_filter(self) -> &'a mut W {
        self.variant(SDA_ZIFW::ENABLE_INPUT_FILTER)
    }
    #[doc = "Disable input filter"]
    #[inline]
    pub fn disable_input_filter(self) -> &'a mut W {
        self.variant(SDA_ZIFW::DISABLE_INPUT_FILTER)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Select input glitch filter time constant for the SCL pin."]
    #[inline]
    pub fn scl_efp(&self) -> SCL_EFPR {
        SCL_EFPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Select I2C mode for the SCL pin."]
    #[inline]
    pub fn scl_ehd(&self) -> SCL_EHDR {
        SCL_EHDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable the input receiver for the SCL pin. Always write a 1 to this bit when using the I2C0."]
    #[inline]
    pub fn scl_ezi(&self) -> SCL_EZIR {
        SCL_EZIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable or disable input glitch filter for the SCL pin. The filter time constant is determined by bit EFP."]
    #[inline]
    pub fn scl_zif(&self) -> SCL_ZIFR {
        SCL_ZIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Select input glitch filter time constant for the SDA pin."]
    #[inline]
    pub fn sda_efp(&self) -> SDA_EFPR {
        SDA_EFPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Select I2C mode for the SDA pin."]
    #[inline]
    pub fn sda_ehd(&self) -> SDA_EHDR {
        SDA_EHDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enable the input receiver for the SDA pin. Always write a 1 to this bit when using the I2C0."]
    #[inline]
    pub fn sda_ezi(&self) -> SDA_EZIR {
        SDA_EZIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enable or disable input glitch filter for the SDA pin. The filter time constant is determined by bit SDA_EFP."]
    #[inline]
    pub fn sda_zif(&self) -> SDA_ZIFR {
        SDA_ZIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Select input glitch filter time constant for the SCL pin."]
    #[inline]
    pub fn scl_efp(&mut self) -> _SCL_EFPW {
        _SCL_EFPW { w: self }
    }
    #[doc = "Bit 2 - Select I2C mode for the SCL pin."]
    #[inline]
    pub fn scl_ehd(&mut self) -> _SCL_EHDW {
        _SCL_EHDW { w: self }
    }
    #[doc = "Bit 3 - Enable the input receiver for the SCL pin. Always write a 1 to this bit when using the I2C0."]
    #[inline]
    pub fn scl_ezi(&mut self) -> _SCL_EZIW {
        _SCL_EZIW { w: self }
    }
    #[doc = "Bit 7 - Enable or disable input glitch filter for the SCL pin. The filter time constant is determined by bit EFP."]
    #[inline]
    pub fn scl_zif(&mut self) -> _SCL_ZIFW {
        _SCL_ZIFW { w: self }
    }
    #[doc = "Bit 8 - Select input glitch filter time constant for the SDA pin."]
    #[inline]
    pub fn sda_efp(&mut self) -> _SDA_EFPW {
        _SDA_EFPW { w: self }
    }
    #[doc = "Bit 10 - Select I2C mode for the SDA pin."]
    #[inline]
    pub fn sda_ehd(&mut self) -> _SDA_EHDW {
        _SDA_EHDW { w: self }
    }
    #[doc = "Bit 11 - Enable the input receiver for the SDA pin. Always write a 1 to this bit when using the I2C0."]
    #[inline]
    pub fn sda_ezi(&mut self) -> _SDA_EZIW {
        _SDA_EZIW { w: self }
    }
    #[doc = "Bit 15 - Enable or disable input glitch filter for the SDA pin. The filter time constant is determined by bit SDA_EFP."]
    #[inline]
    pub fn sda_zif(&mut self) -> _SDA_ZIFW {
        _SDA_ZIFW { w: self }
    }
}
