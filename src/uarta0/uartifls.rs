#[doc = "Reader of register UARTIFLS"]
pub type R = crate::R<u32, super::UARTIFLS>;
#[doc = "Writer for register UARTIFLS"]
pub type W = crate::W<u32, super::UARTIFLS>;
#[doc = "Register UARTIFLS `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTIFLS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXIFLSEL`"]
pub type RXIFLSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXIFLSEL`"]
pub struct RXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIFLSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `TXIFLSEL`"]
pub type TXIFLSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXIFLSEL`"]
pub struct TXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIFLSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RXIFLSEL_R {
        RXIFLSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TXIFLSEL_R {
        TXIFLSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RXIFLSEL_W {
        RXIFLSEL_W { w: self }
    }
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TXIFLSEL_W {
        TXIFLSEL_W { w: self }
    }
}
