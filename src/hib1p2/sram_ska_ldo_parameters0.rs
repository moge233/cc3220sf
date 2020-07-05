#[doc = "Reader of register SRAM_SKA_LDO_PARAMETERS0"]
pub type R = crate::R<u32, super::SRAM_SKA_LDO_PARAMETERS0>;
#[doc = "Writer for register SRAM_SKA_LDO_PARAMETERS0"]
pub type W = crate::W<u32, super::SRAM_SKA_LDO_PARAMETERS0>;
#[doc = "Register SRAM_SKA_LDO_PARAMETERS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRAM_SKA_LDO_PARAMETERS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_SLDO_EN_SC_ITRIM_LOWV`"]
pub type MEM_SLDO_EN_SC_ITRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SLDO_EN_SC_ITRIM_LOWV`"]
pub struct MEM_SLDO_EN_SC_ITRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SLDO_EN_SC_ITRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `MEM_SLDO_EN_IQ_TRIM_LOWV`"]
pub type MEM_SLDO_EN_IQ_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SLDO_EN_IQ_TRIM_LOWV`"]
pub struct MEM_SLDO_EN_IQ_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SLDO_EN_IQ_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `MEM_SLDO_VTRIM_LOWV`"]
pub type MEM_SLDO_VTRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SLDO_VTRIM_LOWV`"]
pub struct MEM_SLDO_VTRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SLDO_VTRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `MEM_SLDO_SPARE_LOWV`"]
pub type MEM_SLDO_SPARE_LOWV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_SLDO_SPARE_LOWV`"]
pub struct MEM_SLDO_SPARE_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SLDO_SPARE_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 8)) | (((value as u32) & 0x03ff) << 8);
        self.w
    }
}
#[doc = "Reader of field `NA1`"]
pub type NA1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NA1`"]
pub struct NA1_W<'a> {
    w: &'a mut W,
}
impl<'a> NA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - MEM_SLDO_EN_SC_ITRIM_LOWV"]
    #[inline(always)]
    pub fn mem_sldo_en_sc_itrim_lowv(&self) -> MEM_SLDO_EN_SC_ITRIM_LOWV_R {
        MEM_SLDO_EN_SC_ITRIM_LOWV_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - MEM_SLDO_EN_IQ_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_sldo_en_iq_trim_lowv(&self) -> MEM_SLDO_EN_IQ_TRIM_LOWV_R {
        MEM_SLDO_EN_IQ_TRIM_LOWV_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 18:23 - MEM_SLDO_VTRIM_LOWV"]
    #[inline(always)]
    pub fn mem_sldo_vtrim_lowv(&self) -> MEM_SLDO_VTRIM_LOWV_R {
        MEM_SLDO_VTRIM_LOWV_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 8:17 - MEM_SLDO_SPARE_LOWV"]
    #[inline(always)]
    pub fn mem_sldo_spare_lowv(&self) -> MEM_SLDO_SPARE_LOWV_R {
        MEM_SLDO_SPARE_LOWV_R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:2 - NA1"]
    #[inline(always)]
    pub fn na1(&self) -> NA1_R {
        NA1_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - MEM_SLDO_EN_SC_ITRIM_LOWV"]
    #[inline(always)]
    pub fn mem_sldo_en_sc_itrim_lowv(&mut self) -> MEM_SLDO_EN_SC_ITRIM_LOWV_W {
        MEM_SLDO_EN_SC_ITRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 28:29 - MEM_SLDO_EN_IQ_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_sldo_en_iq_trim_lowv(&mut self) -> MEM_SLDO_EN_IQ_TRIM_LOWV_W {
        MEM_SLDO_EN_IQ_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 18:23 - MEM_SLDO_VTRIM_LOWV"]
    #[inline(always)]
    pub fn mem_sldo_vtrim_lowv(&mut self) -> MEM_SLDO_VTRIM_LOWV_W {
        MEM_SLDO_VTRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 8:17 - MEM_SLDO_SPARE_LOWV"]
    #[inline(always)]
    pub fn mem_sldo_spare_lowv(&mut self) -> MEM_SLDO_SPARE_LOWV_W {
        MEM_SLDO_SPARE_LOWV_W { w: self }
    }
    #[doc = "Bits 0:2 - NA1"]
    #[inline(always)]
    pub fn na1(&mut self) -> NA1_W {
        NA1_W { w: self }
    }
}
