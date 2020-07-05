#[doc = "Reader of register APPS_RESET_CAUSE"]
pub type R = crate::R<u32, super::APPS_RESET_CAUSE>;
#[doc = "Writer for register APPS_RESET_CAUSE"]
pub type W = crate::W<u32, super::APPS_RESET_CAUSE>;
#[doc = "Register APPS_RESET_CAUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::APPS_RESET_CAUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APPS_RESET_CAUSE`"]
pub type APPS_RESET_CAUSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APPS_RESET_CAUSE`"]
pub struct APPS_RESET_CAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> APPS_RESET_CAUSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Indicates the reset cause for APPS : &quot;0000&quot; - Wake from HIB/OFF mode; &quot;0001&quot; - Wake from LPDS ; &quot;0010&quot; - Reserved ; &quot;0011&quot; - Soft-reset0 (Only APPS Cortex-sysrstn is asserted); &quot;0100&quot; - Soft-reset1 (APPS Cortex-sysrstn and APPS peripherals are reset); &quot;0101&quot; - WDOG0 (APPS Cortex-sysrstn and APPS peripherals are reset); &quot;0110&quot; - MCU Soft-reset (APPS + NWP Cortex-sysrstn + Peripherals are reset); &quot;0111&quot; - Secure Init done (Indication that reset has happened after DevInit); &quot;1000&quot; - Dev Mode Patch Init done (During development mode, patch downloading and Cortex re-vectoring is completed)"]
    #[inline(always)]
    pub fn apps_reset_cause(&self) -> APPS_RESET_CAUSE_R {
        APPS_RESET_CAUSE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates the reset cause for APPS : &quot;0000&quot; - Wake from HIB/OFF mode; &quot;0001&quot; - Wake from LPDS ; &quot;0010&quot; - Reserved ; &quot;0011&quot; - Soft-reset0 (Only APPS Cortex-sysrstn is asserted); &quot;0100&quot; - Soft-reset1 (APPS Cortex-sysrstn and APPS peripherals are reset); &quot;0101&quot; - WDOG0 (APPS Cortex-sysrstn and APPS peripherals are reset); &quot;0110&quot; - MCU Soft-reset (APPS + NWP Cortex-sysrstn + Peripherals are reset); &quot;0111&quot; - Secure Init done (Indication that reset has happened after DevInit); &quot;1000&quot; - Dev Mode Patch Init done (During development mode, patch downloading and Cortex re-vectoring is completed)"]
    #[inline(always)]
    pub fn apps_reset_cause(&mut self) -> APPS_RESET_CAUSE_W {
        APPS_RESET_CAUSE_W { w: self }
    }
}
