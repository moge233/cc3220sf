#[doc = "Reader of register DIG_DCDC_PARAMETERS0"]
pub type R = crate::R<u32, super::DIG_DCDC_PARAMETERS0>;
#[doc = "Writer for register DIG_DCDC_PARAMETERS0"]
pub type W = crate::W<u32, super::DIG_DCDC_PARAMETERS0>;
#[doc = "Register DIG_DCDC_PARAMETERS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DIG_DCDC_PARAMETERS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_VTRIM_LOWV_OVERRIDE`"]
pub type MEM_DCDC_DIG_VTRIM_LOWV_OVERRIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_VTRIM_LOWV_OVERRIDE`"]
pub struct MEM_DCDC_DIG_VTRIM_LOWV_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_VTRIM_LOWV_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_PFM_RIPPLE_TRIM_LOWV`"]
pub type MEM_DCDC_DIG_PFM_RIPPLE_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_PFM_RIPPLE_TRIM_LOWV`"]
pub struct MEM_DCDC_DIG_PFM_RIPPLE_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_PFM_RIPPLE_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_IQ_CTRL_LOWV`"]
pub type MEM_DCDC_DIG_IQ_CTRL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_IQ_CTRL_LOWV`"]
pub struct MEM_DCDC_DIG_IQ_CTRL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_IQ_CTRL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_NON_OV_CTRL_LOWV`"]
pub type MEM_DCDC_DIG_NON_OV_CTRL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_NON_OV_CTRL_LOWV`"]
pub struct MEM_DCDC_DIG_NON_OV_CTRL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_NON_OV_CTRL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_SLP_DRV_DLY_SEL_LOWV`"]
pub type MEM_DCDC_DIG_SLP_DRV_DLY_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_SLP_DRV_DLY_SEL_LOWV`"]
pub struct MEM_DCDC_DIG_SLP_DRV_DLY_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_SLP_DRV_DLY_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "Reader of field `NA3`"]
pub type NA3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NA3`"]
pub struct NA3_W<'a> {
    w: &'a mut W,
}
impl<'a> NA3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:21 - Override value for DCDC_DIG_VTRIM : Applicable only when bit \\[27\\]
of DIG_DCDC_PARAMETERS1 \\[0x000C\\]
is set to 1."]
    #[inline(always)]
    pub fn mem_dcdc_dig_vtrim_lowv_override(&self) -> MEM_DCDC_DIG_VTRIM_LOWV_OVERRIDE_R {
        MEM_DCDC_DIG_VTRIM_LOWV_OVERRIDE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - MEM_DCDC_DIG_PFM_RIPPLE_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_pfm_ripple_trim_lowv(&self) -> MEM_DCDC_DIG_PFM_RIPPLE_TRIM_LOWV_R {
        MEM_DCDC_DIG_PFM_RIPPLE_TRIM_LOWV_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - MEM_DCDC_DIG_IQ_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_iq_ctrl_lowv(&self) -> MEM_DCDC_DIG_IQ_CTRL_LOWV_R {
        MEM_DCDC_DIG_IQ_CTRL_LOWV_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 7:10 - MEM_DCDC_DIG_NON_OV_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_non_ov_ctrl_lowv(&self) -> MEM_DCDC_DIG_NON_OV_CTRL_LOWV_R {
        MEM_DCDC_DIG_NON_OV_CTRL_LOWV_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 3:6 - MEM_DCDC_DIG_SLP_DRV_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_slp_drv_dly_sel_lowv(&self) -> MEM_DCDC_DIG_SLP_DRV_DLY_SEL_LOWV_R {
        MEM_DCDC_DIG_SLP_DRV_DLY_SEL_LOWV_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2 - NA3"]
    #[inline(always)]
    pub fn na3(&self) -> NA3_R {
        NA3_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:21 - Override value for DCDC_DIG_VTRIM : Applicable only when bit \\[27\\]
of DIG_DCDC_PARAMETERS1 \\[0x000C\\]
is set to 1."]
    #[inline(always)]
    pub fn mem_dcdc_dig_vtrim_lowv_override(&mut self) -> MEM_DCDC_DIG_VTRIM_LOWV_OVERRIDE_W {
        MEM_DCDC_DIG_VTRIM_LOWV_OVERRIDE_W { w: self }
    }
    #[doc = "Bits 14:15 - MEM_DCDC_DIG_PFM_RIPPLE_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_pfm_ripple_trim_lowv(&mut self) -> MEM_DCDC_DIG_PFM_RIPPLE_TRIM_LOWV_W {
        MEM_DCDC_DIG_PFM_RIPPLE_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 12:13 - MEM_DCDC_DIG_IQ_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_iq_ctrl_lowv(&mut self) -> MEM_DCDC_DIG_IQ_CTRL_LOWV_W {
        MEM_DCDC_DIG_IQ_CTRL_LOWV_W { w: self }
    }
    #[doc = "Bits 7:10 - MEM_DCDC_DIG_NON_OV_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_non_ov_ctrl_lowv(&mut self) -> MEM_DCDC_DIG_NON_OV_CTRL_LOWV_W {
        MEM_DCDC_DIG_NON_OV_CTRL_LOWV_W { w: self }
    }
    #[doc = "Bits 3:6 - MEM_DCDC_DIG_SLP_DRV_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_slp_drv_dly_sel_lowv(&mut self) -> MEM_DCDC_DIG_SLP_DRV_DLY_SEL_LOWV_W {
        MEM_DCDC_DIG_SLP_DRV_DLY_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 0:2 - NA3"]
    #[inline(always)]
    pub fn na3(&mut self) -> NA3_W {
        NA3_W { w: self }
    }
}
