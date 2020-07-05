#[doc = "Reader of register HIB_TIMER_RTC_GTS_TIMESTAMP_MSW"]
pub type R = crate::R<u32, super::HIB_TIMER_RTC_GTS_TIMESTAMP_MSW>;
#[doc = "Writer for register HIB_TIMER_RTC_GTS_TIMESTAMP_MSW"]
pub type W = crate::W<u32, super::HIB_TIMER_RTC_GTS_TIMESTAMP_MSW>;
#[doc = "Register HIB_TIMER_RTC_GTS_TIMESTAMP_MSW `reset()`'s with value 0"]
impl crate::ResetValue for super::HIB_TIMER_RTC_GTS_TIMESTAMP_MSW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_GTS_TIMESTAMP_MSW`"]
pub type RTC_GTS_TIMESTAMP_MSW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_GTS_TIMESTAMP_MSW`"]
pub struct RTC_GTS_TIMESTAMP_MSW_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GTS_TIMESTAMP_MSW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RTC_GTS_TIMESTAMP_MSW"]
    #[inline(always)]
    pub fn rtc_gts_timestamp_msw(&self) -> RTC_GTS_TIMESTAMP_MSW_R {
        RTC_GTS_TIMESTAMP_MSW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC_GTS_TIMESTAMP_MSW"]
    #[inline(always)]
    pub fn rtc_gts_timestamp_msw(&mut self) -> RTC_GTS_TIMESTAMP_MSW_W {
        RTC_GTS_TIMESTAMP_MSW_W { w: self }
    }
}
