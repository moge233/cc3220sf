#[doc = "Reader of register UARTLCTL"]
pub type R = crate::R<u32, super::UARTLCTL>;
#[doc = "Writer for register UARTLCTL"]
pub type W = crate::W<u32, super::UARTLCTL>;
#[doc = "Register UARTLCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTLCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLEN`"]
pub type BLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLEN`"]
pub struct BLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - BLEN"]
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - BLEN"]
    #[inline(always)]
    pub fn blen(&mut self) -> BLEN_W {
        BLEN_W { w: self }
    }
}
