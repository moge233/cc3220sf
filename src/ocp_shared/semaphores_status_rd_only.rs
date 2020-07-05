#[doc = "Reader of register SEMAPHORES_STATUS_RD_ONLY"]
pub type R = crate::R<u32, super::SEMAPHORES_STATUS_RD_ONLY>;
#[doc = "Writer for register SEMAPHORES_STATUS_RD_ONLY"]
pub type W = crate::W<u32, super::SEMAPHORES_STATUS_RD_ONLY>;
#[doc = "Register SEMAPHORES_STATUS_RD_ONLY `reset()`'s with value 0"]
impl crate::ResetValue for super::SEMAPHORES_STATUS_RD_ONLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEMAPHORES_STATUS`"]
pub type SEMAPHORES_STATUS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SEMAPHORES_STATUS`"]
pub struct SEMAPHORES_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SEMAPHORES_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Captured/released semaphores status for the 12 semaphores. Each bit of the 12 bits represents a semaphore. 0 => Semaphore Free. 1 => Semaphore Captured."]
    #[inline(always)]
    pub fn semaphores_status(&self) -> SEMAPHORES_STATUS_R {
        SEMAPHORES_STATUS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Captured/released semaphores status for the 12 semaphores. Each bit of the 12 bits represents a semaphore. 0 => Semaphore Free. 1 => Semaphore Captured."]
    #[inline(always)]
    pub fn semaphores_status(&mut self) -> SEMAPHORES_STATUS_W {
        SEMAPHORES_STATUS_W { w: self }
    }
}
