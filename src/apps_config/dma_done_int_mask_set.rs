#[doc = "Reader of register DMA_DONE_INT_MASK_SET"]
pub type R = crate::R<u32, super::DMA_DONE_INT_MASK_SET>;
#[doc = "Writer for register DMA_DONE_INT_MASK_SET"]
pub type W = crate::W<u32, super::DMA_DONE_INT_MASK_SET>;
#[doc = "Register DMA_DONE_INT_MASK_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_DONE_INT_MASK_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_WR_DMA_DONE_INT_MASK_SET`"]
pub type ADC_WR_DMA_DONE_INT_MASK_SET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_WR_DMA_DONE_INT_MASK_SET`"]
pub struct ADC_WR_DMA_DONE_INT_MASK_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_WR_DMA_DONE_INT_MASK_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - write 1 to set mask of the corresponding DMA DONE IRQ;0 = no effect bit 14: ADC channel 7 DMA Done IRQ bit 13: ADC channel 5 DMA Done IRQ bit 12: ADC channel 3 DMA Done IRQ bit 11: ADC channel 1 DMA Done IRQ"]
    #[inline(always)]
    pub fn adc_wr_dma_done_int_mask_set(&self) -> ADC_WR_DMA_DONE_INT_MASK_SET_R {
        ADC_WR_DMA_DONE_INT_MASK_SET_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - write 1 to set mask of the corresponding DMA DONE IRQ;0 = no effect bit 14: ADC channel 7 DMA Done IRQ bit 13: ADC channel 5 DMA Done IRQ bit 12: ADC channel 3 DMA Done IRQ bit 11: ADC channel 1 DMA Done IRQ"]
    #[inline(always)]
    pub fn adc_wr_dma_done_int_mask_set(&mut self) -> ADC_WR_DMA_DONE_INT_MASK_SET_W {
        ADC_WR_DMA_DONE_INT_MASK_SET_W { w: self }
    }
}
