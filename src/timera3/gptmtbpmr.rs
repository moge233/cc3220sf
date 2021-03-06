#[doc = "Reader of register GPTMTBPMR"]
pub type R = crate::R<u32, super::GPTMTBPMR>;
#[doc = "Writer for register GPTMTBPMR"]
pub type W = crate::W<u32, super::GPTMTBPMR>;
#[doc = "Register GPTMTBPMR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTMTBPMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBPSMR`"]
pub type TBPSMR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBPSMR`"]
pub struct TBPSMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPSMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPTM TimerB Prescale Match"]
    #[inline(always)]
    pub fn tbpsmr(&self) -> TBPSMR_R {
        TBPSMR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM TimerB Prescale Match"]
    #[inline(always)]
    pub fn tbpsmr(&mut self) -> TBPSMR_W {
        TBPSMR_W { w: self }
    }
}
