#[doc = "Reader of register COEX_CLK_SWALLOW_CFG0"]
pub type R = crate::R<u32, super::COEX_CLK_SWALLOW_CFG0>;
#[doc = "Writer for register COEX_CLK_SWALLOW_CFG0"]
pub type W = crate::W<u32, super::COEX_CLK_SWALLOW_CFG0>;
#[doc = "Register COEX_CLK_SWALLOW_CFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::COEX_CLK_SWALLOW_CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Q_FACTOR`"]
pub type Q_FACTOR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Q_FACTOR`"]
pub struct Q_FACTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> Q_FACTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x007f_ffff) | ((value as u32) & 0x007f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:22 - TBD"]
    #[inline(always)]
    pub fn q_factor(&self) -> Q_FACTOR_R {
        Q_FACTOR_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:22 - TBD"]
    #[inline(always)]
    pub fn q_factor(&mut self) -> Q_FACTOR_W {
        Q_FACTOR_W { w: self }
    }
}
