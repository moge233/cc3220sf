#[doc = "Reader of register GPTMTAPV"]
pub type R = crate::R<u32, super::GPTMTAPV>;
#[doc = "Writer for register GPTMTAPV"]
pub type W = crate::W<u32, super::GPTMTAPV>;
#[doc = "Register GPTMTAPV `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTMTAPV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSV`"]
pub type PSV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PSV`"]
pub struct PSV_W<'a> {
    w: &'a mut W,
}
impl<'a> PSV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Value"]
    #[inline(always)]
    pub fn psv(&self) -> PSV_R {
        PSV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Value"]
    #[inline(always)]
    pub fn psv(&mut self) -> PSV_W {
        PSV_W { w: self }
    }
}
