#[doc = "Reader of register FLASH_DCDC_PARAMETERS2"]
pub type R = crate::R<u32, super::FLASH_DCDC_PARAMETERS2>;
#[doc = "Writer for register FLASH_DCDC_PARAMETERS2"]
pub type W = crate::W<u32, super::FLASH_DCDC_PARAMETERS2>;
#[doc = "Register FLASH_DCDC_PARAMETERS2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_DCDC_PARAMETERS2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_P1FET_STAGGER_LOWV`"]
pub type MEM_DCDC_FLASH_P1FET_STAGGER_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_P1FET_STAGGER_LOWV`"]
pub struct MEM_DCDC_FLASH_P1FET_STAGGER_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_P1FET_STAGGER_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_N1FET_STAGGER_LOWV`"]
pub type MEM_DCDC_FLASH_N1FET_STAGGER_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_N1FET_STAGGER_LOWV`"]
pub struct MEM_DCDC_FLASH_N1FET_STAGGER_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_N1FET_STAGGER_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_P2FET_STAGGER_LOWV`"]
pub type MEM_DCDC_FLASH_P2FET_STAGGER_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_P2FET_STAGGER_LOWV`"]
pub struct MEM_DCDC_FLASH_P2FET_STAGGER_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_P2FET_STAGGER_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_N2FET_STAGGER_LOWV`"]
pub type MEM_DCDC_FLASH_N2FET_STAGGER_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_N2FET_STAGGER_LOWV`"]
pub struct MEM_DCDC_FLASH_N2FET_STAGGER_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_N2FET_STAGGER_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_NCOMP_TRIM_LOWV`"]
pub type MEM_DCDC_FLASH_NCOMP_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_NCOMP_TRIM_LOWV`"]
pub struct MEM_DCDC_FLASH_NCOMP_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_NCOMP_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_NCOMP_MASK_DLY_TRIM_LOWV`"]
pub type MEM_DCDC_FLASH_NCOMP_MASK_DLY_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_NCOMP_MASK_DLY_TRIM_LOWV`"]
pub struct MEM_DCDC_FLASH_NCOMP_MASK_DLY_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_NCOMP_MASK_DLY_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_ILIM_TRIM_LOWV_OVERRIDE`"]
pub type MEM_DCDC_FLASH_ILIM_TRIM_LOWV_OVERRIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_ILIM_TRIM_LOWV_OVERRIDE`"]
pub struct MEM_DCDC_FLASH_ILIM_TRIM_LOWV_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_ILIM_TRIM_LOWV_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 2)) | (((value as u32) & 0xff) << 2);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_ILIM_MASK_DLY_SEL_LOWV`"]
pub type MEM_DCDC_FLASH_ILIM_MASK_DLY_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_ILIM_MASK_DLY_SEL_LOWV`"]
pub struct MEM_DCDC_FLASH_ILIM_MASK_DLY_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_ILIM_MASK_DLY_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - MEM_DCDC_FLASH_P1FET_STAGGER_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p1fet_stagger_lowv(&self) -> MEM_DCDC_FLASH_P1FET_STAGGER_LOWV_R {
        MEM_DCDC_FLASH_P1FET_STAGGER_LOWV_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - MEM_DCDC_FLASH_N1FET_STAGGER_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n1fet_stagger_lowv(&self) -> MEM_DCDC_FLASH_N1FET_STAGGER_LOWV_R {
        MEM_DCDC_FLASH_N1FET_STAGGER_LOWV_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - MEM_DCDC_FLASH_P2FET_STAGGER_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p2fet_stagger_lowv(&self) -> MEM_DCDC_FLASH_P2FET_STAGGER_LOWV_R {
        MEM_DCDC_FLASH_P2FET_STAGGER_LOWV_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - MEM_DCDC_FLASH_N2FET_STAGGER_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n2fet_stagger_lowv(&self) -> MEM_DCDC_FLASH_N2FET_STAGGER_LOWV_R {
        MEM_DCDC_FLASH_N2FET_STAGGER_LOWV_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 16:20 - MEM_DCDC_FLASH_NCOMP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_ncomp_trim_lowv(&self) -> MEM_DCDC_FLASH_NCOMP_TRIM_LOWV_R {
        MEM_DCDC_FLASH_NCOMP_TRIM_LOWV_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - MEM_DCDC_FLASH_NCOMP_MASK_DLY_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_ncomp_mask_dly_trim_lowv(
        &self,
    ) -> MEM_DCDC_FLASH_NCOMP_MASK_DLY_TRIM_LOWV_R {
        MEM_DCDC_FLASH_NCOMP_MASK_DLY_TRIM_LOWV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 2:9 - MEM_DCDC_FLASH_ILIM_TRIM_LOWV_OVERRIDE"]
    #[inline(always)]
    pub fn mem_dcdc_flash_ilim_trim_lowv_override(
        &self,
    ) -> MEM_DCDC_FLASH_ILIM_TRIM_LOWV_OVERRIDE_R {
        MEM_DCDC_FLASH_ILIM_TRIM_LOWV_OVERRIDE_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 0:1 - MEM_DCDC_FLASH_ILIM_MASK_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_ilim_mask_dly_sel_lowv(&self) -> MEM_DCDC_FLASH_ILIM_MASK_DLY_SEL_LOWV_R {
        MEM_DCDC_FLASH_ILIM_MASK_DLY_SEL_LOWV_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - MEM_DCDC_FLASH_P1FET_STAGGER_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p1fet_stagger_lowv(&mut self) -> MEM_DCDC_FLASH_P1FET_STAGGER_LOWV_W {
        MEM_DCDC_FLASH_P1FET_STAGGER_LOWV_W { w: self }
    }
    #[doc = "Bits 28:29 - MEM_DCDC_FLASH_N1FET_STAGGER_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n1fet_stagger_lowv(&mut self) -> MEM_DCDC_FLASH_N1FET_STAGGER_LOWV_W {
        MEM_DCDC_FLASH_N1FET_STAGGER_LOWV_W { w: self }
    }
    #[doc = "Bits 26:27 - MEM_DCDC_FLASH_P2FET_STAGGER_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p2fet_stagger_lowv(&mut self) -> MEM_DCDC_FLASH_P2FET_STAGGER_LOWV_W {
        MEM_DCDC_FLASH_P2FET_STAGGER_LOWV_W { w: self }
    }
    #[doc = "Bits 24:25 - MEM_DCDC_FLASH_N2FET_STAGGER_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n2fet_stagger_lowv(&mut self) -> MEM_DCDC_FLASH_N2FET_STAGGER_LOWV_W {
        MEM_DCDC_FLASH_N2FET_STAGGER_LOWV_W { w: self }
    }
    #[doc = "Bits 16:20 - MEM_DCDC_FLASH_NCOMP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_ncomp_trim_lowv(&mut self) -> MEM_DCDC_FLASH_NCOMP_TRIM_LOWV_W {
        MEM_DCDC_FLASH_NCOMP_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 12:15 - MEM_DCDC_FLASH_NCOMP_MASK_DLY_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_ncomp_mask_dly_trim_lowv(
        &mut self,
    ) -> MEM_DCDC_FLASH_NCOMP_MASK_DLY_TRIM_LOWV_W {
        MEM_DCDC_FLASH_NCOMP_MASK_DLY_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 2:9 - MEM_DCDC_FLASH_ILIM_TRIM_LOWV_OVERRIDE"]
    #[inline(always)]
    pub fn mem_dcdc_flash_ilim_trim_lowv_override(
        &mut self,
    ) -> MEM_DCDC_FLASH_ILIM_TRIM_LOWV_OVERRIDE_W {
        MEM_DCDC_FLASH_ILIM_TRIM_LOWV_OVERRIDE_W { w: self }
    }
    #[doc = "Bits 0:1 - MEM_DCDC_FLASH_ILIM_MASK_DLY_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_ilim_mask_dly_sel_lowv(
        &mut self,
    ) -> MEM_DCDC_FLASH_ILIM_MASK_DLY_SEL_LOWV_W {
        MEM_DCDC_FLASH_ILIM_MASK_DLY_SEL_LOWV_W { w: self }
    }
}
