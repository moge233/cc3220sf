#[doc = "Reader of register CH6_FIFO_LVL"]
pub type R = crate::R<u32, super::CH6_FIFO_LVL>;
#[doc = "Writer for register CH6_FIFO_LVL"]
pub type W = crate::W<u32, super::CH6_FIFO_LVL>;
#[doc = "Register CH6_FIFO_LVL `reset()`'s with value 0"]
impl crate::ResetValue for super::CH6_FIFO_LVL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_CHANNEL6_FIFO_LVL`"]
pub type ADC_CHANNEL6_FIFO_LVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_CHANNEL6_FIFO_LVL`"]
pub struct ADC_CHANNEL6_FIFO_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CHANNEL6_FIFO_LVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - This register shows the current FIFO level. FIFO is 4 words wide. Possible supported levels are : 0x0 to 0x4."]
    #[inline(always)]
    pub fn adc_channel6_fifo_lvl(&self) -> ADC_CHANNEL6_FIFO_LVL_R {
        ADC_CHANNEL6_FIFO_LVL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - This register shows the current FIFO level. FIFO is 4 words wide. Possible supported levels are : 0x0 to 0x4."]
    #[inline(always)]
    pub fn adc_channel6_fifo_lvl(&mut self) -> ADC_CHANNEL6_FIFO_LVL_W {
        ADC_CHANNEL6_FIFO_LVL_W { w: self }
    }
}
