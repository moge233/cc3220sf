#[doc = "Reader of register TIMER_CONFIGURATION"]
pub type R = crate::R<u32, super::TIMER_CONFIGURATION>;
#[doc = "Writer for register TIMER_CONFIGURATION"]
pub type W = crate::W<u32, super::TIMER_CONFIGURATION>;
#[doc = "Register TIMER_CONFIGURATION `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER_CONFIGURATION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMEREN`"]
pub type TIMEREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMEREN`"]
pub struct TIMEREN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEREN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `TIMERRESET`"]
pub type TIMERRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMERRESET`"]
pub struct TIMERRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERRESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `TIMERCOUNT`"]
pub type TIMERCOUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMERCOUNT`"]
pub struct TIMERCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - Enable/disable ADC timer."]
    #[inline(always)]
    pub fn timeren(&self) -> TIMEREN_R {
        TIMEREN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Reset ADC timer."]
    #[inline(always)]
    pub fn timerreset(&self) -> TIMERRESET_R {
        TIMERRESET_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 0:23 - ADC timer count configuration."]
    #[inline(always)]
    pub fn timercount(&self) -> TIMERCOUNT_R {
        TIMERCOUNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 25 - Enable/disable ADC timer."]
    #[inline(always)]
    pub fn timeren(&mut self) -> TIMEREN_W {
        TIMEREN_W { w: self }
    }
    #[doc = "Bit 24 - Reset ADC timer."]
    #[inline(always)]
    pub fn timerreset(&mut self) -> TIMERRESET_W {
        TIMERRESET_W { w: self }
    }
    #[doc = "Bits 0:23 - ADC timer count configuration."]
    #[inline(always)]
    pub fn timercount(&mut self) -> TIMERCOUNT_W {
        TIMERCOUNT_W { w: self }
    }
}
