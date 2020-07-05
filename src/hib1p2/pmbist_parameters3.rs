#[doc = "Reader of register PMBIST_PARAMETERS3"]
pub type R = crate::R<u32, super::PMBIST_PARAMETERS3>;
#[doc = "Writer for register PMBIST_PARAMETERS3"]
pub type W = crate::W<u32, super::PMBIST_PARAMETERS3>;
#[doc = "Register PMBIST_PARAMETERS3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PMBIST_PARAMETERS3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_PMTEST_SPARE_LOWV`"]
pub type MEM_PMTEST_SPARE_LOWV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_PMTEST_SPARE_LOWV`"]
pub struct MEM_PMTEST_SPARE_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_PMTEST_SPARE_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEM_PMTEST_LOAD_TRIM_LOWV`"]
pub type MEM_PMTEST_LOAD_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_PMTEST_LOAD_TRIM_LOWV`"]
pub struct MEM_PMTEST_LOAD_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_PMTEST_LOAD_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `NA23`"]
pub type NA23_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NA23`"]
pub struct NA23_W<'a> {
    w: &'a mut W,
}
impl<'a> NA23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - MEM_PMTEST_SPARE_LOWV"]
    #[inline(always)]
    pub fn mem_pmtest_spare_lowv(&self) -> MEM_PMTEST_SPARE_LOWV_R {
        MEM_PMTEST_SPARE_LOWV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 13:15 - MEM_PMTEST_LOAD_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_pmtest_load_trim_lowv(&self) -> MEM_PMTEST_LOAD_TRIM_LOWV_R {
        MEM_PMTEST_LOAD_TRIM_LOWV_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 0:11 - NA23"]
    #[inline(always)]
    pub fn na23(&self) -> NA23_R {
        NA23_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - MEM_PMTEST_SPARE_LOWV"]
    #[inline(always)]
    pub fn mem_pmtest_spare_lowv(&mut self) -> MEM_PMTEST_SPARE_LOWV_W {
        MEM_PMTEST_SPARE_LOWV_W { w: self }
    }
    #[doc = "Bits 13:15 - MEM_PMTEST_LOAD_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_pmtest_load_trim_lowv(&mut self) -> MEM_PMTEST_LOAD_TRIM_LOWV_W {
        MEM_PMTEST_LOAD_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 0:11 - NA23"]
    #[inline(always)]
    pub fn na23(&mut self) -> NA23_W {
        NA23_W { w: self }
    }
}
