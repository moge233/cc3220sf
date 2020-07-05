#[doc = "Reader of register FLASH_DCDC_PARAMETERS0"]
pub type R = crate::R<u32, super::FLASH_DCDC_PARAMETERS0>;
#[doc = "Writer for register FLASH_DCDC_PARAMETERS0"]
pub type W = crate::W<u32, super::FLASH_DCDC_PARAMETERS0>;
#[doc = "Register FLASH_DCDC_PARAMETERS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_DCDC_PARAMETERS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_IQ_CTRL_LOWV`"]
pub type MEM_DCDC_FLASH_IQ_CTRL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_IQ_CTRL_LOWV`"]
pub struct MEM_DCDC_FLASH_IQ_CTRL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_IQ_CTRL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_NON_OV_CTRL_LOWV`"]
pub type MEM_DCDC_FLASH_NON_OV_CTRL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_NON_OV_CTRL_LOWV`"]
pub struct MEM_DCDC_FLASH_NON_OV_CTRL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_NON_OV_CTRL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_P1FET_SEL_LOWV`"]
pub type MEM_DCDC_FLASH_P1FET_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_P1FET_SEL_LOWV`"]
pub struct MEM_DCDC_FLASH_P1FET_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_P1FET_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_FLASH_N1FET_SEL_LOWV`"]
pub type MEM_DCDC_FLASH_N1FET_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_FLASH_N1FET_SEL_LOWV`"]
pub struct MEM_DCDC_FLASH_N1FET_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_FLASH_N1FET_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:28 - MEM_DCDC_FLASH_IQ_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_iq_ctrl_lowv(&self) -> MEM_DCDC_FLASH_IQ_CTRL_LOWV_R {
        MEM_DCDC_FLASH_IQ_CTRL_LOWV_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 18:21 - MEM_DCDC_FLASH_NON_OV_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_non_ov_ctrl_lowv(&self) -> MEM_DCDC_FLASH_NON_OV_CTRL_LOWV_R {
        MEM_DCDC_FLASH_NON_OV_CTRL_LOWV_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - MEM_DCDC_FLASH_P1FET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p1fet_sel_lowv(&self) -> MEM_DCDC_FLASH_P1FET_SEL_LOWV_R {
        MEM_DCDC_FLASH_P1FET_SEL_LOWV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 1:4 - MEM_DCDC_FLASH_N1FET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n1fet_sel_lowv(&self) -> MEM_DCDC_FLASH_N1FET_SEL_LOWV_R {
        MEM_DCDC_FLASH_N1FET_SEL_LOWV_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:28 - MEM_DCDC_FLASH_IQ_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_iq_ctrl_lowv(&mut self) -> MEM_DCDC_FLASH_IQ_CTRL_LOWV_W {
        MEM_DCDC_FLASH_IQ_CTRL_LOWV_W { w: self }
    }
    #[doc = "Bits 18:21 - MEM_DCDC_FLASH_NON_OV_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_non_ov_ctrl_lowv(&mut self) -> MEM_DCDC_FLASH_NON_OV_CTRL_LOWV_W {
        MEM_DCDC_FLASH_NON_OV_CTRL_LOWV_W { w: self }
    }
    #[doc = "Bits 5:8 - MEM_DCDC_FLASH_P1FET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_p1fet_sel_lowv(&mut self) -> MEM_DCDC_FLASH_P1FET_SEL_LOWV_W {
        MEM_DCDC_FLASH_P1FET_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 1:4 - MEM_DCDC_FLASH_N1FET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_flash_n1fet_sel_lowv(&mut self) -> MEM_DCDC_FLASH_N1FET_SEL_LOWV_W {
        MEM_DCDC_FLASH_N1FET_SEL_LOWV_W { w: self }
    }
}
