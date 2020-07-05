#[doc = "Reader of register RXFMT"]
pub type R = crate::R<u32, super::RXFMT>;
#[doc = "Writer for register RXFMT"]
pub type W = crate::W<u32, super::RXFMT>;
#[doc = "Register RXFMT `reset()`'s with value 0"]
impl crate::ResetValue for super::RXFMT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDATDLY`"]
pub type RDATDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDATDLY`"]
pub struct RDATDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> RDATDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `RPAD`"]
pub type RPAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPAD`"]
pub struct RPAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RPAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `RPBIT`"]
pub type RPBIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPBIT`"]
pub struct RPBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RPBIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RSSZ`"]
pub type RSSZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSSZ`"]
pub struct RSSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `RROT`"]
pub type RROT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RROT`"]
pub struct RROT_W<'a> {
    w: &'a mut W,
}
impl<'a> RROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - RCV Frame sync delay 0x0 0 Bit delay 0x1 1 Bit delay 0x2 2 Bit delay"]
    #[inline(always)]
    pub fn rdatdly(&self) -> RDATDLY_R {
        RDATDLY_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - Pad value 0x0 0x1 0x2"]
    #[inline(always)]
    pub fn rpad(&self) -> RPAD_R {
        RPAD_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - Pad bit position"]
    #[inline(always)]
    pub fn rpbit(&self) -> RPBIT_R {
        RPBIT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 4:7 - RCV slot Size 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7 0x8 0x9 0xA 0xB 0xC 0xD 0xE 0xF"]
    #[inline(always)]
    pub fn rssz(&self) -> RSSZ_R {
        RSSZ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2 - Right Rotate Value 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7"]
    #[inline(always)]
    pub fn rrot(&self) -> RROT_R {
        RROT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - RCV Frame sync delay 0x0 0 Bit delay 0x1 1 Bit delay 0x2 2 Bit delay"]
    #[inline(always)]
    pub fn rdatdly(&mut self) -> RDATDLY_W {
        RDATDLY_W { w: self }
    }
    #[doc = "Bits 13:14 - Pad value 0x0 0x1 0x2"]
    #[inline(always)]
    pub fn rpad(&mut self) -> RPAD_W {
        RPAD_W { w: self }
    }
    #[doc = "Bits 8:12 - Pad bit position"]
    #[inline(always)]
    pub fn rpbit(&mut self) -> RPBIT_W {
        RPBIT_W { w: self }
    }
    #[doc = "Bits 4:7 - RCV slot Size 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7 0x8 0x9 0xA 0xB 0xC 0xD 0xE 0xF"]
    #[inline(always)]
    pub fn rssz(&mut self) -> RSSZ_W {
        RSSZ_W { w: self }
    }
    #[doc = "Bits 0:2 - Right Rotate Value 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7"]
    #[inline(always)]
    pub fn rrot(&mut self) -> RROT_W {
        RROT_W { w: self }
    }
}
