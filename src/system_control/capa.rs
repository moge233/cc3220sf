#[doc = "Reader of register CAPA"]
pub type R = crate::R<u32, super::CAPA>;
#[doc = "Writer for register CAPA"]
pub type W = crate::W<u32, super::CAPA>;
#[doc = "Register CAPA `reset()`'s with value 0"]
impl crate::ResetValue for super::CAPA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MBL`"]
pub type MBL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MBL`"]
pub struct MBL_W<'a> {
    w: &'a mut W,
}
impl<'a> MBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `BCF`"]
pub type BCF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BCF`"]
pub struct BCF_W<'a> {
    w: &'a mut W,
}
impl<'a> BCF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TCF`"]
pub type TCF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCF`"]
pub struct TCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - MBL"]
    #[inline(always)]
    pub fn mbl(&self) -> MBL_R {
        MBL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:13 - BCF"]
    #[inline(always)]
    pub fn bcf(&self) -> BCF_R {
        BCF_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - TCF"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - MBL"]
    #[inline(always)]
    pub fn mbl(&mut self) -> MBL_W {
        MBL_W { w: self }
    }
    #[doc = "Bits 8:13 - BCF"]
    #[inline(always)]
    pub fn bcf(&mut self) -> BCF_W {
        BCF_W { w: self }
    }
    #[doc = "Bits 0:5 - TCF"]
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
}
