#[doc = "Reader of register RTCPD"]
pub type R = crate::R<u32, super::RTCPD>;
#[doc = "Writer for register RTCPD"]
pub type W = crate::W<u32, super::RTCPD>;
#[doc = "Register RTCPD `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCPD`"]
pub type RTCPD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTCPD`"]
pub struct RTCPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RTC Predivide Counter Value"]
    #[inline(always)]
    pub fn rtcpd(&self) -> RTCPD_R {
        RTCPD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC Predivide Counter Value"]
    #[inline(always)]
    pub fn rtcpd(&mut self) -> RTCPD_W {
        RTCPD_W { w: self }
    }
}
