#[doc = "Reader of register HIB_TIMER_SYNC_CALIB_CFG1"]
pub type R = crate::R<u32, super::HIB_TIMER_SYNC_CALIB_CFG1>;
#[doc = "Writer for register HIB_TIMER_SYNC_CALIB_CFG1"]
pub type W = crate::W<u32, super::HIB_TIMER_SYNC_CALIB_CFG1>;
#[doc = "Register HIB_TIMER_SYNC_CALIB_CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::HIB_TIMER_SYNC_CALIB_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FAST_CALIB_COUNT`"]
pub type FAST_CALIB_COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FAST_CALIB_COUNT`"]
pub struct FAST_CALIB_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_CALIB_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - FAST_CALIB_COUNT"]
    #[inline(always)]
    pub fn fast_calib_count(&self) -> FAST_CALIB_COUNT_R {
        FAST_CALIB_COUNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - FAST_CALIB_COUNT"]
    #[inline(always)]
    pub fn fast_calib_count(&mut self) -> FAST_CALIB_COUNT_W {
        FAST_CALIB_COUNT_W { w: self }
    }
}
