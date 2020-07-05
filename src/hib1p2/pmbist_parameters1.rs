#[doc = "Reader of register PMBIST_PARAMETERS1"]
pub type R = crate::R<u32, super::PMBIST_PARAMETERS1>;
#[doc = "Writer for register PMBIST_PARAMETERS1"]
pub type W = crate::W<u32, super::PMBIST_PARAMETERS1>;
#[doc = "Register PMBIST_PARAMETERS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PMBIST_PARAMETERS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_PM_BIST_SPARE_LOWV`"]
pub type MEM_PM_BIST_SPARE_LOWV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_PM_BIST_SPARE_LOWV`"]
pub struct MEM_PM_BIST_SPARE_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_PM_BIST_SPARE_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `NA22`"]
pub type NA22_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NA22`"]
pub struct NA22_W<'a> {
    w: &'a mut W,
}
impl<'a> NA22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - MEM_PM_BIST_SPARE_LOWV"]
    #[inline(always)]
    pub fn mem_pm_bist_spare_lowv(&self) -> MEM_PM_BIST_SPARE_LOWV_R {
        MEM_PM_BIST_SPARE_LOWV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:14 - NA22"]
    #[inline(always)]
    pub fn na22(&self) -> NA22_R {
        NA22_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - MEM_PM_BIST_SPARE_LOWV"]
    #[inline(always)]
    pub fn mem_pm_bist_spare_lowv(&mut self) -> MEM_PM_BIST_SPARE_LOWV_W {
        MEM_PM_BIST_SPARE_LOWV_W { w: self }
    }
    #[doc = "Bits 0:14 - NA22"]
    #[inline(always)]
    pub fn na22(&mut self) -> NA22_W {
        NA22_W { w: self }
    }
}
