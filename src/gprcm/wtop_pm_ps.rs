#[doc = "Reader of register WTOP_PM_PS"]
pub type R = crate::R<u32, super::WTOP_PM_PS>;
#[doc = "Writer for register WTOP_PM_PS"]
pub type W = crate::W<u32, super::WTOP_PM_PS>;
#[doc = "Register WTOP_PM_PS `reset()`'s with value 0"]
impl crate::ResetValue for super::WTOP_PM_PS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTOP_PM_PS`"]
pub type WTOP_PM_PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WTOP_PM_PS`"]
pub struct WTOP_PM_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> WTOP_PM_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - &quot;011&quot; - WTOP_PM_ACTIVE (Default) :- WTOP_Pd is in ACTIVE mode; &quot;100&quot; - WTOP_PM_ACTIVE_TO_SLEEP :- WTOP_Pd is in transition from ACTIVE to SLEEP ; &quot;000&quot; - WTOP_PM_SLEEP : WTOP-Pd is in Sleep-state ; &quot;100&quot; - WTOP_PM_SLEEP_TO_ACTIVE : WTOP_Pd is in transition from SLEEP to ACTIVE ; &quot;000&quot; - WTOP_PM_WAIT_FOR_OPP : Wait for OPP to be stable ;"]
    #[inline(always)]
    pub fn wtop_pm_ps(&self) -> WTOP_PM_PS_R {
        WTOP_PM_PS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - &quot;011&quot; - WTOP_PM_ACTIVE (Default) :- WTOP_Pd is in ACTIVE mode; &quot;100&quot; - WTOP_PM_ACTIVE_TO_SLEEP :- WTOP_Pd is in transition from ACTIVE to SLEEP ; &quot;000&quot; - WTOP_PM_SLEEP : WTOP-Pd is in Sleep-state ; &quot;100&quot; - WTOP_PM_SLEEP_TO_ACTIVE : WTOP_Pd is in transition from SLEEP to ACTIVE ; &quot;000&quot; - WTOP_PM_WAIT_FOR_OPP : Wait for OPP to be stable ;"]
    #[inline(always)]
    pub fn wtop_pm_ps(&mut self) -> WTOP_PM_PS_W {
        WTOP_PM_PS_W { w: self }
    }
}
