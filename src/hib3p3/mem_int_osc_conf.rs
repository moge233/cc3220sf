#[doc = "Reader of register MEM_INT_OSC_CONF"]
pub type R = crate::R<u32, super::MEM_INT_OSC_CONF>;
#[doc = "Writer for register MEM_INT_OSC_CONF"]
pub type W = crate::W<u32, super::MEM_INT_OSC_CONF>;
#[doc = "Register MEM_INT_OSC_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_INT_OSC_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_CM_INTOSC_32K_SPARE`"]
pub type MEM_CM_INTOSC_32K_SPARE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_CM_INTOSC_32K_SPARE`"]
pub struct MEM_CM_INTOSC_32K_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CM_INTOSC_32K_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 9)) | (((value as u32) & 0x3f) << 9);
        self.w
    }
}
#[doc = "Reader of field `MEM_CM_INTOSC_32K_TRIM`"]
pub type MEM_CM_INTOSC_32K_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_CM_INTOSC_32K_TRIM`"]
pub struct MEM_CM_INTOSC_32K_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CM_INTOSC_32K_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 1)) | (((value as u32) & 0x3f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:14 - MEM_CM_INTOSC_32K_SPARE"]
    #[inline(always)]
    pub fn mem_cm_intosc_32k_spare(&self) -> MEM_CM_INTOSC_32K_SPARE_R {
        MEM_CM_INTOSC_32K_SPARE_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bits 1:6 - MEM_CM_INTOSC_32K_TRIM"]
    #[inline(always)]
    pub fn mem_cm_intosc_32k_trim(&self) -> MEM_CM_INTOSC_32K_TRIM_R {
        MEM_CM_INTOSC_32K_TRIM_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 9:14 - MEM_CM_INTOSC_32K_SPARE"]
    #[inline(always)]
    pub fn mem_cm_intosc_32k_spare(&mut self) -> MEM_CM_INTOSC_32K_SPARE_W {
        MEM_CM_INTOSC_32K_SPARE_W { w: self }
    }
    #[doc = "Bits 1:6 - MEM_CM_INTOSC_32K_TRIM"]
    #[inline(always)]
    pub fn mem_cm_intosc_32k_trim(&mut self) -> MEM_CM_INTOSC_32K_TRIM_W {
        MEM_CM_INTOSC_32K_TRIM_W { w: self }
    }
}
