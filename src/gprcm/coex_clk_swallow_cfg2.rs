#[doc = "Reader of register COEX_CLK_SWALLOW_CFG2"]
pub type R = crate::R<u32, super::COEX_CLK_SWALLOW_CFG2>;
#[doc = "Writer for register COEX_CLK_SWALLOW_CFG2"]
pub type W = crate::W<u32, super::COEX_CLK_SWALLOW_CFG2>;
#[doc = "Register COEX_CLK_SWALLOW_CFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::COEX_CLK_SWALLOW_CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONSECUTIVE_SWALLOW`"]
pub type CONSECUTIVE_SWALLOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONSECUTIVE_SWALLOW`"]
pub struct CONSECUTIVE_SWALLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> CONSECUTIVE_SWALLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:4 - CONSECUTIVE_SWALLOW"]
    #[inline(always)]
    pub fn consecutive_swallow(&self) -> CONSECUTIVE_SWALLOW_R {
        CONSECUTIVE_SWALLOW_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 3:4 - CONSECUTIVE_SWALLOW"]
    #[inline(always)]
    pub fn consecutive_swallow(&mut self) -> CONSECUTIVE_SWALLOW_W {
        CONSECUTIVE_SWALLOW_W { w: self }
    }
}
