#[doc = "Reader of register UARTCC"]
pub type R = crate::R<u32, super::UARTCC>;
#[doc = "Writer for register UARTCC"]
pub type W = crate::W<u32, super::UARTCC>;
#[doc = "Register UARTCC `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS`"]
pub type CS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CS`"]
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - UART Baud Clock Source 0x00000005 : UART_CC_CS_PIOSC : PIOSC 0x00000000 : UART_CC_CS_SYSCLK : The system clock (default)"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - UART Baud Clock Source 0x00000005 : UART_CC_CS_PIOSC : PIOSC 0x00000000 : UART_CC_CS_SYSCLK : The system clock (default)"]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
}
