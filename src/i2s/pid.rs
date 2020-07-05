#[doc = "Reader of register PID"]
pub type R = crate::R<u32, super::PID>;
#[doc = "Writer for register PID"]
pub type W = crate::W<u32, super::PID>;
#[doc = "Register PID `reset()`'s with value 0"]
impl crate::ResetValue for super::PID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCHEME`"]
pub type SCHEME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCHEME`"]
pub struct SCHEME_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHEME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `RESV`"]
pub type RESV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESV`"]
pub struct RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> RESV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `FUNCTION`"]
pub type FUNCTION_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FUNCTION`"]
pub struct FUNCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNCTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RTL`"]
pub type RTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTL`"]
pub struct RTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `REVMAJOR`"]
pub type REVMAJOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REVMAJOR`"]
pub struct REVMAJOR_W<'a> {
    w: &'a mut W,
}
impl<'a> REVMAJOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `CUSTOM`"]
pub type CUSTOM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CUSTOM`"]
pub struct CUSTOM_W<'a> {
    w: &'a mut W,
}
impl<'a> CUSTOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `REVMINOR`"]
pub type REVMINOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REVMINOR`"]
pub struct REVMINOR_W<'a> {
    w: &'a mut W,
}
impl<'a> REVMINOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - SCHEME"]
    #[inline(always)]
    pub fn scheme(&self) -> SCHEME_R {
        SCHEME_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - RESV"]
    #[inline(always)]
    pub fn resv(&self) -> RESV_R {
        RESV_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 16:27 - McASP"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 11:15 - RTL"]
    #[inline(always)]
    pub fn rtl(&self) -> RTL_R {
        RTL_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - REVMAJOR"]
    #[inline(always)]
    pub fn revmajor(&self) -> REVMAJOR_R {
        REVMAJOR_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - non-custom"]
    #[inline(always)]
    pub fn custom(&self) -> CUSTOM_R {
        CUSTOM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - REVMINOR"]
    #[inline(always)]
    pub fn revminor(&self) -> REVMINOR_R {
        REVMINOR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - SCHEME"]
    #[inline(always)]
    pub fn scheme(&mut self) -> SCHEME_W {
        SCHEME_W { w: self }
    }
    #[doc = "Bits 28:29 - RESV"]
    #[inline(always)]
    pub fn resv(&mut self) -> RESV_W {
        RESV_W { w: self }
    }
    #[doc = "Bits 16:27 - McASP"]
    #[inline(always)]
    pub fn function(&mut self) -> FUNCTION_W {
        FUNCTION_W { w: self }
    }
    #[doc = "Bits 11:15 - RTL"]
    #[inline(always)]
    pub fn rtl(&mut self) -> RTL_W {
        RTL_W { w: self }
    }
    #[doc = "Bits 8:10 - REVMAJOR"]
    #[inline(always)]
    pub fn revmajor(&mut self) -> REVMAJOR_W {
        REVMAJOR_W { w: self }
    }
    #[doc = "Bits 6:7 - non-custom"]
    #[inline(always)]
    pub fn custom(&mut self) -> CUSTOM_W {
        CUSTOM_W { w: self }
    }
    #[doc = "Bits 0:5 - REVMINOR"]
    #[inline(always)]
    pub fn revminor(&mut self) -> REVMINOR_W {
        REVMINOR_W { w: self }
    }
}
