#[doc = "Reader of register REF_FSM_CFG1"]
pub type R = crate::R<u32, super::REF_FSM_CFG1>;
#[doc = "Writer for register REF_FSM_CFG1"]
pub type W = crate::W<u32, super::REF_FSM_CFG1>;
#[doc = "Register REF_FSM_CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::REF_FSM_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XTAL_SETTLING_TIME`"]
pub type XTAL_SETTLING_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTAL_SETTLING_TIME`"]
pub struct XTAL_SETTLING_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_SETTLING_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `SLICER_LV_SETTLING_TIME`"]
pub type SLICER_LV_SETTLING_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLICER_LV_SETTLING_TIME`"]
pub struct SLICER_LV_SETTLING_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SLICER_LV_SETTLING_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLICER_HV_PD_SETTLING_TIME`"]
pub type SLICER_HV_PD_SETTLING_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLICER_HV_PD_SETTLING_TIME`"]
pub struct SLICER_HV_PD_SETTLING_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SLICER_HV_PD_SETTLING_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SLICER_HV_SETTLING_TIME`"]
pub type SLICER_HV_SETTLING_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLICER_HV_SETTLING_TIME`"]
pub struct SLICER_HV_SETTLING_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SLICER_HV_SETTLING_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - XTAL settling time (In number of slow clks)"]
    #[inline(always)]
    pub fn xtal_settling_time(&self) -> XTAL_SETTLING_TIME_R {
        XTAL_SETTLING_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LV Slicer settling time"]
    #[inline(always)]
    pub fn slicer_lv_settling_time(&self) -> SLICER_LV_SETTLING_TIME_R {
        SLICER_LV_SETTLING_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - HV Slicer Pull-down settling time"]
    #[inline(always)]
    pub fn slicer_hv_pd_settling_time(&self) -> SLICER_HV_PD_SETTLING_TIME_R {
        SLICER_HV_PD_SETTLING_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - HV Slicer settling time"]
    #[inline(always)]
    pub fn slicer_hv_settling_time(&self) -> SLICER_HV_SETTLING_TIME_R {
        SLICER_HV_SETTLING_TIME_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - XTAL settling time (In number of slow clks)"]
    #[inline(always)]
    pub fn xtal_settling_time(&mut self) -> XTAL_SETTLING_TIME_W {
        XTAL_SETTLING_TIME_W { w: self }
    }
    #[doc = "Bits 16:23 - LV Slicer settling time"]
    #[inline(always)]
    pub fn slicer_lv_settling_time(&mut self) -> SLICER_LV_SETTLING_TIME_W {
        SLICER_LV_SETTLING_TIME_W { w: self }
    }
    #[doc = "Bits 8:15 - HV Slicer Pull-down settling time"]
    #[inline(always)]
    pub fn slicer_hv_pd_settling_time(&mut self) -> SLICER_HV_PD_SETTLING_TIME_W {
        SLICER_HV_PD_SETTLING_TIME_W { w: self }
    }
    #[doc = "Bits 0:7 - HV Slicer settling time"]
    #[inline(always)]
    pub fn slicer_hv_settling_time(&mut self) -> SLICER_HV_SETTLING_TIME_W {
        SLICER_HV_SETTLING_TIME_W { w: self }
    }
}
