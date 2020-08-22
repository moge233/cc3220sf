#[doc = "Reader of register GPTMRTCPD"]
pub type R = crate::R<u32, super::GPTMRTCPD>;
#[doc = "Writer for register GPTMRTCPD"]
pub type W = crate::W<u32, super::GPTMRTCPD>;
#[doc = "Register GPTMRTCPD `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTMRTCPD {
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
