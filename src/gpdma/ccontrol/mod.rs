#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCONTROL {
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
pub struct TRANSFERSIZER {
    bits: u16,
}
impl TRANSFERSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `SBSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBSIZER {
    #[doc = "Source burst size = 1"]
    SOURCE_BURST_1,
    #[doc = "Source burst size = 4"]
    SOURCE_BURST_4,
    #[doc = "Source burst size = 8"]
    SOURCE_BURST_8,
    #[doc = "Source burst size = 16"]
    SOURCE_BURST_16,
    #[doc = "Source burst size = 32"]
    SOURCE_BURST_32,
    #[doc = "Source burst size = 64"]
    SOURCE_BURST_64,
    #[doc = "Source burst size = 128"]
    SOURCE_BURST_128,
    #[doc = "Source burst size = 256"]
    SOURCE_BURST_256,
}
impl SBSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SBSIZER::SOURCE_BURST_1 => 0,
            SBSIZER::SOURCE_BURST_4 => 1,
            SBSIZER::SOURCE_BURST_8 => 2,
            SBSIZER::SOURCE_BURST_16 => 3,
            SBSIZER::SOURCE_BURST_32 => 4,
            SBSIZER::SOURCE_BURST_64 => 5,
            SBSIZER::SOURCE_BURST_128 => 6,
            SBSIZER::SOURCE_BURST_256 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SBSIZER {
        match value {
            0 => SBSIZER::SOURCE_BURST_1,
            1 => SBSIZER::SOURCE_BURST_4,
            2 => SBSIZER::SOURCE_BURST_8,
            3 => SBSIZER::SOURCE_BURST_16,
            4 => SBSIZER::SOURCE_BURST_32,
            5 => SBSIZER::SOURCE_BURST_64,
            6 => SBSIZER::SOURCE_BURST_128,
            7 => SBSIZER::SOURCE_BURST_256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SOURCE_BURST_1`"]
    #[inline]
    pub fn is_source_burst_1(&self) -> bool {
        *self == SBSIZER::SOURCE_BURST_1
    }
    #[doc = "Checks if the value of the field is `SOURCE_BURST_4`"]
    #[inline]
    pub fn is_source_burst_4(&self) -> bool {
        *self == SBSIZER::SOURCE_BURST_4
    }
    #[doc = "Checks if the value of the field is `SOURCE_BURST_8`"]
    #[inline]
    pub fn is_source_burst_8(&self) -> bool {
        *self == SBSIZER::SOURCE_BURST_8
    }
    #[doc = "Checks if the value of the field is `SOURCE_BURST_16`"]
    #[inline]
    pub fn is_source_burst_16(&self) -> bool {
        *self == SBSIZER::SOURCE_BURST_16
    }
    #[doc = "Checks if the value of the field is `SOURCE_BURST_32`"]
    #[inline]
    pub fn is_source_burst_32(&self) -> bool {
        *self == SBSIZER::SOURCE_BURST_32
    }
    #[doc = "Checks if the value of the field is `SOURCE_BURST_64`"]
    #[inline]
    pub fn is_source_burst_64(&self) -> bool {
        *self == SBSIZER::SOURCE_BURST_64
    }
    #[doc = "Checks if the value of the field is `SOURCE_BURST_128`"]
    #[inline]
    pub fn is_source_burst_128(&self) -> bool {
        *self == SBSIZER::SOURCE_BURST_128
    }
    #[doc = "Checks if the value of the field is `SOURCE_BURST_256`"]
    #[inline]
    pub fn is_source_burst_256(&self) -> bool {
        *self == SBSIZER::SOURCE_BURST_256
    }
}
#[doc = "Possible values of the field `DBSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBSIZER {
    #[doc = "Destination burst size = 1"]
    DESTINATION_BURST_1,
    #[doc = "Destination burst size = 4"]
    DESTINATION_BURST_4,
    #[doc = "Destination burst size = 8"]
    DESTINATION_BURST_8,
    #[doc = "Destination burst size = 16"]
    DESTINATION_BURST_16,
    #[doc = "Destination burst size = 32"]
    DESTINATION_BURST_32,
    #[doc = "Destination burst size = 64"]
    DESTINATION_BURST_64,
    #[doc = "Destination burst size = 128"]
    DESTINATION_BURST_128,
    #[doc = "Destination burst size = 256"]
    DESTINATION_BURST_256,
}
impl DBSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DBSIZER::DESTINATION_BURST_1 => 0,
            DBSIZER::DESTINATION_BURST_4 => 1,
            DBSIZER::DESTINATION_BURST_8 => 2,
            DBSIZER::DESTINATION_BURST_16 => 3,
            DBSIZER::DESTINATION_BURST_32 => 4,
            DBSIZER::DESTINATION_BURST_64 => 5,
            DBSIZER::DESTINATION_BURST_128 => 6,
            DBSIZER::DESTINATION_BURST_256 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DBSIZER {
        match value {
            0 => DBSIZER::DESTINATION_BURST_1,
            1 => DBSIZER::DESTINATION_BURST_4,
            2 => DBSIZER::DESTINATION_BURST_8,
            3 => DBSIZER::DESTINATION_BURST_16,
            4 => DBSIZER::DESTINATION_BURST_32,
            5 => DBSIZER::DESTINATION_BURST_64,
            6 => DBSIZER::DESTINATION_BURST_128,
            7 => DBSIZER::DESTINATION_BURST_256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DESTINATION_BURST_1`"]
    #[inline]
    pub fn is_destination_burst_1(&self) -> bool {
        *self == DBSIZER::DESTINATION_BURST_1
    }
    #[doc = "Checks if the value of the field is `DESTINATION_BURST_4`"]
    #[inline]
    pub fn is_destination_burst_4(&self) -> bool {
        *self == DBSIZER::DESTINATION_BURST_4
    }
    #[doc = "Checks if the value of the field is `DESTINATION_BURST_8`"]
    #[inline]
    pub fn is_destination_burst_8(&self) -> bool {
        *self == DBSIZER::DESTINATION_BURST_8
    }
    #[doc = "Checks if the value of the field is `DESTINATION_BURST_16`"]
    #[inline]
    pub fn is_destination_burst_16(&self) -> bool {
        *self == DBSIZER::DESTINATION_BURST_16
    }
    #[doc = "Checks if the value of the field is `DESTINATION_BURST_32`"]
    #[inline]
    pub fn is_destination_burst_32(&self) -> bool {
        *self == DBSIZER::DESTINATION_BURST_32
    }
    #[doc = "Checks if the value of the field is `DESTINATION_BURST_64`"]
    #[inline]
    pub fn is_destination_burst_64(&self) -> bool {
        *self == DBSIZER::DESTINATION_BURST_64
    }
    #[doc = "Checks if the value of the field is `DESTINATION_BURST_128`"]
    #[inline]
    pub fn is_destination_burst_128(&self) -> bool {
        *self == DBSIZER::DESTINATION_BURST_128
    }
    #[doc = "Checks if the value of the field is `DESTINATION_BURST_256`"]
    #[inline]
    pub fn is_destination_burst_256(&self) -> bool {
        *self == DBSIZER::DESTINATION_BURST_256
    }
}
#[doc = "Possible values of the field `SWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWIDTHR {
    #[doc = "Byte (8-bit)"]
    BYTE_8_BIT,
    #[doc = "Halfword (16-bit)"]
    HALFWORD_16_BIT,
    #[doc = "Word (32-bit)"]
    WORD_32_BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SWIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SWIDTHR::BYTE_8_BIT => 0,
            SWIDTHR::HALFWORD_16_BIT => 1,
            SWIDTHR::WORD_32_BIT => 2,
            SWIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SWIDTHR {
        match value {
            0 => SWIDTHR::BYTE_8_BIT,
            1 => SWIDTHR::HALFWORD_16_BIT,
            2 => SWIDTHR::WORD_32_BIT,
            i => SWIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE_8_BIT`"]
    #[inline]
    pub fn is_byte_8_bit(&self) -> bool {
        *self == SWIDTHR::BYTE_8_BIT
    }
    #[doc = "Checks if the value of the field is `HALFWORD_16_BIT`"]
    #[inline]
    pub fn is_halfword_16_bit(&self) -> bool {
        *self == SWIDTHR::HALFWORD_16_BIT
    }
    #[doc = "Checks if the value of the field is `WORD_32_BIT`"]
    #[inline]
    pub fn is_word_32_bit(&self) -> bool {
        *self == SWIDTHR::WORD_32_BIT
    }
}
#[doc = "Possible values of the field `DWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWIDTHR {
    #[doc = "Byte (8-bit)"]
    BYTE_8_BIT,
    #[doc = "Halfword (16-bit)"]
    HALFWORD_16_BIT,
    #[doc = "Word (32-bit)"]
    WORD_32_BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DWIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DWIDTHR::BYTE_8_BIT => 0,
            DWIDTHR::HALFWORD_16_BIT => 1,
            DWIDTHR::WORD_32_BIT => 2,
            DWIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DWIDTHR {
        match value {
            0 => DWIDTHR::BYTE_8_BIT,
            1 => DWIDTHR::HALFWORD_16_BIT,
            2 => DWIDTHR::WORD_32_BIT,
            i => DWIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE_8_BIT`"]
    #[inline]
    pub fn is_byte_8_bit(&self) -> bool {
        *self == DWIDTHR::BYTE_8_BIT
    }
    #[doc = "Checks if the value of the field is `HALFWORD_16_BIT`"]
    #[inline]
    pub fn is_halfword_16_bit(&self) -> bool {
        *self == DWIDTHR::HALFWORD_16_BIT
    }
    #[doc = "Checks if the value of the field is `WORD_32_BIT`"]
    #[inline]
    pub fn is_word_32_bit(&self) -> bool {
        *self == DWIDTHR::WORD_32_BIT
    }
}
#[doc = "Possible values of the field `S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR {
    #[doc = "AHB Master 0 selected for source transfer."]
    AHB_MASTER_0_SELECTE,
    #[doc = "AHB Master 1 selected for source transfer."]
    AHB_MASTER_1_SELECTE,
}
impl SR {
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
            SR::AHB_MASTER_0_SELECTE => false,
            SR::AHB_MASTER_1_SELECTE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR {
        match value {
            false => SR::AHB_MASTER_0_SELECTE,
            true => SR::AHB_MASTER_1_SELECTE,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_MASTER_0_SELECTE`"]
    #[inline]
    pub fn is_ahb_master_0_selecte(&self) -> bool {
        *self == SR::AHB_MASTER_0_SELECTE
    }
    #[doc = "Checks if the value of the field is `AHB_MASTER_1_SELECTE`"]
    #[inline]
    pub fn is_ahb_master_1_selecte(&self) -> bool {
        *self == SR::AHB_MASTER_1_SELECTE
    }
}
#[doc = "Possible values of the field `D`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DR {
    #[doc = "AHB Master 0 selected for destination transfer."]
    AHB_MASTER_0_SELECTE,
    #[doc = "AHB Master 1 selected for destination transfer."]
    AHB_MASTER_1_SELECTE,
}
impl DR {
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
            DR::AHB_MASTER_0_SELECTE => false,
            DR::AHB_MASTER_1_SELECTE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DR {
        match value {
            false => DR::AHB_MASTER_0_SELECTE,
            true => DR::AHB_MASTER_1_SELECTE,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_MASTER_0_SELECTE`"]
    #[inline]
    pub fn is_ahb_master_0_selecte(&self) -> bool {
        *self == DR::AHB_MASTER_0_SELECTE
    }
    #[doc = "Checks if the value of the field is `AHB_MASTER_1_SELECTE`"]
    #[inline]
    pub fn is_ahb_master_1_selecte(&self) -> bool {
        *self == DR::AHB_MASTER_1_SELECTE
    }
}
#[doc = "Possible values of the field `SI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIR {
    #[doc = "The source address is not incremented after each transfer."]
    NOT_INCREMENT,
    #[doc = "The source address is incremented after each transfer."]
    INCREMENT,
}
impl SIR {
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
            SIR::NOT_INCREMENT => false,
            SIR::INCREMENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIR {
        match value {
            false => SIR::NOT_INCREMENT,
            true => SIR::INCREMENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INCREMENT`"]
    #[inline]
    pub fn is_not_increment(&self) -> bool {
        *self == SIR::NOT_INCREMENT
    }
    #[doc = "Checks if the value of the field is `INCREMENT`"]
    #[inline]
    pub fn is_increment(&self) -> bool {
        *self == SIR::INCREMENT
    }
}
#[doc = "Possible values of the field `DI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR {
    #[doc = "The destination address is not incremented after each transfer."]
    THE_DESTINATION_ADDR,
    #[doc = "The destination address is incremented after each transfer."]
    THE_DESTINATION_ADDR,
}
impl DIR {
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
            DIR::THE_DESTINATION_ADDR => false,
            DIR::THE_DESTINATION_ADDR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIR {
        match value {
            false => DIR::THE_DESTINATION_ADDR,
            true => DIR::THE_DESTINATION_ADDR,
        }
    }
    #[doc = "Checks if the value of the field is `THE_DESTINATION_ADDR`"]
    #[inline]
    pub fn is_the_destination_addr(&self) -> bool {
        *self == DIR::THE_DESTINATION_ADDR
    }
    #[doc = "Checks if the value of the field is `THE_DESTINATION_ADDR`"]
    #[inline]
    pub fn is_the_destination_addr(&self) -> bool {
        *self == DIR::THE_DESTINATION_ADDR
    }
}
#[doc = "Possible values of the field `PROT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROT1R {
    #[doc = "Access is in user mode"]
    ACCESS_IS_IN_USER_MO,
    #[doc = "Access is in privileged mode."]
    ACCESS_IS_IN_PRIVILE,
}
impl PROT1R {
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
            PROT1R::ACCESS_IS_IN_USER_MO => false,
            PROT1R::ACCESS_IS_IN_PRIVILE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROT1R {
        match value {
            false => PROT1R::ACCESS_IS_IN_USER_MO,
            true => PROT1R::ACCESS_IS_IN_PRIVILE,
        }
    }
    #[doc = "Checks if the value of the field is `ACCESS_IS_IN_USER_MO`"]
    #[inline]
    pub fn is_access_is_in_user_mo(&self) -> bool {
        *self == PROT1R::ACCESS_IS_IN_USER_MO
    }
    #[doc = "Checks if the value of the field is `ACCESS_IS_IN_PRIVILE`"]
    #[inline]
    pub fn is_access_is_in_privile(&self) -> bool {
        *self == PROT1R::ACCESS_IS_IN_PRIVILE
    }
}
#[doc = "Possible values of the field `PROT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROT2R {
    #[doc = "Access is not bufferable."]
    ACCESS_IS_NOT_BUFFER,
    #[doc = "Access is bufferable."]
    ACCESS_IS_BUFFERABLE,
}
impl PROT2R {
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
            PROT2R::ACCESS_IS_NOT_BUFFER => false,
            PROT2R::ACCESS_IS_BUFFERABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROT2R {
        match value {
            false => PROT2R::ACCESS_IS_NOT_BUFFER,
            true => PROT2R::ACCESS_IS_BUFFERABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ACCESS_IS_NOT_BUFFER`"]
    #[inline]
    pub fn is_access_is_not_buffer(&self) -> bool {
        *self == PROT2R::ACCESS_IS_NOT_BUFFER
    }
    #[doc = "Checks if the value of the field is `ACCESS_IS_BUFFERABLE`"]
    #[inline]
    pub fn is_access_is_bufferable(&self) -> bool {
        *self == PROT2R::ACCESS_IS_BUFFERABLE
    }
}
#[doc = "Possible values of the field `PROT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROT3R {
    #[doc = "Access is not cacheable."]
    ACCESS_IS_NOT_CACHEA,
    #[doc = "Access is cacheable."]
    ACCESS_IS_CACHEABLE_,
}
impl PROT3R {
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
            PROT3R::ACCESS_IS_NOT_CACHEA => false,
            PROT3R::ACCESS_IS_CACHEABLE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROT3R {
        match value {
            false => PROT3R::ACCESS_IS_NOT_CACHEA,
            true => PROT3R::ACCESS_IS_CACHEABLE_,
        }
    }
    #[doc = "Checks if the value of the field is `ACCESS_IS_NOT_CACHEA`"]
    #[inline]
    pub fn is_access_is_not_cachea(&self) -> bool {
        *self == PROT3R::ACCESS_IS_NOT_CACHEA
    }
    #[doc = "Checks if the value of the field is `ACCESS_IS_CACHEABLE_`"]
    #[inline]
    pub fn is_access_is_cacheable_(&self) -> bool {
        *self == PROT3R::ACCESS_IS_CACHEABLE_
    }
}
#[doc = "Possible values of the field `I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IR {
    #[doc = "The terminal count interrupt is disabled."]
    THE_TERMINAL_COUNT_I,
    #[doc = "The terminal count interrupt is enabled."]
    THE_TERMINAL_COUNT_I,
}
impl IR {
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
            IR::THE_TERMINAL_COUNT_I => false,
            IR::THE_TERMINAL_COUNT_I => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IR {
        match value {
            false => IR::THE_TERMINAL_COUNT_I,
            true => IR::THE_TERMINAL_COUNT_I,
        }
    }
    #[doc = "Checks if the value of the field is `THE_TERMINAL_COUNT_I`"]
    #[inline]
    pub fn is_the_terminal_count_i(&self) -> bool {
        *self == IR::THE_TERMINAL_COUNT_I
    }
    #[doc = "Checks if the value of the field is `THE_TERMINAL_COUNT_I`"]
    #[inline]
    pub fn is_the_terminal_count_i(&self) -> bool {
        *self == IR::THE_TERMINAL_COUNT_I
    }
}
#[doc = r" Proxy"]
pub struct _TRANSFERSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRANSFERSIZEW<'a> {
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
#[doc = "Values that can be written to the field `SBSIZE`"]
pub enum SBSIZEW {
    #[doc = "Source burst size = 1"]
    SOURCE_BURST_1,
    #[doc = "Source burst size = 4"]
    SOURCE_BURST_4,
    #[doc = "Source burst size = 8"]
    SOURCE_BURST_8,
    #[doc = "Source burst size = 16"]
    SOURCE_BURST_16,
    #[doc = "Source burst size = 32"]
    SOURCE_BURST_32,
    #[doc = "Source burst size = 64"]
    SOURCE_BURST_64,
    #[doc = "Source burst size = 128"]
    SOURCE_BURST_128,
    #[doc = "Source burst size = 256"]
    SOURCE_BURST_256,
}
impl SBSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SBSIZEW::SOURCE_BURST_1 => 0,
            SBSIZEW::SOURCE_BURST_4 => 1,
            SBSIZEW::SOURCE_BURST_8 => 2,
            SBSIZEW::SOURCE_BURST_16 => 3,
            SBSIZEW::SOURCE_BURST_32 => 4,
            SBSIZEW::SOURCE_BURST_64 => 5,
            SBSIZEW::SOURCE_BURST_128 => 6,
            SBSIZEW::SOURCE_BURST_256 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SBSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Source burst size = 1"]
    #[inline]
    pub fn source_burst_1(self) -> &'a mut W {
        self.variant(SBSIZEW::SOURCE_BURST_1)
    }
    #[doc = "Source burst size = 4"]
    #[inline]
    pub fn source_burst_4(self) -> &'a mut W {
        self.variant(SBSIZEW::SOURCE_BURST_4)
    }
    #[doc = "Source burst size = 8"]
    #[inline]
    pub fn source_burst_8(self) -> &'a mut W {
        self.variant(SBSIZEW::SOURCE_BURST_8)
    }
    #[doc = "Source burst size = 16"]
    #[inline]
    pub fn source_burst_16(self) -> &'a mut W {
        self.variant(SBSIZEW::SOURCE_BURST_16)
    }
    #[doc = "Source burst size = 32"]
    #[inline]
    pub fn source_burst_32(self) -> &'a mut W {
        self.variant(SBSIZEW::SOURCE_BURST_32)
    }
    #[doc = "Source burst size = 64"]
    #[inline]
    pub fn source_burst_64(self) -> &'a mut W {
        self.variant(SBSIZEW::SOURCE_BURST_64)
    }
    #[doc = "Source burst size = 128"]
    #[inline]
    pub fn source_burst_128(self) -> &'a mut W {
        self.variant(SBSIZEW::SOURCE_BURST_128)
    }
    #[doc = "Source burst size = 256"]
    #[inline]
    pub fn source_burst_256(self) -> &'a mut W {
        self.variant(SBSIZEW::SOURCE_BURST_256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DBSIZE`"]
pub enum DBSIZEW {
    #[doc = "Destination burst size = 1"]
    DESTINATION_BURST_1,
    #[doc = "Destination burst size = 4"]
    DESTINATION_BURST_4,
    #[doc = "Destination burst size = 8"]
    DESTINATION_BURST_8,
    #[doc = "Destination burst size = 16"]
    DESTINATION_BURST_16,
    #[doc = "Destination burst size = 32"]
    DESTINATION_BURST_32,
    #[doc = "Destination burst size = 64"]
    DESTINATION_BURST_64,
    #[doc = "Destination burst size = 128"]
    DESTINATION_BURST_128,
    #[doc = "Destination burst size = 256"]
    DESTINATION_BURST_256,
}
impl DBSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DBSIZEW::DESTINATION_BURST_1 => 0,
            DBSIZEW::DESTINATION_BURST_4 => 1,
            DBSIZEW::DESTINATION_BURST_8 => 2,
            DBSIZEW::DESTINATION_BURST_16 => 3,
            DBSIZEW::DESTINATION_BURST_32 => 4,
            DBSIZEW::DESTINATION_BURST_64 => 5,
            DBSIZEW::DESTINATION_BURST_128 => 6,
            DBSIZEW::DESTINATION_BURST_256 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DBSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Destination burst size = 1"]
    #[inline]
    pub fn destination_burst_1(self) -> &'a mut W {
        self.variant(DBSIZEW::DESTINATION_BURST_1)
    }
    #[doc = "Destination burst size = 4"]
    #[inline]
    pub fn destination_burst_4(self) -> &'a mut W {
        self.variant(DBSIZEW::DESTINATION_BURST_4)
    }
    #[doc = "Destination burst size = 8"]
    #[inline]
    pub fn destination_burst_8(self) -> &'a mut W {
        self.variant(DBSIZEW::DESTINATION_BURST_8)
    }
    #[doc = "Destination burst size = 16"]
    #[inline]
    pub fn destination_burst_16(self) -> &'a mut W {
        self.variant(DBSIZEW::DESTINATION_BURST_16)
    }
    #[doc = "Destination burst size = 32"]
    #[inline]
    pub fn destination_burst_32(self) -> &'a mut W {
        self.variant(DBSIZEW::DESTINATION_BURST_32)
    }
    #[doc = "Destination burst size = 64"]
    #[inline]
    pub fn destination_burst_64(self) -> &'a mut W {
        self.variant(DBSIZEW::DESTINATION_BURST_64)
    }
    #[doc = "Destination burst size = 128"]
    #[inline]
    pub fn destination_burst_128(self) -> &'a mut W {
        self.variant(DBSIZEW::DESTINATION_BURST_128)
    }
    #[doc = "Destination burst size = 256"]
    #[inline]
    pub fn destination_burst_256(self) -> &'a mut W {
        self.variant(DBSIZEW::DESTINATION_BURST_256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SWIDTH`"]
pub enum SWIDTHW {
    #[doc = "Byte (8-bit)"]
    BYTE_8_BIT,
    #[doc = "Halfword (16-bit)"]
    HALFWORD_16_BIT,
    #[doc = "Word (32-bit)"]
    WORD_32_BIT,
}
impl SWIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SWIDTHW::BYTE_8_BIT => 0,
            SWIDTHW::HALFWORD_16_BIT => 1,
            SWIDTHW::WORD_32_BIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _SWIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Byte (8-bit)"]
    #[inline]
    pub fn byte_8_bit(self) -> &'a mut W {
        self.variant(SWIDTHW::BYTE_8_BIT)
    }
    #[doc = "Halfword (16-bit)"]
    #[inline]
    pub fn halfword_16_bit(self) -> &'a mut W {
        self.variant(SWIDTHW::HALFWORD_16_BIT)
    }
    #[doc = "Word (32-bit)"]
    #[inline]
    pub fn word_32_bit(self) -> &'a mut W {
        self.variant(SWIDTHW::WORD_32_BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DWIDTH`"]
pub enum DWIDTHW {
    #[doc = "Byte (8-bit)"]
    BYTE_8_BIT,
    #[doc = "Halfword (16-bit)"]
    HALFWORD_16_BIT,
    #[doc = "Word (32-bit)"]
    WORD_32_BIT,
}
impl DWIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DWIDTHW::BYTE_8_BIT => 0,
            DWIDTHW::HALFWORD_16_BIT => 1,
            DWIDTHW::WORD_32_BIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _DWIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DWIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Byte (8-bit)"]
    #[inline]
    pub fn byte_8_bit(self) -> &'a mut W {
        self.variant(DWIDTHW::BYTE_8_BIT)
    }
    #[doc = "Halfword (16-bit)"]
    #[inline]
    pub fn halfword_16_bit(self) -> &'a mut W {
        self.variant(DWIDTHW::HALFWORD_16_BIT)
    }
    #[doc = "Word (32-bit)"]
    #[inline]
    pub fn word_32_bit(self) -> &'a mut W {
        self.variant(DWIDTHW::WORD_32_BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S`"]
pub enum SW {
    #[doc = "AHB Master 0 selected for source transfer."]
    AHB_MASTER_0_SELECTE,
    #[doc = "AHB Master 1 selected for source transfer."]
    AHB_MASTER_1_SELECTE,
}
impl SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SW::AHB_MASTER_0_SELECTE => false,
            SW::AHB_MASTER_1_SELECTE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SW<'a> {
    w: &'a mut W,
}
impl<'a> _SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AHB Master 0 selected for source transfer."]
    #[inline]
    pub fn ahb_master_0_selecte(self) -> &'a mut W {
        self.variant(SW::AHB_MASTER_0_SELECTE)
    }
    #[doc = "AHB Master 1 selected for source transfer."]
    #[inline]
    pub fn ahb_master_1_selecte(self) -> &'a mut W {
        self.variant(SW::AHB_MASTER_1_SELECTE)
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
#[doc = "Values that can be written to the field `D`"]
pub enum DW {
    #[doc = "AHB Master 0 selected for destination transfer."]
    AHB_MASTER_0_SELECTE,
    #[doc = "AHB Master 1 selected for destination transfer."]
    AHB_MASTER_1_SELECTE,
}
impl DW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DW::AHB_MASTER_0_SELECTE => false,
            DW::AHB_MASTER_1_SELECTE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DW<'a> {
    w: &'a mut W,
}
impl<'a> _DW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AHB Master 0 selected for destination transfer."]
    #[inline]
    pub fn ahb_master_0_selecte(self) -> &'a mut W {
        self.variant(DW::AHB_MASTER_0_SELECTE)
    }
    #[doc = "AHB Master 1 selected for destination transfer."]
    #[inline]
    pub fn ahb_master_1_selecte(self) -> &'a mut W {
        self.variant(DW::AHB_MASTER_1_SELECTE)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SI`"]
pub enum SIW {
    #[doc = "The source address is not incremented after each transfer."]
    NOT_INCREMENT,
    #[doc = "The source address is incremented after each transfer."]
    INCREMENT,
}
impl SIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIW::NOT_INCREMENT => false,
            SIW::INCREMENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIW<'a> {
    w: &'a mut W,
}
impl<'a> _SIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The source address is not incremented after each transfer."]
    #[inline]
    pub fn not_increment(self) -> &'a mut W {
        self.variant(SIW::NOT_INCREMENT)
    }
    #[doc = "The source address is incremented after each transfer."]
    #[inline]
    pub fn increment(self) -> &'a mut W {
        self.variant(SIW::INCREMENT)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DI`"]
pub enum DIW {
    #[doc = "The destination address is not incremented after each transfer."]
    THE_DESTINATION_ADDR,
    #[doc = "The destination address is incremented after each transfer."]
    THE_DESTINATION_ADDR,
}
impl DIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIW::THE_DESTINATION_ADDR => false,
            DIW::THE_DESTINATION_ADDR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIW<'a> {
    w: &'a mut W,
}
impl<'a> _DIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The destination address is not incremented after each transfer."]
    #[inline]
    pub fn the_destination_addr(self) -> &'a mut W {
        self.variant(DIW::THE_DESTINATION_ADDR)
    }
    #[doc = "The destination address is incremented after each transfer."]
    #[inline]
    pub fn the_destination_addr(self) -> &'a mut W {
        self.variant(DIW::THE_DESTINATION_ADDR)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PROT1`"]
pub enum PROT1W {
    #[doc = "Access is in user mode"]
    ACCESS_IS_IN_USER_MO,
    #[doc = "Access is in privileged mode."]
    ACCESS_IS_IN_PRIVILE,
}
impl PROT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROT1W::ACCESS_IS_IN_USER_MO => false,
            PROT1W::ACCESS_IS_IN_PRIVILE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROT1W<'a> {
    w: &'a mut W,
}
impl<'a> _PROT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Access is in user mode"]
    #[inline]
    pub fn access_is_in_user_mo(self) -> &'a mut W {
        self.variant(PROT1W::ACCESS_IS_IN_USER_MO)
    }
    #[doc = "Access is in privileged mode."]
    #[inline]
    pub fn access_is_in_privile(self) -> &'a mut W {
        self.variant(PROT1W::ACCESS_IS_IN_PRIVILE)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PROT2`"]
