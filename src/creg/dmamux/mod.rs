#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMAMUX {
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
#[doc = "Possible values of the field `DMAMUXPER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER0R {
    #[doc = "SPIFI"]
    SPIFI,
    #[doc = "SCT CTOUT_2"]
    SCT_CTOUT_2,
    #[doc = "SGPIO14"]
    SGPIO14,
    #[doc = "Timer3 match 1"]
    TIMER3_MATCH_1,
}
impl DMAMUXPER0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER0R::SPIFI => 0,
            DMAMUXPER0R::SCT_CTOUT_2 => 1,
            DMAMUXPER0R::SGPIO14 => 2,
            DMAMUXPER0R::TIMER3_MATCH_1 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER0R {
        match value {
            0 => DMAMUXPER0R::SPIFI,
            1 => DMAMUXPER0R::SCT_CTOUT_2,
            2 => DMAMUXPER0R::SGPIO14,
            3 => DMAMUXPER0R::TIMER3_MATCH_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPIFI`"]
    #[inline]
    pub fn is_spifi(&self) -> bool {
        *self == DMAMUXPER0R::SPIFI
    }
    #[doc = "Checks if the value of the field is `SCT_CTOUT_2`"]
    #[inline]
    pub fn is_sct_ctout_2(&self) -> bool {
        *self == DMAMUXPER0R::SCT_CTOUT_2
    }
    #[doc = "Checks if the value of the field is `SGPIO14`"]
    #[inline]
    pub fn is_sgpio14(&self) -> bool {
        *self == DMAMUXPER0R::SGPIO14
    }
    #[doc = "Checks if the value of the field is `TIMER3_MATCH_1`"]
    #[inline]
    pub fn is_timer3_match_1(&self) -> bool {
        *self == DMAMUXPER0R::TIMER3_MATCH_1
    }
}
#[doc = "Possible values of the field `DMAMUXPER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER1R {
    #[doc = "Timer0 match 0"]
    TIMER0_MATCH_0,
    #[doc = "USART0 transmit"]
    USART0_TRANSMIT,
}
impl DMAMUXPER1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER1R::TIMER0_MATCH_0 => 0,
            DMAMUXPER1R::USART0_TRANSMIT => 1,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER1R {
        match value {
            0 => DMAMUXPER1R::TIMER0_MATCH_0,
            1 => DMAMUXPER1R::USART0_TRANSMIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER0_MATCH_0`"]
    #[inline]
    pub fn is_timer0_match_0(&self) -> bool {
        *self == DMAMUXPER1R::TIMER0_MATCH_0
    }
    #[doc = "Checks if the value of the field is `USART0_TRANSMIT`"]
    #[inline]
    pub fn is_usart0_transmit(&self) -> bool {
        *self == DMAMUXPER1R::USART0_TRANSMIT
    }
}
#[doc = "Possible values of the field `DMAMUXPER2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER2R {
    #[doc = "Timer0 match 1"]
    TIMER0_MATCH_1,
    #[doc = "USART0 receive"]
    USART0_RECEIVE,
}
impl DMAMUXPER2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER2R::TIMER0_MATCH_1 => 0,
            DMAMUXPER2R::USART0_RECEIVE => 1,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER2R {
        match value {
            0 => DMAMUXPER2R::TIMER0_MATCH_1,
            1 => DMAMUXPER2R::USART0_RECEIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER0_MATCH_1`"]
    #[inline]
    pub fn is_timer0_match_1(&self) -> bool {
        *self == DMAMUXPER2R::TIMER0_MATCH_1
    }
    #[doc = "Checks if the value of the field is `USART0_RECEIVE`"]
    #[inline]
    pub fn is_usart0_receive(&self) -> bool {
        *self == DMAMUXPER2R::USART0_RECEIVE
    }
}
#[doc = "Possible values of the field `DMAMUXPER3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER3R {
    #[doc = "Timer1 match 0"]
    TIMER1_MATCH_0,
    #[doc = "UART1 transmit"]
    UART1_TRANSMIT,
    #[doc = "I2S1 DMA request 1"]
    I2S1_DMA_REQUEST_1,
    #[doc = "SSP1 transmit"]
    SSP1_TRANSMIT,
}
impl DMAMUXPER3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER3R::TIMER1_MATCH_0 => 0,
            DMAMUXPER3R::UART1_TRANSMIT => 1,
            DMAMUXPER3R::I2S1_DMA_REQUEST_1 => 2,
            DMAMUXPER3R::SSP1_TRANSMIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER3R {
        match value {
            0 => DMAMUXPER3R::TIMER1_MATCH_0,
            1 => DMAMUXPER3R::UART1_TRANSMIT,
            2 => DMAMUXPER3R::I2S1_DMA_REQUEST_1,
            3 => DMAMUXPER3R::SSP1_TRANSMIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER1_MATCH_0`"]
    #[inline]
    pub fn is_timer1_match_0(&self) -> bool {
        *self == DMAMUXPER3R::TIMER1_MATCH_0
    }
    #[doc = "Checks if the value of the field is `UART1_TRANSMIT`"]
    #[inline]
    pub fn is_uart1_transmit(&self) -> bool {
        *self == DMAMUXPER3R::UART1_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `I2S1_DMA_REQUEST_1`"]
    #[inline]
    pub fn is_i2s1_dma_request_1(&self) -> bool {
        *self == DMAMUXPER3R::I2S1_DMA_REQUEST_1
    }
    #[doc = "Checks if the value of the field is `SSP1_TRANSMIT`"]
    #[inline]
    pub fn is_ssp1_transmit(&self) -> bool {
        *self == DMAMUXPER3R::SSP1_TRANSMIT
    }
}
#[doc = "Possible values of the field `DMAMUXPER4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER4R {
    #[doc = "Timer1 match 1"]
    TIMER1_MATCH_1,
    #[doc = "UART1 receive"]
    UART1_RECEIVE,
    #[doc = "I2S1 DMA request 2"]
    I2S1_DMA_REQUEST_2,
    #[doc = "SSP1 receive"]
    SSP1_RECEIVE,
}
impl DMAMUXPER4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER4R::TIMER1_MATCH_1 => 0,
            DMAMUXPER4R::UART1_RECEIVE => 1,
            DMAMUXPER4R::I2S1_DMA_REQUEST_2 => 2,
            DMAMUXPER4R::SSP1_RECEIVE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER4R {
        match value {
            0 => DMAMUXPER4R::TIMER1_MATCH_1,
            1 => DMAMUXPER4R::UART1_RECEIVE,
            2 => DMAMUXPER4R::I2S1_DMA_REQUEST_2,
            3 => DMAMUXPER4R::SSP1_RECEIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER1_MATCH_1`"]
    #[inline]
    pub fn is_timer1_match_1(&self) -> bool {
        *self == DMAMUXPER4R::TIMER1_MATCH_1
    }
    #[doc = "Checks if the value of the field is `UART1_RECEIVE`"]
    #[inline]
    pub fn is_uart1_receive(&self) -> bool {
        *self == DMAMUXPER4R::UART1_RECEIVE
    }
    #[doc = "Checks if the value of the field is `I2S1_DMA_REQUEST_2`"]
    #[inline]
    pub fn is_i2s1_dma_request_2(&self) -> bool {
        *self == DMAMUXPER4R::I2S1_DMA_REQUEST_2
    }
    #[doc = "Checks if the value of the field is `SSP1_RECEIVE`"]
    #[inline]
    pub fn is_ssp1_receive(&self) -> bool {
        *self == DMAMUXPER4R::SSP1_RECEIVE
    }
}
#[doc = "Possible values of the field `DMAMUXPER5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER5R {
    #[doc = "Timer2 match 0"]
    TIMER2_MATCH_0,
    #[doc = "USART2 transmit"]
    USART2_TRANSMIT,
    #[doc = "SSP1 transmit"]
    SSP1_TRANSMIT,
    #[doc = "SGPIO15"]
    SGPIO15,
}
impl DMAMUXPER5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER5R::TIMER2_MATCH_0 => 0,
            DMAMUXPER5R::USART2_TRANSMIT => 1,
            DMAMUXPER5R::SSP1_TRANSMIT => 2,
            DMAMUXPER5R::SGPIO15 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER5R {
        match value {
            0 => DMAMUXPER5R::TIMER2_MATCH_0,
            1 => DMAMUXPER5R::USART2_TRANSMIT,
            2 => DMAMUXPER5R::SSP1_TRANSMIT,
            3 => DMAMUXPER5R::SGPIO15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER2_MATCH_0`"]
    #[inline]
    pub fn is_timer2_match_0(&self) -> bool {
        *self == DMAMUXPER5R::TIMER2_MATCH_0
    }
    #[doc = "Checks if the value of the field is `USART2_TRANSMIT`"]
    #[inline]
    pub fn is_usart2_transmit(&self) -> bool {
        *self == DMAMUXPER5R::USART2_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `SSP1_TRANSMIT`"]
    #[inline]
    pub fn is_ssp1_transmit(&self) -> bool {
        *self == DMAMUXPER5R::SSP1_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `SGPIO15`"]
    #[inline]
    pub fn is_sgpio15(&self) -> bool {
        *self == DMAMUXPER5R::SGPIO15
    }
}
#[doc = "Possible values of the field `DMAMUXPER6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER6R {
    #[doc = "Timer2 match 1"]
    TIMER2_MATCH_1,
    #[doc = "USART2 receive"]
    USART2_RECEIVE,
    #[doc = "SSP1 receive"]
    SSP1_RECEIVE,
    #[doc = "SGPIO14"]
    SGPIO14,
}
impl DMAMUXPER6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER6R::TIMER2_MATCH_1 => 0,
            DMAMUXPER6R::USART2_RECEIVE => 1,
            DMAMUXPER6R::SSP1_RECEIVE => 2,
            DMAMUXPER6R::SGPIO14 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER6R {
        match value {
            0 => DMAMUXPER6R::TIMER2_MATCH_1,
            1 => DMAMUXPER6R::USART2_RECEIVE,
            2 => DMAMUXPER6R::SSP1_RECEIVE,
            3 => DMAMUXPER6R::SGPIO14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER2_MATCH_1`"]
    #[inline]
    pub fn is_timer2_match_1(&self) -> bool {
        *self == DMAMUXPER6R::TIMER2_MATCH_1
    }
    #[doc = "Checks if the value of the field is `USART2_RECEIVE`"]
    #[inline]
    pub fn is_usart2_receive(&self) -> bool {
        *self == DMAMUXPER6R::USART2_RECEIVE
    }
    #[doc = "Checks if the value of the field is `SSP1_RECEIVE`"]
    #[inline]
    pub fn is_ssp1_receive(&self) -> bool {
        *self == DMAMUXPER6R::SSP1_RECEIVE
    }
    #[doc = "Checks if the value of the field is `SGPIO14`"]
    #[inline]
    pub fn is_sgpio14(&self) -> bool {
        *self == DMAMUXPER6R::SGPIO14
    }
}
#[doc = "Possible values of the field `DMAMUXPER7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER7R {
    #[doc = "Timer3 match 0"]
    TIMER3_MATCH_0,
    #[doc = "USART3 transmit"]
    USART3_TRANSMIT,
    #[doc = "SCT DMA request 0"]
    SCT_DMA_REQUEST_0,
    #[doc = "ADCHS write"]
    ADCHS_WRITE,
}
impl DMAMUXPER7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER7R::TIMER3_MATCH_0 => 0,
            DMAMUXPER7R::USART3_TRANSMIT => 1,
            DMAMUXPER7R::SCT_DMA_REQUEST_0 => 2,
            DMAMUXPER7R::ADCHS_WRITE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER7R {
        match value {
            0 => DMAMUXPER7R::TIMER3_MATCH_0,
            1 => DMAMUXPER7R::USART3_TRANSMIT,
            2 => DMAMUXPER7R::SCT_DMA_REQUEST_0,
            3 => DMAMUXPER7R::ADCHS_WRITE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER3_MATCH_0`"]
    #[inline]
    pub fn is_timer3_match_0(&self) -> bool {
        *self == DMAMUXPER7R::TIMER3_MATCH_0
    }
    #[doc = "Checks if the value of the field is `USART3_TRANSMIT`"]
    #[inline]
    pub fn is_usart3_transmit(&self) -> bool {
        *self == DMAMUXPER7R::USART3_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `SCT_DMA_REQUEST_0`"]
    #[inline]
    pub fn is_sct_dma_request_0(&self) -> bool {
        *self == DMAMUXPER7R::SCT_DMA_REQUEST_0
    }
    #[doc = "Checks if the value of the field is `ADCHS_WRITE`"]
    #[inline]
    pub fn is_adchs_write(&self) -> bool {
        *self == DMAMUXPER7R::ADCHS_WRITE
    }
}
#[doc = "Possible values of the field `DMAMUXPER8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER8R {
    #[doc = "Timer3 match 1"]
    TIMER3_MATCH_1,
    #[doc = "USART3 receive"]
    USART3_RECEIVE,
    #[doc = "SCT DMA request 1"]
    SCT_DMA_REQUEST_1,
    #[doc = "ADCHS read"]
    ADCHS_READ,
}
impl DMAMUXPER8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER8R::TIMER3_MATCH_1 => 0,
            DMAMUXPER8R::USART3_RECEIVE => 1,
            DMAMUXPER8R::SCT_DMA_REQUEST_1 => 2,
            DMAMUXPER8R::ADCHS_READ => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER8R {
        match value {
            0 => DMAMUXPER8R::TIMER3_MATCH_1,
            1 => DMAMUXPER8R::USART3_RECEIVE,
            2 => DMAMUXPER8R::SCT_DMA_REQUEST_1,
            3 => DMAMUXPER8R::ADCHS_READ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER3_MATCH_1`"]
    #[inline]
    pub fn is_timer3_match_1(&self) -> bool {
        *self == DMAMUXPER8R::TIMER3_MATCH_1
    }
    #[doc = "Checks if the value of the field is `USART3_RECEIVE`"]
    #[inline]
    pub fn is_usart3_receive(&self) -> bool {
        *self == DMAMUXPER8R::USART3_RECEIVE
    }
    #[doc = "Checks if the value of the field is `SCT_DMA_REQUEST_1`"]
    #[inline]
    pub fn is_sct_dma_request_1(&self) -> bool {
        *self == DMAMUXPER8R::SCT_DMA_REQUEST_1
    }
    #[doc = "Checks if the value of the field is `ADCHS_READ`"]
    #[inline]
    pub fn is_adchs_read(&self) -> bool {
        *self == DMAMUXPER8R::ADCHS_READ
    }
}
#[doc = "Possible values of the field `DMAMUXPER9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER9R {
    #[doc = "SSP0 receive"]
    SSP0_RECEIVE,
    #[doc = "I2S0 DMA request 1"]
    I2S0_DMA_REQUEST_1,
    #[doc = "SCT DMA request 1"]
    SCT_DMA_REQUEST_1,
}
impl DMAMUXPER9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER9R::SSP0_RECEIVE => 0,
            DMAMUXPER9R::I2S0_DMA_REQUEST_1 => 1,
            DMAMUXPER9R::SCT_DMA_REQUEST_1 => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER9R {
        match value {
            0 => DMAMUXPER9R::SSP0_RECEIVE,
            1 => DMAMUXPER9R::I2S0_DMA_REQUEST_1,
            2 => DMAMUXPER9R::SCT_DMA_REQUEST_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SSP0_RECEIVE`"]
    #[inline]
    pub fn is_ssp0_receive(&self) -> bool {
        *self == DMAMUXPER9R::SSP0_RECEIVE
    }
    #[doc = "Checks if the value of the field is `I2S0_DMA_REQUEST_1`"]
    #[inline]
    pub fn is_i2s0_dma_request_1(&self) -> bool {
        *self == DMAMUXPER9R::I2S0_DMA_REQUEST_1
    }
    #[doc = "Checks if the value of the field is `SCT_DMA_REQUEST_1`"]
    #[inline]
    pub fn is_sct_dma_request_1(&self) -> bool {
        *self == DMAMUXPER9R::SCT_DMA_REQUEST_1
    }
}
#[doc = "Possible values of the field `DMAMUXPER10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER10R {
    #[doc = "SSP0 transmit"]
    SSP0_TRANSMIT,
    #[doc = "I2S0 DMA request 2"]
    I2S0_DMA_REQUEST_2,
    #[doc = "SCT DMA request 0"]
    SCT_DMA_REQUEST_0,
}
impl DMAMUXPER10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER10R::SSP0_TRANSMIT => 0,
            DMAMUXPER10R::I2S0_DMA_REQUEST_2 => 1,
            DMAMUXPER10R::SCT_DMA_REQUEST_0 => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER10R {
        match value {
            0 => DMAMUXPER10R::SSP0_TRANSMIT,
            1 => DMAMUXPER10R::I2S0_DMA_REQUEST_2,
            2 => DMAMUXPER10R::SCT_DMA_REQUEST_0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SSP0_TRANSMIT`"]
    #[inline]
    pub fn is_ssp0_transmit(&self) -> bool {
        *self == DMAMUXPER10R::SSP0_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `I2S0_DMA_REQUEST_2`"]
    #[inline]
    pub fn is_i2s0_dma_request_2(&self) -> bool {
        *self == DMAMUXPER10R::I2S0_DMA_REQUEST_2
    }
    #[doc = "Checks if the value of the field is `SCT_DMA_REQUEST_0`"]
    #[inline]
    pub fn is_sct_dma_request_0(&self) -> bool {
        *self == DMAMUXPER10R::SCT_DMA_REQUEST_0
    }
}
#[doc = "Possible values of the field `DMAMUXPER11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER11R {
    #[doc = "SSP1 receive"]
    SSP1_RECEIVE,
    #[doc = "SGPIO14"]
    SGPIO14,
    #[doc = "USART0 transmit"]
    USART0_TRANSMIT,
}
impl DMAMUXPER11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER11R::SSP1_RECEIVE => 0,
            DMAMUXPER11R::SGPIO14 => 1,
            DMAMUXPER11R::USART0_TRANSMIT => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER11R {
        match value {
            0 => DMAMUXPER11R::SSP1_RECEIVE,
            1 => DMAMUXPER11R::SGPIO14,
            2 => DMAMUXPER11R::USART0_TRANSMIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SSP1_RECEIVE`"]
    #[inline]
    pub fn is_ssp1_receive(&self) -> bool {
        *self == DMAMUXPER11R::SSP1_RECEIVE
    }
    #[doc = "Checks if the value of the field is `SGPIO14`"]
    #[inline]
    pub fn is_sgpio14(&self) -> bool {
        *self == DMAMUXPER11R::SGPIO14
    }
    #[doc = "Checks if the value of the field is `USART0_TRANSMIT`"]
    #[inline]
    pub fn is_usart0_transmit(&self) -> bool {
        *self == DMAMUXPER11R::USART0_TRANSMIT
    }
}
#[doc = "Possible values of the field `DMAMUXPER12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER12R {
    #[doc = "SSP1 transmit"]
    SSP1_TRANSMIT,
    #[doc = "SGPIO15"]
    SGPIO15,
    #[doc = "USART0 receive"]
    USART0_RECEIVE,
}
impl DMAMUXPER12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER12R::SSP1_TRANSMIT => 0,
            DMAMUXPER12R::SGPIO15 => 1,
            DMAMUXPER12R::USART0_RECEIVE => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER12R {
        match value {
            0 => DMAMUXPER12R::SSP1_TRANSMIT,
            1 => DMAMUXPER12R::SGPIO15,
            2 => DMAMUXPER12R::USART0_RECEIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SSP1_TRANSMIT`"]
    #[inline]
    pub fn is_ssp1_transmit(&self) -> bool {
        *self == DMAMUXPER12R::SSP1_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `SGPIO15`"]
    #[inline]
    pub fn is_sgpio15(&self) -> bool {
        *self == DMAMUXPER12R::SGPIO15
    }
    #[doc = "Checks if the value of the field is `USART0_RECEIVE`"]
    #[inline]
    pub fn is_usart0_receive(&self) -> bool {
        *self == DMAMUXPER12R::USART0_RECEIVE
    }
}
#[doc = "Possible values of the field `DMAMUXPER13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER13R {
    #[doc = "ADC0"]
    ADC0,
    #[doc = "SSP1 receive"]
    SSP1_RECEIVE,
    #[doc = "USART3 receive"]
    USART3_RECEIVE,
}
impl DMAMUXPER13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER13R::ADC0 => 0,
            DMAMUXPER13R::SSP1_RECEIVE => 2,
            DMAMUXPER13R::USART3_RECEIVE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER13R {
        match value {
            0 => DMAMUXPER13R::ADC0,
            2 => DMAMUXPER13R::SSP1_RECEIVE,
            3 => DMAMUXPER13R::USART3_RECEIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline]
    pub fn is_adc0(&self) -> bool {
        *self == DMAMUXPER13R::ADC0
    }
    #[doc = "Checks if the value of the field is `SSP1_RECEIVE`"]
    #[inline]
    pub fn is_ssp1_receive(&self) -> bool {
        *self == DMAMUXPER13R::SSP1_RECEIVE
    }
    #[doc = "Checks if the value of the field is `USART3_RECEIVE`"]
    #[inline]
    pub fn is_usart3_receive(&self) -> bool {
        *self == DMAMUXPER13R::USART3_RECEIVE
    }
}
#[doc = "Possible values of the field `DMAMUXPER14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER14R {
    #[doc = "ADC1"]
    ADC1,
    #[doc = "SSP1 transmit"]
    SSP1_TRANSMIT,
    #[doc = "USART3 transmit"]
    USART3_TRANSMIT,
}
impl DMAMUXPER14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER14R::ADC1 => 0,
            DMAMUXPER14R::SSP1_TRANSMIT => 2,
            DMAMUXPER14R::USART3_TRANSMIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER14R {
        match value {
            0 => DMAMUXPER14R::ADC1,
            2 => DMAMUXPER14R::SSP1_TRANSMIT,
            3 => DMAMUXPER14R::USART3_TRANSMIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC1`"]
    #[inline]
    pub fn is_adc1(&self) -> bool {
        *self == DMAMUXPER14R::ADC1
    }
    #[doc = "Checks if the value of the field is `SSP1_TRANSMIT`"]
    #[inline]
    pub fn is_ssp1_transmit(&self) -> bool {
        *self == DMAMUXPER14R::SSP1_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `USART3_TRANSMIT`"]
    #[inline]
    pub fn is_usart3_transmit(&self) -> bool {
        *self == DMAMUXPER14R::USART3_TRANSMIT
    }
}
#[doc = "Possible values of the field `DMAMUXPER15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXPER15R {
    #[doc = "DAC"]
    DAC,
    #[doc = "SCT CTOUT_3"]
    SCT_CTOUT_3,
    #[doc = "SGPIO15"]
    SGPIO15,
    #[doc = "Timer3 match 0"]
    TIMER3_MATCH_0,
}
impl DMAMUXPER15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAMUXPER15R::DAC => 0,
            DMAMUXPER15R::SCT_CTOUT_3 => 1,
            DMAMUXPER15R::SGPIO15 => 2,
            DMAMUXPER15R::TIMER3_MATCH_0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAMUXPER15R {
        match value {
            0 => DMAMUXPER15R::DAC,
            1 => DMAMUXPER15R::SCT_CTOUT_3,
            2 => DMAMUXPER15R::SGPIO15,
            3 => DMAMUXPER15R::TIMER3_MATCH_0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == DMAMUXPER15R::DAC
    }
    #[doc = "Checks if the value of the field is `SCT_CTOUT_3`"]
    #[inline]
    pub fn is_sct_ctout_3(&self) -> bool {
        *self == DMAMUXPER15R::SCT_CTOUT_3
    }
    #[doc = "Checks if the value of the field is `SGPIO15`"]
    #[inline]
    pub fn is_sgpio15(&self) -> bool {
        *self == DMAMUXPER15R::SGPIO15
    }
    #[doc = "Checks if the value of the field is `TIMER3_MATCH_0`"]
    #[inline]
    pub fn is_timer3_match_0(&self) -> bool {
        *self == DMAMUXPER15R::TIMER3_MATCH_0
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER0`"]
pub enum DMAMUXPER0W {
    #[doc = "SPIFI"]
    SPIFI,
    #[doc = "SCT CTOUT_2"]
    SCT_CTOUT_2,
    #[doc = "SGPIO14"]
    SGPIO14,
    #[doc = "Timer3 match 1"]
    TIMER3_MATCH_1,
}
impl DMAMUXPER0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER0W::SPIFI => 0,
            DMAMUXPER0W::SCT_CTOUT_2 => 1,
            DMAMUXPER0W::SGPIO14 => 2,
            DMAMUXPER0W::TIMER3_MATCH_1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER0W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SPIFI"]
    #[inline]
    pub fn spifi(self) -> &'a mut W {
        self.variant(DMAMUXPER0W::SPIFI)
    }
    #[doc = "SCT CTOUT_2"]
    #[inline]
    pub fn sct_ctout_2(self) -> &'a mut W {
        self.variant(DMAMUXPER0W::SCT_CTOUT_2)
    }
    #[doc = "SGPIO14"]
    #[inline]
    pub fn sgpio14(self) -> &'a mut W {
        self.variant(DMAMUXPER0W::SGPIO14)
    }
    #[doc = "Timer3 match 1"]
    #[inline]
    pub fn timer3_match_1(self) -> &'a mut W {
        self.variant(DMAMUXPER0W::TIMER3_MATCH_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER1`"]
pub enum DMAMUXPER1W {
    #[doc = "Timer0 match 0"]
    TIMER0_MATCH_0,
    #[doc = "USART0 transmit"]
    USART0_TRANSMIT,
}
impl DMAMUXPER1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER1W::TIMER0_MATCH_0 => 0,
            DMAMUXPER1W::USART0_TRANSMIT => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER1W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer0 match 0"]
    #[inline]
    pub fn timer0_match_0(self) -> &'a mut W {
        self.variant(DMAMUXPER1W::TIMER0_MATCH_0)
    }
    #[doc = "USART0 transmit"]
    #[inline]
    pub fn usart0_transmit(self) -> &'a mut W {
        self.variant(DMAMUXPER1W::USART0_TRANSMIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER2`"]
pub enum DMAMUXPER2W {
    #[doc = "Timer0 match 1"]
    TIMER0_MATCH_1,
    #[doc = "USART0 receive"]
    USART0_RECEIVE,
}
impl DMAMUXPER2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER2W::TIMER0_MATCH_1 => 0,
            DMAMUXPER2W::USART0_RECEIVE => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER2W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer0 match 1"]
    #[inline]
    pub fn timer0_match_1(self) -> &'a mut W {
        self.variant(DMAMUXPER2W::TIMER0_MATCH_1)
    }
    #[doc = "USART0 receive"]
    #[inline]
    pub fn usart0_receive(self) -> &'a mut W {
        self.variant(DMAMUXPER2W::USART0_RECEIVE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER3`"]
pub enum DMAMUXPER3W {
    #[doc = "Timer1 match 0"]
    TIMER1_MATCH_0,
    #[doc = "UART1 transmit"]
    UART1_TRANSMIT,
    #[doc = "I2S1 DMA request 1"]
    I2S1_DMA_REQUEST_1,
    #[doc = "SSP1 transmit"]
    SSP1_TRANSMIT,
}
impl DMAMUXPER3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER3W::TIMER1_MATCH_0 => 0,
            DMAMUXPER3W::UART1_TRANSMIT => 1,
            DMAMUXPER3W::I2S1_DMA_REQUEST_1 => 2,
            DMAMUXPER3W::SSP1_TRANSMIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER3W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer1 match 0"]
    #[inline]
    pub fn timer1_match_0(self) -> &'a mut W {
        self.variant(DMAMUXPER3W::TIMER1_MATCH_0)
    }
    #[doc = "UART1 transmit"]
    #[inline]
    pub fn uart1_transmit(self) -> &'a mut W {
        self.variant(DMAMUXPER3W::UART1_TRANSMIT)
    }
    #[doc = "I2S1 DMA request 1"]
    #[inline]
    pub fn i2s1_dma_request_1(self) -> &'a mut W {
        self.variant(DMAMUXPER3W::I2S1_DMA_REQUEST_1)
    }
    #[doc = "SSP1 transmit"]
    #[inline]
    pub fn ssp1_transmit(self) -> &'a mut W {
        self.variant(DMAMUXPER3W::SSP1_TRANSMIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER4`"]
pub enum DMAMUXPER4W {
    #[doc = "Timer1 match 1"]
    TIMER1_MATCH_1,
    #[doc = "UART1 receive"]
    UART1_RECEIVE,
    #[doc = "I2S1 DMA request 2"]
    I2S1_DMA_REQUEST_2,
    #[doc = "SSP1 receive"]
    SSP1_RECEIVE,
}
impl DMAMUXPER4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER4W::TIMER1_MATCH_1 => 0,
            DMAMUXPER4W::UART1_RECEIVE => 1,
            DMAMUXPER4W::I2S1_DMA_REQUEST_2 => 2,
            DMAMUXPER4W::SSP1_RECEIVE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER4W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer1 match 1"]
    #[inline]
    pub fn timer1_match_1(self) -> &'a mut W {
        self.variant(DMAMUXPER4W::TIMER1_MATCH_1)
    }
    #[doc = "UART1 receive"]
    #[inline]
    pub fn uart1_receive(self) -> &'a mut W {
        self.variant(DMAMUXPER4W::UART1_RECEIVE)
    }
    #[doc = "I2S1 DMA request 2"]
    #[inline]
    pub fn i2s1_dma_request_2(self) -> &'a mut W {
        self.variant(DMAMUXPER4W::I2S1_DMA_REQUEST_2)
    }
    #[doc = "SSP1 receive"]
    #[inline]
    pub fn ssp1_receive(self) -> &'a mut W {
        self.variant(DMAMUXPER4W::SSP1_RECEIVE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER5`"]
pub enum DMAMUXPER5W {
    #[doc = "Timer2 match 0"]
    TIMER2_MATCH_0,
    #[doc = "USART2 transmit"]
    USART2_TRANSMIT,
    #[doc = "SSP1 transmit"]
    SSP1_TRANSMIT,
    #[doc = "SGPIO15"]
    SGPIO15,
}
impl DMAMUXPER5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER5W::TIMER2_MATCH_0 => 0,
            DMAMUXPER5W::USART2_TRANSMIT => 1,
            DMAMUXPER5W::SSP1_TRANSMIT => 2,
            DMAMUXPER5W::SGPIO15 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER5W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer2 match 0"]
    #[inline]
    pub fn timer2_match_0(self) -> &'a mut W {
        self.variant(DMAMUXPER5W::TIMER2_MATCH_0)
    }
    #[doc = "USART2 transmit"]
    #[inline]
    pub fn usart2_transmit(self) -> &'a mut W {
        self.variant(DMAMUXPER5W::USART2_TRANSMIT)
    }
    #[doc = "SSP1 transmit"]
    #[inline]
    pub fn ssp1_transmit(self) -> &'a mut W {
        self.variant(DMAMUXPER5W::SSP1_TRANSMIT)
    }
    #[doc = "SGPIO15"]
    #[inline]
    pub fn sgpio15(self) -> &'a mut W {
        self.variant(DMAMUXPER5W::SGPIO15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER6`"]
pub enum DMAMUXPER6W {
    #[doc = "Timer2 match 1"]
    TIMER2_MATCH_1,
    #[doc = "USART2 receive"]
    USART2_RECEIVE,
    #[doc = "SSP1 receive"]
    SSP1_RECEIVE,
    #[doc = "SGPIO14"]
    SGPIO14,
}
impl DMAMUXPER6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER6W::TIMER2_MATCH_1 => 0,
            DMAMUXPER6W::USART2_RECEIVE => 1,
            DMAMUXPER6W::SSP1_RECEIVE => 2,
            DMAMUXPER6W::SGPIO14 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER6W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer2 match 1"]
    #[inline]
    pub fn timer2_match_1(self) -> &'a mut W {
        self.variant(DMAMUXPER6W::TIMER2_MATCH_1)
    }
    #[doc = "USART2 receive"]
    #[inline]
    pub fn usart2_receive(self) -> &'a mut W {
        self.variant(DMAMUXPER6W::USART2_RECEIVE)
    }
    #[doc = "SSP1 receive"]
    #[inline]
    pub fn ssp1_receive(self) -> &'a mut W {
        self.variant(DMAMUXPER6W::SSP1_RECEIVE)
    }
    #[doc = "SGPIO14"]
    #[inline]
    pub fn sgpio14(self) -> &'a mut W {
        self.variant(DMAMUXPER6W::SGPIO14)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER7`"]
pub enum DMAMUXPER7W {
    #[doc = "Timer3 match 0"]
    TIMER3_MATCH_0,
    #[doc = "USART3 transmit"]
    USART3_TRANSMIT,
    #[doc = "SCT DMA request 0"]
    SCT_DMA_REQUEST_0,
    #[doc = "ADCHS write"]
    ADCHS_WRITE,
}
impl DMAMUXPER7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER7W::TIMER3_MATCH_0 => 0,
            DMAMUXPER7W::USART3_TRANSMIT => 1,
            DMAMUXPER7W::SCT_DMA_REQUEST_0 => 2,
            DMAMUXPER7W::ADCHS_WRITE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER7W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer3 match 0"]
    #[inline]
    pub fn timer3_match_0(self) -> &'a mut W {
        self.variant(DMAMUXPER7W::TIMER3_MATCH_0)
    }
    #[doc = "USART3 transmit"]
    #[inline]
    pub fn usart3_transmit(self) -> &'a mut W {
        self.variant(DMAMUXPER7W::USART3_TRANSMIT)
    }
    #[doc = "SCT DMA request 0"]
    #[inline]
    pub fn sct_dma_request_0(self) -> &'a mut W {
        self.variant(DMAMUXPER7W::SCT_DMA_REQUEST_0)
    }
    #[doc = "ADCHS write"]
    #[inline]
    pub fn adchs_write(self) -> &'a mut W {
        self.variant(DMAMUXPER7W::ADCHS_WRITE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER8`"]
pub enum DMAMUXPER8W {
    #[doc = "Timer3 match 1"]
    TIMER3_MATCH_1,
    #[doc = "USART3 receive"]
    USART3_RECEIVE,
    #[doc = "SCT DMA request 1"]
    SCT_DMA_REQUEST_1,
    #[doc = "ADCHS read"]
    ADCHS_READ,
}
impl DMAMUXPER8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER8W::TIMER3_MATCH_1 => 0,
            DMAMUXPER8W::USART3_RECEIVE => 1,
            DMAMUXPER8W::SCT_DMA_REQUEST_1 => 2,
            DMAMUXPER8W::ADCHS_READ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER8W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer3 match 1"]
    #[inline]
    pub fn timer3_match_1(self) -> &'a mut W {
        self.variant(DMAMUXPER8W::TIMER3_MATCH_1)
    }
    #[doc = "USART3 receive"]
    #[inline]
    pub fn usart3_receive(self) -> &'a mut W {
        self.variant(DMAMUXPER8W::USART3_RECEIVE)
    }
    #[doc = "SCT DMA request 1"]
    #[inline]
    pub fn sct_dma_request_1(self) -> &'a mut W {
        self.variant(DMAMUXPER8W::SCT_DMA_REQUEST_1)
    }
    #[doc = "ADCHS read"]
    #[inline]
    pub fn adchs_read(self) -> &'a mut W {
        self.variant(DMAMUXPER8W::ADCHS_READ)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER9`"]
pub enum DMAMUXPER9W {
    #[doc = "SSP0 receive"]
    SSP0_RECEIVE,
    #[doc = "I2S0 DMA request 1"]
    I2S0_DMA_REQUEST_1,
    #[doc = "SCT DMA request 1"]
    SCT_DMA_REQUEST_1,
}
impl DMAMUXPER9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER9W::SSP0_RECEIVE => 0,
            DMAMUXPER9W::I2S0_DMA_REQUEST_1 => 1,
            DMAMUXPER9W::SCT_DMA_REQUEST_1 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER9W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SSP0 receive"]
    #[inline]
    pub fn ssp0_receive(self) -> &'a mut W {
        self.variant(DMAMUXPER9W::SSP0_RECEIVE)
    }
    #[doc = "I2S0 DMA request 1"]
    #[inline]
    pub fn i2s0_dma_request_1(self) -> &'a mut W {
        self.variant(DMAMUXPER9W::I2S0_DMA_REQUEST_1)
    }
    #[doc = "SCT DMA request 1"]
    #[inline]
    pub fn sct_dma_request_1(self) -> &'a mut W {
        self.variant(DMAMUXPER9W::SCT_DMA_REQUEST_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER10`"]
pub enum DMAMUXPER10W {
    #[doc = "SSP0 transmit"]
    SSP0_TRANSMIT,
    #[doc = "I2S0 DMA request 2"]
    I2S0_DMA_REQUEST_2,
    #[doc = "SCT DMA request 0"]
    SCT_DMA_REQUEST_0,
}
impl DMAMUXPER10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER10W::SSP0_TRANSMIT => 0,
            DMAMUXPER10W::I2S0_DMA_REQUEST_2 => 1,
            DMAMUXPER10W::SCT_DMA_REQUEST_0 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER10W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SSP0 transmit"]
    #[inline]
    pub fn ssp0_transmit(self) -> &'a mut W {
        self.variant(DMAMUXPER10W::SSP0_TRANSMIT)
    }
    #[doc = "I2S0 DMA request 2"]
    #[inline]
    pub fn i2s0_dma_request_2(self) -> &'a mut W {
        self.variant(DMAMUXPER10W::I2S0_DMA_REQUEST_2)
    }
    #[doc = "SCT DMA request 0"]
    #[inline]
    pub fn sct_dma_request_0(self) -> &'a mut W {
        self.variant(DMAMUXPER10W::SCT_DMA_REQUEST_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER11`"]
pub enum DMAMUXPER11W {
    #[doc = "SSP1 receive"]
    SSP1_RECEIVE,
    #[doc = "SGPIO14"]
    SGPIO14,
    #[doc = "USART0 transmit"]
    USART0_TRANSMIT,
}
impl DMAMUXPER11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER11W::SSP1_RECEIVE => 0,
            DMAMUXPER11W::SGPIO14 => 1,
            DMAMUXPER11W::USART0_TRANSMIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER11W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SSP1 receive"]
    #[inline]
    pub fn ssp1_receive(self) -> &'a mut W {
        self.variant(DMAMUXPER11W::SSP1_RECEIVE)
    }
    #[doc = "SGPIO14"]
    #[inline]
    pub fn sgpio14(self) -> &'a mut W {
        self.variant(DMAMUXPER11W::SGPIO14)
    }
    #[doc = "USART0 transmit"]
    #[inline]
    pub fn usart0_transmit(self) -> &'a mut W {
        self.variant(DMAMUXPER11W::USART0_TRANSMIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER12`"]
pub enum DMAMUXPER12W {
    #[doc = "SSP1 transmit"]
    SSP1_TRANSMIT,
    #[doc = "SGPIO15"]
    SGPIO15,
    #[doc = "USART0 receive"]
    USART0_RECEIVE,
}
impl DMAMUXPER12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER12W::SSP1_TRANSMIT => 0,
            DMAMUXPER12W::SGPIO15 => 1,
            DMAMUXPER12W::USART0_RECEIVE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER12W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SSP1 transmit"]
    #[inline]
    pub fn ssp1_transmit(self) -> &'a mut W {
        self.variant(DMAMUXPER12W::SSP1_TRANSMIT)
    }
    #[doc = "SGPIO15"]
    #[inline]
    pub fn sgpio15(self) -> &'a mut W {
        self.variant(DMAMUXPER12W::SGPIO15)
    }
    #[doc = "USART0 receive"]
    #[inline]
    pub fn usart0_receive(self) -> &'a mut W {
        self.variant(DMAMUXPER12W::USART0_RECEIVE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER13`"]
pub enum DMAMUXPER13W {
    #[doc = "ADC0"]
    ADC0,
    #[doc = "SSP1 receive"]
    SSP1_RECEIVE,
    #[doc = "USART3 receive"]
    USART3_RECEIVE,
}
impl DMAMUXPER13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER13W::ADC0 => 0,
            DMAMUXPER13W::SSP1_RECEIVE => 2,
            DMAMUXPER13W::USART3_RECEIVE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER13W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ADC0"]
    #[inline]
    pub fn adc0(self) -> &'a mut W {
        self.variant(DMAMUXPER13W::ADC0)
    }
    #[doc = "SSP1 receive"]
    #[inline]
    pub fn ssp1_receive(self) -> &'a mut W {
        self.variant(DMAMUXPER13W::SSP1_RECEIVE)
    }
    #[doc = "USART3 receive"]
    #[inline]
    pub fn usart3_receive(self) -> &'a mut W {
        self.variant(DMAMUXPER13W::USART3_RECEIVE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER14`"]
pub enum DMAMUXPER14W {
    #[doc = "ADC1"]
    ADC1,
    #[doc = "SSP1 transmit"]
    SSP1_TRANSMIT,
    #[doc = "USART3 transmit"]
    USART3_TRANSMIT,
}
impl DMAMUXPER14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER14W::ADC1 => 0,
            DMAMUXPER14W::SSP1_TRANSMIT => 2,
            DMAMUXPER14W::USART3_TRANSMIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER14W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ADC1"]
    #[inline]
    pub fn adc1(self) -> &'a mut W {
        self.variant(DMAMUXPER14W::ADC1)
    }
    #[doc = "SSP1 transmit"]
    #[inline]
    pub fn ssp1_transmit(self) -> &'a mut W {
        self.variant(DMAMUXPER14W::SSP1_TRANSMIT)
    }
    #[doc = "USART3 transmit"]
    #[inline]
    pub fn usart3_transmit(self) -> &'a mut W {
        self.variant(DMAMUXPER14W::USART3_TRANSMIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAMUXPER15`"]
pub enum DMAMUXPER15W {
    #[doc = "DAC"]
    DAC,
    #[doc = "SCT CTOUT_3"]
    SCT_CTOUT_3,
    #[doc = "SGPIO15"]
    SGPIO15,
    #[doc = "Timer3 match 0"]
    TIMER3_MATCH_0,
}
impl DMAMUXPER15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMAMUXPER15W::DAC => 0,
            DMAMUXPER15W::SCT_CTOUT_3 => 1,
            DMAMUXPER15W::SGPIO15 => 2,
            DMAMUXPER15W::TIMER3_MATCH_0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAMUXPER15W<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMUXPER15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAMUXPER15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DAC"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(DMAMUXPER15W::DAC)
    }
    #[doc = "SCT CTOUT_3"]
    #[inline]
    pub fn sct_ctout_3(self) -> &'a mut W {
        self.variant(DMAMUXPER15W::SCT_CTOUT_3)
    }
    #[doc = "SGPIO15"]
    #[inline]
    pub fn sgpio15(self) -> &'a mut W {
        self.variant(DMAMUXPER15W::SGPIO15)
    }
    #[doc = "Timer3 match 0"]
    #[inline]
    pub fn timer3_match_0(self) -> &'a mut W {
        self.variant(DMAMUXPER15W::TIMER3_MATCH_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Select DMA to peripheral connection for DMA peripheral 0."]
    #[inline]
    pub fn dmamuxper0(&self) -> DMAMUXPER0R {
        DMAMUXPER0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Select DMA to peripheral connection for DMA peripheral 1"]
    #[inline]
    pub fn dmamuxper1(&self) -> DMAMUXPER1R {
        DMAMUXPER1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Select DMA to peripheral connection for DMA peripheral 2."]
    #[inline]
    pub fn dmamuxper2(&self) -> DMAMUXPER2R {
        DMAMUXPER2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Select DMA to peripheral connection for DMA peripheral 3."]
    #[inline]
    pub fn dmamuxper3(&self) -> DMAMUXPER3R {
        DMAMUXPER3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Select DMA to peripheral connection for DMA peripheral 4."]
    #[inline]
    pub fn dmamuxper4(&self) -> DMAMUXPER4R {
        DMAMUXPER4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Select DMA to peripheral connection for DMA peripheral 5."]
    #[inline]
    pub fn dmamuxper5(&self) -> DMAMUXPER5R {
        DMAMUXPER5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Selects DMA to peripheral connection for DMA peripheral 6."]
    #[inline]
    pub fn dmamuxper6(&self) -> DMAMUXPER6R {
        DMAMUXPER6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Selects DMA to peripheral connection for DMA peripheral 7."]
    #[inline]
    pub fn dmamuxper7(&self) -> DMAMUXPER7R {
        DMAMUXPER7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Select DMA to peripheral connection for DMA peripheral 8."]
    #[inline]
    pub fn dmamuxper8(&self) -> DMAMUXPER8R {
        DMAMUXPER8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Select DMA to peripheral connection for DMA peripheral 9."]
    #[inline]
    pub fn dmamuxper9(&self) -> DMAMUXPER9R {
        DMAMUXPER9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Select DMA to peripheral connection for DMA peripheral 10."]
    #[inline]
    pub fn dmamuxper10(&self) -> DMAMUXPER10R {
        DMAMUXPER10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Selects DMA to peripheral connection for DMA peripheral 11."]
    #[inline]
    pub fn dmamuxper11(&self) -> DMAMUXPER11R {
        DMAMUXPER11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Select DMA to peripheral connection for DMA peripheral 12."]
    #[inline]
    pub fn dmamuxper12(&self) -> DMAMUXPER12R {
        DMAMUXPER12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Select DMA to peripheral connection for DMA peripheral 13."]
    #[inline]
    pub fn dmamuxper13(&self) -> DMAMUXPER13R {
        DMAMUXPER13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Select DMA to peripheral connection for DMA peripheral 14."]
    #[inline]
    pub fn dmamuxper14(&self) -> DMAMUXPER14R {
        DMAMUXPER14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Select DMA to peripheral connection for DMA peripheral 15."]
    #[inline]
    pub fn dmamuxper15(&self) -> DMAMUXPER15R {
        DMAMUXPER15R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Select DMA to peripheral connection for DMA peripheral 0."]
    #[inline]
    pub fn dmamuxper0(&mut self) -> _DMAMUXPER0W {
        _DMAMUXPER0W { w: self }
    }
    #[doc = "Bits 2:3 - Select DMA to peripheral connection for DMA peripheral 1"]
    #[inline]
    pub fn dmamuxper1(&mut self) -> _DMAMUXPER1W {
        _DMAMUXPER1W { w: self }
    }
    #[doc = "Bits 4:5 - Select DMA to peripheral connection for DMA peripheral 2."]
    #[inline]
    pub fn dmamuxper2(&mut self) -> _DMAMUXPER2W {
        _DMAMUXPER2W { w: self }
    }
    #[doc = "Bits 6:7 - Select DMA to peripheral connection for DMA peripheral 3."]
    #[inline]
    pub fn dmamuxper3(&mut self) -> _DMAMUXPER3W {
        _DMAMUXPER3W { w: self }
    }
    #[doc = "Bits 8:9 - Select DMA to peripheral connection for DMA peripheral 4."]
    #[inline]
    pub fn dmamuxper4(&mut self) -> _DMAMUXPER4W {
        _DMAMUXPER4W { w: self }
    }
    #[doc = "Bits 10:11 - Select DMA to peripheral connection for DMA peripheral 5."]
    #[inline]
    pub fn dmamuxper5(&mut self) -> _DMAMUXPER5W {
        _DMAMUXPER5W { w: self }
    }
    #[doc = "Bits 12:13 - Selects DMA to peripheral connection for DMA peripheral 6."]
    #[inline]
    pub fn dmamuxper6(&mut self) -> _DMAMUXPER6W {
        _DMAMUXPER6W { w: self }
    }
    #[doc = "Bits 14:15 - Selects DMA to peripheral connection for DMA peripheral 7."]
    #[inline]
    pub fn dmamuxper7(&mut self) -> _DMAMUXPER7W {
        _DMAMUXPER7W { w: self }
    }
    #[doc = "Bits 16:17 - Select DMA to peripheral connection for DMA peripheral 8."]
    #[inline]
    pub fn dmamuxper8(&mut self) -> _DMAMUXPER8W {
        _DMAMUXPER8W { w: self }
    }
    #[doc = "Bits 18:19 - Select DMA to peripheral connection for DMA peripheral 9."]
    #[inline]
    pub fn dmamuxper9(&mut self) -> _DMAMUXPER9W {
        _DMAMUXPER9W { w: self }
    }
    #[doc = "Bits 20:21 - Select DMA to peripheral connection for DMA peripheral 10."]
    #[inline]
    pub fn dmamuxper10(&mut self) -> _DMAMUXPER10W {
        _DMAMUXPER10W { w: self }
    }
    #[doc = "Bits 22:23 - Selects DMA to peripheral connection for DMA peripheral 11."]
    #[inline]
    pub fn dmamuxper11(&mut self) -> _DMAMUXPER11W {
        _DMAMUXPER11W { w: self }
    }
    #[doc = "Bits 24:25 - Select DMA to peripheral connection for DMA peripheral 12."]
    #[inline]
    pub fn dmamuxper12(&mut self) -> _DMAMUXPER12W {
        _DMAMUXPER12W { w: self }
    }
    #[doc = "Bits 26:27 - Select DMA to peripheral connection for DMA peripheral 13."]
    #[inline]
    pub fn dmamuxper13(&mut self) -> _DMAMUXPER13W {
        _DMAMUXPER13W { w: self }
    }
    #[doc = "Bits 28:29 - Select DMA to peripheral connection for DMA peripheral 14."]
    #[inline]
    pub fn dmamuxper14(&mut self) -> _DMAMUXPER14W {
        _DMAMUXPER14W { w: self }
    }
    #[doc = "Bits 30:31 - Select DMA to peripheral connection for DMA peripheral 15."]
    #[inline]
    pub fn dmamuxper15(&mut self) -> _DMAMUXPER15W {
        _DMAMUXPER15W { w: self }
    }
}
