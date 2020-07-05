#[doc = "Reader of register DSLPTIMRCFG"]
pub type R = crate::R<u32, super::DSLPTIMRCFG>;
#[doc = "Writer for register DSLPTIMRCFG"]
pub type W = crate::W<u32, super::DSLPTIMRCFG>;
#[doc = "Register DSLPTIMRCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DSLPTIMRCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSLP_WAKE_TIMER_OPP_CFG`"]
pub type DSLP_WAKE_TIMER_OPP_CFG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DSLP_WAKE_TIMER_OPP_CFG`"]
pub struct DSLP_WAKE_TIMER_OPP_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DSLP_WAKE_TIMER_OPP_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DSLP_WAKE_TIMER_WAKE_CFG`"]
pub type DSLP_WAKE_TIMER_WAKE_CFG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DSLP_WAKE_TIMER_WAKE_CFG`"]
pub struct DSLP_WAKE_TIMER_WAKE_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DSLP_WAKE_TIMER_WAKE_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Configuration (in slow_clks) which says when to request for OPP during deep-sleep exit"]
    #[inline(always)]
    pub fn dslp_wake_timer_opp_cfg(&self) -> DSLP_WAKE_TIMER_OPP_CFG_R {
        DSLP_WAKE_TIMER_OPP_CFG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Configuration (in slow_clks) which says when to request for WAKE during deep-sleep exit"]
    #[inline(always)]
    pub fn dslp_wake_timer_wake_cfg(&self) -> DSLP_WAKE_TIMER_WAKE_CFG_R {
        DSLP_WAKE_TIMER_WAKE_CFG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Configuration (in slow_clks) which says when to request for OPP during deep-sleep exit"]
    #[inline(always)]
    pub fn dslp_wake_timer_opp_cfg(&mut self) -> DSLP_WAKE_TIMER_OPP_CFG_W {
        DSLP_WAKE_TIMER_OPP_CFG_W { w: self }
    }
    #[doc = "Bits 0:15 - Configuration (in slow_clks) which says when to request for WAKE during deep-sleep exit"]
    #[inline(always)]
    pub fn dslp_wake_timer_wake_cfg(&mut self) -> DSLP_WAKE_TIMER_WAKE_CFG_W {
        DSLP_WAKE_TIMER_WAKE_CFG_W { w: self }
    }
}
