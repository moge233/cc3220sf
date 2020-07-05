#[doc = "Reader of register FLASH_DCDC_PARAMETERS6"]
pub type R = crate::R<u32, super::FLASH_DCDC_PARAMETERS6>;
#[doc = "Writer for register FLASH_DCDC_PARAMETERS6"]
pub type W = crate::W<u32, super::FLASH_DCDC_PARAMETERS6>;
#[doc = "Register FLASH_DCDC_PARAMETERS6 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_DCDC_PARAMETERS6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NA20`"]
pub type NA20_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NA20`"]
pub struct NA20_W<'a> {
    w: &'a mut W,
}
impl<'a> NA20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - NA20"]
    #[inline(always)]
    pub fn na20(&self) -> NA20_R {
        NA20_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - NA20"]
    #[inline(always)]
    pub fn na20(&mut self) -> NA20_W {
        NA20_W { w: self }
    }
}
