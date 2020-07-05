#[doc = "Reader of register REF_SLICER_CONTROLS1"]
pub type R = crate::R<u32, super::REF_SLICER_CONTROLS1>;
#[doc = "Writer for register REF_SLICER_CONTROLS1"]
pub type W = crate::W<u32, super::REF_SLICER_CONTROLS1>;
#[doc = "Register REF_SLICER_CONTROLS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::REF_SLICER_CONTROLS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLICER_SPARE1`"]
pub type SLICER_SPARE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLICER_SPARE1`"]
pub struct SLICER_SPARE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLICER_SPARE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `XOSC_TRIM`"]
pub type XOSC_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSC_TRIM`"]
pub struct XOSC_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SLICER_LV_TRIM`"]
pub type SLICER_LV_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLICER_LV_TRIM`"]
pub struct SLICER_LV_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SLICER_LV_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:15 - Slicer spare1. Reaches on port TOP_CLKM_REG2_IN\\[15:10\\]
of gprcm."]
    #[inline(always)]
    pub fn slicer_spare1(&self) -> SLICER_SPARE1_R {
        SLICER_SPARE1_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 4:9 - XOSC Trim. Reaches on port TOP_CLKM_REG2_IN\\[9:4\\]
of gprcm"]
    #[inline(always)]
    pub fn xosc_trim(&self) -> XOSC_TRIM_R {
        XOSC_TRIM_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 0:2 - LV Slicer trim. Reaches on port TOP_CLKM_REG2_IN\\[2:0\\]
of gprcm."]
    #[inline(always)]
    pub fn slicer_lv_trim(&self) -> SLICER_LV_TRIM_R {
        SLICER_LV_TRIM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 10:15 - Slicer spare1. Reaches on port TOP_CLKM_REG2_IN\\[15:10\\]
of gprcm."]
    #[inline(always)]
    pub fn slicer_spare1(&mut self) -> SLICER_SPARE1_W {
        SLICER_SPARE1_W { w: self }
    }
    #[doc = "Bits 4:9 - XOSC Trim. Reaches on port TOP_CLKM_REG2_IN\\[9:4\\]
of gprcm"]
    #[inline(always)]
    pub fn xosc_trim(&mut self) -> XOSC_TRIM_W {
        XOSC_TRIM_W { w: self }
    }
    #[doc = "Bits 0:2 - LV Slicer trim. Reaches on port TOP_CLKM_REG2_IN\\[2:0\\]
of gprcm."]
    #[inline(always)]
    pub fn slicer_lv_trim(&mut self) -> SLICER_LV_TRIM_W {
        SLICER_LV_TRIM_W { w: self }
    }
}
