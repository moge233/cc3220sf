#[doc = "Reader of register CH_ENABLE"]
pub type R = crate::R<u32, super::CH_ENABLE>;
#[doc = "Writer for register CH_ENABLE"]
pub type W = crate::W<u32, super::CH_ENABLE>;
#[doc = "Register CH_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::CH_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_CHANNEL6_EN`"]
pub type ADC_CHANNEL6_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_CHANNEL6_EN`"]
pub struct ADC_CHANNEL6_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CHANNEL6_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADC_CHANNEL4_EN`"]
pub type ADC_CHANNEL4_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_CHANNEL4_EN`"]
pub struct ADC_CHANNEL4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CHANNEL4_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ADC_CHANNEL2_EN`"]
pub type ADC_CHANNEL2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_CHANNEL2_EN`"]
pub struct ADC_CHANNEL2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CHANNEL2_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ADC_CHANNEL0_EN`"]
pub type ADC_CHANNEL0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_CHANNEL0_EN`"]
pub struct ADC_CHANNEL0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CHANNEL0_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Controls ADC channel isolation switches"]
    #[inline(always)]
    pub fn adc_channel6_en(&self) -> ADC_CHANNEL6_EN_R {
        ADC_CHANNEL6_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls ADC channel isolation switches"]
    #[inline(always)]
    pub fn adc_channel4_en(&self) -> ADC_CHANNEL4_EN_R {
        ADC_CHANNEL4_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls ADC channel isolation switches"]
    #[inline(always)]
    pub fn adc_channel2_en(&self) -> ADC_CHANNEL2_EN_R {
        ADC_CHANNEL2_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controls ADC channel isolation switches"]
    #[inline(always)]
    pub fn adc_channel0_en(&self) -> ADC_CHANNEL0_EN_R {
        ADC_CHANNEL0_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Controls ADC channel isolation switches"]
    #[inline(always)]
    pub fn adc_channel6_en(&mut self) -> ADC_CHANNEL6_EN_W {
        ADC_CHANNEL6_EN_W { w: self }
    }
    #[doc = "Bit 3 - Controls ADC channel isolation switches"]
    #[inline(always)]
    pub fn adc_channel4_en(&mut self) -> ADC_CHANNEL4_EN_W {
        ADC_CHANNEL4_EN_W { w: self }
    }
    #[doc = "Bit 2 - Controls ADC channel isolation switches"]
    #[inline(always)]
    pub fn adc_channel2_en(&mut self) -> ADC_CHANNEL2_EN_W {
        ADC_CHANNEL2_EN_W { w: self }
    }
    #[doc = "Bit 1 - Controls ADC channel isolation switches"]
    #[inline(always)]
    pub fn adc_channel0_en(&mut self) -> ADC_CHANNEL0_EN_W {
        ADC_CHANNEL0_EN_W { w: self }
    }
}
