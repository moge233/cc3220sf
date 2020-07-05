#[doc = "Reader of register DMA_DONE_INT_MASK"]
pub type R = crate::R<u32, super::DMA_DONE_INT_MASK>;
#[doc = "Writer for register DMA_DONE_INT_MASK"]
pub type W = crate::W<u32, super::DMA_DONE_INT_MASK>;
#[doc = "Register DMA_DONE_INT_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_DONE_INT_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_WR_DMA_DONE_INT_MASK`"]
pub type ADC_WR_DMA_DONE_INT_MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_WR_DMA_DONE_INT_MASK`"]
pub struct ADC_WR_DMA_DONE_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_WR_DMA_DONE_INT_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - 1= disable corresponding interrupt;0 = interrupt enabled bit 14: ADC channel 7 interrupt enable/disable bit 13: ADC channel 5 interrupt enable/disable bit 12: ADC channel 3 interrupt enable/disable bit 11: ADC channel 1 interrupt enable/disable"]
    #[inline(always)]
    pub fn adc_wr_dma_done_int_mask(&self) -> ADC_WR_DMA_DONE_INT_MASK_R {
        ADC_WR_DMA_DONE_INT_MASK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - 1= disable corresponding interrupt;0 = interrupt enabled bit 14: ADC channel 7 interrupt enable/disable bit 13: ADC channel 5 interrupt enable/disable bit 12: ADC channel 3 interrupt enable/disable bit 11: ADC channel 1 interrupt enable/disable"]
    #[inline(always)]
    pub fn adc_wr_dma_done_int_mask(&mut self) -> ADC_WR_DMA_DONE_INT_MASK_W {
        ADC_WR_DMA_DONE_INT_MASK_W { w: self }
    }
}
