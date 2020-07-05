#[doc = "Reader of register ANA_DCDC_PARAMETERS0"]
pub type R = crate::R<u32, super::ANA_DCDC_PARAMETERS0>;
#[doc = "Writer for register ANA_DCDC_PARAMETERS0"]
pub type W = crate::W<u32, super::ANA_DCDC_PARAMETERS0>;
#[doc = "Register ANA_DCDC_PARAMETERS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ANA_DCDC_PARAMETERS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_VTRIM_LOWV`"]
pub type MEM_DCDC_ANA_VTRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_VTRIM_LOWV`"]
pub struct MEM_DCDC_ANA_VTRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_VTRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 17)) | (((value as u32) & 0x0f) << 17);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_PFM_RIPPLE_TRIM_LOWV`"]
pub type MEM_DCDC_ANA_PFM_RIPPLE_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_PFM_RIPPLE_TRIM_LOWV`"]
pub struct MEM_DCDC_ANA_PFM_RIPPLE_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_PFM_RIPPLE_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_IQ_CTRL_LOWV`"]
pub type MEM_DCDC_ANA_IQ_CTRL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_IQ_CTRL_LOWV`"]
pub struct MEM_DCDC_ANA_IQ_CTRL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_IQ_CTRL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_NON_OV_CTRL_LOWV`"]
pub type MEM_DCDC_ANA_NON_OV_CTRL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_NON_OV_CTRL_LOWV`"]
pub struct MEM_DCDC_ANA_NON_OV_CTRL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_NON_OV_CTRL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_SLP_DRV_DLY_SEL_LOWV`"]
pub type MEM_DCDC_ANA_SLP_DRV_DLY_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_SLP_DRV_DLY_SEL_LOWV`"]
pub struct MEM_DCDC_ANA_SLP_DRV_DLY_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_SLP_DRV_DLY_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_PFET_SEL_LOWV`"]
pub type MEM_DCDC_ANA_PFET_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_PFET_SEL_LOWV`"]
pub struct MEM_DCDC_ANA_PFET_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_PFET_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 17:20 - MEM_DCDC_ANA_VTRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_vtrim_lowv(&self) -> MEM_DCDC_ANA_VTRIM_LOWV_R {
        MEM_DCDC_ANA_VTRIM_LOWV_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 15:16 - MEM_DCDC_ANA_PFM_RIPPLE_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_pfm_ripple_trim_lowv(&self) -> MEM_DCDC_ANA_PFM_RIPPLE_TRIM_LOWV_R {
        MEM_DCDC_ANA_PFM_RIPPLE_TRIM_LOWV_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - MEM_DCDC_ANA_IQ_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_iq_ctrl_lowv(&self) -> MEM_DCDC_ANA_IQ_CTRL_LOWV_R {
        MEM_DCDC_ANA_IQ_CTRL_LOWV_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - MEM_DCDC_ANA_NON_OV_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_non_ov_ctrl_lowv(&self) -> MEM_DCDC_ANA_NON_OV_CTRL_LOWV_R {
        MEM_DCDC_ANA_NON_OV_CTRL_LOWV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MEM_DCDC_ANA_SLP_DRV_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_slp_drv_dly_sel_lowv(&self) -> MEM_DCDC_ANA_SLP_DRV_DLY_SEL_LOWV_R {
        MEM_DCDC_ANA_SLP_DRV_DLY_SEL_LOWV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - MEM_DCDC_ANA_PFET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_pfet_sel_lowv(&self) -> MEM_DCDC_ANA_PFET_SEL_LOWV_R {
        MEM_DCDC_ANA_PFET_SEL_LOWV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 17:20 - MEM_DCDC_ANA_VTRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_vtrim_lowv(&mut self) -> MEM_DCDC_ANA_VTRIM_LOWV_W {
        MEM_DCDC_ANA_VTRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 15:16 - MEM_DCDC_ANA_PFM_RIPPLE_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_pfm_ripple_trim_lowv(&mut self) -> MEM_DCDC_ANA_PFM_RIPPLE_TRIM_LOWV_W {
        MEM_DCDC_ANA_PFM_RIPPLE_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 13:14 - MEM_DCDC_ANA_IQ_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_iq_ctrl_lowv(&mut self) -> MEM_DCDC_ANA_IQ_CTRL_LOWV_W {
        MEM_DCDC_ANA_IQ_CTRL_LOWV_W { w: self }
    }
    #[doc = "Bits 8:11 - MEM_DCDC_ANA_NON_OV_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_non_ov_ctrl_lowv(&mut self) -> MEM_DCDC_ANA_NON_OV_CTRL_LOWV_W {
        MEM_DCDC_ANA_NON_OV_CTRL_LOWV_W { w: self }
    }
    #[doc = "Bits 4:7 - MEM_DCDC_ANA_SLP_DRV_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_slp_drv_dly_sel_lowv(&mut self) -> MEM_DCDC_ANA_SLP_DRV_DLY_SEL_LOWV_W {
        MEM_DCDC_ANA_SLP_DRV_DLY_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 0:3 - MEM_DCDC_ANA_PFET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_pfet_sel_lowv(&mut self) -> MEM_DCDC_ANA_PFET_SEL_LOWV_W {
        MEM_DCDC_ANA_PFET_SEL_LOWV_W { w: self }
    }
}
