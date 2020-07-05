#[doc = "Reader of register RXFMCTL"]
pub type R = crate::R<u32, super::RXFMCTL>;
#[doc = "Writer for register RXFMCTL"]
pub type W = crate::W<u32, super::RXFMCTL>;
#[doc = "Register RXFMCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RXFMCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMOD`"]
pub type RMOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RMOD`"]
pub struct RMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 7)) | (((value as u32) & 0x01ff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:15 - RCV Frame sync mode"]
    #[inline(always)]
    pub fn rmod(&self) -> RMOD_R {
        RMOD_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 7:15 - RCV Frame sync mode"]
    #[inline(always)]
    pub fn rmod(&mut self) -> RMOD_W {
        RMOD_W { w: self }
    }
}
