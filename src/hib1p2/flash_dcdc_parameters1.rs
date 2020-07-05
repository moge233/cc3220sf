#[doc = "Reader of register FLASH_DCDC_PARAMETERS1"]
pub type R = crate::R<u32, super::FLASH_DCDC_PARAMETERS1>;
#[doc = "Writer for register FLASH_DCDC_PARAMETERS1"]
pub type W = crate::W<u32, super::FLASH_DCDC_PARAMETERS1>;
#[doc = "Register FLASH_DCDC_PARAMETERS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_DCDC_PARAMETERS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_P2FET_SEL_LOWV`"]
pub type MEM_DCDC_FLASH_P2FET_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_P2FET_SEL_LOWV`"]
pub struct MEM_DCDC_FLASH_P2FET_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_P2FET_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_N2FET_SEL_LOWV`"]
pub type MEM_DCDC_FLASH_N2FET_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_N2FET_SEL_LOWV`"]
pub struct MEM_DCDC_FLASH_N2FET_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_N2FET_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_P1DRV_STR_SEL_LOWV`"]
pub type MEM_DCDC_FLASH_P1DRV_STR_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_P1DRV_STR_SEL_LOWV`"]
pub struct MEM_DCDC_FLASH_P1DRV_STR_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_P1DRV_STR_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_N1DRV_STR_SEL_LOWV`"]
pub type MEM_DCDC_FLASH_N1DRV_STR_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_N1DRV_STR_SEL_LOWV`"]
pub struct MEM_DCDC_FLASH_N1DRV_STR_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_N1DRV_STR_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_P2DRV_STR_SEL_LOWV`"]
pub type MEM_DCDC_FLASH_P2DRV_STR_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_P2DRV_STR_SEL_LOWV`"]
pub struct MEM_DCDC_FLASH_P2DRV_STR_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_P2DRV_STR_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_N2DRV_STR_SEL_LOWV`"]
pub type MEM_DCDC_FLASH_N2DRV_STR_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_N2DRV_STR_SEL_LOWV`"]
pub struct MEM_DCDC_FLASH_N2DRV_STR_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_N2DRV_STR_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_P1FET_NON_OV_LOWV`"]
pub type MEM_DCDC_FLASH_P1FET_NON_OV_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_P1FET_NON_OV_LOWV`"]
pub struct MEM_DCDC_FLASH_P1FET_NON_OV_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_P1FET_NON_OV_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_N1FET_NON_OV_LOWV`"]
pub type MEM_DCDC_FLASH_N1FET_NON_OV_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_N1FET_NON_OV_LOWV`"]
pub struct MEM_DCDC_FLASH_N1FET_NON_OV_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_N1FET_NON_OV_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_P2FET_NON_OV_LOWV`"]
pub type MEM_DCDC_FLASH_P2FET_NON_OV_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_P2FET_NON_OV_LOWV`"]
pub struct MEM_DCDC_FLASH_P2FET_NON_OV_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_P2FET_NON_OV_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_N2FET_NON_OV_LOWV`"]
pub type MEM_DCDC_FLASH_N2FET_NON_OV_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_N2FET_NON_OV_LOWV`"]
pub struct MEM_DCDC_FLASH_N2FET_NON_OV_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_N2FET_NON_OV_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - MEM_DCDC_FLASH_P2FET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p2fet_sel_lowv(&self) -> MEM_DCDC_FLASH_P2FET_SEL_LOWV_R {
        MEM_DCDC_FLASH_P2FET_SEL_LOWV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - MEM_DCDC_FLASH_N2FET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n2fet_sel_lowv(&self) -> MEM_DCDC_FLASH_N2FET_SEL_LOWV_R {
        MEM_DCDC_FLASH_N2FET_SEL_LOWV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - MEM_DCDC_FLASH_P1DRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p1drv_str_sel_lowv(&self) -> MEM_DCDC_FLASH_P1DRV_STR_SEL_LOWV_R {
        MEM_DCDC_FLASH_P1DRV_STR_SEL_LOWV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - MEM_DCDC_FLASH_N1DRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n1drv_str_sel_lowv(&self) -> MEM_DCDC_FLASH_N1DRV_STR_SEL_LOWV_R {
        MEM_DCDC_FLASH_N1DRV_STR_SEL_LOWV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - MEM_DCDC_FLASH_P2DRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p2drv_str_sel_lowv(&self) -> MEM_DCDC_FLASH_P2DRV_STR_SEL_LOWV_R {
        MEM_DCDC_FLASH_P2DRV_STR_SEL_LOWV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - MEM_DCDC_FLASH_N2DRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n2drv_str_sel_lowv(&self) -> MEM_DCDC_FLASH_N2DRV_STR_SEL_LOWV_R {
        MEM_DCDC_FLASH_N2DRV_STR_SEL_LOWV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - MEM_DCDC_FLASH_P1FET_NON_OV_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p1fet_non_ov_lowv(&self) -> MEM_DCDC_FLASH_P1FET_NON_OV_LOWV_R {
        MEM_DCDC_FLASH_P1FET_NON_OV_LOWV_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - MEM_DCDC_FLASH_N1FET_NON_OV_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n1fet_non_ov_lowv(&self) -> MEM_DCDC_FLASH_N1FET_NON_OV_LOWV_R {
        MEM_DCDC_FLASH_N1FET_NON_OV_LOWV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - MEM_DCDC_FLASH_P2FET_NON_OV_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p2fet_non_ov_lowv(&self) -> MEM_DCDC_FLASH_P2FET_NON_OV_LOWV_R {
        MEM_DCDC_FLASH_P2FET_NON_OV_LOWV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - MEM_DCDC_FLASH_N2FET_NON_OV_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n2fet_non_ov_lowv(&self) -> MEM_DCDC_FLASH_N2FET_NON_OV_LOWV_R {
        MEM_DCDC_FLASH_N2FET_NON_OV_LOWV_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - MEM_DCDC_FLASH_P2FET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p2fet_sel_lowv(&mut self) -> MEM_DCDC_FLASH_P2FET_SEL_LOWV_W {
        MEM_DCDC_FLASH_P2FET_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 24:27 - MEM_DCDC_FLASH_N2FET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n2fet_sel_lowv(&mut self) -> MEM_DCDC_FLASH_N2FET_SEL_LOWV_W {
        MEM_DCDC_FLASH_N2FET_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 20:23 - MEM_DCDC_FLASH_P1DRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p1drv_str_sel_lowv(&mut self) -> MEM_DCDC_FLASH_P1DRV_STR_SEL_LOWV_W {
        MEM_DCDC_FLASH_P1DRV_STR_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 16:19 - MEM_DCDC_FLASH_N1DRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n1drv_str_sel_lowv(&mut self) -> MEM_DCDC_FLASH_N1DRV_STR_SEL_LOWV_W {
        MEM_DCDC_FLASH_N1DRV_STR_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 12:15 - MEM_DCDC_FLASH_P2DRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p2drv_str_sel_lowv(&mut self) -> MEM_DCDC_FLASH_P2DRV_STR_SEL_LOWV_W {
        MEM_DCDC_FLASH_P2DRV_STR_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 8:11 - MEM_DCDC_FLASH_N2DRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n2drv_str_sel_lowv(&mut self) -> MEM_DCDC_FLASH_N2DRV_STR_SEL_LOWV_W {
        MEM_DCDC_FLASH_N2DRV_STR_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 6:7 - MEM_DCDC_FLASH_P1FET_NON_OV_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p1fet_non_ov_lowv(&mut self) -> MEM_DCDC_FLASH_P1FET_NON_OV_LOWV_W {
        MEM_DCDC_FLASH_P1FET_NON_OV_LOWV_W { w: self }
    }
    #[doc = "Bits 4:5 - MEM_DCDC_FLASH_N1FET_NON_OV_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n1fet_non_ov_lowv(&mut self) -> MEM_DCDC_FLASH_N1FET_NON_OV_LOWV_W {
        MEM_DCDC_FLASH_N1FET_NON_OV_LOWV_W { w: self }
    }
    #[doc = "Bits 2:3 - MEM_DCDC_FLASH_P2FET_NON_OV_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p2fet_non_ov_lowv(&mut self) -> MEM_DCDC_FLASH_P2FET_NON_OV_LOWV_W {
        MEM_DCDC_FLASH_P2FET_NON_OV_LOWV_W { w: self }
    }
    #[doc = "Bits 0:1 - MEM_DCDC_FLASH_N2FET_NON_OV_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n2fet_non_ov_lowv(&mut self) -> MEM_DCDC_FLASH_N2FET_NON_OV_LOWV_W {
        MEM_DCDC_FLASH_N2FET_NON_OV_LOWV_W { w: self }
    }
}
