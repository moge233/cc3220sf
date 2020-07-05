#[doc = "Reader of register FLASH_DCDC_PARAMETERS8"]
pub type R = crate::R<u32, super::FLASH_DCDC_PARAMETERS8>;
#[doc = "Writer for register FLASH_DCDC_PARAMETERS8"]
pub type W = crate::W<u32, super::FLASH_DCDC_PARAMETERS8>;
#[doc = "Register FLASH_DCDC_PARAMETERS8 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_DCDC_PARAMETERS8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_FLASH_HIGH_SUP_TRIM_LOWV`"]
pub type MEM_FLASH_HIGH_SUP_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_FLASH_HIGH_SUP_TRIM_LOWV`"]
pub struct MEM_FLASH_HIGH_SUP_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FLASH_HIGH_SUP_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
#[doc = "Reader of field `MEM_FLASH_LOW_SUP_TRIM_LOWV`"]
pub type MEM_FLASH_LOW_SUP_TRIM_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_FLASH_LOW_SUP_TRIM_LOWV`"]
pub struct MEM_FLASH_LOW_SUP_TRIM_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FLASH_LOW_SUP_TRIM_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | (((value as u32) & 0x1f) << 21);
        self.w
    }
}
#[doc = "Reader of field `NA24`"]
pub type NA24_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NA24`"]
pub struct NA24_W<'a> {
    w: &'a mut W,
}
impl<'a> NA24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x001f_ffff) | ((value as u32) & 0x001f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:30 - MEM_FLASH_HIGH_SUP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_flash_high_sup_trim_lowv(&self) -> MEM_FLASH_HIGH_SUP_TRIM_LOWV_R {
        MEM_FLASH_HIGH_SUP_TRIM_LOWV_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - MEM_FLASH_LOW_SUP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_flash_low_sup_trim_lowv(&self) -> MEM_FLASH_LOW_SUP_TRIM_LOWV_R {
        MEM_FLASH_LOW_SUP_TRIM_LOWV_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 0:20 - NA24"]
    #[inline(always)]
    pub fn na24(&self) -> NA24_R {
        NA24_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 26:30 - MEM_FLASH_HIGH_SUP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_flash_high_sup_trim_lowv(&mut self) -> MEM_FLASH_HIGH_SUP_TRIM_LOWV_W {
        MEM_FLASH_HIGH_SUP_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 21:25 - MEM_FLASH_LOW_SUP_TRIM_LOWV"]
    #[inline(always)]
    pub fn mem_flash_low_sup_trim_lowv(&mut self) -> MEM_FLASH_LOW_SUP_TRIM_LOWV_W {
        MEM_FLASH_LOW_SUP_TRIM_LOWV_W { w: self }
    }
    #[doc = "Bits 0:20 - NA24"]
    #[inline(always)]
    pub fn na24(&mut self) -> NA24_W {
        NA24_W { w: self }
    }
}
