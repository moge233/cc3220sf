#[doc = "Reader of register DIG_DCDC_PARAMETERS1"]
pub type R = crate::R<u32, super::DIG_DCDC_PARAMETERS1>;
#[doc = "Writer for register DIG_DCDC_PARAMETERS1"]
pub type W = crate::W<u32, super::DIG_DCDC_PARAMETERS1>;
#[doc = "Register DIG_DCDC_PARAMETERS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DIG_DCDC_PARAMETERS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NA4`"]
pub type NA4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NA4`"]
pub struct NA4_W<'a> {
    w: &'a mut W,
}
impl<'a> NA4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24 - NA4"]
    #[inline(always)]
    pub fn na4(&self) -> NA4_R {
        NA4_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - NA4"]
    #[inline(always)]
    pub fn na4(&mut self) -> NA4_W {
        NA4_W { w: self }
    }
}
