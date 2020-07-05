#[doc = "Reader of register COEX_CLK_SWALLOW_CFG1"]
pub type R = crate::R<u32, super::COEX_CLK_SWALLOW_CFG1>;
#[doc = "Writer for register COEX_CLK_SWALLOW_CFG1"]
pub type W = crate::W<u32, super::COEX_CLK_SWALLOW_CFG1>;
#[doc = "Register COEX_CLK_SWALLOW_CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::COEX_CLK_SWALLOW_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P_FACTOR`"]
pub type P_FACTOR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `P_FACTOR`"]
pub struct P_FACTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> P_FACTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - TBD"]
    #[inline(always)]
    pub fn p_factor(&self) -> P_FACTOR_R {
        P_FACTOR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - TBD"]
    #[inline(always)]
    pub fn p_factor(&mut self) -> P_FACTOR_W {
        P_FACTOR_W { w: self }
    }
}
