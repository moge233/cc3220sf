#[doc = "Reader of register WDTLOAD"]
pub type R = crate::R<u32, super::WDTLOAD>;
#[doc = "Writer for register WDTLOAD"]
pub type W = crate::W<u32, super::WDTLOAD>;
#[doc = "Register WDTLOAD `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTLOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTLOAD`"]
pub type WDTLOAD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WDTLOAD`"]
pub struct WDTLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTLOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Watchdog Load Value"]
    #[inline(always)]
    pub fn wdtload(&self) -> WDTLOAD_R {
        WDTLOAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Load Value"]
    #[inline(always)]
    pub fn wdtload(&mut self) -> WDTLOAD_W {
        WDTLOAD_W { w: self }
    }
}
