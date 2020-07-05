#[doc = "Reader of register MEM_HIB_RTC_WAKE_MSW_CONF"]
pub type R = crate::R<u32, super::MEM_HIB_RTC_WAKE_MSW_CONF>;
#[doc = "Writer for register MEM_HIB_RTC_WAKE_MSW_CONF"]
pub type W = crate::W<u32, super::MEM_HIB_RTC_WAKE_MSW_CONF>;
#[doc = "Register MEM_HIB_RTC_WAKE_MSW_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_HIB_RTC_WAKE_MSW_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_HIB_RTC_WAKE_MSW_CONF`"]
pub type MEM_HIB_RTC_WAKE_MSW_CONF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_HIB_RTC_WAKE_MSW_CONF`"]
pub struct MEM_HIB_RTC_WAKE_MSW_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_HIB_RTC_WAKE_MSW_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Configuration for RTC-Timer Wakeup (Upper 16b word)"]
    #[inline(always)]
    pub fn mem_hib_rtc_wake_msw_conf(&self) -> MEM_HIB_RTC_WAKE_MSW_CONF_R {
        MEM_HIB_RTC_WAKE_MSW_CONF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configuration for RTC-Timer Wakeup (Upper 16b word)"]
    #[inline(always)]
    pub fn mem_hib_rtc_wake_msw_conf(&mut self) -> MEM_HIB_RTC_WAKE_MSW_CONF_W {
        MEM_HIB_RTC_WAKE_MSW_CONF_W { w: self }
    }
}
