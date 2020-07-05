#[doc = "Reader of register CM_OSC_16M_CONFIG"]
pub type R = crate::R<u32, super::CM_OSC_16M_CONFIG>;
#[doc = "Writer for register CM_OSC_16M_CONFIG"]
pub type W = crate::W<u32, super::CM_OSC_16M_CONFIG>;
#[doc = "Register CM_OSC_16M_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::CM_OSC_16M_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_CM_OSC_16M_TRIM`"]
pub type MEM_CM_OSC_16M_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_CM_OSC_16M_TRIM`"]
pub struct MEM_CM_OSC_16M_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CM_OSC_16M_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `MEM_CM_OSC_16M_SPARE`"]
pub type MEM_CM_OSC_16M_SPARE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_CM_OSC_16M_SPARE`"]
pub struct MEM_CM_OSC_16M_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CM_OSC_16M_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MEM_CM_SLI_16M_TRIM`"]
pub type MEM_CM_SLI_16M_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_CM_SLI_16M_TRIM`"]
pub struct MEM_CM_SLI_16M_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CM_SLI_16M_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:15 - MEM_CM_OSC_16M_TRIM"]
    #[inline(always)]
    pub fn mem_cm_osc_16m_trim(&self) -> MEM_CM_OSC_16M_TRIM_R {
        MEM_CM_OSC_16M_TRIM_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 4:9 - MEM_CM_OSC_16M_SPARE"]
    #[inline(always)]
    pub fn mem_cm_osc_16m_spare(&self) -> MEM_CM_OSC_16M_SPARE_R {
        MEM_CM_OSC_16M_SPARE_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 0:2 - MEM_CM_SLI_16M_TRIM"]
    #[inline(always)]
    pub fn mem_cm_sli_16m_trim(&self) -> MEM_CM_SLI_16M_TRIM_R {
        MEM_CM_SLI_16M_TRIM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 10:15 - MEM_CM_OSC_16M_TRIM"]
    #[inline(always)]
    pub fn mem_cm_osc_16m_trim(&mut self) -> MEM_CM_OSC_16M_TRIM_W {
        MEM_CM_OSC_16M_TRIM_W { w: self }
    }
    #[doc = "Bits 4:9 - MEM_CM_OSC_16M_SPARE"]
    #[inline(always)]
    pub fn mem_cm_osc_16m_spare(&mut self) -> MEM_CM_OSC_16M_SPARE_W {
        MEM_CM_OSC_16M_SPARE_W { w: self }
    }
    #[doc = "Bits 0:2 - MEM_CM_SLI_16M_TRIM"]
    #[inline(always)]
    pub fn mem_cm_sli_16m_trim(&mut self) -> MEM_CM_SLI_16M_TRIM_W {
        MEM_CM_SLI_16M_TRIM_W { w: self }
    }
}
