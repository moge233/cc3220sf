#[doc = "Reader of register SLPTMRCFG"]
pub type R = crate::R<u32, super::SLPTMRCFG>;
#[doc = "Writer for register SLPTMRCFG"]
pub type W = crate::W<u32, super::SLPTMRCFG>;
#[doc = "Register SLPTMRCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLPTMRCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TMRCFG`"]
pub type TMRCFG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TMRCFG`"]
pub struct TMRCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SLP_WAKE_TIMER_CFG"]
    #[inline(always)]
    pub fn tmrcfg(&self) -> TMRCFG_R {
        TMRCFG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SLP_WAKE_TIMER_CFG"]
    #[inline(always)]
    pub fn tmrcfg(&mut self) -> TMRCFG_W {
        TMRCFG_W { w: self }
    }
}
