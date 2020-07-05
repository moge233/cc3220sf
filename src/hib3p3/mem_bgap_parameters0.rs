#[doc = "Reader of register MEM_BGAP_PARAMETERS0"]
pub type R = crate::R<u32, super::MEM_BGAP_PARAMETERS0>;
#[doc = "Writer for register MEM_BGAP_PARAMETERS0"]
pub type W = crate::W<u32, super::MEM_BGAP_PARAMETERS0>;
#[doc = "Register MEM_BGAP_PARAMETERS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_BGAP_PARAMETERS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_VBOK4BG_COMP_TRIM`"]
pub type MEM_VBOK4BG_COMP_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_VBOK4BG_COMP_TRIM`"]
pub struct MEM_VBOK4BG_COMP_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_VBOK4BG_COMP_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Reader of field `MEM_BGAP_SPARE`"]
pub type MEM_BGAP_SPARE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_BGAP_SPARE`"]
pub struct MEM_BGAP_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BGAP_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:16 - MEM_VBOK4BG_COMP_TRIM"]
    #[inline(always)]
    pub fn mem_vbok4bg_comp_trim(&self) -> MEM_VBOK4BG_COMP_TRIM_R {
        MEM_VBOK4BG_COMP_TRIM_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 0:9 - MEM_BGAP_SPARE"]
    #[inline(always)]
    pub fn mem_bgap_spare(&self) -> MEM_BGAP_SPARE_R {
        MEM_BGAP_SPARE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 14:16 - MEM_VBOK4BG_COMP_TRIM"]
    #[inline(always)]
    pub fn mem_vbok4bg_comp_trim(&mut self) -> MEM_VBOK4BG_COMP_TRIM_W {
        MEM_VBOK4BG_COMP_TRIM_W { w: self }
    }
    #[doc = "Bits 0:9 - MEM_BGAP_SPARE"]
    #[inline(always)]
    pub fn mem_bgap_spare(&mut self) -> MEM_BGAP_SPARE_W {
        MEM_BGAP_SPARE_W { w: self }
    }
}
