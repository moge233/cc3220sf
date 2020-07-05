#[doc = "Reader of register ADC_CLK_CONFIG"]
pub type R = crate::R<u32, super::ADC_CLK_CONFIG>;
#[doc = "Writer for register ADC_CLK_CONFIG"]
pub type W = crate::W<u32, super::ADC_CLK_CONFIG>;
#[doc = "Register ADC_CLK_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_CLK_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_CLKGEN_OFF_TIME`"]
pub type ADC_CLKGEN_OFF_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_CLKGEN_OFF_TIME`"]
pub struct ADC_CLKGEN_OFF_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLKGEN_OFF_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `ADC_CLKGEN_ON_TIME`"]
pub type ADC_CLKGEN_ON_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_CLKGEN_ON_TIME`"]
pub struct ADC_CLKGEN_ON_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLKGEN_ON_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:10 - Configuration (in number of 38.4 MHz clks) for the OFF-Time in generation of ADC_CLK"]
    #[inline(always)]
    pub fn adc_clkgen_off_time(&self) -> ADC_CLKGEN_OFF_TIME_R {
        ADC_CLKGEN_OFF_TIME_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 1:5 - Configuration (in number of 38.4 MHz clks) for the ON-Time in generation of ADC_CLK"]
    #[inline(always)]
    pub fn adc_clkgen_on_time(&self) -> ADC_CLKGEN_ON_TIME_R {
        ADC_CLKGEN_ON_TIME_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:10 - Configuration (in number of 38.4 MHz clks) for the OFF-Time in generation of ADC_CLK"]
    #[inline(always)]
    pub fn adc_clkgen_off_time(&mut self) -> ADC_CLKGEN_OFF_TIME_W {
        ADC_CLKGEN_OFF_TIME_W { w: self }
    }
    #[doc = "Bits 1:5 - Configuration (in number of 38.4 MHz clks) for the ON-Time in generation of ADC_CLK"]
    #[inline(always)]
    pub fn adc_clkgen_on_time(&mut self) -> ADC_CLKGEN_ON_TIME_W {
        ADC_CLKGEN_ON_TIME_W { w: self }
    }
}