pub enum PROT2W {
    #[doc = "Access is not bufferable."]
    ACCESS_IS_NOT_BUFFER,
    #[doc = "Access is bufferable."]
    ACCESS_IS_BUFFERABLE,
}
impl PROT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROT2W::ACCESS_IS_NOT_BUFFER => false,
            PROT2W::ACCESS_IS_BUFFERABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROT2W<'a> {
    w: &'a mut W,
}
impl<'a> _PROT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Access is not bufferable."]
    #[inline]
    pub fn access_is_not_buffer(self) -> &'a mut W {
        self.variant(PROT2W::ACCESS_IS_NOT_BUFFER)
    }
    #[doc = "Access is bufferable."]
    #[inline]
    pub fn access_is_bufferable(self) -> &'a mut W {
        self.variant(PROT2W::ACCESS_IS_BUFFERABLE)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PROT3`"]
pub enum PROT3W {
    #[doc = "Access is not cacheable."]
    ACCESS_IS_NOT_CACHEA,
    #[doc = "Access is cacheable."]
    ACCESS_IS_CACHEABLE_,
}
impl PROT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROT3W::ACCESS_IS_NOT_CACHEA => false,
            PROT3W::ACCESS_IS_CACHEABLE_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROT3W<'a> {
    w: &'a mut W,
}
impl<'a> _PROT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Access is not cacheable."]
    #[inline]
    pub fn access_is_not_cachea(self) -> &'a mut W {
        self.variant(PROT3W::ACCESS_IS_NOT_CACHEA)
    }
    #[doc = "Access is cacheable."]
    #[inline]
    pub fn access_is_cacheable_(self) -> &'a mut W {
        self.variant(PROT3W::ACCESS_IS_CACHEABLE_)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `I`"]
pub enum IW {
    #[doc = "The terminal count interrupt is disabled."]
    THE_TERMINAL_COUNT_I,
    #[doc = "The terminal count interrupt is enabled."]
    THE_TERMINAL_COUNT_I,
}
impl IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IW::THE_TERMINAL_COUNT_I => false,
            IW::THE_TERMINAL_COUNT_I => true,
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
            self.bit(variant._bits())
        }
    }
    #[doc = "The terminal count interrupt is disabled."]
    #[inline]
    pub fn the_terminal_count_i(self) -> &'a mut W {
        self.variant(IW::THE_TERMINAL_COUNT_I)
    }
    #[doc = "The terminal count interrupt is enabled."]
    #[inline]
    pub fn the_terminal_count_i(self) -> &'a mut W {
        self.variant(IW::THE_TERMINAL_COUNT_I)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:11 - Transfer size in byte. A write to this field sets the size of the transfer when the DMA Controller is the flow controller. The transfer size value must be set before the channel is enabled. Transfer size is updated as data transfers are completed. A read from this field indicates the number of transfers completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time that the software has processed the value read, the channel might have progressed. It is intended to be used only when a channel is enabled and then disabled. The transfer size value is not used if the DMA Controller is not the flow controller."]
    #[inline]
    pub fn transfersize(&self) -> TRANSFERSIZER {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TRANSFERSIZER { bits }
    }
    #[doc = "Bits 12:14 - Source burst size. Indicates the number of transfers that make up a source burst. This value must be set to the burst size of the source peripheral, or if the source is memory, to the memory boundary size (see Figure 3). The burst size is the amount of data that is transferred when the BREQ signal goes active in the source peripheral."]
    #[inline]
    pub fn sbsize(&self) -> SBSIZER {
        SBSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 15:17 - Destination burst size. Indicates the number of transfers that make up a destination burst transfer request. This value must be set to the burst size of the destination peripheral or, if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the BREQ signal goes active in the destination peripheral."]
    #[inline]
    pub fn dbsize(&self) -> DBSIZER {
        DBSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:20 - Source transfer width. Transfers wider than the AHB master bus width are illegal. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 0x3 to 0x7 - Reserved."]
    #[inline]
    pub fn swidth(&self) -> SWIDTHR {
        SWIDTHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:23 - Destination transfer width. Transfers wider than the AHB master bus width are not supported. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 0x3 to 0x7 - Reserved."]
    #[inline]
    pub fn dwidth(&self) -> DWIDTHR {
        DWIDTHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Source AHB master select:"]
    #[inline]
    pub fn s(&self) -> SR {
        SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Destination AHB master select: Only Master1 can access a peripheral. Master0 can only access memory."]
    #[inline]
    pub fn d(&self) -> DR {
        DR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Source increment:"]
    #[inline]
    pub fn si(&self) -> SIR {
        SIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Destination increment:"]
    #[inline]
    pub fn di(&self) -> DIR {
        DIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Indicates that the access is in user mode or privileged mode:"]
    #[inline]
    pub fn prot1(&self) -> PROT1R {
        PROT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Indicates that the access is bufferable or not bufferable:"]
    #[inline]
    pub fn prot2(&self) -> PROT2R {
        PROT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Indicates that the access is cacheable or not cacheable:"]
    #[inline]
    pub fn prot3(&self) -> PROT3R {
        PROT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Terminal count interrupt enable bit."]
    #[inline]
    pub fn i(&self) -> IR {
        IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:11 - Transfer size in byte. A write to this field sets the size of the transfer when the DMA Controller is the flow controller. The transfer size value must be set before the channel is enabled. Transfer size is updated as data transfers are completed. A read from this field indicates the number of transfers completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time that the software has processed the value read, the channel might have progressed. It is intended to be used only when a channel is enabled and then disabled. The transfer size value is not used if the DMA Controller is not the flow controller."]
    #[inline]
    pub fn transfersize(&mut self) -> _TRANSFERSIZEW {
        _TRANSFERSIZEW { w: self }
    }
    #[doc = "Bits 12:14 - Source burst size. Indicates the number of transfers that make up a source burst. This value must be set to the burst size of the source peripheral, or if the source is memory, to the memory boundary size (see Figure 3). The burst size is the amount of data that is transferred when the BREQ signal goes active in the source peripheral."]
    #[inline]
    pub fn sbsize(&mut self) -> _SBSIZEW {
        _SBSIZEW { w: self }
    }
    #[doc = "Bits 15:17 - Destination burst size. Indicates the number of transfers that make up a destination burst transfer request. This value must be set to the burst size of the destination peripheral or, if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the BREQ signal goes active in the destination peripheral."]
    #[inline]
    pub fn dbsize(&mut self) -> _DBSIZEW {
        _DBSIZEW { w: self }
    }
    #[doc = "Bits 18:20 - Source transfer width. Transfers wider than the AHB master bus width are illegal. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 0x3 to 0x7 - Reserved."]
    #[inline]
    pub fn swidth(&mut self) -> _SWIDTHW {
        _SWIDTHW { w: self }
    }
    #[doc = "Bits 21:23 - Destination transfer width. Transfers wider than the AHB master bus width are not supported. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 0x3 to 0x7 - Reserved."]
    #[inline]
    pub fn dwidth(&mut self) -> _DWIDTHW {
        _DWIDTHW { w: self }
    }
    #[doc = "Bit 24 - Source AHB master select:"]
    #[inline]
    pub fn s(&mut self) -> _SW {
        _SW { w: self }
    }
    #[doc = "Bit 25 - Destination AHB master select: Only Master1 can access a peripheral. Master0 can only access memory."]
    #[inline]
    pub fn d(&mut self) -> _DW {
        _DW { w: self }
    }
    #[doc = "Bit 26 - Source increment:"]
    #[inline]
    pub fn si(&mut self) -> _SIW {
        _SIW { w: self }
    }
    #[doc = "Bit 27 - Destination increment:"]
    #[inline]
    pub fn di(&mut self) -> _DIW {
        _DIW { w: self }
    }
    #[doc = "Bit 28 - Indicates that the access is in user mode or privileged mode:"]
    #[inline]
    pub fn prot1(&mut self) -> _PROT1W {
        _PROT1W { w: self }
    }
    #[doc = "Bit 29 - Indicates that the access is bufferable or not bufferable:"]
    #[inline]
    pub fn prot2(&mut self) -> _PROT2W {
        _PROT2W { w: self }
    }
    #[doc = "Bit 30 - Indicates that the access is cacheable or not cacheable:"]
    #[inline]
    pub fn prot3(&mut self) -> _PROT3W {
        _PROT3W { w: self }
    }
    #[doc = "Bit 31 - Terminal count interrupt enable bit."]
    #[inline]
    pub fn i(&mut self) -> _IW {
        _IW { w: self }
    }
}
