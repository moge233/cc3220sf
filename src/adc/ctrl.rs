#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_EN_APPS`"]
pub type ADC_EN_APPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_EN_APPS`"]
pub struct ADC_EN_APPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_EN_APPS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC enable for application processor"]
    #[inline(always)]
    pub fn adc_en_apps(&self) -> ADC_EN_APPS_R {
        ADC_EN_APPS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable for application processor"]
    #[inline(always)]
    pub fn adc_en_apps(&mut self) -> ADC_EN_APPS_W {
        ADC_EN_APPS_W { w: self }
    }
}
