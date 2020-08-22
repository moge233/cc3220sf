#[doc = "Reader of register CH6_IRQ_EN"]
pub type R = crate::R<u32, super::CH6_IRQ_EN>;
#[doc = "Writer for register CH6_IRQ_EN"]
pub type W = crate::W<u32, super::CH6_IRQ_EN>;
#[doc = "Register CH6_IRQ_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::CH6_IRQ_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_CHANNEL6_IRQ_EN`"]
pub type ADC_CHANNEL6_IRQ_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_CHANNEL6_IRQ_EN`"]
pub struct ADC_CHANNEL6_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CHANNEL6_IRQ_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Interrupt enable register for ADC channel"]
    #[inline(always)]
    pub fn adc_channel6_irq_en(&self) -> ADC_CHANNEL6_IRQ_EN_R {
        ADC_CHANNEL6_IRQ_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Interrupt enable register for ADC channel"]
    #[inline(always)]
    pub fn adc_channel6_irq_en(&mut self) -> ADC_CHANNEL6_IRQ_EN_W {
        ADC_CHANNEL6_IRQ_EN_W { w: self }
    }
}
