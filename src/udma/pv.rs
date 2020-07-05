#[doc = "Reader of register PV"]
pub type R = crate::R<u32, super::PV>;
#[doc = "Writer for register PV"]
pub type W = crate::W<u32, super::PV>;
#[doc = "Register PV `reset()`'s with value 0"]
impl crate::ResetValue for super::PV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAJOR`"]
pub type MAJOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAJOR`"]
pub struct MAJOR_W<'a> {
    w: &'a mut W,
}
impl<'a> MAJOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `MINOR`"]
pub type MINOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MINOR`"]
pub struct MINOR_W<'a> {
    w: &'a mut W,
}
impl<'a> MINOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Major Revision"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Minor Revision"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Major Revision"]
    #[inline(always)]
    pub fn major(&mut self) -> MAJOR_W {
        MAJOR_W { w: self }
    }
    #[doc = "Bits 0:7 - Minor Revision"]
    #[inline(always)]
    pub fn minor(&mut self) -> MINOR_W {
        MINOR_W { w: self }
    }
}
