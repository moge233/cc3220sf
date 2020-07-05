#[doc = "Reader of register NWP_PWR_STATE"]
pub type R = crate::R<u32, super::NWP_PWR_STATE>;
#[doc = "Writer for register NWP_PWR_STATE"]
pub type W = crate::W<u32, super::NWP_PWR_STATE>;
#[doc = "Register NWP_PWR_STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::NWP_PWR_STATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NWP_PWR_STATE_PS`"]
pub type NWP_PWR_STATE_PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NWP_PWR_STATE_PS`"]
pub struct NWP_PWR_STATE_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> NWP_PWR_STATE_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `NWP_RCM_PS`"]
pub type NWP_RCM_PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NWP_RCM_PS`"]
pub struct NWP_RCM_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> NWP_RCM_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - &quot;0000&quot;- PORZ :- NWP is yet to be enabled by APPS during powerup (from HIB/OFF) ; &quot;0011&quot;- ACTIVE :- NWP is enabled, clocks and resets to NWP-SubSystem are enabled ; &quot;0010&quot;- LPDS :- NWP is in LPDS-mode ; Clocks and reset to NWP-SubSystem are gated ; &quot;0101&quot;- WAIT_FOR_OPP :- NWP is in transition from LPDS to ACTIVE, where it is waiting for OPP to be stable ; &quot;1000&quot;- WAKE_TIMER_OPP_REQ :- NWP is in transition from LPDS, where the wakeup cause is LPDS_Wake timer OTHERS : NA"]
    #[inline(always)]
    pub fn nwp_pwr_state_ps(&self) -> NWP_PWR_STATE_PS_R {
        NWP_PWR_STATE_PS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2 - &quot;000&quot; - NWP_RUN : NWP is in RUN state (default) - Applicable only when NWP_PWR_STATE_PS = ACTIVE ; &quot;001&quot; - NWP_SLP : NWP is in SLEEP state (default) - Applicable only when NWP_PWR_STATE_PS = ACTIVE ; &quot;010&quot; - NWP_DSLP : NWP is in Deep-Sleep state (default) - Applicable only when NWP_PWR_STATE_PS = ACTIVE ; &quot;011&quot; - WAIT_FOR_ACTIVE : NWP is in transition from Deep-sleep to Run, where it is waiting for OPP to be stable ; &quot;100&quot; - WAIT_FOR_DSLP_TIMER_WAKE_REQ : NWP is in transition from Deep-sleep to Run, where the wakeup cause is deep-sleep wake-timer"]
    #[inline(always)]
    pub fn nwp_rcm_ps(&self) -> NWP_RCM_PS_R {
        NWP_RCM_PS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - &quot;0000&quot;- PORZ :- NWP is yet to be enabled by APPS during powerup (from HIB/OFF) ; &quot;0011&quot;- ACTIVE :- NWP is enabled, clocks and resets to NWP-SubSystem are enabled ; &quot;0010&quot;- LPDS :- NWP is in LPDS-mode ; Clocks and reset to NWP-SubSystem are gated ; &quot;0101&quot;- WAIT_FOR_OPP :- NWP is in transition from LPDS to ACTIVE, where it is waiting for OPP to be stable ; &quot;1000&quot;- WAKE_TIMER_OPP_REQ :- NWP is in transition from LPDS, where the wakeup cause is LPDS_Wake timer OTHERS : NA"]
    #[inline(always)]
    pub fn nwp_pwr_state_ps(&mut self) -> NWP_PWR_STATE_PS_W {
        NWP_PWR_STATE_PS_W { w: self }
    }
    #[doc = "Bits 0:2 - &quot;000&quot; - NWP_RUN : NWP is in RUN state (default) - Applicable only when NWP_PWR_STATE_PS = ACTIVE ; &quot;001&quot; - NWP_SLP : NWP is in SLEEP state (default) - Applicable only when NWP_PWR_STATE_PS = ACTIVE ; &quot;010&quot; - NWP_DSLP : NWP is in Deep-Sleep state (default) - Applicable only when NWP_PWR_STATE_PS = ACTIVE ; &quot;011&quot; - WAIT_FOR_ACTIVE : NWP is in transition from Deep-sleep to Run, where it is waiting for OPP to be stable ; &quot;100&quot; - WAIT_FOR_DSLP_TIMER_WAKE_REQ : NWP is in transition from Deep-sleep to Run, where the wakeup cause is deep-sleep wake-timer"]
    #[inline(always)]
    pub fn nwp_rcm_ps(&mut self) -> NWP_RCM_PS_W {
        NWP_RCM_PS_W { w: self }
    }
}
