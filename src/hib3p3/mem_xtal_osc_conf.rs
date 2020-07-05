#[doc = "Reader of register MEM_XTAL_OSC_CONF"]
pub type R = crate::R<u32, super::MEM_XTAL_OSC_CONF>;
#[doc = "Writer for register MEM_XTAL_OSC_CONF"]
pub type W = crate::W<u32, super::MEM_XTAL_OSC_CONF>;
#[doc = "Register MEM_XTAL_OSC_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_XTAL_OSC_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_CM_XTAL_TRIM`"]
pub type MEM_CM_XTAL_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_CM_XTAL_TRIM`"]
pub struct MEM_CM_XTAL_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CM_XTAL_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 11)) | (((value as u32) & 0x3f) << 11);
        self.w
    }
}
#[doc = "Reader of field `MEM_CM_SLI_32K_TRIM`"]
pub type MEM_CM_SLI_32K_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_CM_SLI_32K_TRIM`"]
pub struct MEM_CM_SLI_32K_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CM_SLI_32K_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `MEM_CM_FREF_32K_SLICER_ITRIM`"]
pub type MEM_CM_FREF_32K_SLICER_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_CM_FREF_32K_SLICER_ITRIM`"]
pub struct MEM_CM_FREF_32K_SLICER_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CM_FREF_32K_SLICER_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `MEM_CM_EN_INPUT_SENSE`"]
pub type MEM_CM_EN_INPUT_SENSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_CM_EN_INPUT_SENSE`"]
pub struct MEM_CM_EN_INPUT_SENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CM_EN_INPUT_SENSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:16 - MEM_CM_XTAL_TRIM"]
    #[inline(always)]
    pub fn mem_cm_xtal_trim(&self) -> MEM_CM_XTAL_TRIM_R {
        MEM_CM_XTAL_TRIM_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 7:9 - MEM_CM_SLI_32K_TRIM"]
    #[inline(always)]
    pub fn mem_cm_sli_32k_trim(&self) -> MEM_CM_SLI_32K_TRIM_R {
        MEM_CM_SLI_32K_TRIM_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - MEM_CM_FREF_32K_SLICER_ITRIM"]
    #[inline(always)]
    pub fn mem_cm_fref_32k_slicer_itrim(&self) -> MEM_CM_FREF_32K_SLICER_ITRIM_R {
        MEM_CM_FREF_32K_SLICER_ITRIM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 1:2 - MEM_CM_EN_INPUT_SENSE"]
    #[inline(always)]
    pub fn mem_cm_en_input_sense(&self) -> MEM_CM_EN_INPUT_SENSE_R {
        MEM_CM_EN_INPUT_SENSE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 11:16 - MEM_CM_XTAL_TRIM"]
    #[inline(always)]
    pub fn mem_cm_xtal_trim(&mut self) -> MEM_CM_XTAL_TRIM_W {
        MEM_CM_XTAL_TRIM_W { w: self }
    }
    #[doc = "Bits 7:9 - MEM_CM_SLI_32K_TRIM"]
    #[inline(always)]
    pub fn mem_cm_sli_32k_trim(&mut self) -> MEM_CM_SLI_32K_TRIM_W {
        MEM_CM_SLI_32K_TRIM_W { w: self }
    }
    #[doc = "Bits 4:6 - MEM_CM_FREF_32K_SLICER_ITRIM"]
    #[inline(always)]
    pub fn mem_cm_fref_32k_slicer_itrim(&mut self) -> MEM_CM_FREF_32K_SLICER_ITRIM_W {
        MEM_CM_FREF_32K_SLICER_ITRIM_W { w: self }
    }
    #[doc = "Bits 1:2 - MEM_CM_EN_INPUT_SENSE"]
    #[inline(always)]
    pub fn mem_cm_en_input_sense(&mut self) -> MEM_CM_EN_INPUT_SENSE_W {
        MEM_CM_EN_INPUT_SENSE_W { w: self }
    }
}
