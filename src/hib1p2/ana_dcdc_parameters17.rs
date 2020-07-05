#[doc = "Reader of register ANA_DCDC_PARAMETERS17"]
pub type R = crate::R<u32, super::ANA_DCDC_PARAMETERS17>;
#[doc = "Writer for register ANA_DCDC_PARAMETERS17"]
pub type W = crate::W<u32, super::ANA_DCDC_PARAMETERS17>;
#[doc = "Register ANA_DCDC_PARAMETERS17 `reset()`'s with value 0"]
impl crate::ResetValue for super::ANA_DCDC_PARAMETERS17 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NA17`"]
pub type NA17_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NA17`"]
pub struct NA17_W<'a> {
    w: &'a mut W,
}
impl<'a> NA17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - NA17"]
    #[inline(always)]
    pub fn na17(&self) -> NA17_R {
        NA17_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - NA17"]
    #[inline(always)]
    pub fn na17(&mut self) -> NA17_W {
        NA17_W { w: self }
    }
}
