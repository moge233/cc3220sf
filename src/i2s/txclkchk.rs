#[doc = "Reader of register TXCLKCHK"]
pub type R = crate::R<u32, super::TXCLKCHK>;
#[doc = "Writer for register TXCLKCHK"]
pub type W = crate::W<u32, super::TXCLKCHK>;
#[doc = "Register TXCLKCHK `reset()`'s with value 0"]
impl crate::ResetValue for super::TXCLKCHK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XCNT`"]
pub type XCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XCNT`"]
pub struct XCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> XCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `XMAX`"]
pub type XMAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XMAX`"]
pub struct XMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> XMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `XMIN`"]
pub type XMIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XMIN`"]
pub struct XMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> XMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `XPS`"]
pub type XPS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XPS`"]
pub struct XPS_W<'a> {
    w: &'a mut W,
}
impl<'a> XPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - XMT clock count value"]
    #[inline(always)]
    pub fn xcnt(&self) -> XCNT_R {
        XCNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - XMT clock maximum boundary"]
    #[inline(always)]
    pub fn xmax(&self) -> XMAX_R {
        XMAX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - XMT clock minimum boundary"]
    #[inline(always)]
    pub fn xmin(&self) -> XMIN_R {
        XMIN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:3 - XMT clock check prescaler 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7 0x8"]
    #[inline(always)]
    pub fn xps(&self) -> XPS_R {
        XPS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - XMT clock count value"]
    #[inline(always)]
    pub fn xcnt(&mut self) -> XCNT_W {
        XCNT_W { w: self }
    }
    #[doc = "Bits 16:23 - XMT clock maximum boundary"]
    #[inline(always)]
    pub fn xmax(&mut self) -> XMAX_W {
        XMAX_W { w: self }
    }
    #[doc = "Bits 8:15 - XMT clock minimum boundary"]
    #[inline(always)]
    pub fn xmin(&mut self) -> XMIN_W {
        XMIN_W { w: self }
    }
    #[doc = "Bits 0:3 - XMT clock check prescaler 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7 0x8"]
    #[inline(always)]
    pub fn xps(&mut self) -> XPS_W {
        XPS_W { w: self }
    }
}
