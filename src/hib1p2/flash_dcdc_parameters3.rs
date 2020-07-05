#[doc = "Reader of register FLASH_DCDC_PARAMETERS3"]
pub type R = crate::R<u32, super::FLASH_DCDC_PARAMETERS3>;
#[doc = "Writer for register FLASH_DCDC_PARAMETERS3"]
pub type W = crate::W<u32, super::FLASH_DCDC_PARAMETERS3>;
#[doc = "Register FLASH_DCDC_PARAMETERS3 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_DCDC_PARAMETERS3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_RAMP_HGT_LOWV`"]
pub type MEM_DCDC_FLASH_RAMP_HGT_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_RAMP_HGT_LOWV`"]
pub struct MEM_DCDC_FLASH_RAMP_HGT_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_RAMP_HGT_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_VCLAMPH_TRIM_LOWV`"]
pub type MEM_DCDC_FLASH_VCLAMPH_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_VCLAMPH_TRIM_LOWV`"]
pub struct MEM_DCDC_FLASH_VCLAMPH_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_VCLAMPH_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_VCLAMPL_TRIM_LOWV`"]
pub type MEM_DCDC_FLASH_VCLAMPL_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_VCLAMPL_TRIM_LOWV`"]
pub struct MEM_DCDC_FLASH_VCLAMPL_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_VCLAMPL_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_VTRIM_LOWV`"]
pub type MEM_DCDC_FLASH_VTRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_VTRIM_LOWV`"]
pub struct MEM_DCDC_FLASH_VTRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_VTRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | (((value as u32) & 0x0f) << 14);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_PFM_RIPPLE_TRIM_LOWV`"]
pub type MEM_DCDC_FLASH_PFM_RIPPLE_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_PFM_RIPPLE_TRIM_LOWV`"]
pub struct MEM_DCDC_FLASH_PFM_RIPPLE_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_PFM_RIPPLE_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_SLP_DRV_DLY_SEL_LOWV`"]
pub type MEM_DCDC_FLASH_SLP_DRV_DLY_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_SLP_DRV_DLY_SEL_LOWV`"]
pub struct MEM_DCDC_FLASH_SLP_DRV_DLY_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_SLP_DRV_DLY_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `NA19`"]
pub type NA19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NA19`"]
pub struct NA19_W<'a> {
    w: &'a mut W,
}
impl<'a> NA19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28 - MEM_DCDC_FLASH_RAMP_HGT_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_ramp_hgt_lowv(&self) -> MEM_DCDC_FLASH_RAMP_HGT_LOWV_R {
        MEM_DCDC_FLASH_RAMP_HGT_LOWV_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - MEM_DCDC_FLASH_VCLAMPH_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_vclamph_trim_lowv(&self) -> MEM_DCDC_FLASH_VCLAMPH_TRIM_LOWV_R {
        MEM_DCDC_FLASH_VCLAMPH_TRIM_LOWV_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - MEM_DCDC_FLASH_VCLAMPL_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_vclampl_trim_lowv(&self) -> MEM_DCDC_FLASH_VCLAMPL_TRIM_LOWV_R {
        MEM_DCDC_FLASH_VCLAMPL_TRIM_LOWV_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 14:17 - MEM_DCDC_FLASH_VTRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_vtrim_lowv(&self) -> MEM_DCDC_FLASH_VTRIM_LOWV_R {
        MEM_DCDC_FLASH_VTRIM_LOWV_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13 - MEM_DCDC_FLASH_PFM_RIPPLE_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_pfm_ripple_trim_lowv(&self) -> MEM_DCDC_FLASH_PFM_RIPPLE_TRIM_LOWV_R {
        MEM_DCDC_FLASH_PFM_RIPPLE_TRIM_LOWV_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - MEM_DCDC_FLASH_SLP_DRV_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_slp_drv_dly_sel_lowv(&self) -> MEM_DCDC_FLASH_SLP_DRV_DLY_SEL_LOWV_R {
        MEM_DCDC_FLASH_SLP_DRV_DLY_SEL_LOWV_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:4 - NA19"]
    #[inline(always)]
    pub fn na19(&self) -> NA19_R {
        NA19_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - MEM_DCDC_FLASH_RAMP_HGT_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_ramp_hgt_lowv(&mut self) -> MEM_DCDC_FLASH_RAMP_HGT_LOWV_W {
        MEM_DCDC_FLASH_RAMP_HGT_LOWV_W { w: self }
    }
    #[doc = "Bits 21:23 - MEM_DCDC_FLASH_VCLAMPH_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_vclamph_trim_lowv(&mut self) -> MEM_DCDC_FLASH_VCLAMPH_TRIM_LOWV_W {
        MEM_DCDC_FLASH_VCLAMPH_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 18:20 - MEM_DCDC_FLASH_VCLAMPL_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_vclampl_trim_lowv(&mut self) -> MEM_DCDC_FLASH_VCLAMPL_TRIM_LOWV_W {
        MEM_DCDC_FLASH_VCLAMPL_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 14:17 - MEM_DCDC_FLASH_VTRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_vtrim_lowv(&mut self) -> MEM_DCDC_FLASH_VTRIM_LOWV_W {
        MEM_DCDC_FLASH_VTRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 10:13 - MEM_DCDC_FLASH_PFM_RIPPLE_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_pfm_ripple_trim_lowv(&mut self) -> MEM_DCDC_FLASH_PFM_RIPPLE_TRIM_LOWV_W {
        MEM_DCDC_FLASH_PFM_RIPPLE_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 8:9 - MEM_DCDC_FLASH_SLP_DRV_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_slp_drv_dly_sel_lowv(&mut self) -> MEM_DCDC_FLASH_SLP_DRV_DLY_SEL_LOWV_W {
        MEM_DCDC_FLASH_SLP_DRV_DLY_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 0:4 - NA19"]
    #[inline(always)]
    pub fn na19(&mut self) -> NA19_W {
        NA19_W { w: self }
    }
}
