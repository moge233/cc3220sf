#[doc = "Reader of register CC_GEN_PAR"]
pub type R = crate::R<u32, super::CC_GEN_PAR>;
#[doc = "Writer for register CC_GEN_PAR"]
pub type W = crate::W<u32, super::CC_GEN_PAR>;
#[doc = "Register CC_GEN_PAR `reset()`'s with value 0"]
impl crate::ResetValue for super::CC_GEN_PAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CC_FIFO_DEPTH`"]
pub type CC_FIFO_DEPTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CC_FIFO_DEPTH`"]
pub struct CC_FIFO_DEPTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_FIFO_DEPTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Camera Core FIFO DEPTH generic parameter"]
    #[inline(always)]
    pub fn cc_fifo_depth(&self) -> CC_FIFO_DEPTH_R {
        CC_FIFO_DEPTH_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Camera Core FIFO DEPTH generic parameter"]
    #[inline(always)]
    pub fn cc_fifo_depth(&mut self) -> CC_FIFO_DEPTH_W {
        CC_FIFO_DEPTH_W { w: self }
    }
}
