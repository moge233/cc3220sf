#[doc = "Reader of register TXFMT"]
pub type R = crate::R<u32, super::TXFMT>;
#[doc = "Writer for register TXFMT"]
pub type W = crate::W<u32, super::TXFMT>;
#[doc = "Register TXFMT `reset()`'s with value 0"]
impl crate::ResetValue for super::TXFMT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XDATDLY`"]
pub type XDATDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XDATDLY`"]
pub struct XDATDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> XDATDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `XPAD`"]
pub type XPAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XPAD`"]
pub struct XPAD_W<'a> {
    w: &'a mut W,
}
impl<'a> XPAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `XPBIT`"]
pub type XPBIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XPBIT`"]
pub struct XPBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> XPBIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `XSSZ`"]
pub type XSSZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XSSZ`"]
pub struct XSSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> XSSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `XROT`"]
pub type XROT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XROT`"]
pub struct XROT_W<'a> {
    w: &'a mut W,
}
impl<'a> XROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - XMT Frame sync delay 0x0 0 Bit delay 0x1 1 Bit delay 0x2 2 Bit delay"]
    #[inline(always)]
    pub fn xdatdly(&self) -> XDATDLY_R {
        XDATDLY_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - Pad value 0x0 0x1 0x2"]
    #[inline(always)]
    pub fn xpad(&self) -> XPAD_R {
        XPAD_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - Pad bit position"]
    #[inline(always)]
    pub fn xpbit(&self) -> XPBIT_R {
        XPBIT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 4:7 - XMT slot Size 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7 0x8 0x9 0xA 0xB 0xC 0xD 0xE 0xF"]
    #[inline(always)]
    pub fn xssz(&self) -> XSSZ_R {
        XSSZ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2 - Right Rotate Value 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7"]
    #[inline(always)]
    pub fn xrot(&self) -> XROT_R {
        XROT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - XMT Frame sync delay 0x0 0 Bit delay 0x1 1 Bit delay 0x2 2 Bit delay"]
    #[inline(always)]
    pub fn xdatdly(&mut self) -> XDATDLY_W {
        XDATDLY_W { w: self }
    }
    #[doc = "Bits 13:14 - Pad value 0x0 0x1 0x2"]
    #[inline(always)]
    pub fn xpad(&mut self) -> XPAD_W {
        XPAD_W { w: self }
    }
    #[doc = "Bits 8:12 - Pad bit position"]
    #[inline(always)]
    pub fn xpbit(&mut self) -> XPBIT_W {
        XPBIT_W { w: self }
    }
    #[doc = "Bits 4:7 - XMT slot Size 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7 0x8 0x9 0xA 0xB 0xC 0xD 0xE 0xF"]
    #[inline(always)]
    pub fn xssz(&mut self) -> XSSZ_W {
        XSSZ_W { w: self }
    }
    #[doc = "Bits 0:2 - Right Rotate Value 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7"]
    #[inline(always)]
    pub fn xrot(&mut self) -> XROT_W {
        XROT_W { w: self }
    }
}
