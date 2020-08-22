#[doc = "Reader of register CH6_IRQ_STATUS"]
pub type R = crate::R<u32, super::CH6_IRQ_STATUS>;
#[doc = "Writer for register CH6_IRQ_STATUS"]
pub type W = crate::W<u32, super::CH6_IRQ_STATUS>;
#[doc = "Register CH6_IRQ_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::CH6_IRQ_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_CHANNEL6_IRQ_STATUS`"]
pub type ADC_CHANNEL6_IRQ_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_CHANNEL6_IRQ_STATUS`"]
pub struct ADC_CHANNEL6_IRQ_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CHANNEL6_IRQ_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Interrupt status register for ADC channel. Interrupt status can be cleared on write."]
    #[inline(always)]
    pub fn adc_channel6_irq_status(&self) -> ADC_CHANNEL6_IRQ_STATUS_R {
        ADC_CHANNEL6_IRQ_STATUS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Interrupt status register for ADC channel. Interrupt status can be cleared on write."]
    #[inline(always)]
    pub fn adc_channel6_irq_status(&mut self) -> ADC_CHANNEL6_IRQ_STATUS_W {
        ADC_CHANNEL6_IRQ_STATUS_W { w: self }
    }
}
