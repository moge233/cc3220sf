#[doc = "Reader of register SRAM_SKA_LDO_PARAMETERS1"]
pub type R = crate::R<u32, super::SRAM_SKA_LDO_PARAMETERS1>;
#[doc = "Writer for register SRAM_SKA_LDO_PARAMETERS1"]
pub type W = crate::W<u32, super::SRAM_SKA_LDO_PARAMETERS1>;
#[doc = "Register SRAM_SKA_LDO_PARAMETERS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRAM_SKA_LDO_PARAMETERS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_SKALDO_CTRL_LOWV`"]
pub type MEM_SKALDO_CTRL_LOWV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_SKALDO_CTRL_LOWV`"]
pub struct MEM_SKALDO_CTRL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SKALDO_CTRL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 22)) | (((value as u32) & 0x03ff) << 22);
        self.w
    }
}
#[doc = "Reader of field `MEM_SKALDO_VTRIM_LOWV`"]
pub type MEM_SKALDO_VTRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SKALDO_VTRIM_LOWV`"]
pub struct MEM_SKALDO_VTRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SKALDO_VTRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `NA2`"]
pub type NA2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NA2`"]
pub struct NA2_W<'a> {
    w: &'a mut W,
}
impl<'a> NA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:31 - MEM_SKALDO_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_skaldo_ctrl_lowv(&self) -> MEM_SKALDO_CTRL_LOWV_R {
        MEM_SKALDO_CTRL_LOWV_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:21 - MEM_SKALDO_VTRIM_LOWV"]
    #[inline(always)]
    pub fn mem_skaldo_vtrim_lowv(&self) -> MEM_SKALDO_VTRIM_LOWV_R {
        MEM_SKALDO_VTRIM_LOWV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 0:10 - NA2"]
    #[inline(always)]
    pub fn na2(&self) -> NA2_R {
        NA2_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 22:31 - MEM_SKALDO_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_skaldo_ctrl_lowv(&mut self) -> MEM_SKALDO_CTRL_LOWV_W {
        MEM_SKALDO_CTRL_LOWV_W { w: self }
    }
    #[doc = "Bits 16:21 - MEM_SKALDO_VTRIM_LOWV"]
    #[inline(always)]
    pub fn mem_skaldo_vtrim_lowv(&mut self) -> MEM_SKALDO_VTRIM_LOWV_W {
        MEM_SKALDO_VTRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 0:10 - NA2"]
    #[inline(always)]
    pub fn na2(&mut self) -> NA2_W {
        NA2_W { w: self }
    }
}
