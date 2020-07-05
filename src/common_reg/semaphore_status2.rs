#[doc = "Reader of register SEMAPHORE_STATUS2"]
pub type R = crate::R<u32, super::SEMAPHORE_STATUS2>;
#[doc = "Writer for register SEMAPHORE_STATUS2"]
pub type W = crate::W<u32, super::SEMAPHORE_STATUS2>;
#[doc = "Register SEMAPHORE_STATUS2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEMAPHORE_STATUS2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEMPAPHORE_STATUS2`"]
pub type SEMPAPHORE_STATUS2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEMPAPHORE_STATUS2`"]
pub struct SEMPAPHORE_STATUS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEMPAPHORE_STATUS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - SEMAPHORE STATUS 23:22 :semaphore status of apps_nw_semaphore11 21:20 :semaphore status of apps_nw_semaphore11 19:18 :semaphore status of apps_nw_semaphore10 17:16 :semaphore status of apps_nw_semaphore9 15:14 :semaphore status of apps_nw_semaphore8 13:12 :semaphore status of apps_nw_semaphore7 11:10 :semaphore status of apps_nw_semaphore6 9:8 :semaphore status of apps_nw_semaphore5 7:6 :semaphore status of apps_nw_semaphore4 5:4 :semaphore status of apps_nw_semaphore3 3:2 :semaphore status of apps_nw_semaphore2 1:0 :semaphore status of apps_nw_semaphore1"]
    #[inline(always)]
    pub fn sempaphore_status2(&self) -> SEMPAPHORE_STATUS2_R {
        SEMPAPHORE_STATUS2_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - SEMAPHORE STATUS 23:22 :semaphore status of apps_nw_semaphore11 21:20 :semaphore status of apps_nw_semaphore11 19:18 :semaphore status of apps_nw_semaphore10 17:16 :semaphore status of apps_nw_semaphore9 15:14 :semaphore status of apps_nw_semaphore8 13:12 :semaphore status of apps_nw_semaphore7 11:10 :semaphore status of apps_nw_semaphore6 9:8 :semaphore status of apps_nw_semaphore5 7:6 :semaphore status of apps_nw_semaphore4 5:4 :semaphore status of apps_nw_semaphore3 3:2 :semaphore status of apps_nw_semaphore2 1:0 :semaphore status of apps_nw_semaphore1"]
    #[inline(always)]
    pub fn sempaphore_status2(&mut self) -> SEMPAPHORE_STATUS2_W {
        SEMPAPHORE_STATUS2_W { w: self }
    }
}
