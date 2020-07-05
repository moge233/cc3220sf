#[doc = "Reader of register HL_SYSCONFIG"]
pub type R = crate::R<u32, super::HL_SYSCONFIG>;
#[doc = "Writer for register HL_SYSCONFIG"]
pub type W = crate::W<u32, super::HL_SYSCONFIG>;
#[doc = "Register HL_SYSCONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::HL_SYSCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDLEMODE`"]
pub type IDLEMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDLEMODE`"]
pub struct IDLEMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3 - Configuration of the local target state management mode. By definition target can handle read/write transaction as long as it is out of IDLE state. 0x0 Force-idle mode: local target's idle state follows (acknowledges) the system's idle requests unconditionally i.e. regardless of the IP module's internal requirements.Backup mode for debug only. 0x1 No-idle mode: local target never enters idle state.Backup mode for debug only. 0x2 Smart-idle mode: local target's idle state eventually follows (acknowledges) the system's idle requests depending on the IP module's internal requirements.IP module shall not generate (IRQ- or DMA-request-related) wakeup events. 0x3 &quot;Smart-idle wakeup-capable mode: local target's idle state eventually follows (acknowledges) the system's idle requests depending on the IP module's internal requirements.IP module may generate (IRQ- or DMA-request-related) wakeup events when in idle state.Mode is only relevant if the appropriate IP module &quot;&quot;swakeup&quot;&quot; output(s) is (are) implemented.&quot;"]
    #[inline(always)]
    pub fn idlemode(&self) -> IDLEMODE_R {
        IDLEMODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - Configuration of the local target state management mode. By definition target can handle read/write transaction as long as it is out of IDLE state. 0x0 Force-idle mode: local target's idle state follows (acknowledges) the system's idle requests unconditionally i.e. regardless of the IP module's internal requirements.Backup mode for debug only. 0x1 No-idle mode: local target never enters idle state.Backup mode for debug only. 0x2 Smart-idle mode: local target's idle state eventually follows (acknowledges) the system's idle requests depending on the IP module's internal requirements.IP module shall not generate (IRQ- or DMA-request-related) wakeup events. 0x3 &quot;Smart-idle wakeup-capable mode: local target's idle state eventually follows (acknowledges) the system's idle requests depending on the IP module's internal requirements.IP module may generate (IRQ- or DMA-request-related) wakeup events when in idle state.Mode is only relevant if the appropriate IP module &quot;&quot;swakeup&quot;&quot; output(s) is (are) implemented.&quot;"]
    #[inline(always)]
    pub fn idlemode(&mut self) -> IDLEMODE_W {
        IDLEMODE_W { w: self }
    }
}
