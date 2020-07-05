#[doc = "Reader of register CC_CTRL_DMA"]
pub type R = crate::R<u32, super::CC_CTRL_DMA>;
#[doc = "Writer for register CC_CTRL_DMA"]
pub type W = crate::W<u32, super::CC_CTRL_DMA>;
#[doc = "Register CC_CTRL_DMA `reset()`'s with value 0"]
impl crate::ResetValue for super::CC_CTRL_DMA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFO_THRESHOLD`"]
pub type FIFO_THRESHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFO_THRESHOLD`"]
pub struct FIFO_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Sets the threshold of the FIFO the assertion of the dmarequest line takes place when the threshold is reached. &quot;&quot;&quot;0000000&quot;&quot; threshold set to 1&quot; &quot;&quot;&quot;0000001&quot;&quot; threshold set to 2&quot; &quot;&quot;&quot;1111111&quot;&quot; threshold set to 128&quot;"]
    #[inline(always)]
    pub fn fifo_threshold(&self) -> FIFO_THRESHOLD_R {
        FIFO_THRESHOLD_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Sets the threshold of the FIFO the assertion of the dmarequest line takes place when the threshold is reached. &quot;&quot;&quot;0000000&quot;&quot; threshold set to 1&quot; &quot;&quot;&quot;0000001&quot;&quot; threshold set to 2&quot; &quot;&quot;&quot;1111111&quot;&quot; threshold set to 128&quot;"]
    #[inline(always)]
    pub fn fifo_threshold(&mut self) -> FIFO_THRESHOLD_W {
        FIFO_THRESHOLD_W { w: self }
    }
}
