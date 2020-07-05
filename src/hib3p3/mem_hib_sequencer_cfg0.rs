#[doc = "Reader of register MEM_HIB_SEQUENCER_CFG0"]
pub type R = crate::R<u32, super::MEM_HIB_SEQUENCER_CFG0>;
#[doc = "Writer for register MEM_HIB_SEQUENCER_CFG0"]
pub type W = crate::W<u32, super::MEM_HIB_SEQUENCER_CFG0>;
#[doc = "Register MEM_HIB_SEQUENCER_CFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_HIB_SEQUENCER_CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_BDC_EV0_TO_EV1_TIME`"]
pub type MEM_BDC_EV0_TO_EV1_TIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_BDC_EV0_TO_EV1_TIME`"]
pub struct MEM_BDC_EV0_TO_EV1_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BDC_EV0_TO_EV1_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEM_BDC_EV3_TO_EV4_TIME`"]
pub type MEM_BDC_EV3_TO_EV4_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_BDC_EV3_TO_EV4_TIME`"]
pub struct MEM_BDC_EV3_TO_EV4_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BDC_EV3_TO_EV4_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `MEM_BDC_EV2_TO_EV3_TIME`"]
pub type MEM_BDC_EV2_TO_EV3_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_BDC_EV2_TO_EV3_TIME`"]
pub struct MEM_BDC_EV2_TO_EV3_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BDC_EV2_TO_EV3_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `MEM_BDC_EV1_TO_EV2_TIME`"]
pub type MEM_BDC_EV1_TO_EV2_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_BDC_EV1_TO_EV2_TIME`"]
pub struct MEM_BDC_EV1_TO_EV2_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BDC_EV1_TO_EV2_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Configuration for the number of slow-clks between de-assertion of EN_BG_3P3V to assertion of EN_BG_3P3V"]
    #[inline(always)]
    pub fn mem_bdc_ev0_to_ev1_time(&self) -> MEM_BDC_EV0_TO_EV1_TIME_R {
        MEM_BDC_EV0_TO_EV1_TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 13:14 - Configuration for the number of slow-clks between assertion of EN_COMP_3P3V and assertion of EN_COMP_LATCH_3P3V"]
    #[inline(always)]
    pub fn mem_bdc_ev3_to_ev4_time(&self) -> MEM_BDC_EV3_TO_EV4_TIME_R {
        MEM_BDC_EV3_TO_EV4_TIME_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - Configuration for the number of slow-clks between assertion of (EN_CAP_SW_3P3V,EN_COMP_REF) and assertion of (EN_COMP_3P3V)"]
    #[inline(always)]
    pub fn mem_bdc_ev2_to_ev3_time(&self) -> MEM_BDC_EV2_TO_EV3_TIME_R {
        MEM_BDC_EV2_TO_EV3_TIME_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 9:10 - Configuration for the number of slow-clks between assertion of (EN_BG_3P3V) and assertion of (EN_CAP_SW_3P3V, EN_COMP_REF_3P3V)"]
    #[inline(always)]
    pub fn mem_bdc_ev1_to_ev2_time(&self) -> MEM_BDC_EV1_TO_EV2_TIME_R {
        MEM_BDC_EV1_TO_EV2_TIME_R::new(((self.bits >> 9) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - Configuration for the number of slow-clks between de-assertion of EN_BG_3P3V to assertion of EN_BG_3P3V"]
    #[inline(always)]
    pub fn mem_bdc_ev0_to_ev1_time(&mut self) -> MEM_BDC_EV0_TO_EV1_TIME_W {
        MEM_BDC_EV0_TO_EV1_TIME_W { w: self }
    }
    #[doc = "Bits 13:14 - Configuration for the number of slow-clks between assertion of EN_COMP_3P3V and assertion of EN_COMP_LATCH_3P3V"]
    #[inline(always)]
    pub fn mem_bdc_ev3_to_ev4_time(&mut self) -> MEM_BDC_EV3_TO_EV4_TIME_W {
        MEM_BDC_EV3_TO_EV4_TIME_W { w: self }
    }
    #[doc = "Bits 11:12 - Configuration for the number of slow-clks between assertion of (EN_CAP_SW_3P3V,EN_COMP_REF) and assertion of (EN_COMP_3P3V)"]
    #[inline(always)]
    pub fn mem_bdc_ev2_to_ev3_time(&mut self) -> MEM_BDC_EV2_TO_EV3_TIME_W {
        MEM_BDC_EV2_TO_EV3_TIME_W { w: self }
    }
    #[doc = "Bits 9:10 - Configuration for the number of slow-clks between assertion of (EN_BG_3P3V) and assertion of (EN_CAP_SW_3P3V, EN_COMP_REF_3P3V)"]
    #[inline(always)]
    pub fn mem_bdc_ev1_to_ev2_time(&mut self) -> MEM_BDC_EV1_TO_EV2_TIME_W {
        MEM_BDC_EV1_TO_EV2_TIME_W { w: self }
    }
}
