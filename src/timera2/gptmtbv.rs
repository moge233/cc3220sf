#[doc = "Reader of register GPTMTBV"]
pub type R = crate::R<u32, super::GPTMTBV>;
#[doc = "Writer for register GPTMTBV"]
pub type W = crate::W<u32, super::GPTMTBV>;
#[doc = "Register GPTMTBV `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTMTBV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBV`"]
pub type TBV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TBV`"]
pub struct TBV_W<'a> {
    w: &'a mut W,
}
impl<'a> TBV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B Register"]
    #[inline(always)]
    pub fn tbv(&self) -> TBV_R {
        TBV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer B Register"]
    #[inline(always)]
    pub fn tbv(&mut self) -> TBV_W {
        TBV_W { w: self }
    }
}
