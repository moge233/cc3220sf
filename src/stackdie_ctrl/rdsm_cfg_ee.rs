#[doc = "Reader of register RDSM_CFG_EE"]
pub type R = crate::R<u32, super::RDSM_CFG_EE>;
#[doc = "Writer for register RDSM_CFG_EE"]
pub type W = crate::W<u32, super::RDSM_CFG_EE>;
#[doc = "Register RDSM_CFG_EE `reset()`'s with value 0"]
impl crate::ResetValue for super::RDSM_CFG_EE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLCLK_PULSE_WIDTH`"]
pub type FLCLK_PULSE_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLCLK_PULSE_WIDTH`"]
pub struct FLCLK_PULSE_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLCLK_PULSE_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `READ_WAIT_STATE`"]
pub type READ_WAIT_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `READ_WAIT_STATE`"]
pub struct READ_WAIT_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_WAIT_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Bank Clock Hi Time 00 : HCLK pulse 01 : 1 cycle of HCLK 10 : 1.5 cycles of HCLK 11 : 2 cycles of HCLK"]
    #[inline(always)]
    pub fn flclk_pulse_width(&self) -> FLCLK_PULSE_WIDTH_R {
        FLCLK_PULSE_WIDTH_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - Number of wait states inserted"]
    #[inline(always)]
    pub fn read_wait_state(&self) -> READ_WAIT_STATE_R {
        READ_WAIT_STATE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Bank Clock Hi Time 00 : HCLK pulse 01 : 1 cycle of HCLK 10 : 1.5 cycles of HCLK 11 : 2 cycles of HCLK"]
    #[inline(always)]
    pub fn flclk_pulse_width(&mut self) -> FLCLK_PULSE_WIDTH_W {
        FLCLK_PULSE_WIDTH_W { w: self }
    }
    #[doc = "Bits 0:3 - Number of wait states inserted"]
    #[inline(always)]
    pub fn read_wait_state(&mut self) -> READ_WAIT_STATE_W {
        READ_WAIT_STATE_W { w: self }
    }
}
