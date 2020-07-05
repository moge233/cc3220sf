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
    #[doc = "Bits 8:9 - Clocks activity during wake up mode period 0x0 OCP and Functional clocks may be switched off. 0x1 OCP clock is maintained. Functional clock may be switched-off. 0x2 Functional clock is maintained. OCP clock may be switched-off. 0x3 OCP and Functional clocks are maintained."]
    #[inline(always)]
    pub fn clockactivity(&self) -> CLOCKACTIVITY_R {
        CLOCKACTIVITY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Power management 0x0 If an idle request is detected the McSPI acknowledges it unconditionally and goes in Inactive mode. Interrupt DMA requests and wake up lines are unconditionally de-asserted and the module wakeup capability is deactivated even if the bit MCSPI_SYSCONFIG\\[EnaWakeUp\\]
is set. 0x1 If an idle request is detected the request is ignored and the module does not switch to wake up mode and keeps on behaving normally. 0x2 If an idle request is detected the module will switch to idle mode based on its internal activity. The wake up capability cannot be used. 0x3 If an idle request is detected the module will switch to idle mode based on its internal activity and the wake up capability can be used if the bit MCSPI_SYSCONFIG\\[EnaWakeUp\\]
is set."]
    #[inline(always)]
    pub fn sidlemode(&self) -> SIDLEMODE_R {
        SIDLEMODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - Clocks activity during wake up mode period 0x0 OCP and Functional clocks may be switched off. 0x1 OCP clock is maintained. Functional clock may be switched-off. 0x2 Functional clock is maintained. OCP clock may be switched-off. 0x3 OCP and Functional clocks are maintained."]
    #[inline(always)]
    pub fn clockactivity(&mut self) -> CLOCKACTIVITY_W {
        CLOCKACTIVITY_W { w: self }
    }
    #[doc = "Bits 3:4 - Power management 0x0 If an idle request is detected the McSPI acknowledges it unconditionally and goes in Inactive mode. Interrupt DMA requests and wake up lines are unconditionally de-asserted and the module wakeup capability is deactivated even if the bit MCSPI_SYSCONFIG\\[EnaWakeUp\\]
is set. 0x1 If an idle request is detected the request is ignored and the module does not switch to wake up mode and keeps on behaving normally. 0x2 If an idle request is detected the module will switch to idle mode based on its internal activity. The wake up capability cannot be used. 0x3 If an idle request is detected the module will switch to idle mode based on its internal activity and the wake up capability can be used if the bit MCSPI_SYSCONFIG\\[EnaWakeUp\\]
is set."]
    #[inline(always)]
    pub fn sidlemode(&mut self) -> SIDLEMODE_W {
        SIDLEMODE_W { w: self }
    }
}
