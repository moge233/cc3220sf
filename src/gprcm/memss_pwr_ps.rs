#[doc = "Reader of register MEMSS_PWR_PS"]
pub type R = crate::R<u32, super::MEMSS_PWR_PS>;
#[doc = "Writer for register MEMSS_PWR_PS"]
pub type W = crate::W<u32, super::MEMSS_PWR_PS>;
#[doc = "Register MEMSS_PWR_PS `reset()`'s with value 0"]
impl crate::ResetValue for super::MEMSS_PWR_PS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWR_PS_MEMSS`"]
pub type PWR_PS_MEMSS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWR_PS_MEMSS`"]
pub struct PWR_PS_MEMSS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_PS_MEMSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - MEMSS_PM_SLEEP = &quot;000&quot;; MEMSS_PM_WAIT_OPP = &quot;010&quot;; MEMSS_PM_ACTIVE = &quot;011&quot;; MEMSS_PM_SLEEP_TO_ACTIVE = &quot;100&quot;; MEMSS_PM_ACTIVE_TO_SLEEP = &quot;101&quot;;"]
    #[inline(always)]
    pub fn pwr_ps_memss(&self) -> PWR_PS_MEMSS_R {
        PWR_PS_MEMSS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MEMSS_PM_SLEEP = &quot;000&quot;; MEMSS_PM_WAIT_OPP = &quot;010&quot;; MEMSS_PM_ACTIVE = &quot;011&quot;; MEMSS_PM_SLEEP_TO_ACTIVE = &quot;100&quot;; MEMSS_PM_ACTIVE_TO_SLEEP = &quot;101&quot;;"]
    #[inline(always)]
    pub fn pwr_ps_memss(&mut self) -> PWR_PS_MEMSS_W {
        PWR_PS_MEMSS_W { w: self }
    }
}
