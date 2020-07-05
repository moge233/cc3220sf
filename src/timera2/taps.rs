#[doc = "Reader of register TAPS"]
pub type R = crate::R<u32, super::TAPS>;
#[doc = "Writer for register TAPS"]
pub type W = crate::W<u32, super::TAPS>;
#[doc = "Register TAPS `reset()`'s with value 0"]
impl crate::ResetValue for super::TAPS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSS`"]
pub type PSS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PSS`"]
pub struct PSS_W<'a> {
    w: &'a mut W,
}
impl<'a> PSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Snapshot"]
    #[inline(always)]
    pub fn pss(&self) -> PSS_R {
        PSS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Snapshot"]
    #[inline(always)]
    pub fn pss(&mut self) -> PSS_W {
        PSS_W { w: self }
    }
}
