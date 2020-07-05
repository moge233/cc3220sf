#[doc = "Reader of register TOP_DIE_SPARE_DIN_REG"]
pub type R = crate::R<u32, super::TOP_DIE_SPARE_DIN_REG>;
#[doc = "Writer for register TOP_DIE_SPARE_DIN_REG"]
pub type W = crate::W<u32, super::TOP_DIE_SPARE_DIN_REG>;
#[doc = "Register TOP_DIE_SPARE_DIN_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::TOP_DIE_SPARE_DIN_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D2D_SPARE_DIN`"]
pub type D2D_SPARE_DIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `D2D_SPARE_DIN`"]
pub struct D2D_SPARE_DIN_W<'a> {
    w: &'a mut W,
}
impl<'a> D2D_SPARE_DIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Capture data from d2d_spare pads"]
    #[inline(always)]
    pub fn d2d_spare_din(&self) -> D2D_SPARE_DIN_R {
        D2D_SPARE_DIN_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Capture data from d2d_spare pads"]
    #[inline(always)]
    pub fn d2d_spare_din(&mut self) -> D2D_SPARE_DIN_W {
        D2D_SPARE_DIN_W { w: self }
    }
}
