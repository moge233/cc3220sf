#[doc = "Reader of register GPTMTBMATCHR"]
pub type R = crate::R<u32, super::GPTMTBMATCHR>;
#[doc = "Writer for register GPTMTBMATCHR"]
pub type W = crate::W<u32, super::GPTMTBMATCHR>;
#[doc = "Register GPTMTBMATCHR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTMTBMATCHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBMR`"]
pub type TBMR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TBMR`"]
pub struct TBMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B Match Register"]
    #[inline(always)]
    pub fn tbmr(&self) -> TBMR_R {
        TBMR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer B Match Register"]
    #[inline(always)]
    pub fn tbmr(&mut self) -> TBMR_W {
        TBMR_W { w: self }
    }
}
