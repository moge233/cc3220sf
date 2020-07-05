#[doc = "Reader of register ESYSCONFIG"]
pub type R = crate::R<u32, super::ESYSCONFIG>;
#[doc = "Writer for register ESYSCONFIG"]
pub type W = crate::W<u32, super::ESYSCONFIG>;
#[doc = "Register ESYSCONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::ESYSCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSV`"]
pub type RSV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RSV`"]
pub struct RSV_W<'a> {
    w: &'a mut W,
}
impl<'a> RSV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `OTHER`"]
pub type OTHER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OTHER`"]
pub struct OTHER_W<'a> {
    w: &'a mut W,
}
impl<'a> OTHER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `IDLE_MODE`"]
pub type IDLE_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDLE_MODE`"]
pub struct IDLE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:31 - Reserved as per PDR 3.5"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bits 2:5 - Reserved for future expansion"]
    #[inline(always)]
    pub fn other(&self) -> OTHER_R {
        OTHER_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - Idle Mode"]
    #[inline(always)]
    pub fn idle_mode(&self) -> IDLE_MODE_R {
        IDLE_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:31 - Reserved as per PDR 3.5"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W {
        RSV_W { w: self }
    }
    #[doc = "Bits 2:5 - Reserved for future expansion"]
    #[inline(always)]
    pub fn other(&mut self) -> OTHER_W {
        OTHER_W { w: self }
    }
    #[doc = "Bits 0:1 - Idle Mode"]
    #[inline(always)]
    pub fn idle_mode(&mut self) -> IDLE_MODE_W {
        IDLE_MODE_W { w: self }
    }
}
