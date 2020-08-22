#[doc = "Reader of register DMA_MODE_EN"]
pub type R = crate::R<u32, super::DMA_MODE_EN>;
#[doc = "Writer for register DMA_MODE_EN"]
pub type W = crate::W<u32, super::DMA_MODE_EN>;
#[doc = "Register DMA_MODE_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_MODE_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_MODEENABLE`"]
pub type DMA_MODEENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA_MODEENABLE`"]
pub struct DMA_MODEENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_MODEENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register enables DMA mode."]
    #[inline(always)]
    pub fn dma_modeenable(&self) -> DMA_MODEENABLE_R {
        DMA_MODEENABLE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register enables DMA mode."]
    #[inline(always)]
    pub fn dma_modeenable(&mut self) -> DMA_MODEENABLE_W {
        DMA_MODEENABLE_W { w: self }
    }
}
