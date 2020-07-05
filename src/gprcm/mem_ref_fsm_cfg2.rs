#[doc = "Reader of register MEM_REF_FSM_CFG2"]
pub type R = crate::R<u32, super::MEM_REF_FSM_CFG2>;
#[doc = "Writer for register MEM_REF_FSM_CFG2"]
pub type W = crate::W<u32, super::MEM_REF_FSM_CFG2>;
#[doc = "Register MEM_REF_FSM_CFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_REF_FSM_CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_FC_DEASSERT_DELAY`"]
pub type MEM_FC_DEASSERT_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_FC_DEASSERT_DELAY`"]
pub struct MEM_FC_DEASSERT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FC_DEASSERT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Reader of field `MEM_STARTUP_DEASSERT_DELAY`"]
pub type MEM_STARTUP_DEASSERT_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_STARTUP_DEASSERT_DELAY`"]
pub struct MEM_STARTUP_DEASSERT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_STARTUP_DEASSERT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEM_EXT_TCXO_SETTLING_TIME`"]
pub type MEM_EXT_TCXO_SETTLING_TIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_EXT_TCXO_SETTLING_TIME`"]
pub struct MEM_EXT_TCXO_SETTLING_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_EXT_TCXO_SETTLING_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 19:21 - Number of RTC clocks for keeping the FC_EN asserted high"]
    #[inline(always)]
    pub fn mem_fc_deassert_delay(&self) -> MEM_FC_DEASSERT_DELAY_R {
        MEM_FC_DEASSERT_DELAY_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Number of RTC clocks for keeping the STARTUP_EN asserted high"]
    #[inline(always)]
    pub fn mem_startup_deassert_delay(&self) -> MEM_STARTUP_DEASSERT_DELAY_R {
        MEM_STARTUP_DEASSERT_DELAY_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 0:15 - Number of RTC clocks for waiting for clock to settle."]
    #[inline(always)]
    pub fn mem_ext_tcxo_settling_time(&self) -> MEM_EXT_TCXO_SETTLING_TIME_R {
        MEM_EXT_TCXO_SETTLING_TIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 19:21 - Number of RTC clocks for keeping the FC_EN asserted high"]
    #[inline(always)]
    pub fn mem_fc_deassert_delay(&mut self) -> MEM_FC_DEASSERT_DELAY_W {
        MEM_FC_DEASSERT_DELAY_W { w: self }
    }
    #[doc = "Bits 16:18 - Number of RTC clocks for keeping the STARTUP_EN asserted high"]
    #[inline(always)]
    pub fn mem_startup_deassert_delay(&mut self) -> MEM_STARTUP_DEASSERT_DELAY_W {
        MEM_STARTUP_DEASSERT_DELAY_W { w: self }
    }
    #[doc = "Bits 0:15 - Number of RTC clocks for waiting for clock to settle."]
    #[inline(always)]
    pub fn mem_ext_tcxo_settling_time(&mut self) -> MEM_EXT_TCXO_SETTLING_TIME_W {
        MEM_EXT_TCXO_SETTLING_TIME_W { w: self }
    }
}
