#[doc = "Reader of register WDTLOCK"]
pub type R = crate::R<u32, super::WDTLOCK>;
#[doc = "Writer for register WDTLOCK"]
pub type W = crate::W<u32, super::WDTLOCK>;
#[doc = "Register WDTLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTLOCK`"]
pub type WDTLOCK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WDTLOCK`"]
pub struct WDTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Watchdog Lock"]
    #[inline(always)]
    pub fn wdtlock(&self) -> WDTLOCK_R {
        WDTLOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Lock"]
    #[inline(always)]
    pub fn wdtlock(&mut self) -> WDTLOCK_W {
        WDTLOCK_W { w: self }
    }
}
