#[doc = "Reader of register REV"]
pub type R = crate::R<u32, super::REV>;
#[doc = "Writer for register REV"]
pub type W = crate::W<u32, super::REV>;
#[doc = "Register REV `reset()`'s with value 0"]
impl crate::ResetValue for super::REV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VREV`"]
pub type VREV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VREV`"]
pub struct VREV_W<'a> {
    w: &'a mut W,
}
impl<'a> VREV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `SREV`"]
pub type SREV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SREV`"]
pub struct SREV_W<'a> {
    w: &'a mut W,
}
impl<'a> SREV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Vendor Version Number: IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision Examples: 0x10 for 1.0 0x21 for 2.1"]
    #[inline(always)]
    pub fn vrev(&self) -> VREV_R {
        VREV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SREV"]
    #[inline(always)]
    pub fn srev(&self) -> SREV_R {
        SREV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Vendor Version Number: IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision Examples: 0x10 for 1.0 0x21 for 2.1"]
    #[inline(always)]
    pub fn vrev(&mut self) -> VREV_W {
        VREV_W { w: self }
    }
    #[doc = "Bits 16:23 - SREV"]
    #[inline(always)]
    pub fn srev(&mut self) -> SREV_W {
        SREV_W { w: self }
    }
}
