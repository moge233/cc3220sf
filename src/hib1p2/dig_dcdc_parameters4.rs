#[doc = "Reader of register DIG_DCDC_PARAMETERS4"]
pub type R = crate::R<u32, super::DIG_DCDC_PARAMETERS4>;
#[doc = "Writer for register DIG_DCDC_PARAMETERS4"]
pub type W = crate::W<u32, super::DIG_DCDC_PARAMETERS4>;
#[doc = "Register DIG_DCDC_PARAMETERS4 `reset()`'s with value 0"]
impl crate::ResetValue for super::DIG_DCDC_PARAMETERS4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NA7`"]
pub type NA7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NA7`"]
pub struct NA7_W<'a> {
    w: &'a mut W,
}
impl<'a> NA7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - NA7"]
    #[inline(always)]
    pub fn na7(&self) -> NA7_R {
        NA7_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - NA7"]
    #[inline(always)]
    pub fn na7(&mut self) -> NA7_W {
        NA7_W { w: self }
    }
}
