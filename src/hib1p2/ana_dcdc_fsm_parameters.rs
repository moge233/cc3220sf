#[doc = "Reader of register ANA_DCDC_FSM_PARAMETERS"]
pub type R = crate::R<u32, super::ANA_DCDC_FSM_PARAMETERS>;
#[doc = "Writer for register ANA_DCDC_FSM_PARAMETERS"]
pub type W = crate::W<u32, super::ANA_DCDC_FSM_PARAMETERS>;
#[doc = "Register ANA_DCDC_FSM_PARAMETERS `reset()`'s with value 0"]
impl crate::ResetValue for super::ANA_DCDC_FSM_PARAMETERS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DCDC_ANA_DSLP_EXIT_SLEEP_TO_RUN`"]
pub type MEM_DCDC_ANA_DSLP_EXIT_SLEEP_TO_RUN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_ANA_DSLP_EXIT_SLEEP_TO_RUN`"]
pub struct MEM_DCDC_ANA_DSLP_EXIT_SLEEP_TO_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_ANA_DSLP_EXIT_SLEEP_TO_RUN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - MEM_DCDC_ANA_DSLP_EXIT_SLEEP_TO_RUN"]
    #[inline(always)]
    pub fn mem_dcdc_ana_dslp_exit_sleep_to_run(&self) -> MEM_DCDC_ANA_DSLP_EXIT_SLEEP_TO_RUN_R {
        MEM_DCDC_ANA_DSLP_EXIT_SLEEP_TO_RUN_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MEM_DCDC_ANA_DSLP_EXIT_SLEEP_TO_RUN"]
    #[inline(always)]
    pub fn mem_dcdc_ana_dslp_exit_sleep_to_run(&mut self) -> MEM_DCDC_ANA_DSLP_EXIT_SLEEP_TO_RUN_W {
        MEM_DCDC_ANA_DSLP_EXIT_SLEEP_TO_RUN_W { w: self }
    }
}
