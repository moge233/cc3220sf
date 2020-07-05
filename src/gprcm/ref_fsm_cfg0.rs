#[doc = "Reader of register REF_FSM_CFG0"]
pub type R = crate::R<u32, super::REF_FSM_CFG0>;
#[doc = "Writer for register REF_FSM_CFG0"]
pub type W = crate::W<u32, super::REF_FSM_CFG0>;
#[doc = "Register REF_FSM_CFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::REF_FSM_CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BGAP_SETTLING_TIME`"]
pub type BGAP_SETTLING_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BGAP_SETTLING_TIME`"]
pub struct BGAP_SETTLING_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> BGAP_SETTLING_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `FREF_LDO_SETTLING_TIME`"]
pub type FREF_LDO_SETTLING_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FREF_LDO_SETTLING_TIME`"]
pub struct FREF_LDO_SETTLING_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> FREF_LDO_SETTLING_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DIG_BUF_SETTLING_TIME`"]
pub type DIG_BUF_SETTLING_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIG_BUF_SETTLING_TIME`"]
pub struct DIG_BUF_SETTLING_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_BUF_SETTLING_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - ANA-BGAP Settling time (In number of slow_clks)"]
    #[inline(always)]
    pub fn bgap_settling_time(&self) -> BGAP_SETTLING_TIME_R {
        BGAP_SETTLING_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Slicer LDO settling time (In number of slow clks)"]
    #[inline(always)]
    pub fn fref_ldo_settling_time(&self) -> FREF_LDO_SETTLING_TIME_R {
        FREF_LDO_SETTLING_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Dig-buffer settling time (In number of slow clks)"]
    #[inline(always)]
    pub fn dig_buf_settling_time(&self) -> DIG_BUF_SETTLING_TIME_R {
        DIG_BUF_SETTLING_TIME_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - ANA-BGAP Settling time (In number of slow_clks)"]
    #[inline(always)]
    pub fn bgap_settling_time(&mut self) -> BGAP_SETTLING_TIME_W {
        BGAP_SETTLING_TIME_W { w: self }
    }
    #[doc = "Bits 8:15 - Slicer LDO settling time (In number of slow clks)"]
    #[inline(always)]
    pub fn fref_ldo_settling_time(&mut self) -> FREF_LDO_SETTLING_TIME_W {
        FREF_LDO_SETTLING_TIME_W { w: self }
    }
    #[doc = "Bits 0:7 - Dig-buffer settling time (In number of slow clks)"]
    #[inline(always)]
    pub fn dig_buf_settling_time(&mut self) -> DIG_BUF_SETTLING_TIME_W {
        DIG_BUF_SETTLING_TIME_W { w: self }
    }
}
