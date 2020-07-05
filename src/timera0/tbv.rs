#[doc = "Reader of register TBV"]
pub type R = crate::R<u32, super::TBV>;
#[doc = "Writer for register TBV"]
pub type W = crate::W<u32, super::TBV>;
#[doc = "Register TBV `reset()`'s with value 0"]
impl crate::ResetValue for super::TBV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBVL`"]
pub type TBVL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TBVL`"]
pub struct TBVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TBVL_W<'a> {
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
    pub fn tbvl(&self) -> TBVL_R {
        TBVL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer B Register"]
    #[inline(always)]
    pub fn tbvl(&mut self) -> TBVL_W {
        TBVL_W { w: self }
    }
}
