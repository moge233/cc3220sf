#[doc = "Reader of register NWP_RESET_CAUSE"]
pub type R = crate::R<u32, super::NWP_RESET_CAUSE>;
#[doc = "Writer for register NWP_RESET_CAUSE"]
pub type W = crate::W<u32, super::NWP_RESET_CAUSE>;
#[doc = "Register NWP_RESET_CAUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::NWP_RESET_CAUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NWP_RESET_CAUSE`"]
pub type NWP_RESET_CAUSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NWP_RESET_CAUSE`"]
pub struct NWP_RESET_CAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> NWP_RESET_CAUSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Indicates the reset cause for NWP : &quot;0000&quot; - Wake from HIB/OFF mode; &quot;0001&quot; - Wake from LPDS ; &quot;0010&quot; - Reserved ; &quot;0011&quot; - Soft-reset0 (Only NWP Cortex-sysrstn is asserted); &quot;0100&quot; - Soft-reset1 (NWP Cortex-sysrstn and NWP peripherals are reset); &quot;0101&quot; - WDOG0 (NWP Cortex-sysrstn and NWP peripherals are reset); &quot;0110&quot; - MCU Soft-reset (APPS + NWP Cortex-sysrstn + Peripherals are reset); &quot;0111&quot; - SSDIO Function2 reset (Only Cortex-sysrstn is asserted) ; &quot;1000&quot; - Reset due to WDOG of APPS (NWP Cortex-sysrstn and NWP peripherals are reset);"]
    #[inline(always)]
    pub fn nwp_reset_cause(&self) -> NWP_RESET_CAUSE_R {
        NWP_RESET_CAUSE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates the reset cause for NWP : &quot;0000&quot; - Wake from HIB/OFF mode; &quot;0001&quot; - Wake from LPDS ; &quot;0010&quot; - Reserved ; &quot;0011&quot; - Soft-reset0 (Only NWP Cortex-sysrstn is asserted); &quot;0100&quot; - Soft-reset1 (NWP Cortex-sysrstn and NWP peripherals are reset); &quot;0101&quot; - WDOG0 (NWP Cortex-sysrstn and NWP peripherals are reset); &quot;0110&quot; - MCU Soft-reset (APPS + NWP Cortex-sysrstn + Peripherals are reset); &quot;0111&quot; - SSDIO Function2 reset (Only Cortex-sysrstn is asserted) ; &quot;1000&quot; - Reset due to WDOG of APPS (NWP Cortex-sysrstn and NWP peripherals are reset);"]
    #[inline(always)]
    pub fn nwp_reset_cause(&mut self) -> NWP_RESET_CAUSE_W {
        NWP_RESET_CAUSE_W { w: self }
    }
}
