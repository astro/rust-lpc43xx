#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINTSEL0 {
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
pub struct INTPIN0R {
    bits: u8,
}
impl INTPIN0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PORTSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTSEL0R {
    #[doc = "GPIO Port 0"]
    GPIO_PORT_0,
    #[doc = "GPIO Port 1"]
    GPIO_PORT_1,
    #[doc = "GPIO Port 2"]
    GPIO_PORT_2,
    #[doc = "GPIO Port 3"]
    GPIO_PORT_3,
    #[doc = "GPIO Port 4"]
    GPIO_PORT_4,
    #[doc = "GPIO Port 5"]
    GPIO_PORT_5,
    #[doc = "GPIO Port 6"]
    GPIO_PORT_6,
    #[doc = "GPIO Port 7"]
    GPIO_PORT_7,
}
impl PORTSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORTSEL0R::GPIO_PORT_0 => 0,
            PORTSEL0R::GPIO_PORT_1 => 1,
            PORTSEL0R::GPIO_PORT_2 => 2,
            PORTSEL0R::GPIO_PORT_3 => 3,
            PORTSEL0R::GPIO_PORT_4 => 4,
            PORTSEL0R::GPIO_PORT_5 => 5,
            PORTSEL0R::GPIO_PORT_6 => 6,
            PORTSEL0R::GPIO_PORT_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORTSEL0R {
        match value {
            0 => PORTSEL0R::GPIO_PORT_0,
            1 => PORTSEL0R::GPIO_PORT_1,
            2 => PORTSEL0R::GPIO_PORT_2,
            3 => PORTSEL0R::GPIO_PORT_3,
            4 => PORTSEL0R::GPIO_PORT_4,
            5 => PORTSEL0R::GPIO_PORT_5,
            6 => PORTSEL0R::GPIO_PORT_6,
            7 => PORTSEL0R::GPIO_PORT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_0`"]
    #[inline]
    pub fn is_gpio_port_0(&self) -> bool {
        *self == PORTSEL0R::GPIO_PORT_0
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == PORTSEL0R::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_2`"]
    #[inline]
    pub fn is_gpio_port_2(&self) -> bool {
        *self == PORTSEL0R::GPIO_PORT_2
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_3`"]
    #[inline]
    pub fn is_gpio_port_3(&self) -> bool {
        *self == PORTSEL0R::GPIO_PORT_3
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_4`"]
    #[inline]
    pub fn is_gpio_port_4(&self) -> bool {
        *self == PORTSEL0R::GPIO_PORT_4
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_5`"]
    #[inline]
    pub fn is_gpio_port_5(&self) -> bool {
        *self == PORTSEL0R::GPIO_PORT_5
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_6`"]
    #[inline]
    pub fn is_gpio_port_6(&self) -> bool {
        *self == PORTSEL0R::GPIO_PORT_6
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_7`"]
    #[inline]
    pub fn is_gpio_port_7(&self) -> bool {
        *self == PORTSEL0R::GPIO_PORT_7
    }
}
#[doc = r" Value of the field"]
pub struct INTPIN1R {
    bits: u8,
}
impl INTPIN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PORTSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTSEL1R {
    #[doc = "GPIO Port 0"]
    GPIO_PORT_0,
    #[doc = "GPIO Port 1"]
    GPIO_PORT_1,
    #[doc = "GPIO Port 2"]
    GPIO_PORT_2,
    #[doc = "GPIO Port 3"]
    GPIO_PORT_3,
    #[doc = "GPIO Port 4"]
    GPIO_PORT_4,
    #[doc = "GPIO Port 5"]
    GPIO_PORT_5,
    #[doc = "GPIO Port 6"]
    GPIO_PORT_6,
    #[doc = "GPIO Port 7"]
    GPIO_PORT_7,
}
impl PORTSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORTSEL1R::GPIO_PORT_0 => 0,
            PORTSEL1R::GPIO_PORT_1 => 1,
            PORTSEL1R::GPIO_PORT_2 => 2,
            PORTSEL1R::GPIO_PORT_3 => 3,
            PORTSEL1R::GPIO_PORT_4 => 4,
            PORTSEL1R::GPIO_PORT_5 => 5,
            PORTSEL1R::GPIO_PORT_6 => 6,
            PORTSEL1R::GPIO_PORT_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORTSEL1R {
        match value {
            0 => PORTSEL1R::GPIO_PORT_0,
            1 => PORTSEL1R::GPIO_PORT_1,
            2 => PORTSEL1R::GPIO_PORT_2,
            3 => PORTSEL1R::GPIO_PORT_3,
            4 => PORTSEL1R::GPIO_PORT_4,
            5 => PORTSEL1R::GPIO_PORT_5,
            6 => PORTSEL1R::GPIO_PORT_6,
            7 => PORTSEL1R::GPIO_PORT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_0`"]
    #[inline]
    pub fn is_gpio_port_0(&self) -> bool {
        *self == PORTSEL1R::GPIO_PORT_0
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == PORTSEL1R::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_2`"]
    #[inline]
    pub fn is_gpio_port_2(&self) -> bool {
        *self == PORTSEL1R::GPIO_PORT_2
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_3`"]
    #[inline]
    pub fn is_gpio_port_3(&self) -> bool {
        *self == PORTSEL1R::GPIO_PORT_3
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_4`"]
    #[inline]
    pub fn is_gpio_port_4(&self) -> bool {
        *self == PORTSEL1R::GPIO_PORT_4
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_5`"]
    #[inline]
    pub fn is_gpio_port_5(&self) -> bool {
        *self == PORTSEL1R::GPIO_PORT_5
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_6`"]
    #[inline]
    pub fn is_gpio_port_6(&self) -> bool {
        *self == PORTSEL1R::GPIO_PORT_6
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_7`"]
    #[inline]
    pub fn is_gpio_port_7(&self) -> bool {
        *self == PORTSEL1R::GPIO_PORT_7
    }
}
#[doc = r" Value of the field"]
pub struct INTPIN2R {
    bits: u8,
}
impl INTPIN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PORTSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTSEL2R {
    #[doc = "GPIO Port 0"]
    GPIO_PORT_0,
    #[doc = "GPIO Port 1"]
    GPIO_PORT_1,
    #[doc = "GPIO Port 2"]
    GPIO_PORT_2,
    #[doc = "GPIO Port 3"]
    GPIO_PORT_3,
    #[doc = "GPIO Port 4"]
    GPIO_PORT_4,
    #[doc = "GPIO Port 5"]
    GPIO_PORT_5,
    #[doc = "GPIO Port 6"]
    GPIO_PORT_6,
    #[doc = "GPIO Port 7"]
    GPIO_PORT_7,
}
impl PORTSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORTSEL2R::GPIO_PORT_0 => 0,
            PORTSEL2R::GPIO_PORT_1 => 1,
            PORTSEL2R::GPIO_PORT_2 => 2,
            PORTSEL2R::GPIO_PORT_3 => 3,
            PORTSEL2R::GPIO_PORT_4 => 4,
            PORTSEL2R::GPIO_PORT_5 => 5,
            PORTSEL2R::GPIO_PORT_6 => 6,
            PORTSEL2R::GPIO_PORT_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORTSEL2R {
        match value {
            0 => PORTSEL2R::GPIO_PORT_0,
            1 => PORTSEL2R::GPIO_PORT_1,
            2 => PORTSEL2R::GPIO_PORT_2,
            3 => PORTSEL2R::GPIO_PORT_3,
            4 => PORTSEL2R::GPIO_PORT_4,
            5 => PORTSEL2R::GPIO_PORT_5,
            6 => PORTSEL2R::GPIO_PORT_6,
            7 => PORTSEL2R::GPIO_PORT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_0`"]
    #[inline]
    pub fn is_gpio_port_0(&self) -> bool {
        *self == PORTSEL2R::GPIO_PORT_0
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == PORTSEL2R::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_2`"]
    #[inline]
    pub fn is_gpio_port_2(&self) -> bool {
        *self == PORTSEL2R::GPIO_PORT_2
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_3`"]
    #[inline]
    pub fn is_gpio_port_3(&self) -> bool {
        *self == PORTSEL2R::GPIO_PORT_3
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_4`"]
    #[inline]
    pub fn is_gpio_port_4(&self) -> bool {
        *self == PORTSEL2R::GPIO_PORT_4
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_5`"]
    #[inline]
    pub fn is_gpio_port_5(&self) -> bool {
        *self == PORTSEL2R::GPIO_PORT_5
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_6`"]
    #[inline]
    pub fn is_gpio_port_6(&self) -> bool {
        *self == PORTSEL2R::GPIO_PORT_6
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_7`"]
    #[inline]
    pub fn is_gpio_port_7(&self) -> bool {
        *self == PORTSEL2R::GPIO_PORT_7
    }
}
#[doc = r" Value of the field"]
pub struct INTPIN3R {
    bits: u8,
}
impl INTPIN3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PORTSEL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTSEL3R {
    #[doc = "GPIO Port 0"]
    GPIO_PORT_0,
    #[doc = "GPIO Port 1"]
    GPIO_PORT_1,
    #[doc = "GPIO Port 2"]
    GPIO_PORT_2,
    #[doc = "GPIO Port 3"]
    GPIO_PORT_3,
    #[doc = "GPIO Port 4"]
    GPIO_PORT_4,
    #[doc = "GPIO Port 5"]
    GPIO_PORT_5,
    #[doc = "GPIO Port 6"]
    GPIO_PORT_6,
    #[doc = "GPIO Port 7"]
    GPIO_PORT_7,
}
impl PORTSEL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORTSEL3R::GPIO_PORT_0 => 0,
            PORTSEL3R::GPIO_PORT_1 => 1,
            PORTSEL3R::GPIO_PORT_2 => 2,
            PORTSEL3R::GPIO_PORT_3 => 3,
            PORTSEL3R::GPIO_PORT_4 => 4,
            PORTSEL3R::GPIO_PORT_5 => 5,
            PORTSEL3R::GPIO_PORT_6 => 6,
            PORTSEL3R::GPIO_PORT_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORTSEL3R {
        match value {
            0 => PORTSEL3R::GPIO_PORT_0,
            1 => PORTSEL3R::GPIO_PORT_1,
            2 => PORTSEL3R::GPIO_PORT_2,
            3 => PORTSEL3R::GPIO_PORT_3,
            4 => PORTSEL3R::GPIO_PORT_4,
            5 => PORTSEL3R::GPIO_PORT_5,
            6 => PORTSEL3R::GPIO_PORT_6,
            7 => PORTSEL3R::GPIO_PORT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_0`"]
    #[inline]
    pub fn is_gpio_port_0(&self) -> bool {
        *self == PORTSEL3R::GPIO_PORT_0
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == PORTSEL3R::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_2`"]
    #[inline]
    pub fn is_gpio_port_2(&self) -> bool {
        *self == PORTSEL3R::GPIO_PORT_2
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_3`"]
    #[inline]
    pub fn is_gpio_port_3(&self) -> bool {
        *self == PORTSEL3R::GPIO_PORT_3
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_4`"]
    #[inline]
    pub fn is_gpio_port_4(&self) -> bool {
        *self == PORTSEL3R::GPIO_PORT_4
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_5`"]
    #[inline]
    pub fn is_gpio_port_5(&self) -> bool {
        *self == PORTSEL3R::GPIO_PORT_5
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_6`"]
    #[inline]
    pub fn is_gpio_port_6(&self) -> bool {
        *self == PORTSEL3R::GPIO_PORT_6
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_7`"]
    #[inline]
    pub fn is_gpio_port_7(&self) -> bool {
        *self == PORTSEL3R::GPIO_PORT_7
    }
}
#[doc = r" Proxy"]
pub struct _INTPIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _INTPIN0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PORTSEL0`"]
pub enum PORTSEL0W {
    #[doc = "GPIO Port 0"]
    GPIO_PORT_0,
    #[doc = "GPIO Port 1"]
    GPIO_PORT_1,
    #[doc = "GPIO Port 2"]
    GPIO_PORT_2,
    #[doc = "GPIO Port 3"]
    GPIO_PORT_3,
    #[doc = "GPIO Port 4"]
    GPIO_PORT_4,
    #[doc = "GPIO Port 5"]
    GPIO_PORT_5,
    #[doc = "GPIO Port 6"]
    GPIO_PORT_6,
    #[doc = "GPIO Port 7"]
    GPIO_PORT_7,
}
impl PORTSEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PORTSEL0W::GPIO_PORT_0 => 0,
            PORTSEL0W::GPIO_PORT_1 => 1,
            PORTSEL0W::GPIO_PORT_2 => 2,
            PORTSEL0W::GPIO_PORT_3 => 3,
            PORTSEL0W::GPIO_PORT_4 => 4,
            PORTSEL0W::GPIO_PORT_5 => 5,
            PORTSEL0W::GPIO_PORT_6 => 6,
            PORTSEL0W::GPIO_PORT_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PORTSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _PORTSEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PORTSEL0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO Port 0"]
    #[inline]
    pub fn gpio_port_0(self) -> &'a mut W {
        self.variant(PORTSEL0W::GPIO_PORT_0)
    }
    #[doc = "GPIO Port 1"]
    #[inline]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(PORTSEL0W::GPIO_PORT_1)
    }
    #[doc = "GPIO Port 2"]
    #[inline]
    pub fn gpio_port_2(self) -> &'a mut W {
        self.variant(PORTSEL0W::GPIO_PORT_2)
    }
    #[doc = "GPIO Port 3"]
    #[inline]
    pub fn gpio_port_3(self) -> &'a mut W {
        self.variant(PORTSEL0W::GPIO_PORT_3)
    }
    #[doc = "GPIO Port 4"]
    #[inline]
    pub fn gpio_port_4(self) -> &'a mut W {
        self.variant(PORTSEL0W::GPIO_PORT_4)
    }
    #[doc = "GPIO Port 5"]
    #[inline]
    pub fn gpio_port_5(self) -> &'a mut W {
        self.variant(PORTSEL0W::GPIO_PORT_5)
    }
    #[doc = "GPIO Port 6"]
    #[inline]
    pub fn gpio_port_6(self) -> &'a mut W {
        self.variant(PORTSEL0W::GPIO_PORT_6)
    }
    #[doc = "GPIO Port 7"]
    #[inline]
    pub fn gpio_port_7(self) -> &'a mut W {
        self.variant(PORTSEL0W::GPIO_PORT_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTPIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _INTPIN1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PORTSEL1`"]
pub enum PORTSEL1W {
    #[doc = "GPIO Port 0"]
    GPIO_PORT_0,
    #[doc = "GPIO Port 1"]
    GPIO_PORT_1,
    #[doc = "GPIO Port 2"]
    GPIO_PORT_2,
    #[doc = "GPIO Port 3"]
    GPIO_PORT_3,
    #[doc = "GPIO Port 4"]
    GPIO_PORT_4,
    #[doc = "GPIO Port 5"]
    GPIO_PORT_5,
    #[doc = "GPIO Port 6"]
    GPIO_PORT_6,
    #[doc = "GPIO Port 7"]
    GPIO_PORT_7,
}
impl PORTSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PORTSEL1W::GPIO_PORT_0 => 0,
            PORTSEL1W::GPIO_PORT_1 => 1,
            PORTSEL1W::GPIO_PORT_2 => 2,
            PORTSEL1W::GPIO_PORT_3 => 3,
            PORTSEL1W::GPIO_PORT_4 => 4,
            PORTSEL1W::GPIO_PORT_5 => 5,
            PORTSEL1W::GPIO_PORT_6 => 6,
            PORTSEL1W::GPIO_PORT_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PORTSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _PORTSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PORTSEL1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO Port 0"]
    #[inline]
    pub fn gpio_port_0(self) -> &'a mut W {
        self.variant(PORTSEL1W::GPIO_PORT_0)
    }
    #[doc = "GPIO Port 1"]
    #[inline]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(PORTSEL1W::GPIO_PORT_1)
    }
    #[doc = "GPIO Port 2"]
    #[inline]
    pub fn gpio_port_2(self) -> &'a mut W {
        self.variant(PORTSEL1W::GPIO_PORT_2)
    }
    #[doc = "GPIO Port 3"]
    #[inline]
    pub fn gpio_port_3(self) -> &'a mut W {
        self.variant(PORTSEL1W::GPIO_PORT_3)
    }
    #[doc = "GPIO Port 4"]
    #[inline]
    pub fn gpio_port_4(self) -> &'a mut W {
        self.variant(PORTSEL1W::GPIO_PORT_4)
    }
    #[doc = "GPIO Port 5"]
    #[inline]
    pub fn gpio_port_5(self) -> &'a mut W {
        self.variant(PORTSEL1W::GPIO_PORT_5)
    }
    #[doc = "GPIO Port 6"]
    #[inline]
    pub fn gpio_port_6(self) -> &'a mut W {
        self.variant(PORTSEL1W::GPIO_PORT_6)
    }
    #[doc = "GPIO Port 7"]
    #[inline]
    pub fn gpio_port_7(self) -> &'a mut W {
        self.variant(PORTSEL1W::GPIO_PORT_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTPIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _INTPIN2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PORTSEL2`"]
pub enum PORTSEL2W {
    #[doc = "GPIO Port 0"]
    GPIO_PORT_0,
    #[doc = "GPIO Port 1"]
    GPIO_PORT_1,
    #[doc = "GPIO Port 2"]
    GPIO_PORT_2,
    #[doc = "GPIO Port 3"]
    GPIO_PORT_3,
    #[doc = "GPIO Port 4"]
    GPIO_PORT_4,
    #[doc = "GPIO Port 5"]
    GPIO_PORT_5,
    #[doc = "GPIO Port 6"]
    GPIO_PORT_6,
    #[doc = "GPIO Port 7"]
    GPIO_PORT_7,
}
impl PORTSEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PORTSEL2W::GPIO_PORT_0 => 0,
            PORTSEL2W::GPIO_PORT_1 => 1,
            PORTSEL2W::GPIO_PORT_2 => 2,
            PORTSEL2W::GPIO_PORT_3 => 3,
            PORTSEL2W::GPIO_PORT_4 => 4,
            PORTSEL2W::GPIO_PORT_5 => 5,
            PORTSEL2W::GPIO_PORT_6 => 6,
            PORTSEL2W::GPIO_PORT_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PORTSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _PORTSEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PORTSEL2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO Port 0"]
    #[inline]
    pub fn gpio_port_0(self) -> &'a mut W {
        self.variant(PORTSEL2W::GPIO_PORT_0)
    }
    #[doc = "GPIO Port 1"]
    #[inline]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(PORTSEL2W::GPIO_PORT_1)
    }
    #[doc = "GPIO Port 2"]
    #[inline]
    pub fn gpio_port_2(self) -> &'a mut W {
        self.variant(PORTSEL2W::GPIO_PORT_2)
    }
    #[doc = "GPIO Port 3"]
    #[inline]
    pub fn gpio_port_3(self) -> &'a mut W {
        self.variant(PORTSEL2W::GPIO_PORT_3)
    }
    #[doc = "GPIO Port 4"]
    #[inline]
    pub fn gpio_port_4(self) -> &'a mut W {
        self.variant(PORTSEL2W::GPIO_PORT_4)
    }
    #[doc = "GPIO Port 5"]
    #[inline]
    pub fn gpio_port_5(self) -> &'a mut W {
        self.variant(PORTSEL2W::GPIO_PORT_5)
    }
    #[doc = "GPIO Port 6"]
    #[inline]
    pub fn gpio_port_6(self) -> &'a mut W {
        self.variant(PORTSEL2W::GPIO_PORT_6)
    }
    #[doc = "GPIO Port 7"]
    #[inline]
    pub fn gpio_port_7(self) -> &'a mut W {
        self.variant(PORTSEL2W::GPIO_PORT_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTPIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _INTPIN3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PORTSEL3`"]
pub enum PORTSEL3W {
    #[doc = "GPIO Port 0"]
    GPIO_PORT_0,
    #[doc = "GPIO Port 1"]
    GPIO_PORT_1,
    #[doc = "GPIO Port 2"]
    GPIO_PORT_2,
    #[doc = "GPIO Port 3"]
    GPIO_PORT_3,
    #[doc = "GPIO Port 4"]
    GPIO_PORT_4,
    #[doc = "GPIO Port 5"]
    GPIO_PORT_5,
    #[doc = "GPIO Port 6"]
    GPIO_PORT_6,
    #[doc = "GPIO Port 7"]
    GPIO_PORT_7,
}
impl PORTSEL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PORTSEL3W::GPIO_PORT_0 => 0,
            PORTSEL3W::GPIO_PORT_1 => 1,
            PORTSEL3W::GPIO_PORT_2 => 2,
            PORTSEL3W::GPIO_PORT_3 => 3,
            PORTSEL3W::GPIO_PORT_4 => 4,
            PORTSEL3W::GPIO_PORT_5 => 5,
            PORTSEL3W::GPIO_PORT_6 => 6,
            PORTSEL3W::GPIO_PORT_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PORTSEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _PORTSEL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PORTSEL3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO Port 0"]
    #[inline]
    pub fn gpio_port_0(self) -> &'a mut W {
        self.variant(PORTSEL3W::GPIO_PORT_0)
    }
    #[doc = "GPIO Port 1"]
    #[inline]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(PORTSEL3W::GPIO_PORT_1)
    }
    #[doc = "GPIO Port 2"]
    #[inline]
    pub fn gpio_port_2(self) -> &'a mut W {
        self.variant(PORTSEL3W::GPIO_PORT_2)
    }
    #[doc = "GPIO Port 3"]
    #[inline]
    pub fn gpio_port_3(self) -> &'a mut W {
        self.variant(PORTSEL3W::GPIO_PORT_3)
    }
    #[doc = "GPIO Port 4"]
    #[inline]
    pub fn gpio_port_4(self) -> &'a mut W {
        self.variant(PORTSEL3W::GPIO_PORT_4)
    }
    #[doc = "GPIO Port 5"]
    #[inline]
    pub fn gpio_port_5(self) -> &'a mut W {
        self.variant(PORTSEL3W::GPIO_PORT_5)
    }
    #[doc = "GPIO Port 6"]
    #[inline]
    pub fn gpio_port_6(self) -> &'a mut W {
        self.variant(PORTSEL3W::GPIO_PORT_6)
    }
    #[doc = "GPIO Port 7"]
    #[inline]
    pub fn gpio_port_7(self) -> &'a mut W {
        self.variant(PORTSEL3W::GPIO_PORT_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:4 - Pint interrupt 0: Select the pin number within the GPIO port selected by the PORTSEL0 bit in this register."]
    #[inline]
    pub fn intpin0(&self) -> INTPIN0R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPIN0R { bits }
    }
    #[doc = "Bits 5:7 - Pin interrupt 0: Select the port for the pin number to be selected in the INTPIN0 bits of this register."]
    #[inline]
    pub fn portsel0(&self) -> PORTSEL0R {
        PORTSEL0R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:12 - Pint interrupt 1: Select the pin number within the GPIO port selected by the PORTSEL1 bit in this register."]
    #[inline]
    pub fn intpin1(&self) -> INTPIN1R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPIN1R { bits }
    }
    #[doc = "Bits 13:15 - Pin interrupt 1: Select the port for the pin number to be selected in the INTPIN1 bits of this register."]
    #[inline]
    pub fn portsel1(&self) -> PORTSEL1R {
        PORTSEL1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:20 - Pint interrupt 2: Select the pin number within the GPIO port selected by the PORTSEL2 bit in this register."]
    #[inline]
    pub fn intpin2(&self) -> INTPIN2R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPIN2R { bits }
    }
    #[doc = "Bits 21:23 - Pin interrupt 2: Select the port for the pin number to be selected in the INTPIN2 bits of this register."]
    #[inline]
    pub fn portsel2(&self) -> PORTSEL2R {
        PORTSEL2R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:28 - Pint interrupt 3: Select the pin number within the GPIO port selected by the PORTSEL3 bit in this register."]
    #[inline]
    pub fn intpin3(&self) -> INTPIN3R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPIN3R { bits }
    }
    #[doc = "Bits 29:31 - Pin interrupt 3: Select the port for the pin number to be selected in the INTPIN3 bits of this register."]
    #[inline]
    pub fn portsel3(&self) -> PORTSEL3R {
        PORTSEL3R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:4 - Pint interrupt 0: Select the pin number within the GPIO port selected by the PORTSEL0 bit in this register."]
    #[inline]
    pub fn intpin0(&mut self) -> _INTPIN0W {
        _INTPIN0W { w: self }
    }
    #[doc = "Bits 5:7 - Pin interrupt 0: Select the port for the pin number to be selected in the INTPIN0 bits of this register."]
    #[inline]
    pub fn portsel0(&mut self) -> _PORTSEL0W {
        _PORTSEL0W { w: self }
    }
    #[doc = "Bits 8:12 - Pint interrupt 1: Select the pin number within the GPIO port selected by the PORTSEL1 bit in this register."]
    #[inline]
    pub fn intpin1(&mut self) -> _INTPIN1W {
        _INTPIN1W { w: self }
    }
    #[doc = "Bits 13:15 - Pin interrupt 1: Select the port for the pin number to be selected in the INTPIN1 bits of this register."]
    #[inline]
    pub fn portsel1(&mut self) -> _PORTSEL1W {
        _PORTSEL1W { w: self }
    }
    #[doc = "Bits 16:20 - Pint interrupt 2: Select the pin number within the GPIO port selected by the PORTSEL2 bit in this register."]
    #[inline]
    pub fn intpin2(&mut self) -> _INTPIN2W {
        _INTPIN2W { w: self }
    }
    #[doc = "Bits 21:23 - Pin interrupt 2: Select the port for the pin number to be selected in the INTPIN2 bits of this register."]
    #[inline]
    pub fn portsel2(&mut self) -> _PORTSEL2W {
        _PORTSEL2W { w: self }
    }
    #[doc = "Bits 24:28 - Pint interrupt 3: Select the pin number within the GPIO port selected by the PORTSEL3 bit in this register."]
    #[inline]
    pub fn intpin3(&mut self) -> _INTPIN3W {
        _INTPIN3W { w: self }
    }
    #[doc = "Bits 29:31 - Pin interrupt 3: Select the port for the pin number to be selected in the INTPIN3 bits of this register."]
    #[inline]
    pub fn portsel3(&mut self) -> _PORTSEL3W {
        _PORTSEL3W { w: self }
    }
}
