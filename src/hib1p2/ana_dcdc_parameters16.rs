#[doc = "Reader of register ANA_DCDC_PARAMETERS16"]
pub type R = crate::R<u32, super::ANA_DCDC_PARAMETERS16>;
#[doc = "Writer for register ANA_DCDC_PARAMETERS16"]
pub type W = crate::W<u32, super::ANA_DCDC_PARAMETERS16>;
#[doc = "Register ANA_DCDC_PARAMETERS16 `reset()`'s with value 0"]
impl crate::ResetValue for super::ANA_DCDC_PARAMETERS16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_ILIM_TRIM_LOWV_OVERRIDE`"]
pub type MEM_DCDC_ANA_ILIM_TRIM_LOWV_OVERRIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_ILIM_TRIM_LOWV_OVERRIDE`"]
pub struct MEM_DCDC_ANA_ILIM_TRIM_LOWV_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_ILIM_TRIM_LOWV_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | (((value as u32) & 0xff) << 12);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_ILIM_MASK_DLY_SEL_LOWV`"]
pub type MEM_DCDC_ANA_ILIM_MASK_DLY_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_ILIM_MASK_DLY_SEL_LOWV`"]
pub struct MEM_DCDC_ANA_ILIM_MASK_DLY_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_ILIM_MASK_DLY_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_NCOMP_TRIM_LOWV`"]
pub type MEM_DCDC_ANA_NCOMP_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_NCOMP_TRIM_LOWV`"]
pub struct MEM_DCDC_ANA_NCOMP_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_NCOMP_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_NCOMP_MASK_DLY_SEL_LOWV`"]
pub type MEM_DCDC_ANA_NCOMP_MASK_DLY_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_NCOMP_MASK_DLY_SEL_LOWV`"]
pub struct MEM_DCDC_ANA_NCOMP_MASK_DLY_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_NCOMP_MASK_DLY_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:19 - MEM_DCDC_ANA_ILIM_TRIM_LOWV_OVERRIDE"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ilim_trim_lowv_override(&self) -> MEM_DCDC_ANA_ILIM_TRIM_LOWV_OVERRIDE_R {
        MEM_DCDC_ANA_ILIM_TRIM_LOWV_OVERRIDE_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 10:11 - MEM_DCDC_ANA_ILIM_MASK_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ilim_mask_dly_sel_lowv(&self) -> MEM_DCDC_ANA_ILIM_MASK_DLY_SEL_LOWV_R {
        MEM_DCDC_ANA_ILIM_MASK_DLY_SEL_LOWV_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 3:7 - MEM_DCDC_ANA_NCOMP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ncomp_trim_lowv(&self) -> MEM_DCDC_ANA_NCOMP_TRIM_LOWV_R {
        MEM_DCDC_ANA_NCOMP_TRIM_LOWV_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 1:2 - MEM_DCDC_ANA_NCOMP_MASK_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ncomp_mask_dly_sel_lowv(&self) -> MEM_DCDC_ANA_NCOMP_MASK_DLY_SEL_LOWV_R {
        MEM_DCDC_ANA_NCOMP_MASK_DLY_SEL_LOWV_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:19 - MEM_DCDC_ANA_ILIM_TRIM_LOWV_OVERRIDE"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ilim_trim_lowv_override(
        &mut self,
    ) -> MEM_DCDC_ANA_ILIM_TRIM_LOWV_OVERRIDE_W {
        MEM_DCDC_ANA_ILIM_TRIM_LOWV_OVERRIDE_W { w: self }
    }
    #[doc = "Bits 10:11 - MEM_DCDC_ANA_ILIM_MASK_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ilim_mask_dly_sel_lowv(&mut self) -> MEM_DCDC_ANA_ILIM_MASK_DLY_SEL_LOWV_W {
        MEM_DCDC_ANA_ILIM_MASK_DLY_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 3:7 - MEM_DCDC_ANA_NCOMP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ncomp_trim_lowv(&mut self) -> MEM_DCDC_ANA_NCOMP_TRIM_LOWV_W {
        MEM_DCDC_ANA_NCOMP_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 1:2 - MEM_DCDC_ANA_NCOMP_MASK_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ncomp_mask_dly_sel_lowv(
        &mut self,
    ) -> MEM_DCDC_ANA_NCOMP_MASK_DLY_SEL_LOWV_W {
        MEM_DCDC_ANA_NCOMP_MASK_DLY_SEL_LOWV_W { w: self }
    }
}
