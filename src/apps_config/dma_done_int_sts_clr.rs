#[doc = "Reader of register DMA_DONE_INT_STS_CLR"]
pub type R = crate::R<u32, super::DMA_DONE_INT_STS_CLR>;
#[doc = "Writer for register DMA_DONE_INT_STS_CLR"]
pub type W = crate::W<u32, super::DMA_DONE_INT_STS_CLR>;
#[doc = "Register DMA_DONE_INT_STS_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_DONE_INT_STS_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
