#[doc = "Reader of register HIB_TIMER_SYNC_CALIB_CFG0"]
pub type R = crate::R<u32, super::HIB_TIMER_SYNC_CALIB_CFG0>;
#[doc = "Writer for register HIB_TIMER_SYNC_CALIB_CFG0"]
pub type W = crate::W<u32, super::HIB_TIMER_SYNC_CALIB_CFG0>;
#[doc = "Register HIB_TIMER_SYNC_CALIB_CFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::HIB_TIMER_SYNC_CALIB_CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_CFG_CALIB_TIME`"]
pub type MEM_CFG_CALIB_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_CFG_CALIB_TIME`"]
pub struct MEM_CFG_CALIB_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CFG_CALIB_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
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
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - MEM_CFG_CALIB_TIME"]
    #[inline(always)]
    pub fn mem_cfg_calib_time(&self) -> MEM_CFG_CALIB_TIME_R {
        MEM_CFG_CALIB_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 1:7 - NU1"]
    #[inline(always)]
    pub fn nu1(&self) -> NU1_R {
        NU1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - MEM_CFG_CALIB_TIME"]
    #[inline(always)]
    pub fn mem_cfg_calib_time(&mut self) -> MEM_CFG_CALIB_TIME_W {
        MEM_CFG_CALIB_TIME_W { w: self }
    }
    #[doc = "Bits 1:7 - NU1"]
    #[inline(always)]
    pub fn nu1(&mut self) -> NU1_W {
        NU1_W { w: self }
    }
}
