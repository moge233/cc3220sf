#[doc = "Reader of register UARTLSS"]
pub type R = crate::R<u32, super::UARTLSS>;
#[doc = "Writer for register UARTLSS"]
pub type W = crate::W<u32, super::UARTLSS>;
#[doc = "Register UARTLSS `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTLSS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSS`"]
pub type TSS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSS`"]
pub struct TSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer Snap Shot"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Snap Shot"]
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W {
        TSS_W { w: self }
    }
}
