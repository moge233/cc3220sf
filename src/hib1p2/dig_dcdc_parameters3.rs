#[doc = "Reader of register DIG_DCDC_PARAMETERS3"]
pub type R = crate::R<u32, super::DIG_DCDC_PARAMETERS3>;
#[doc = "Writer for register DIG_DCDC_PARAMETERS3"]
pub type W = crate::W<u32, super::DIG_DCDC_PARAMETERS3>;
#[doc = "Register DIG_DCDC_PARAMETERS3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DIG_DCDC_PARAMETERS3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_COT_CTRL_LOWV`"]
pub type MEM_DCDC_DIG_COT_CTRL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_COT_CTRL_LOWV`"]
pub struct MEM_DCDC_DIG_COT_CTRL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_COT_CTRL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 23)) | (((value as u32) & 0xff) << 23);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_ILIM_TRIM_LOWV_OVERRIDE`"]
pub type MEM_DCDC_DIG_ILIM_TRIM_LOWV_OVERRIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_ILIM_TRIM_LOWV_OVERRIDE`"]
pub struct MEM_DCDC_DIG_ILIM_TRIM_LOWV_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_ILIM_TRIM_LOWV_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 13)) | (((value as u32) & 0xff) << 13);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_ILIM_MASK_DLY_SEL_LOWV`"]
pub type MEM_DCDC_DIG_ILIM_MASK_DLY_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_ILIM_MASK_DLY_SEL_LOWV`"]
pub struct MEM_DCDC_DIG_ILIM_MASK_DLY_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_ILIM_MASK_DLY_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_NCOMP_TRIM_LOWV`"]
pub type MEM_DCDC_DIG_NCOMP_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_NCOMP_TRIM_LOWV`"]
pub struct MEM_DCDC_DIG_NCOMP_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_NCOMP_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_NCOMP_MASK_DLY_SEL_LOWV`"]
pub type MEM_DCDC_DIG_NCOMP_MASK_DLY_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_NCOMP_MASK_DLY_SEL_LOWV`"]
pub struct MEM_DCDC_DIG_NCOMP_MASK_DLY_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_NCOMP_MASK_DLY_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:30 - MEM_DCDC_DIG_COT_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_cot_ctrl_lowv(&self) -> MEM_DCDC_DIG_COT_CTRL_LOWV_R {
        MEM_DCDC_DIG_COT_CTRL_LOWV_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bits 13:20 - Override value for DCDC_DIG_ILIM_TRIM : Applicable only when bit \\[25\\]
of DIG_DCDC_PARAMETERS1 \\[0x000C\\]
is set to 1"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ilim_trim_lowv_override(&self) -> MEM_DCDC_DIG_ILIM_TRIM_LOWV_OVERRIDE_R {
        MEM_DCDC_DIG_ILIM_TRIM_LOWV_OVERRIDE_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 11:12 - MEM_DCDC_DIG_ILIM_MASK_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ilim_mask_dly_sel_lowv(&self) -> MEM_DCDC_DIG_ILIM_MASK_DLY_SEL_LOWV_R {
        MEM_DCDC_DIG_ILIM_MASK_DLY_SEL_LOWV_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 4:8 - MEM_DCDC_DIG_NCOMP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ncomp_trim_lowv(&self) -> MEM_DCDC_DIG_NCOMP_TRIM_LOWV_R {
        MEM_DCDC_DIG_NCOMP_TRIM_LOWV_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 2:3 - MEM_DCDC_DIG_NCOMP_MASK_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ncomp_mask_dly_sel_lowv(&self) -> MEM_DCDC_DIG_NCOMP_MASK_DLY_SEL_LOWV_R {
        MEM_DCDC_DIG_NCOMP_MASK_DLY_SEL_LOWV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 23:30 - MEM_DCDC_DIG_COT_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_cot_ctrl_lowv(&mut self) -> MEM_DCDC_DIG_COT_CTRL_LOWV_W {
        MEM_DCDC_DIG_COT_CTRL_LOWV_W { w: self }
    }
    #[doc = "Bits 13:20 - Override value for DCDC_DIG_ILIM_TRIM : Applicable only when bit \\[25\\]
of DIG_DCDC_PARAMETERS1 \\[0x000C\\]
is set to 1"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ilim_trim_lowv_override(
        &mut self,
    ) -> MEM_DCDC_DIG_ILIM_TRIM_LOWV_OVERRIDE_W {
        MEM_DCDC_DIG_ILIM_TRIM_LOWV_OVERRIDE_W { w: self }
    }
    #[doc = "Bits 11:12 - MEM_DCDC_DIG_ILIM_MASK_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ilim_mask_dly_sel_lowv(&mut self) -> MEM_DCDC_DIG_ILIM_MASK_DLY_SEL_LOWV_W {
        MEM_DCDC_DIG_ILIM_MASK_DLY_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 4:8 - MEM_DCDC_DIG_NCOMP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ncomp_trim_lowv(&mut self) -> MEM_DCDC_DIG_NCOMP_TRIM_LOWV_W {
        MEM_DCDC_DIG_NCOMP_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 2:3 - MEM_DCDC_DIG_NCOMP_MASK_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ncomp_mask_dly_sel_lowv(
        &mut self,
    ) -> MEM_DCDC_DIG_NCOMP_MASK_DLY_SEL_LOWV_W {
        MEM_DCDC_DIG_NCOMP_MASK_DLY_SEL_LOWV_W { w: self }
    }
}
