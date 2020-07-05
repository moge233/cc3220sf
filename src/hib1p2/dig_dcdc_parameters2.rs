#[doc = "Reader of register DIG_DCDC_PARAMETERS2"]
pub type R = crate::R<u32, super::DIG_DCDC_PARAMETERS2>;
#[doc = "Writer for register DIG_DCDC_PARAMETERS2"]
pub type W = crate::W<u32, super::DIG_DCDC_PARAMETERS2>;
#[doc = "Register DIG_DCDC_PARAMETERS2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DIG_DCDC_PARAMETERS2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_PFET_SEL_LOWV`"]
pub type MEM_DCDC_DIG_PFET_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_PFET_SEL_LOWV`"]
pub struct MEM_DCDC_DIG_PFET_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_PFET_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_NFET_SEL_LOWV`"]
pub type MEM_DCDC_DIG_NFET_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_NFET_SEL_LOWV`"]
pub struct MEM_DCDC_DIG_NFET_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_NFET_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_PDRV_STAGGER_CTRL_LOWV`"]
pub type MEM_DCDC_DIG_PDRV_STAGGER_CTRL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_PDRV_STAGGER_CTRL_LOWV`"]
pub struct MEM_DCDC_DIG_PDRV_STAGGER_CTRL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_PDRV_STAGGER_CTRL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_NDRV_STAGGER_CTRL_LOWV`"]
pub type MEM_DCDC_DIG_NDRV_STAGGER_CTRL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_NDRV_STAGGER_CTRL_LOWV`"]
pub struct MEM_DCDC_DIG_NDRV_STAGGER_CTRL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_NDRV_STAGGER_CTRL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_PDRV_STR_SEL_LOWV`"]
pub type MEM_DCDC_DIG_PDRV_STR_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_PDRV_STR_SEL_LOWV`"]
pub struct MEM_DCDC_DIG_PDRV_STR_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_PDRV_STR_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_NDRV_STR_SEL_LOWV`"]
pub type MEM_DCDC_DIG_NDRV_STR_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_NDRV_STR_SEL_LOWV`"]
pub struct MEM_DCDC_DIG_NDRV_STR_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_NDRV_STR_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | (((value as u32) & 0x0f) << 11);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_TON_TRIM_LOWV`"]
pub type MEM_DCDC_DIG_TON_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_TON_TRIM_LOWV`"]
pub struct MEM_DCDC_DIG_TON_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_TON_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 2)) | (((value as u32) & 0xff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - MEM_DCDC_DIG_PFET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_pfet_sel_lowv(&self) -> MEM_DCDC_DIG_PFET_SEL_LOWV_R {
        MEM_DCDC_DIG_PFET_SEL_LOWV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - MEM_DCDC_DIG_NFET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_nfet_sel_lowv(&self) -> MEM_DCDC_DIG_NFET_SEL_LOWV_R {
        MEM_DCDC_DIG_NFET_SEL_LOWV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - MEM_DCDC_DIG_PDRV_STAGGER_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_pdrv_stagger_ctrl_lowv(&self) -> MEM_DCDC_DIG_PDRV_STAGGER_CTRL_LOWV_R {
        MEM_DCDC_DIG_PDRV_STAGGER_CTRL_LOWV_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - MEM_DCDC_DIG_NDRV_STAGGER_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ndrv_stagger_ctrl_lowv(&self) -> MEM_DCDC_DIG_NDRV_STAGGER_CTRL_LOWV_R {
        MEM_DCDC_DIG_NDRV_STAGGER_CTRL_LOWV_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - MEM_DCDC_DIG_PDRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_pdrv_str_sel_lowv(&self) -> MEM_DCDC_DIG_PDRV_STR_SEL_LOWV_R {
        MEM_DCDC_DIG_PDRV_STR_SEL_LOWV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14 - MEM_DCDC_DIG_NDRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ndrv_str_sel_lowv(&self) -> MEM_DCDC_DIG_NDRV_STR_SEL_LOWV_R {
        MEM_DCDC_DIG_NDRV_STR_SEL_LOWV_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 2:9 - MEM_DCDC_DIG_TON_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ton_trim_lowv(&self) -> MEM_DCDC_DIG_TON_TRIM_LOWV_R {
        MEM_DCDC_DIG_TON_TRIM_LOWV_R::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - MEM_DCDC_DIG_PFET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_pfet_sel_lowv(&mut self) -> MEM_DCDC_DIG_PFET_SEL_LOWV_W {
        MEM_DCDC_DIG_PFET_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 24:27 - MEM_DCDC_DIG_NFET_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_nfet_sel_lowv(&mut self) -> MEM_DCDC_DIG_NFET_SEL_LOWV_W {
        MEM_DCDC_DIG_NFET_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 22:23 - MEM_DCDC_DIG_PDRV_STAGGER_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_pdrv_stagger_ctrl_lowv(&mut self) -> MEM_DCDC_DIG_PDRV_STAGGER_CTRL_LOWV_W {
        MEM_DCDC_DIG_PDRV_STAGGER_CTRL_LOWV_W { w: self }
    }
    #[doc = "Bits 20:21 - MEM_DCDC_DIG_NDRV_STAGGER_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ndrv_stagger_ctrl_lowv(&mut self) -> MEM_DCDC_DIG_NDRV_STAGGER_CTRL_LOWV_W {
        MEM_DCDC_DIG_NDRV_STAGGER_CTRL_LOWV_W { w: self }
    }
    #[doc = "Bits 16:19 - MEM_DCDC_DIG_PDRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_pdrv_str_sel_lowv(&mut self) -> MEM_DCDC_DIG_PDRV_STR_SEL_LOWV_W {
        MEM_DCDC_DIG_PDRV_STR_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 11:14 - MEM_DCDC_DIG_NDRV_STR_SEL_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ndrv_str_sel_lowv(&mut self) -> MEM_DCDC_DIG_NDRV_STR_SEL_LOWV_W {
        MEM_DCDC_DIG_NDRV_STR_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 2:9 - MEM_DCDC_DIG_TON_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_dcdc_dig_ton_trim_lowv(&mut self) -> MEM_DCDC_DIG_TON_TRIM_LOWV_W {
        MEM_DCDC_DIG_TON_TRIM_LOWV_W { w: self }
    }
}
