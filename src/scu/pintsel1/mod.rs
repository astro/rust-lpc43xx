#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINTSEL1 {
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
pub struct INTPIN4R {
    bits: u8,
}
impl INTPIN4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PORTSEL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTSEL4R {
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
impl PORTSEL4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORTSEL4R::GPIO_PORT_0 => 0,
            PORTSEL4R::GPIO_PORT_1 => 1,
            PORTSEL4R::GPIO_PORT_2 => 2,
            PORTSEL4R::GPIO_PORT_3 => 3,
            PORTSEL4R::GPIO_PORT_4 => 4,
            PORTSEL4R::GPIO_PORT_5 => 5,
            PORTSEL4R::GPIO_PORT_6 => 6,
            PORTSEL4R::GPIO_PORT_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORTSEL4R {
        match value {
            0 => PORTSEL4R::GPIO_PORT_0,
            1 => PORTSEL4R::GPIO_PORT_1,
            2 => PORTSEL4R::GPIO_PORT_2,
            3 => PORTSEL4R::GPIO_PORT_3,
            4 => PORTSEL4R::GPIO_PORT_4,
            5 => PORTSEL4R::GPIO_PORT_5,
            6 => PORTSEL4R::GPIO_PORT_6,
            7 => PORTSEL4R::GPIO_PORT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_0`"]
    #[inline]
    pub fn is_gpio_port_0(&self) -> bool {
        *self == PORTSEL4R::GPIO_PORT_0
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == PORTSEL4R::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_2`"]
    #[inline]
    pub fn is_gpio_port_2(&self) -> bool {
        *self == PORTSEL4R::GPIO_PORT_2
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_3`"]
    #[inline]
    pub fn is_gpio_port_3(&self) -> bool {
        *self == PORTSEL4R::GPIO_PORT_3
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_4`"]
    #[inline]
    pub fn is_gpio_port_4(&self) -> bool {
        *self == PORTSEL4R::GPIO_PORT_4
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_5`"]
    #[inline]
    pub fn is_gpio_port_5(&self) -> bool {
        *self == PORTSEL4R::GPIO_PORT_5
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_6`"]
    #[inline]
    pub fn is_gpio_port_6(&self) -> bool {
        *self == PORTSEL4R::GPIO_PORT_6
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_7`"]
    #[inline]
    pub fn is_gpio_port_7(&self) -> bool {
        *self == PORTSEL4R::GPIO_PORT_7
    }
}
#[doc = r" Value of the field"]
pub struct INTPIN5R {
    bits: u8,
}
impl INTPIN5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PORTSEL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTSEL5R {
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
impl PORTSEL5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORTSEL5R::GPIO_PORT_0 => 0,
            PORTSEL5R::GPIO_PORT_1 => 1,
            PORTSEL5R::GPIO_PORT_2 => 2,
            PORTSEL5R::GPIO_PORT_3 => 3,
            PORTSEL5R::GPIO_PORT_4 => 4,
            PORTSEL5R::GPIO_PORT_5 => 5,
            PORTSEL5R::GPIO_PORT_6 => 6,
            PORTSEL5R::GPIO_PORT_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORTSEL5R {
        match value {
            0 => PORTSEL5R::GPIO_PORT_0,
            1 => PORTSEL5R::GPIO_PORT_1,
            2 => PORTSEL5R::GPIO_PORT_2,
            3 => PORTSEL5R::GPIO_PORT_3,
            4 => PORTSEL5R::GPIO_PORT_4,
            5 => PORTSEL5R::GPIO_PORT_5,
            6 => PORTSEL5R::GPIO_PORT_6,
            7 => PORTSEL5R::GPIO_PORT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_0`"]
    #[inline]
    pub fn is_gpio_port_0(&self) -> bool {
        *self == PORTSEL5R::GPIO_PORT_0
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == PORTSEL5R::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_2`"]
    #[inline]
    pub fn is_gpio_port_2(&self) -> bool {
        *self == PORTSEL5R::GPIO_PORT_2
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_3`"]
    #[inline]
    pub fn is_gpio_port_3(&self) -> bool {
        *self == PORTSEL5R::GPIO_PORT_3
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_4`"]
    #[inline]
    pub fn is_gpio_port_4(&self) -> bool {
        *self == PORTSEL5R::GPIO_PORT_4
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_5`"]
    #[inline]
    pub fn is_gpio_port_5(&self) -> bool {
        *self == PORTSEL5R::GPIO_PORT_5
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_6`"]
    #[inline]
    pub fn is_gpio_port_6(&self) -> bool {
        *self == PORTSEL5R::GPIO_PORT_6
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_7`"]
    #[inline]
    pub fn is_gpio_port_7(&self) -> bool {
        *self == PORTSEL5R::GPIO_PORT_7
    }
}
#[doc = r" Value of the field"]
pub struct INTPIN6R {
    bits: u8,
}
impl INTPIN6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PORTSEL6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTSEL6R {
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
impl PORTSEL6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORTSEL6R::GPIO_PORT_0 => 0,
            PORTSEL6R::GPIO_PORT_1 => 1,
            PORTSEL6R::GPIO_PORT_2 => 2,
            PORTSEL6R::GPIO_PORT_3 => 3,
            PORTSEL6R::GPIO_PORT_4 => 4,
            PORTSEL6R::GPIO_PORT_5 => 5,
            PORTSEL6R::GPIO_PORT_6 => 6,
            PORTSEL6R::GPIO_PORT_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORTSEL6R {
        match value {
            0 => PORTSEL6R::GPIO_PORT_0,
            1 => PORTSEL6R::GPIO_PORT_1,
            2 => PORTSEL6R::GPIO_PORT_2,
            3 => PORTSEL6R::GPIO_PORT_3,
            4 => PORTSEL6R::GPIO_PORT_4,
            5 => PORTSEL6R::GPIO_PORT_5,
            6 => PORTSEL6R::GPIO_PORT_6,
            7 => PORTSEL6R::GPIO_PORT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_0`"]
    #[inline]
    pub fn is_gpio_port_0(&self) -> bool {
        *self == PORTSEL6R::GPIO_PORT_0
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == PORTSEL6R::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_2`"]
    #[inline]
    pub fn is_gpio_port_2(&self) -> bool {
        *self == PORTSEL6R::GPIO_PORT_2
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_3`"]
    #[inline]
    pub fn is_gpio_port_3(&self) -> bool {
        *self == PORTSEL6R::GPIO_PORT_3
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_4`"]
    #[inline]
    pub fn is_gpio_port_4(&self) -> bool {
        *self == PORTSEL6R::GPIO_PORT_4
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_5`"]
    #[inline]
    pub fn is_gpio_port_5(&self) -> bool {
        *self == PORTSEL6R::GPIO_PORT_5
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_6`"]
    #[inline]
    pub fn is_gpio_port_6(&self) -> bool {
        *self == PORTSEL6R::GPIO_PORT_6
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_7`"]
    #[inline]
    pub fn is_gpio_port_7(&self) -> bool {
        *self == PORTSEL6R::GPIO_PORT_7
    }
}
#[doc = r" Value of the field"]
pub struct INTPIN7R {
    bits: u8,
}
impl INTPIN7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PORTSEL7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTSEL7R {
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
impl PORTSEL7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORTSEL7R::GPIO_PORT_0 => 0,
            PORTSEL7R::GPIO_PORT_1 => 1,
            PORTSEL7R::GPIO_PORT_2 => 2,
            PORTSEL7R::GPIO_PORT_3 => 3,
            PORTSEL7R::GPIO_PORT_4 => 4,
            PORTSEL7R::GPIO_PORT_5 => 5,
            PORTSEL7R::GPIO_PORT_6 => 6,
            PORTSEL7R::GPIO_PORT_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORTSEL7R {
        match value {
            0 => PORTSEL7R::GPIO_PORT_0,
            1 => PORTSEL7R::GPIO_PORT_1,
            2 => PORTSEL7R::GPIO_PORT_2,
            3 => PORTSEL7R::GPIO_PORT_3,
            4 => PORTSEL7R::GPIO_PORT_4,
            5 => PORTSEL7R::GPIO_PORT_5,
            6 => PORTSEL7R::GPIO_PORT_6,
            7 => PORTSEL7R::GPIO_PORT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_0`"]
    #[inline]
    pub fn is_gpio_port_0(&self) -> bool {
        *self == PORTSEL7R::GPIO_PORT_0
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_1`"]
    #[inline]
    pub fn is_gpio_port_1(&self) -> bool {
        *self == PORTSEL7R::GPIO_PORT_1
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_2`"]
    #[inline]
    pub fn is_gpio_port_2(&self) -> bool {
        *self == PORTSEL7R::GPIO_PORT_2
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_3`"]
    #[inline]
    pub fn is_gpio_port_3(&self) -> bool {
        *self == PORTSEL7R::GPIO_PORT_3
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_4`"]
    #[inline]
    pub fn is_gpio_port_4(&self) -> bool {
        *self == PORTSEL7R::GPIO_PORT_4
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_5`"]
    #[inline]
    pub fn is_gpio_port_5(&self) -> bool {
        *self == PORTSEL7R::GPIO_PORT_5
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_6`"]
    #[inline]
    pub fn is_gpio_port_6(&self) -> bool {
        *self == PORTSEL7R::GPIO_PORT_6
    }
    #[doc = "Checks if the value of the field is `GPIO_PORT_7`"]
    #[inline]
    pub fn is_gpio_port_7(&self) -> bool {
        *self == PORTSEL7R::GPIO_PORT_7
    }
}
#[doc = r" Proxy"]
pub struct _INTPIN4W<'a> {
    w: &'a mut W,
}
impl<'a> _INTPIN4W<'a> {
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
#[doc = "Values that can be written to the field `PORTSEL4`"]
pub enum PORTSEL4W {
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
impl PORTSEL4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PORTSEL4W::GPIO_PORT_0 => 0,
            PORTSEL4W::GPIO_PORT_1 => 1,
            PORTSEL4W::GPIO_PORT_2 => 2,
            PORTSEL4W::GPIO_PORT_3 => 3,
            PORTSEL4W::GPIO_PORT_4 => 4,
            PORTSEL4W::GPIO_PORT_5 => 5,
            PORTSEL4W::GPIO_PORT_6 => 6,
            PORTSEL4W::GPIO_PORT_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PORTSEL4W<'a> {
    w: &'a mut W,
}
impl<'a> _PORTSEL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PORTSEL4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO Port 0"]
    #[inline]
    pub fn gpio_port_0(self) -> &'a mut W {
        self.variant(PORTSEL4W::GPIO_PORT_0)
    }
    #[doc = "GPIO Port 1"]
    #[inline]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(PORTSEL4W::GPIO_PORT_1)
    }
    #[doc = "GPIO Port 2"]
    #[inline]
    pub fn gpio_port_2(self) -> &'a mut W {
        self.variant(PORTSEL4W::GPIO_PORT_2)
    }
    #[doc = "GPIO Port 3"]
    #[inline]
    pub fn gpio_port_3(self) -> &'a mut W {
        self.variant(PORTSEL4W::GPIO_PORT_3)
    }
    #[doc = "GPIO Port 4"]
    #[inline]
    pub fn gpio_port_4(self) -> &'a mut W {
        self.variant(PORTSEL4W::GPIO_PORT_4)
    }
    #[doc = "GPIO Port 5"]
    #[inline]
    pub fn gpio_port_5(self) -> &'a mut W {
        self.variant(PORTSEL4W::GPIO_PORT_5)
    }
    #[doc = "GPIO Port 6"]
    #[inline]
    pub fn gpio_port_6(self) -> &'a mut W {
        self.variant(PORTSEL4W::GPIO_PORT_6)
    }
    #[doc = "GPIO Port 7"]
    #[inline]
    pub fn gpio_port_7(self) -> &'a mut W {
        self.variant(PORTSEL4W::GPIO_PORT_7)
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
pub struct _INTPIN5W<'a> {
    w: &'a mut W,
}
impl<'a> _INTPIN5W<'a> {
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
#[doc = "Values that can be written to the field `PORTSEL5`"]
pub enum PORTSEL5W {
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
impl PORTSEL5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PORTSEL5W::GPIO_PORT_0 => 0,
            PORTSEL5W::GPIO_PORT_1 => 1,
            PORTSEL5W::GPIO_PORT_2 => 2,
            PORTSEL5W::GPIO_PORT_3 => 3,
            PORTSEL5W::GPIO_PORT_4 => 4,
            PORTSEL5W::GPIO_PORT_5 => 5,
            PORTSEL5W::GPIO_PORT_6 => 6,
            PORTSEL5W::GPIO_PORT_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PORTSEL5W<'a> {
    w: &'a mut W,
}
impl<'a> _PORTSEL5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PORTSEL5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO Port 0"]
    #[inline]
    pub fn gpio_port_0(self) -> &'a mut W {
        self.variant(PORTSEL5W::GPIO_PORT_0)
    }
    #[doc = "GPIO Port 1"]
    #[inline]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(PORTSEL5W::GPIO_PORT_1)
    }
    #[doc = "GPIO Port 2"]
    #[inline]
    pub fn gpio_port_2(self) -> &'a mut W {
        self.variant(PORTSEL5W::GPIO_PORT_2)
    }
    #[doc = "GPIO Port 3"]
    #[inline]
    pub fn gpio_port_3(self) -> &'a mut W {
        self.variant(PORTSEL5W::GPIO_PORT_3)
    }
    #[doc = "GPIO Port 4"]
    #[inline]
    pub fn gpio_port_4(self) -> &'a mut W {
        self.variant(PORTSEL5W::GPIO_PORT_4)
    }
    #[doc = "GPIO Port 5"]
    #[inline]
    pub fn gpio_port_5(self) -> &'a mut W {
        self.variant(PORTSEL5W::GPIO_PORT_5)
    }
    #[doc = "GPIO Port 6"]
    #[inline]
    pub fn gpio_port_6(self) -> &'a mut W {
        self.variant(PORTSEL5W::GPIO_PORT_6)
    }
    #[doc = "GPIO Port 7"]
    #[inline]
    pub fn gpio_port_7(self) -> &'a mut W {
        self.variant(PORTSEL5W::GPIO_PORT_7)
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
pub struct _INTPIN6W<'a> {
    w: &'a mut W,
}
impl<'a> _INTPIN6W<'a> {
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
#[doc = "Values that can be written to the field `PORTSEL6`"]
pub enum PORTSEL6W {
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
impl PORTSEL6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PORTSEL6W::GPIO_PORT_0 => 0,
            PORTSEL6W::GPIO_PORT_1 => 1,
            PORTSEL6W::GPIO_PORT_2 => 2,
            PORTSEL6W::GPIO_PORT_3 => 3,
            PORTSEL6W::GPIO_PORT_4 => 4,
            PORTSEL6W::GPIO_PORT_5 => 5,
            PORTSEL6W::GPIO_PORT_6 => 6,
            PORTSEL6W::GPIO_PORT_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PORTSEL6W<'a> {
    w: &'a mut W,
}
impl<'a> _PORTSEL6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PORTSEL6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO Port 0"]
    #[inline]
    pub fn gpio_port_0(self) -> &'a mut W {
        self.variant(PORTSEL6W::GPIO_PORT_0)
    }
    #[doc = "GPIO Port 1"]
    #[inline]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(PORTSEL6W::GPIO_PORT_1)
    }
    #[doc = "GPIO Port 2"]
    #[inline]
    pub fn gpio_port_2(self) -> &'a mut W {
        self.variant(PORTSEL6W::GPIO_PORT_2)
    }
    #[doc = "GPIO Port 3"]
    #[inline]
    pub fn gpio_port_3(self) -> &'a mut W {
        self.variant(PORTSEL6W::GPIO_PORT_3)
    }
    #[doc = "GPIO Port 4"]
    #[inline]
    pub fn gpio_port_4(self) -> &'a mut W {
        self.variant(PORTSEL6W::GPIO_PORT_4)
    }
    #[doc = "GPIO Port 5"]
    #[inline]
    pub fn gpio_port_5(self) -> &'a mut W {
        self.variant(PORTSEL6W::GPIO_PORT_5)
    }
    #[doc = "GPIO Port 6"]
    #[inline]
    pub fn gpio_port_6(self) -> &'a mut W {
        self.variant(PORTSEL6W::GPIO_PORT_6)
    }
    #[doc = "GPIO Port 7"]
    #[inline]
    pub fn gpio_port_7(self) -> &'a mut W {
        self.variant(PORTSEL6W::GPIO_PORT_7)
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
pub struct _INTPIN7W<'a> {
    w: &'a mut W,
}
impl<'a> _INTPIN7W<'a> {
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
#[doc = "Values that can be written to the field `PORTSEL7`"]
pub enum PORTSEL7W {
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
impl PORTSEL7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PORTSEL7W::GPIO_PORT_0 => 0,
            PORTSEL7W::GPIO_PORT_1 => 1,
            PORTSEL7W::GPIO_PORT_2 => 2,
            PORTSEL7W::GPIO_PORT_3 => 3,
            PORTSEL7W::GPIO_PORT_4 => 4,
            PORTSEL7W::GPIO_PORT_5 => 5,
            PORTSEL7W::GPIO_PORT_6 => 6,
            PORTSEL7W::GPIO_PORT_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PORTSEL7W<'a> {
    w: &'a mut W,
}
impl<'a> _PORTSEL7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PORTSEL7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPIO Port 0"]
    #[inline]
    pub fn gpio_port_0(self) -> &'a mut W {
        self.variant(PORTSEL7W::GPIO_PORT_0)
    }
    #[doc = "GPIO Port 1"]
    #[inline]
    pub fn gpio_port_1(self) -> &'a mut W {
        self.variant(PORTSEL7W::GPIO_PORT_1)
    }
    #[doc = "GPIO Port 2"]
    #[inline]
    pub fn gpio_port_2(self) -> &'a mut W {
        self.variant(PORTSEL7W::GPIO_PORT_2)
    }
    #[doc = "GPIO Port 3"]
    #[inline]
    pub fn gpio_port_3(self) -> &'a mut W {
        self.variant(PORTSEL7W::GPIO_PORT_3)
    }
    #[doc = "GPIO Port 4"]
    #[inline]
    pub fn gpio_port_4(self) -> &'a mut W {
        self.variant(PORTSEL7W::GPIO_PORT_4)
    }
    #[doc = "GPIO Port 5"]
    #[inline]
    pub fn gpio_port_5(self) -> &'a mut W {
        self.variant(PORTSEL7W::GPIO_PORT_5)
    }
    #[doc = "GPIO Port 6"]
    #[inline]
    pub fn gpio_port_6(self) -> &'a mut W {
        self.variant(PORTSEL7W::GPIO_PORT_6)
    }
    #[doc = "GPIO Port 7"]
    #[inline]
    pub fn gpio_port_7(self) -> &'a mut W {
        self.variant(PORTSEL7W::GPIO_PORT_7)
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
    #[doc = "Bits 0:4 - Pint interrupt 4: Select the pin number within the GPIO port selected by the PORTSEL4 bit in this register."]
    #[inline]
    pub fn intpin4(&self) -> INTPIN4R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPIN4R { bits }
    }
    #[doc = "Bits 5:7 - Pin interrupt 4: Select the port for the pin number to be selected in the INTPIN4 bits of this register."]
    #[inline]
    pub fn portsel4(&self) -> PORTSEL4R {
        PORTSEL4R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:12 - Pint interrupt 5: Select the pin number within the GPIO port selected by the PORTSEL5 bit in this register."]
    #[inline]
    pub fn intpin5(&self) -> INTPIN5R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPIN5R { bits }
    }
    #[doc = "Bits 13:15 - Pin interrupt 5: Select the port for the pin number to be selected in the INTPIN5 bits of this register."]
    #[inline]
    pub fn portsel5(&self) -> PORTSEL5R {
        PORTSEL5R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:20 - Pint interrupt 6: Select the pin number within the GPIO port selected by the PORTSEL6 bit in this register."]
    #[inline]
    pub fn intpin6(&self) -> INTPIN6R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPIN6R { bits }
    }
    #[doc = "Bits 21:23 - Pin interrupt 6: Select the port for the pin number to be selected in the INTPIN6 bits of this register."]
    #[inline]
    pub fn portsel6(&self) -> PORTSEL6R {
        PORTSEL6R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:28 - Pint interrupt 7: Select the pin number within the GPIO port selected by the PORTSEL7 bit in this register."]
    #[inline]
    pub fn intpin7(&self) -> INTPIN7R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTPIN7R { bits }
    }
    #[doc = "Bits 29:31 - Pin interrupt 7: Select the port for the pin number to be selected in the INTPIN7 bits of this register."]
    #[inline]
    pub fn portsel7(&self) -> PORTSEL7R {
        PORTSEL7R::_from({
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
    #[doc = "Bits 0:4 - Pint interrupt 4: Select the pin number within the GPIO port selected by the PORTSEL4 bit in this register."]
    #[inline]
    pub fn intpin4(&mut self) -> _INTPIN4W {
        _INTPIN4W { w: self }
    }
    #[doc = "Bits 5:7 - Pin interrupt 4: Select the port for the pin number to be selected in the INTPIN4 bits of this register."]
    #[inline]
    pub fn portsel4(&mut self) -> _PORTSEL4W {
        _PORTSEL4W { w: self }
    }
    #[doc = "Bits 8:12 - Pint interrupt 5: Select the pin number within the GPIO port selected by the PORTSEL5 bit in this register."]
    #[inline]
    pub fn intpin5(&mut self) -> _INTPIN5W {
        _INTPIN5W { w: self }
    }
    #[doc = "Bits 13:15 - Pin interrupt 5: Select the port for the pin number to be selected in the INTPIN5 bits of this register."]
    #[inline]
    pub fn portsel5(&mut self) -> _PORTSEL5W {
        _PORTSEL5W { w: self }
    }
    #[doc = "Bits 16:20 - Pint interrupt 6: Select the pin number within the GPIO port selected by the PORTSEL6 bit in this register."]
    #[inline]
    pub fn intpin6(&mut self) -> _INTPIN6W {
        _INTPIN6W { w: self }
    }
    #[doc = "Bits 21:23 - Pin interrupt 6: Select the port for the pin number to be selected in the INTPIN6 bits of this register."]
    #[inline]
    pub fn portsel6(&mut self) -> _PORTSEL6W {
        _PORTSEL6W { w: self }
    }
    #[doc = "Bits 24:28 - Pint interrupt 7: Select the pin number within the GPIO port selected by the PORTSEL7 bit in this register."]
    #[inline]
    pub fn intpin7(&mut self) -> _INTPIN7W {
        _INTPIN7W { w: self }
    }
    #[doc = "Bits 29:31 - Pin interrupt 7: Select the port for the pin number to be selected in the INTPIN7 bits of this register."]
    #[inline]
    pub fn portsel7(&mut self) -> _PORTSEL7W {
        _PORTSEL7W { w: self }
    }
}
