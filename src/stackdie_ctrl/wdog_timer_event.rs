#[doc = "Reader of register WDOG_TIMER_EVENT"]
pub type R = crate::R<u32, super::WDOG_TIMER_EVENT>;
#[doc = "Writer for register WDOG_TIMER_EVENT"]
pub type W = crate::W<u32, super::WDOG_TIMER_EVENT>;
#[doc = "Register WDOG_TIMER_EVENT `reset()`'s with value 0"]
impl crate::ResetValue for super::WDOG_TIMER_EVENT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
