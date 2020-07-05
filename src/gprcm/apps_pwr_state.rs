#[doc = "Reader of register APPS_PWR_STATE"]
pub type R = crate::R<u32, super::APPS_PWR_STATE>;
#[doc = "Writer for register APPS_PWR_STATE"]
pub type W = crate::W<u32, super::APPS_PWR_STATE>;
#[doc = "Register APPS_PWR_STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::APPS_PWR_STATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APPS_PWR_STATE_PS`"]
pub type APPS_PWR_STATE_PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APPS_PWR_STATE_PS`"]
pub struct APPS_PWR_STATE_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> APPS_PWR_STATE_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `APPS_RCM_PS`"]
pub type APPS_RCM_PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APPS_RCM_PS`"]
pub struct APPS_RCM_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> APPS_RCM_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - &quot;0000&quot;- PORZ :- APPS is waiting for PLL_clock during powerup (from HIB/OFF) ; &quot;0011&quot;- ACTIVE :- APPS is enabled, clocks and resets to APPS-SubSystem are enabled ; APPS might be either in Secure or Un-secure mode during this state. &quot;1001&quot; - SECURE_MODE_LPDS :- While in ACTIVE (Secure-mode), APPS had to program the DevInit_done bit at the end, after which it enters into this state, where the reset to APPS will be asserted. From this state APPS might either re-boot itself or enter into LPDS depending upon whether the device is 3200 or 3100. &quot;0010&quot;- LPDS :- APPS is in LPDS-mode ; Clocks and reset to APPS-SubSystem are gated ; &quot;0101&quot;- WAIT_FOR_OPP :- APPS is in transition from LPDS to ACTIVE, where it is waiting for OPP to be stable ; &quot;1000&quot; - WAKE_TIMER_OPP_REQ : APPS is in transition from LPDS, where the wakeup cause is LPDS_Wake timer ; &quot;1010&quot; - WAIT_FOR_PATCH_INIT : APPS enters into this state during development-mode #3 (SOP = 3), where it is waiting for patch download to complete and 0x4 hack is programmed. OTHERS : NA"]
    #[inline(always)]
    pub fn apps_pwr_state_ps(&self) -> APPS_PWR_STATE_PS_R {
        APPS_PWR_STATE_PS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2 - &quot;000&quot; - APPS_RUN : APPS is in RUN state (default) - Applicable only when APPS_PWR_STATE_PS = ACTIVE ; &quot;001&quot; - APPS_SLP : APPS is in SLEEP state (default) - Applicable only when APPS_PWR_STATE_PS = ACTIVE ; &quot;010&quot; - APPS_DSLP : APPS is in Deep-Sleep state (default) - Applicable only when APPS_PWR_STATE_PS = ACTIVE ; &quot;011&quot; - WAIT_FOR_ACTIVE : APPS is in transition from Deep-sleep to Run, where it is waiting for OPP to be stable ; &quot;100&quot; - WAIT_FOR_DSLP_TIMER_WAKE_REQ : APPS is in transition from Deep-sleep to Run, where the wakeup cause is deep-sleep wake-timer"]
    #[inline(always)]
    pub fn apps_rcm_ps(&self) -> APPS_RCM_PS_R {
        APPS_RCM_PS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - &quot;0000&quot;- PORZ :- APPS is waiting for PLL_clock during powerup (from HIB/OFF) ; &quot;0011&quot;- ACTIVE :- APPS is enabled, clocks and resets to APPS-SubSystem are enabled ; APPS might be either in Secure or Un-secure mode during this state. &quot;1001&quot; - SECURE_MODE_LPDS :- While in ACTIVE (Secure-mode), APPS had to program the DevInit_done bit at the end, after which it enters into this state, where the reset to APPS will be asserted. From this state APPS might either re-boot itself or enter into LPDS depending upon whether the device is 3200 or 3100. &quot;0010&quot;- LPDS :- APPS is in LPDS-mode ; Clocks and reset to APPS-SubSystem are gated ; &quot;0101&quot;- WAIT_FOR_OPP :- APPS is in transition from LPDS to ACTIVE, where it is waiting for OPP to be stable ; &quot;1000&quot; - WAKE_TIMER_OPP_REQ : APPS is in transition from LPDS, where the wakeup cause is LPDS_Wake timer ; &quot;1010&quot; - WAIT_FOR_PATCH_INIT : APPS enters into this state during development-mode #3 (SOP = 3), where it is waiting for patch download to complete and 0x4 hack is programmed. OTHERS : NA"]
    #[inline(always)]
    pub fn apps_pwr_state_ps(&mut self) -> APPS_PWR_STATE_PS_W {
        APPS_PWR_STATE_PS_W { w: self }
    }
    #[doc = "Bits 0:2 - &quot;000&quot; - APPS_RUN : APPS is in RUN state (default) - Applicable only when APPS_PWR_STATE_PS = ACTIVE ; &quot;001&quot; - APPS_SLP : APPS is in SLEEP state (default) - Applicable only when APPS_PWR_STATE_PS = ACTIVE ; &quot;010&quot; - APPS_DSLP : APPS is in Deep-Sleep state (default) - Applicable only when APPS_PWR_STATE_PS = ACTIVE ; &quot;011&quot; - WAIT_FOR_ACTIVE : APPS is in transition from Deep-sleep to Run, where it is waiting for OPP to be stable ; &quot;100&quot; - WAIT_FOR_DSLP_TIMER_WAKE_REQ : APPS is in transition from Deep-sleep to Run, where the wakeup cause is deep-sleep wake-timer"]
    #[inline(always)]
    pub fn apps_rcm_ps(&mut self) -> APPS_RCM_PS_W {
        APPS_RCM_PS_W { w: self }
    }
}
