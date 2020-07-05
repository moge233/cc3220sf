#[doc = "Reader of register LCRH"]
pub type R = crate::R<u32, super::LCRH>;
#[doc = "Writer for register LCRH"]
pub type W = crate::W<u32, super::LCRH>;
#[doc = "Register LCRH `reset()`'s with value 0"]
impl crate::ResetValue for super::LCRH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WLEN`"]
pub type WLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WLEN`"]
pub struct WLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:6 - UART Word Length 0x00000000 : UART_LCRH_WLEN_5 : 5 bits (default) 0x00000020 : UART_LCRH_WLEN_6 : 6 bits 0x00000040 : UART_LCRH_WLEN_7 : 7 bits 0x00000060 : UART_LCRH_WLEN_8 : 8 bits"]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 5:6 - UART Word Length 0x00000000 : UART_LCRH_WLEN_5 : 5 bits (default) 0x00000020 : UART_LCRH_WLEN_6 : 6 bits 0x00000040 : UART_LCRH_WLEN_7 : 7 bits 0x00000060 : UART_LCRH_WLEN_8 : 8 bits"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WLEN_W {
        WLEN_W { w: self }
    }
}
