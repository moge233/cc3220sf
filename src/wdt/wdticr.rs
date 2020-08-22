#[doc = "Reader of register WDTICR"]
pub type R = crate::R<u32, super::WDTICR>;
#[doc = "Writer for register WDTICR"]
pub type W = crate::W<u32, super::WDTICR>;
#[doc = "Register WDTICR `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTINTCLR`"]
pub type WDTINTCLR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WDTINTCLR`"]
pub struct WDTINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTINTCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Watchdog Interrupt Clear"]
    #[inline(always)]
    pub fn wdtintclr(&self) -> WDTINTCLR_R {
        WDTINTCLR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Interrupt Clear"]
    #[inline(always)]
    pub fn wdtintclr(&mut self) -> WDTINTCLR_W {
        WDTINTCLR_W { w: self }
    }
}
