#[doc = "Reader of register MCU_PSCON_DEBUG"]
pub type R = crate::R<u32, super::MCU_PSCON_DEBUG>;
#[doc = "Writer for register MCU_PSCON_DEBUG"]
pub type W = crate::W<u32, super::MCU_PSCON_DEBUG>;
#[doc = "Register MCU_PSCON_DEBUG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCU_PSCON_DEBUG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCU_PSCON_RTC_PS`"]
pub type MCU_PSCON_RTC_PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCU_PSCON_RTC_PS`"]
pub struct MCU_PSCON_RTC_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_PSCON_RTC_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `MCU_PSCON_SYS_PS`"]
pub type MCU_PSCON_SYS_PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCU_PSCON_SYS_PS`"]
pub struct MCU_PSCON_SYS_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_PSCON_SYS_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5 - MCU_PSCON_RTC_ON = &quot;0000&quot;; MCU_PSCON_RTC_OFF = &quot;0001&quot;; MCU_PSCON_RTC_RET = &quot;0010&quot;; MCU_PSCON_RTC_OFF_TO_ON = &quot;0011&quot;; MCU_PSCON_RTC_RET_TO_ON = &quot;0100&quot;; MCU_PSCON_RTC_ON_TO_RET = &quot;0101&quot;; MCU_PSCON_RTC_ON_TO_OFF = &quot;0110&quot;; MCU_PSCON_RTC_RET_TO_ON_WAIT_OPP = &quot;0111&quot;; MCU_PSCON_RTC_OFF_TO_ON_WAIT_OPP = &quot;1000&quot;;"]
    #[inline(always)]
    pub fn mcu_pscon_rtc_ps(&self) -> MCU_PSCON_RTC_PS_R {
        MCU_PSCON_RTC_PS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - MCU_PSCON_SYS_PS"]
    #[inline(always)]
    pub fn mcu_pscon_sys_ps(&self) -> MCU_PSCON_SYS_PS_R {
        MCU_PSCON_SYS_PS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - MCU_PSCON_RTC_ON = &quot;0000&quot;; MCU_PSCON_RTC_OFF = &quot;0001&quot;; MCU_PSCON_RTC_RET = &quot;0010&quot;; MCU_PSCON_RTC_OFF_TO_ON = &quot;0011&quot;; MCU_PSCON_RTC_RET_TO_ON = &quot;0100&quot;; MCU_PSCON_RTC_ON_TO_RET = &quot;0101&quot;; MCU_PSCON_RTC_ON_TO_OFF = &quot;0110&quot;; MCU_PSCON_RTC_RET_TO_ON_WAIT_OPP = &quot;0111&quot;; MCU_PSCON_RTC_OFF_TO_ON_WAIT_OPP = &quot;1000&quot;;"]
    #[inline(always)]
    pub fn mcu_pscon_rtc_ps(&mut self) -> MCU_PSCON_RTC_PS_W {
        MCU_PSCON_RTC_PS_W { w: self }
    }
    #[doc = "Bits 0:2 - MCU_PSCON_SYS_PS"]
    #[inline(always)]
    pub fn mcu_pscon_sys_ps(&mut self) -> MCU_PSCON_SYS_PS_W {
        MCU_PSCON_SYS_PS_W { w: self }
    }
}
