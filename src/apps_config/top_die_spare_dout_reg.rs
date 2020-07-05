#[doc = "Reader of register TOP_DIE_SPARE_DOUT_REG"]
pub type R = crate::R<u32, super::TOP_DIE_SPARE_DOUT_REG>;
#[doc = "Writer for register TOP_DIE_SPARE_DOUT_REG"]
pub type W = crate::W<u32, super::TOP_DIE_SPARE_DOUT_REG>;
#[doc = "Register TOP_DIE_SPARE_DOUT_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::TOP_DIE_SPARE_DOUT_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D2D_SPARE_DOUT`"]
pub type D2D_SPARE_DOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `D2D_SPARE_DOUT`"]
pub struct D2D_SPARE_DOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> D2D_SPARE_DOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Send data to d2d_spare pads - eventually this will get registered in top die"]
    #[inline(always)]
    pub fn d2d_spare_dout(&self) -> D2D_SPARE_DOUT_R {
        D2D_SPARE_DOUT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Send data to d2d_spare pads - eventually this will get registered in top die"]
    #[inline(always)]
    pub fn d2d_spare_dout(&mut self) -> D2D_SPARE_DOUT_W {
        D2D_SPARE_DOUT_W { w: self }
    }
}
