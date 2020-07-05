#[doc = "Reader of register MEM_HIB_SEQUENCER_CFG1"]
pub type R = crate::R<u32, super::MEM_HIB_SEQUENCER_CFG1>;
#[doc = "Writer for register MEM_HIB_SEQUENCER_CFG1"]
pub type W = crate::W<u32, super::MEM_HIB_SEQUENCER_CFG1>;
#[doc = "Register MEM_HIB_SEQUENCER_CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_HIB_SEQUENCER_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_BDC_EV5_TO_EV6_TIME`"]
pub type MEM_BDC_EV5_TO_EV6_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_BDC_EV5_TO_EV6_TIME`"]
pub struct MEM_BDC_EV5_TO_EV6_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BDC_EV5_TO_EV6_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `MEM_BDC_TO_ACTIVE_EV0_TO_EV1_TIME`"]
pub type MEM_BDC_TO_ACTIVE_EV0_TO_EV1_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_BDC_TO_ACTIVE_EV0_TO_EV1_TIME`"]
pub struct MEM_BDC_TO_ACTIVE_EV0_TO_EV1_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BDC_TO_ACTIVE_EV0_TO_EV1_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `MEM_BDC_TO_ACTIVE_EV0_TO_ACTIVE`"]
pub type MEM_BDC_TO_ACTIVE_EV0_TO_ACTIVE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_BDC_TO_ACTIVE_EV0_TO_ACTIVE`"]
pub struct MEM_BDC_TO_ACTIVE_EV0_TO_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BDC_TO_ACTIVE_EV0_TO_ACTIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `MEM_ACTIVE_TO_BDC_EV1_TO_BDC_EV0_TIME`"]
pub type MEM_ACTIVE_TO_BDC_EV1_TO_BDC_EV0_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_ACTIVE_TO_BDC_EV1_TO_BDC_EV0_TIME`"]
pub struct MEM_ACTIVE_TO_BDC_EV1_TO_BDC_EV0_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_ACTIVE_TO_BDC_EV1_TO_BDC_EV0_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `NU1`"]
pub type NU1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NU1`"]
pub struct NU1_W<'a> {
    w: &'a mut W,
}
impl<'a> NU1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:15 - Configuration for number of slow-clks between de-assertion of EN_COMP_LATCH and assertion of"]
    #[inline(always)]
    pub fn mem_bdc_ev5_to_ev6_time(&self) -> MEM_BDC_EV5_TO_EV6_TIME_R {
        MEM_BDC_EV5_TO_EV6_TIME_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Configuration for number of slow-clks between assertion of EN_COMP_REF to assertion of EN_COMP during HIB-Exit"]
    #[inline(always)]
    pub fn mem_bdc_to_active_ev1_to_ev2_time(&self) -> MEM_BDC_TO_ACTIVE_EV1_TO_EV2_TIME_R {
        MEM_BDC_TO_ACTIVE_EV1_TO_EV2_TIME_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - TBD"]
    #[inline(always)]
    pub fn mem_bdc_to_active_ev0_to_ev1_time(&self) -> MEM_BDC_TO_ACTIVE_EV0_TO_EV1_TIME_R {
        MEM_BDC_TO_ACTIVE_EV0_TO_EV1_TIME_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Configuration in number of slow-clks between assertion of (EN_BGAP_3P3V, EN_CAP_SW_3P3V, EN_ACT_IREF_3P3V, EN_COMP_REF) to assertion of EN_COMP_3P3V"]
    #[inline(always)]
    pub fn mem_bdc_to_active_ev0_to_active(&self) -> MEM_BDC_TO_ACTIVE_EV0_TO_ACTIVE_R {
        MEM_BDC_TO_ACTIVE_EV0_TO_ACTIVE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Configuration in number of slow-clks between de-assertion of (EN_COMP_3P3V, EN_COMP_REF_3P3V, EN_ACT_IREF_3P3V, EN_CAP_SW_3P3V) to deassertion of EN_BGAP_3P3V."]
    #[inline(always)]
    pub fn mem_active_to_bdc_ev1_to_bdc_ev0_time(&self) -> MEM_ACTIVE_TO_BDC_EV1_TO_BDC_EV0_TIME_R {
        MEM_ACTIVE_TO_BDC_EV1_TO_BDC_EV0_TIME_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - NU1"]
    #[inline(always)]
    pub fn nu1(&self) -> NU1_R {
        NU1_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15 - Configuration for number of slow-clks between de-assertion of EN_COMP_LATCH and assertion of"]
    #[inline(always)]
    pub fn mem_bdc_ev5_to_ev6_time(&mut self) -> MEM_BDC_EV5_TO_EV6_TIME_W {
        MEM_BDC_EV5_TO_EV6_TIME_W { w: self }
    }
    #[doc = "Bits 12:13 - Configuration for number of slow-clks between assertion of EN_COMP_REF to assertion of EN_COMP during HIB-Exit"]
    #[inline(always)]
    pub fn mem_bdc_to_active_ev1_to_ev2_time(&mut self) -> MEM_BDC_TO_ACTIVE_EV1_TO_EV2_TIME_W {
        MEM_BDC_TO_ACTIVE_EV1_TO_EV2_TIME_W { w: self }
    }
    #[doc = "Bits 10:11 - TBD"]
    #[inline(always)]
    pub fn mem_bdc_to_active_ev0_to_ev1_time(&mut self) -> MEM_BDC_TO_ACTIVE_EV0_TO_EV1_TIME_W {
        MEM_BDC_TO_ACTIVE_EV0_TO_EV1_TIME_W { w: self }
    }
    #[doc = "Bits 8:9 - Configuration in number of slow-clks between assertion of (EN_BGAP_3P3V, EN_CAP_SW_3P3V, EN_ACT_IREF_3P3V, EN_COMP_REF) to assertion of EN_COMP_3P3V"]
    #[inline(always)]
    pub fn mem_bdc_to_active_ev0_to_active(&mut self) -> MEM_BDC_TO_ACTIVE_EV0_TO_ACTIVE_W {
        MEM_BDC_TO_ACTIVE_EV0_TO_ACTIVE_W { w: self }
    }
    #[doc = "Bits 6:7 - Configuration in number of slow-clks between de-assertion of (EN_COMP_3P3V, EN_COMP_REF_3P3V, EN_ACT_IREF_3P3V, EN_CAP_SW_3P3V) to deassertion of EN_BGAP_3P3V."]
    #[inline(always)]
    pub fn mem_active_to_bdc_ev1_to_bdc_ev0_time(
        &mut self,
    ) -> MEM_ACTIVE_TO_BDC_EV1_TO_BDC_EV0_TIME_W {
        MEM_ACTIVE_TO_BDC_EV1_TO_BDC_EV0_TIME_W { w: self }
    }
    #[doc = "Bits 0:5 - NU1"]
    #[inline(always)]
    pub fn nu1(&mut self) -> NU1_W {
        NU1_W { w: self }
    }
}
