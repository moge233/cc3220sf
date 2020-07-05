#[doc = "Reader of register AHCLKXCTL"]
pub type R = crate::R<u32, super::AHCLKXCTL>;
#[doc = "Writer for register AHCLKXCTL"]
pub type W = crate::W<u32, super::AHCLKXCTL>;
#[doc = "Register AHCLKXCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::AHCLKXCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HCLKXADJ`"]
pub type HCLKXADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HCLKXADJ`"]
pub struct HCLKXADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLKXADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `HCLKXDIV`"]
pub type HCLKXDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HCLKXDIV`"]
pub struct HCLKXDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLKXDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - HCLKXADJ"]
    #[inline(always)]
    pub fn hclkxadj(&self) -> HCLKXADJ_R {
        HCLKXADJ_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:11 - XMT clock Divide Ratio"]
    #[inline(always)]
    pub fn hclkxdiv(&self) -> HCLKXDIV_R {
        HCLKXDIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:17 - HCLKXADJ"]
    #[inline(always)]
    pub fn hclkxadj(&mut self) -> HCLKXADJ_W {
        HCLKXADJ_W { w: self }
    }
    #[doc = "Bits 0:11 - XMT clock Divide Ratio"]
    #[inline(always)]
    pub fn hclkxdiv(&mut self) -> HCLKXDIV_W {
        HCLKXDIV_W { w: self }
    }
}
