#[doc = "Reader of register MEM_DIG_DCDC_CLK_CONFIG"]
pub type R = crate::R<u32, super::MEM_DIG_DCDC_CLK_CONFIG>;
#[doc = "Writer for register MEM_DIG_DCDC_CLK_CONFIG"]
pub type W = crate::W<u32, super::MEM_DIG_DCDC_CLK_CONFIG>;
#[doc = "Register MEM_DIG_DCDC_CLK_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_DIG_DCDC_CLK_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DIG_DCDC_CLK_PLLGEN_OFF_TIME`"]
pub type MEM_DIG_DCDC_CLK_PLLGEN_OFF_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DIG_DCDC_CLK_PLLGEN_OFF_TIME`"]
pub struct MEM_DIG_DCDC_CLK_PLLGEN_OFF_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DIG_DCDC_CLK_PLLGEN_OFF_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MEM_DIG_DCDC_CLK_PLLGEN_ON_TIME`"]
pub type MEM_DIG_DCDC_CLK_PLLGEN_ON_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DIG_DCDC_CLK_PLLGEN_ON_TIME`"]
pub struct MEM_DIG_DCDC_CLK_PLLGEN_ON_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DIG_DCDC_CLK_PLLGEN_ON_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - MEM_DIG_DCDC_CLK_PLLGEN_OFF_TIME"]
    #[inline(always)]
    pub fn mem_dig_dcdc_clk_pllgen_off_time(&self) -> MEM_DIG_DCDC_CLK_PLLGEN_OFF_TIME_R {
        MEM_DIG_DCDC_CLK_PLLGEN_OFF_TIME_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - MEM_DIG_DCDC_CLK_PLLGEN_ON_TIME"]
    #[inline(always)]
    pub fn mem_dig_dcdc_clk_pllgen_on_time(&self) -> MEM_DIG_DCDC_CLK_PLLGEN_ON_TIME_R {
        MEM_DIG_DCDC_CLK_PLLGEN_ON_TIME_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - MEM_DIG_DCDC_CLK_PLLGEN_OFF_TIME"]
    #[inline(always)]
    pub fn mem_dig_dcdc_clk_pllgen_off_time(&mut self) -> MEM_DIG_DCDC_CLK_PLLGEN_OFF_TIME_W {
        MEM_DIG_DCDC_CLK_PLLGEN_OFF_TIME_W { w: self }
    }
    #[doc = "Bits 0:3 - MEM_DIG_DCDC_CLK_PLLGEN_ON_TIME"]
    #[inline(always)]
    pub fn mem_dig_dcdc_clk_pllgen_on_time(&mut self) -> MEM_DIG_DCDC_CLK_PLLGEN_ON_TIME_W {
        MEM_DIG_DCDC_CLK_PLLGEN_ON_TIME_W { w: self }
    }
}
