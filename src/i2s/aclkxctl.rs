#[doc = "Reader of register ACLKXCTL"]
pub type R = crate::R<u32, super::ACLKXCTL>;
#[doc = "Writer for register ACLKXCTL"]
pub type W = crate::W<u32, super::ACLKXCTL>;
#[doc = "Register ACLKXCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ACLKXCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKXADJ`"]
pub type CLKXADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKXADJ`"]
pub struct CLKXADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKXADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CLKXDIV`"]
pub type CLKXDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKXDIV`"]
pub struct CLKXDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKXDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - CLKXADJ"]
    #[inline(always)]
    pub fn clkxadj(&self) -> CLKXADJ_R {
        CLKXADJ_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:4 - XMT clock devide ratio"]
    #[inline(always)]
    pub fn clkxdiv(&self) -> CLKXDIV_R {
        CLKXDIV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - CLKXADJ"]
    #[inline(always)]
    pub fn clkxadj(&mut self) -> CLKXADJ_W {
        CLKXADJ_W { w: self }
    }
    #[doc = "Bits 0:4 - XMT clock devide ratio"]
    #[inline(always)]
    pub fn clkxdiv(&mut self) -> CLKXDIV_W {
        CLKXDIV_W { w: self }
    }
}
