#[doc = "Reader of register GPT_TRIG_SEL"]
pub type R = crate::R<u32, super::GPT_TRIG_SEL>;
#[doc = "Writer for register GPT_TRIG_SEL"]
pub type W = crate::W<u32, super::GPT_TRIG_SEL>;
#[doc = "Register GPT_TRIG_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::GPT_TRIG_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPT_TRIG_SEL`"]
pub type GPT_TRIG_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPT_TRIG_SEL`"]
pub struct GPT_TRIG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT_TRIG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This bit is implemented for GPT trigger mode select. GPT IP support 2 modes: RTC mode and external trigger. When this bit is set to logic '1': enable external trigger mode for APPS GPT CP0 and CP1 pin. bit 0: when set '1' enable external GPT trigger 0 on GPIO0 CP0 pin else RTC mode is selected. bit 1: when set '1' enable external GPT trigger 1 on GPIO0 CP1 pin else RTC mode is selected. bit 2: when set '1' enable external GPT trigger 2 on GPIO1 CP0 pin else RTC mode is selected. bit 3: when set '1' enable external GPT trigger 3 on GPIO1 CP1 pin else RTC mode is selected. bit 4: when set '1' enable external GPT trigger 4 on GPIO2 CP0 pin else RTC mode is selected. bit 5: when set '1' enable external GPT trigger 5 on GPIO2 CP1 pin else RTC mode is selected. bit 6: when set '1' enable external GPT trigger 6 on GPIO3 CP0 pin else RTC mode is selected. bit 7: when set '1' enable external GPT trigger 7 on GPIO3 CP1 pin else RTC mode is selected."]
    #[inline(always)]
    pub fn gpt_trig_sel(&self) -> GPT_TRIG_SEL_R {
        GPT_TRIG_SEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This bit is implemented for GPT trigger mode select. GPT IP support 2 modes: RTC mode and external trigger. When this bit is set to logic '1': enable external trigger mode for APPS GPT CP0 and CP1 pin. bit 0: when set '1' enable external GPT trigger 0 on GPIO0 CP0 pin else RTC mode is selected. bit 1: when set '1' enable external GPT trigger 1 on GPIO0 CP1 pin else RTC mode is selected. bit 2: when set '1' enable external GPT trigger 2 on GPIO1 CP0 pin else RTC mode is selected. bit 3: when set '1' enable external GPT trigger 3 on GPIO1 CP1 pin else RTC mode is selected. bit 4: when set '1' enable external GPT trigger 4 on GPIO2 CP0 pin else RTC mode is selected. bit 5: when set '1' enable external GPT trigger 5 on GPIO2 CP1 pin else RTC mode is selected. bit 6: when set '1' enable external GPT trigger 6 on GPIO3 CP0 pin else RTC mode is selected. bit 7: when set '1' enable external GPT trigger 7 on GPIO3 CP1 pin else RTC mode is selected."]
    #[inline(always)]
    pub fn gpt_trig_sel(&mut self) -> GPT_TRIG_SEL_W {
        GPT_TRIG_SEL_W { w: self }
    }
}
