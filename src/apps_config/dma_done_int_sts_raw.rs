#[doc = "Reader of register DMA_DONE_INT_STS_RAW"]
pub type R = crate::R<u32, super::DMA_DONE_INT_STS_RAW>;
#[doc = "Writer for register DMA_DONE_INT_STS_RAW"]
pub type W = crate::W<u32, super::DMA_DONE_INT_STS_RAW>;
#[doc = "Register DMA_DONE_INT_STS_RAW `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_DONE_INT_STS_RAW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_WR_DMA_DONE_INT_STS_RAW`"]
pub type ADC_WR_DMA_DONE_INT_STS_RAW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_WR_DMA_DONE_INT_STS_RAW`"]
pub struct ADC_WR_DMA_DONE_INT_STS_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_WR_DMA_DONE_INT_STS_RAW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - 1= corresponding interrupt is active. read is non-destructive;0 = corresponding interrupt is inactive bit 14: ADC channel 7 DMA Done IRQ bit 13: ADC channel 5 DMA Done IRQ bit 12: ADC channel 3 DMA Done IRQ bit 11: ADC channel 1 DMA Done IRQ"]
    #[inline(always)]
    pub fn adc_wr_dma_done_int_sts_raw(&self) -> ADC_WR_DMA_DONE_INT_STS_RAW_R {
        ADC_WR_DMA_DONE_INT_STS_RAW_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - 1= corresponding interrupt is active. read is non-destructive;0 = corresponding interrupt is inactive bit 14: ADC channel 7 DMA Done IRQ bit 13: ADC channel 5 DMA Done IRQ bit 12: ADC channel 3 DMA Done IRQ bit 11: ADC channel 1 DMA Done IRQ"]
    #[inline(always)]
    pub fn adc_wr_dma_done_int_sts_raw(&mut self) -> ADC_WR_DMA_DONE_INT_STS_RAW_W {
        ADC_WR_DMA_DONE_INT_STS_RAW_W { w: self }
    }
}
