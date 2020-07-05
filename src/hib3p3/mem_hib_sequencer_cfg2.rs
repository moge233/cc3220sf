#[doc = "Reader of register MEM_HIB_SEQUENCER_CFG2"]
pub type R = crate::R<u32, super::MEM_HIB_SEQUENCER_CFG2>;
#[doc = "Writer for register MEM_HIB_SEQUENCER_CFG2"]
pub type W = crate::W<u32, super::MEM_HIB_SEQUENCER_CFG2>;
#[doc = "Register MEM_HIB_SEQUENCER_CFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_HIB_SEQUENCER_CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_ACTIVE_TO_BDC_EV0_TO_ACTIVE_TO_BDC_EV1_TIME`"]
pub type MEM_ACTIVE_TO_BDC_EV0_TO_ACTIVE_TO_BDC_EV1_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_ACTIVE_TO_BDC_EV0_TO_ACTIVE_TO_BDC_EV1_TIME`"]
pub struct MEM_ACTIVE_TO_BDC_EV0_TO_ACTIVE_TO_BDC_EV1_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_ACTIVE_TO_BDC_EV0_TO_ACTIVE_TO_BDC_EV1_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `MEM_BDC_EV4_TO_EV5_TIME`"]
pub type MEM_BDC_EV4_TO_EV5_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_BDC_EV4_TO_EV5_TIME`"]
pub struct MEM_BDC_EV4_TO_EV5_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BDC_EV4_TO_EV5_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `MEM_BDC_EV6_TO_EV7_TIME`"]
pub type MEM_BDC_EV6_TO_EV7_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_BDC_EV6_TO_EV7_TIME`"]
pub struct MEM_BDC_EV6_TO_EV7_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BDC_EV6_TO_EV7_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `MEM_BDC_TO_ACTIVE_EV1_TO_EV2_TIME`"]
pub type MEM_BDC_TO_ACTIVE_EV1_TO_EV2_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_BDC_TO_ACTIVE_EV1_TO_EV2_TIME`"]
pub struct MEM_BDC_TO_ACTIVE_EV1_TO_EV2_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BDC_TO_ACTIVE_EV1_TO_EV2_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `MEM_HIB_TO_ACTIVE_EV2_TO_EV3_TIME`"]
pub type MEM_HIB_TO_ACTIVE_EV2_TO_EV3_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_HIB_TO_ACTIVE_EV2_TO_EV3_TIME`"]
pub struct MEM_HIB_TO_ACTIVE_EV2_TO_EV3_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_HIB_TO_ACTIVE_EV2_TO_EV3_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:10 - Deassertion of EN_COMP_LATCH_3P3 to deassertion of (EN_COMP_3P3, EN_COMP_REF_3P3, EN_ACT_IREF_3P3, EN_CAP_SW_3P3)"]
    #[inline(always)]
    pub fn mem_active_to_bdc_ev0_to_active_to_bdc_ev1_time(
        &self,
    ) -> MEM_ACTIVE_TO_BDC_EV0_TO_ACTIVE_TO_BDC_EV1_TIME_R {
        MEM_ACTIVE_TO_BDC_EV0_TO_ACTIVE_TO_BDC_EV1_TIME_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 6:8 - Assertion of EN_COMP_LATCH_3P3 to deassertion of EN_COMP_LATCH_3P3"]
    #[inline(always)]
    pub fn mem_bdc_ev4_to_ev5_time(&self) -> MEM_BDC_EV4_TO_EV5_TIME_R {
        MEM_BDC_EV4_TO_EV5_TIME_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - Deassertion of (EN_CAP_SW_3P3, EN_COMP_REF_3P3, EN_COMP_3P3, EN_COMP_OUT_LATCH_3P3) to deassertion of EN_BGAP_3P3"]
    #[inline(always)]
    pub fn mem_bdc_ev6_to_ev7_time(&self) -> MEM_BDC_EV6_TO_EV7_TIME_R {
        MEM_BDC_EV6_TO_EV7_TIME_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Assertion of EN_COMP_3P3 to assertion of EN_COMPOUT_LATCH_3P3"]
    #[inline(always)]
    pub fn mem_bdc_to_active_ev1_to_ev2_time(&self) -> MEM_BDC_TO_ACTIVE_EV1_TO_EV2_TIME_R {
        MEM_BDC_TO_ACTIVE_EV1_TO_EV2_TIME_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Assertion of EN_COMP_3P3 to assertion of EN_COMPOUT_LATCH_3P3"]
    #[inline(always)]
    pub fn mem_hib_to_active_ev2_to_ev3_time(&self) -> MEM_HIB_TO_ACTIVE_EV2_TO_EV3_TIME_R {
        MEM_HIB_TO_ACTIVE_EV2_TO_EV3_TIME_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 9:10 - Deassertion of EN_COMP_LATCH_3P3 to deassertion of (EN_COMP_3P3, EN_COMP_REF_3P3, EN_ACT_IREF_3P3, EN_CAP_SW_3P3)"]
    #[inline(always)]
    pub fn mem_active_to_bdc_ev0_to_active_to_bdc_ev1_time(
        &mut self,
    ) -> MEM_ACTIVE_TO_BDC_EV0_TO_ACTIVE_TO_BDC_EV1_TIME_W {
        MEM_ACTIVE_TO_BDC_EV0_TO_ACTIVE_TO_BDC_EV1_TIME_W { w: self }
    }
    #[doc = "Bits 6:8 - Assertion of EN_COMP_LATCH_3P3 to deassertion of EN_COMP_LATCH_3P3"]
    #[inline(always)]
    pub fn mem_bdc_ev4_to_ev5_time(&mut self) -> MEM_BDC_EV4_TO_EV5_TIME_W {
        MEM_BDC_EV4_TO_EV5_TIME_W { w: self }
    }
    #[doc = "Bits 4:5 - Deassertion of (EN_CAP_SW_3P3, EN_COMP_REF_3P3, EN_COMP_3P3, EN_COMP_OUT_LATCH_3P3) to deassertion of EN_BGAP_3P3"]
    #[inline(always)]
    pub fn mem_bdc_ev6_to_ev7_time(&mut self) -> MEM_BDC_EV6_TO_EV7_TIME_W {
        MEM_BDC_EV6_TO_EV7_TIME_W { w: self }
    }
    #[doc = "Bits 2:3 - Assertion of EN_COMP_3P3 to assertion of EN_COMPOUT_LATCH_3P3"]
    #[inline(always)]
    pub fn mem_bdc_to_active_ev1_to_ev2_time(&mut self) -> MEM_BDC_TO_ACTIVE_EV1_TO_EV2_TIME_W {
        MEM_BDC_TO_ACTIVE_EV1_TO_EV2_TIME_W { w: self }
    }
    #[doc = "Bits 0:1 - Assertion of EN_COMP_3P3 to assertion of EN_COMPOUT_LATCH_3P3"]
    #[inline(always)]
    pub fn mem_hib_to_active_ev2_to_ev3_time(&mut self) -> MEM_HIB_TO_ACTIVE_EV2_TO_EV3_TIME_W {
        MEM_HIB_TO_ACTIVE_EV2_TO_EV3_TIME_W { w: self }
    }
}
