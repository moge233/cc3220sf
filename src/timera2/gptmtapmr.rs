#[doc = "Reader of register GPTMTAPMR"]
pub type R = crate::R<u32, super::GPTMTAPMR>;
#[doc = "Writer for register GPTMTAPMR"]
pub type W = crate::W<u32, super::GPTMTAPMR>;
#[doc = "Register GPTMTAPMR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTMTAPMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAPSMR`"]
pub type TAPSMR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TAPSMR`"]
pub struct TAPSMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPSMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPTM TimerA Prescale Match"]
    #[inline(always)]
    pub fn tapsmr(&self) -> TAPSMR_R {
        TAPSMR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM TimerA Prescale Match"]
    #[inline(always)]
    pub fn tapsmr(&mut self) -> TAPSMR_W {
        TAPSMR_W { w: self }
    }
}
