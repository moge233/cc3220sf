#[doc = "Reader of register TXFMCTL"]
pub type R = crate::R<u32, super::TXFMCTL>;
#[doc = "Writer for register TXFMCTL"]
pub type W = crate::W<u32, super::TXFMCTL>;
#[doc = "Register TXFMCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TXFMCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XMOD`"]
pub type XMOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XMOD`"]
pub struct XMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> XMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 7)) | (((value as u32) & 0x01ff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:15 - XMT Frame sync mode"]
    #[inline(always)]
    pub fn xmod(&self) -> XMOD_R {
        XMOD_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 7:15 - XMT Frame sync mode"]
    #[inline(always)]
    pub fn xmod(&mut self) -> XMOD_W {
        XMOD_W { w: self }
    }
}
