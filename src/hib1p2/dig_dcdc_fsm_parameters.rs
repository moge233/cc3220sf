#[doc = "Reader of register DIG_DCDC_FSM_PARAMETERS"]
pub type R = crate::R<u32, super::DIG_DCDC_FSM_PARAMETERS>;
#[doc = "Writer for register DIG_DCDC_FSM_PARAMETERS"]
pub type W = crate::W<u32, super::DIG_DCDC_FSM_PARAMETERS>;
#[doc = "Register DIG_DCDC_FSM_PARAMETERS `reset()`'s with value 0"]
impl crate::ResetValue for super::DIG_DCDC_FSM_PARAMETERS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_DSLP_ENTER_COT_TO_VTRIM`"]
pub type MEM_DCDC_DIG_DSLP_ENTER_COT_TO_VTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_DSLP_ENTER_COT_TO_VTRIM`"]
pub struct MEM_DCDC_DIG_DSLP_ENTER_COT_TO_VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_DSLP_ENTER_COT_TO_VTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_DSLP_ENTER_VTRIM_TO_SLEEP`"]
pub type MEM_DCDC_DIG_DSLP_ENTER_VTRIM_TO_SLEEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_DSLP_ENTER_VTRIM_TO_SLEEP`"]
pub struct MEM_DCDC_DIG_DSLP_ENTER_VTRIM_TO_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_DSLP_ENTER_VTRIM_TO_SLEEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_DSLP_EXIT_SLEEP_TO_VTRIM`"]
pub type MEM_DCDC_DIG_DSLP_EXIT_SLEEP_TO_VTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_DSLP_EXIT_SLEEP_TO_VTRIM`"]
pub struct MEM_DCDC_DIG_DSLP_EXIT_SLEEP_TO_VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_DSLP_EXIT_SLEEP_TO_VTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_DSLP_EXIT_VTRIM_TO_COT`"]
pub type MEM_DCDC_DIG_DSLP_EXIT_VTRIM_TO_COT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_DSLP_EXIT_VTRIM_TO_COT`"]
pub struct MEM_DCDC_DIG_DSLP_EXIT_VTRIM_TO_COT_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_DSLP_EXIT_VTRIM_TO_COT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_DSLP_EXIT_COT_TO_RUN`"]
pub type MEM_DCDC_DIG_DSLP_EXIT_COT_TO_RUN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_DSLP_EXIT_COT_TO_RUN`"]
pub struct MEM_DCDC_DIG_DSLP_EXIT_COT_TO_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_DSLP_EXIT_COT_TO_RUN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:14 - MEM_DCDC_DIG_DSLP_ENTER_COT_TO_VTRIM"]
    #[inline(always)]
    pub fn mem_dcdc_dig_dslp_enter_cot_to_vtrim(&self) -> MEM_DCDC_DIG_DSLP_ENTER_COT_TO_VTRIM_R {
        MEM_DCDC_DIG_DSLP_ENTER_COT_TO_VTRIM_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - MEM_DCDC_DIG_DSLP_ENTER_VTRIM_TO_SLEEP"]
    #[inline(always)]
    pub fn mem_dcdc_dig_dslp_enter_vtrim_to_sleep(
        &self,
    ) -> MEM_DCDC_DIG_DSLP_ENTER_VTRIM_TO_SLEEP_R {
        MEM_DCDC_DIG_DSLP_ENTER_VTRIM_TO_SLEEP_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - MEM_DCDC_DIG_DSLP_EXIT_SLEEP_TO_VTRIM"]
    #[inline(always)]
    pub fn mem_dcdc_dig_dslp_exit_sleep_to_vtrim(&self) -> MEM_DCDC_DIG_DSLP_EXIT_SLEEP_TO_VTRIM_R {
        MEM_DCDC_DIG_DSLP_EXIT_SLEEP_TO_VTRIM_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - MEM_DCDC_DIG_DSLP_EXIT_VTRIM_TO_COT"]
    #[inline(always)]
    pub fn mem_dcdc_dig_dslp_exit_vtrim_to_cot(&self) -> MEM_DCDC_DIG_DSLP_EXIT_VTRIM_TO_COT_R {
        MEM_DCDC_DIG_DSLP_EXIT_VTRIM_TO_COT_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - MEM_DCDC_DIG_DSLP_EXIT_COT_TO_RUN"]
    #[inline(always)]
    pub fn mem_dcdc_dig_dslp_exit_cot_to_run(&self) -> MEM_DCDC_DIG_DSLP_EXIT_COT_TO_RUN_R {
        MEM_DCDC_DIG_DSLP_EXIT_COT_TO_RUN_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14 - MEM_DCDC_DIG_DSLP_ENTER_COT_TO_VTRIM"]
    #[inline(always)]
    pub fn mem_dcdc_dig_dslp_enter_cot_to_vtrim(
        &mut self,
    ) -> MEM_DCDC_DIG_DSLP_ENTER_COT_TO_VTRIM_W {
        MEM_DCDC_DIG_DSLP_ENTER_COT_TO_VTRIM_W { w: self }
    }
    #[doc = "Bits 9:11 - MEM_DCDC_DIG_DSLP_ENTER_VTRIM_TO_SLEEP"]
    #[inline(always)]
    pub fn mem_dcdc_dig_dslp_enter_vtrim_to_sleep(
        &mut self,
    ) -> MEM_DCDC_DIG_DSLP_ENTER_VTRIM_TO_SLEEP_W {
        MEM_DCDC_DIG_DSLP_ENTER_VTRIM_TO_SLEEP_W { w: self }
    }
    #[doc = "Bits 6:8 - MEM_DCDC_DIG_DSLP_EXIT_SLEEP_TO_VTRIM"]
    #[inline(always)]
    pub fn mem_dcdc_dig_dslp_exit_sleep_to_vtrim(
        &mut self,
    ) -> MEM_DCDC_DIG_DSLP_EXIT_SLEEP_TO_VTRIM_W {
        MEM_DCDC_DIG_DSLP_EXIT_SLEEP_TO_VTRIM_W { w: self }
    }
    #[doc = "Bits 3:5 - MEM_DCDC_DIG_DSLP_EXIT_VTRIM_TO_COT"]
    #[inline(always)]
    pub fn mem_dcdc_dig_dslp_exit_vtrim_to_cot(&mut self) -> MEM_DCDC_DIG_DSLP_EXIT_VTRIM_TO_COT_W {
        MEM_DCDC_DIG_DSLP_EXIT_VTRIM_TO_COT_W { w: self }
    }
    #[doc = "Bits 0:2 - MEM_DCDC_DIG_DSLP_EXIT_COT_TO_RUN"]
    #[inline(always)]
    pub fn mem_dcdc_dig_dslp_exit_cot_to_run(&mut self) -> MEM_DCDC_DIG_DSLP_EXIT_COT_TO_RUN_W {
        MEM_DCDC_DIG_DSLP_EXIT_COT_TO_RUN_W { w: self }
    }
}
