#[doc = "Reader of register PLATFORM_DETECTION_RD_ONLY"]
pub type R = crate::R<u32, super::PLATFORM_DETECTION_RD_ONLY>;
#[doc = "Writer for register PLATFORM_DETECTION_RD_ONLY"]
pub type W = crate::W<u32, super::PLATFORM_DETECTION_RD_ONLY>;
#[doc = "Register PLATFORM_DETECTION_RD_ONLY `reset()`'s with value 0"]
impl crate::ResetValue for super::PLATFORM_DETECTION_RD_ONLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLATFORM_DETECTION`"]
pub type PLATFORM_DETECTION_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PLATFORM_DETECTION`"]
pub struct PLATFORM_DETECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> PLATFORM_DETECTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This information serves the IPs for knowing in which platform are they integrated at: 0 = CC31XX."]
    #[inline(always)]
    pub fn platform_detection(&self) -> PLATFORM_DETECTION_R {
        PLATFORM_DETECTION_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This information serves the IPs for knowing in which platform are they integrated at: 0 = CC31XX."]
    #[inline(always)]
    pub fn platform_detection(&mut self) -> PLATFORM_DETECTION_W {
        PLATFORM_DETECTION_W { w: self }
    }
}
