#[doc = "Reader of register GPTMTAILR"]
pub type R = crate::R<u32, super::GPTMTAILR>;
#[doc = "Writer for register GPTMTAILR"]
pub type W = crate::W<u32, super::GPTMTAILR>;
#[doc = "Register GPTMTAILR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTMTAILR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAILR`"]
pub type TAILR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TAILR`"]
pub struct TAILR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAILR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPTM Timer A Interval Load Register"]
    #[inline(always)]
    pub fn tailr(&self) -> TAILR_R {
        TAILR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPTM Timer A Interval Load Register"]
    #[inline(always)]
    pub fn tailr(&mut self) -> TAILR_W {
        TAILR_W { w: self }
    }
}
