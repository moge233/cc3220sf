#[doc = "Reader of register TIMER_CURRENT_COUNT"]
pub type R = crate::R<u32, super::TIMER_CURRENT_COUNT>;
#[doc = "Writer for register TIMER_CURRENT_COUNT"]
pub type W = crate::W<u32, super::TIMER_CURRENT_COUNT>;
#[doc = "Register TIMER_CURRENT_COUNT `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER_CURRENT_COUNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMERCURRENTCOUNT`"]
pub type TIMERCURRENTCOUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMERCURRENTCOUNT`"]
pub struct TIMERCURRENTCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERCURRENTCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - ADC timer count."]
    #[inline(always)]
    pub fn timercurrentcount(&self) -> TIMERCURRENTCOUNT_R {
        TIMERCURRENTCOUNT_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - ADC timer count."]
    #[inline(always)]
    pub fn timercurrentcount(&mut self) -> TIMERCURRENTCOUNT_W {
        TIMERCURRENTCOUNT_W { w: self }
    }
}
