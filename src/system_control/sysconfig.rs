#[doc = "Reader of register SYSCONFIG"]
pub type R = crate::R<u32, super::SYSCONFIG>;
#[doc = "Writer for register SYSCONFIG"]
pub type W = crate::W<u32, super::SYSCONFIG>;
#[doc = "Register SYSCONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STANDBYMODE`"]
pub type STANDBYMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STANDBYMODE`"]
pub struct STANDBYMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBYMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `CLOCKACTIVITY`"]
pub type CLOCKACTIVITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLOCKACTIVITY`"]
pub struct CLOCKACTIVITY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKACTIVITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SIDLEMODE`"]
pub type SIDLEMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIDLEMODE`"]
pub struct SIDLEMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDLEMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:13 - Master interface power Management standby/wait control. The bit field is only useful when generic parameter MADMA_EN (Master ADMA enable) is set as active otherwise it is a read only register read a '0'. 0x0 Force-standby. Mstandby is forced unconditionnaly. 0x1 No-standby. Mstandby is never asserted. 0x2 Smart-standby mode: local initiator standby status depends on local conditions i.e. the module's functional requirement from the initiator.IP module shall not generate (initiator-related) wakeup events. 0x3 Smart-Standby wakeup-capable mode: &quot;local initiator standby status depends on local conditions i.e. the module's functional requirement from the initiator. IP module may generate (master-related) wakeup events when in standby state.Mode is only relevant if the appropriate IP module &quot;&quot;mwakeup&quot;&quot; output is implemented.&quot;"]
    #[inline(always)]
    pub fn standbymode(&self) -> STANDBYMODE_R {
        STANDBYMODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Clocks activity during wake up mode period. Bit8: OCP interface clock Bit9: Functional clock 0x0 OCP and Functional clock may be switched off. 0x1 OCP clock is maintained. Functional clock may be switched-off. 0x2 Functional clock is maintained. OCP clock may be switched-off. 0x3 OCP and Functional clocks are maintained."]
    #[inline(always)]
    pub fn clockactivity(&self) -> CLOCKACTIVITY_R {
        CLOCKACTIVITY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Power management 0x0 If an idle request is detected the MMCHS acknowledges it unconditionally and goes in Inactive mode. Interrupt and DMA requests are unconditionally de-asserted. 0x1 If an idle request is detected the request is ignored and the module keeps on behaving normally. 0x2 Smart-idle mode: local target's idle state eventually follows (acknowledges) the system's idle requests depending on the IP module's internal requirements.IP module shall not generate (IRQ- or DMA-request-related) wakeup events. 0x3 Smart-idle wakeup-capable mode: &quot;local target's idle state eventually follows (acknowledges) the system's idle requests depending on the IP module's internal requirements.IP module may generate (IRQ- or DMA-request-related) wakeup events when in idle state.Mode is only relevant if the appropriate IP module &quot;&quot;swakeup&quot;&quot; output(s) is (are) implemented.&quot;"]
    #[inline(always)]
    pub fn sidlemode(&self) -> SIDLEMODE_R {
        SIDLEMODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13 - Master interface power Management standby/wait control. The bit field is only useful when generic parameter MADMA_EN (Master ADMA enable) is set as active otherwise it is a read only register read a '0'. 0x0 Force-standby. Mstandby is forced unconditionnaly. 0x1 No-standby. Mstandby is never asserted. 0x2 Smart-standby mode: local initiator standby status depends on local conditions i.e. the module's functional requirement from the initiator.IP module shall not generate (initiator-related) wakeup events. 0x3 Smart-Standby wakeup-capable mode: &quot;local initiator standby status depends on local conditions i.e. the module's functional requirement from the initiator. IP module may generate (master-related) wakeup events when in standby state.Mode is only relevant if the appropriate IP module &quot;&quot;mwakeup&quot;&quot; output is implemented.&quot;"]
    #[inline(always)]
    pub fn standbymode(&mut self) -> STANDBYMODE_W {
        STANDBYMODE_W { w: self }
    }
    #[doc = "Bits 8:9 - Clocks activity during wake up mode period. Bit8: OCP interface clock Bit9: Functional clock 0x0 OCP and Functional clock may be switched off. 0x1 OCP clock is maintained. Functional clock may be switched-off. 0x2 Functional clock is maintained. OCP clock may be switched-off. 0x3 OCP and Functional clocks are maintained."]
    #[inline(always)]
    pub fn clockactivity(&mut self) -> CLOCKACTIVITY_W {
        CLOCKACTIVITY_W { w: self }
    }
    #[doc = "Bits 3:4 - Power management 0x0 If an idle request is detected the MMCHS acknowledges it unconditionally and goes in Inactive mode. Interrupt and DMA requests are unconditionally de-asserted. 0x1 If an idle request is detected the request is ignored and the module keeps on behaving normally. 0x2 Smart-idle mode: local target's idle state eventually follows (acknowledges) the system's idle requests depending on the IP module's internal requirements.IP module shall not generate (IRQ- or DMA-request-related) wakeup events. 0x3 Smart-idle wakeup-capable mode: &quot;local target's idle state eventually follows (acknowledges) the system's idle requests depending on the IP module's internal requirements.IP module may generate (IRQ- or DMA-request-related) wakeup events when in idle state.Mode is only relevant if the appropriate IP module &quot;&quot;swakeup&quot;&quot; output(s) is (are) implemented.&quot;"]
    #[inline(always)]
    pub fn sidlemode(&mut self) -> SIDLEMODE_W {
        SIDLEMODE_W { w: self }
    }
}
