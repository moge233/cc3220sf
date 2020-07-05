#[doc = "Reader of register HIB_TIMER_SYNC_WAKE_OFFSET_ERR"]
pub type R = crate::R<u32, super::HIB_TIMER_SYNC_WAKE_OFFSET_ERR>;
#[doc = "Writer for register HIB_TIMER_SYNC_WAKE_OFFSET_ERR"]
pub type W = crate::W<u32, super::HIB_TIMER_SYNC_WAKE_OFFSET_ERR>;
#[doc = "Register HIB_TIMER_SYNC_WAKE_OFFSET_ERR `reset()`'s with value 0"]
impl crate::ResetValue for super::HIB_TIMER_SYNC_WAKE_OFFSET_ERR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WUP_OFFSET_ERROR`"]
pub type WUP_OFFSET_ERROR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WUP_OFFSET_ERROR`"]
pub struct WUP_OFFSET_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP_OFFSET_ERROR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - WUP_OFFSET_ERROR"]
    #[inline(always)]
    pub fn wup_offset_error(&self) -> WUP_OFFSET_ERROR_R {
        WUP_OFFSET_ERROR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - WUP_OFFSET_ERROR"]
    #[inline(always)]
    pub fn wup_offset_error(&mut self) -> WUP_OFFSET_ERROR_W {
        WUP_OFFSET_ERROR_W { w: self }
    }
}
