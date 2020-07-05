#[doc = "Reader of register ANA_DCDC_PARAMETERS1"]
pub type R = crate::R<u32, super::ANA_DCDC_PARAMETERS1>;
#[doc = "Writer for register ANA_DCDC_PARAMETERS1"]
pub type W = crate::W<u32, super::ANA_DCDC_PARAMETERS1>;
#[doc = "Register ANA_DCDC_PARAMETERS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ANA_DCDC_PARAMETERS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_NFET_SEL_LOWV`"]
pub type MEM_DCDC_ANA_NFET_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_NFET_SEL_LOWV`"]
pub struct MEM_DCDC_ANA_NFET_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_NFET_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_PDRV_STAGGER_CTRL_LOWV`"]
pub type MEM_DCDC_ANA_PDRV_STAGGER_CTRL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_PDRV_STAGGER_CTRL_LOWV`"]
pub struct MEM_DCDC_ANA_PDRV_STAGGER_CTRL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_PDRV_STAGGER_CTRL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_NDRV_STAGGER_CTRL_LOWV`"]
pub type MEM_DCDC_ANA_NDRV_STAGGER_CTRL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_NDRV_STAGGER_CTRL_LOWV`"]
pub struct MEM_DCDC_ANA_NDRV_STAGGER_CTRL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_NDRV_STAGGER_CTRL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_PDRV_STR_SEL_LOWV`"]
pub type MEM_DCDC_ANA_PDRV_STR_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_PDRV_STR_SEL_LOWV`"]
pub struct MEM_DCDC_ANA_PDRV_STR_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_PDRV_STR_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_NDRV_STR_SEL_LOWV`"]
pub type MEM_DCDC_ANA_NDRV_STR_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_NDRV_STR_SEL_LOWV`"]
pub struct MEM_DCDC_ANA_NDRV_STR_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_NDRV_STR_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_RAMP_HGT_LOWV`"]
pub type MEM_DCDC_ANA_RAMP_HGT_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_RAMP_HGT_LOWV`"]
pub struct MEM_DCDC_ANA_RAMP_HGT_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_RAMP_HGT_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 9)) | (((value as u32) & 0x1f) << 9);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_HI_CLAMP_TRIM_LOWV`"]
pub type MEM_DCDC_ANA_HI_CLAMP_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_HI_CLAMP_TRIM_LOWV`"]
pub struct MEM_DCDC_ANA_HI_CLAMP_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_HI_CLAMP_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_LO_CLAMP_TRIM_LOWV`"]
pub type MEM_DCDC_ANA_LO_CLAMP_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_LO_CLAMP_TRIM_LOWV`"]
pub struct MEM_DCDC_ANA_LO_CLAMP_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_LO_CLAMP_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `NA8`"]
pub type NA8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NA8`"]
pub struct NA8_W<'a> {
    w: &'a mut W,
}
impl<'a> NA8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - MEM_DCDC_ANA_NFET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_nfet_sel_lowv(&self) -> MEM_DCDC_ANA_NFET_SEL_LOWV_R {
        MEM_DCDC_ANA_NFET_SEL_LOWV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 26:27 - MEM_DCDC_ANA_PDRV_STAGGER_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_pdrv_stagger_ctrl_lowv(&self) -> MEM_DCDC_ANA_PDRV_STAGGER_CTRL_LOWV_R {
        MEM_DCDC_ANA_PDRV_STAGGER_CTRL_LOWV_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - MEM_DCDC_ANA_NDRV_STAGGER_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ndrv_stagger_ctrl_lowv(&self) -> MEM_DCDC_ANA_NDRV_STAGGER_CTRL_LOWV_R {
        MEM_DCDC_ANA_NDRV_STAGGER_CTRL_LOWV_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:23 - MEM_DCDC_ANA_PDRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_pdrv_str_sel_lowv(&self) -> MEM_DCDC_ANA_PDRV_STR_SEL_LOWV_R {
        MEM_DCDC_ANA_PDRV_STR_SEL_LOWV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - MEM_DCDC_ANA_NDRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ndrv_str_sel_lowv(&self) -> MEM_DCDC_ANA_NDRV_STR_SEL_LOWV_R {
        MEM_DCDC_ANA_NDRV_STR_SEL_LOWV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 9:13 - MEM_DCDC_ANA_RAMP_HGT_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ramp_hgt_lowv(&self) -> MEM_DCDC_ANA_RAMP_HGT_LOWV_R {
        MEM_DCDC_ANA_RAMP_HGT_LOWV_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - MEM_DCDC_ANA_HI_CLAMP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_hi_clamp_trim_lowv(&self) -> MEM_DCDC_ANA_HI_CLAMP_TRIM_LOWV_R {
        MEM_DCDC_ANA_HI_CLAMP_TRIM_LOWV_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - MEM_DCDC_ANA_LO_CLAMP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_lo_clamp_trim_lowv(&self) -> MEM_DCDC_ANA_LO_CLAMP_TRIM_LOWV_R {
        MEM_DCDC_ANA_LO_CLAMP_TRIM_LOWV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - NA8"]
    #[inline(always)]
    pub fn na8(&self) -> NA8_R {
        NA8_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - MEM_DCDC_ANA_NFET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_nfet_sel_lowv(&mut self) -> MEM_DCDC_ANA_NFET_SEL_LOWV_W {
        MEM_DCDC_ANA_NFET_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 26:27 - MEM_DCDC_ANA_PDRV_STAGGER_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_pdrv_stagger_ctrl_lowv(&mut self) -> MEM_DCDC_ANA_PDRV_STAGGER_CTRL_LOWV_W {
        MEM_DCDC_ANA_PDRV_STAGGER_CTRL_LOWV_W { w: self }
    }
    #[doc = "Bits 24:25 - MEM_DCDC_ANA_NDRV_STAGGER_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ndrv_stagger_ctrl_lowv(&mut self) -> MEM_DCDC_ANA_NDRV_STAGGER_CTRL_LOWV_W {
        MEM_DCDC_ANA_NDRV_STAGGER_CTRL_LOWV_W { w: self }
    }
    #[doc = "Bits 20:23 - MEM_DCDC_ANA_PDRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_pdrv_str_sel_lowv(&mut self) -> MEM_DCDC_ANA_PDRV_STR_SEL_LOWV_W {
        MEM_DCDC_ANA_PDRV_STR_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 16:19 - MEM_DCDC_ANA_NDRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ndrv_str_sel_lowv(&mut self) -> MEM_DCDC_ANA_NDRV_STR_SEL_LOWV_W {
        MEM_DCDC_ANA_NDRV_STR_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 9:13 - MEM_DCDC_ANA_RAMP_HGT_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_ramp_hgt_lowv(&mut self) -> MEM_DCDC_ANA_RAMP_HGT_LOWV_W {
        MEM_DCDC_ANA_RAMP_HGT_LOWV_W { w: self }
    }
    #[doc = "Bits 5:6 - MEM_DCDC_ANA_HI_CLAMP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_hi_clamp_trim_lowv(&mut self) -> MEM_DCDC_ANA_HI_CLAMP_TRIM_LOWV_W {
        MEM_DCDC_ANA_HI_CLAMP_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 2:3 - MEM_DCDC_ANA_LO_CLAMP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_ana_lo_clamp_trim_lowv(&mut self) -> MEM_DCDC_ANA_LO_CLAMP_TRIM_LOWV_W {
        MEM_DCDC_ANA_LO_CLAMP_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 0:1 - NA8"]
    #[inline(always)]
    pub fn na8(&mut self) -> NA8_W {
        NA8_W { w: self }
    }
}
