#[doc = "Reader of register RXCLKCHK"]
pub type R = crate::R<u32, super::RXCLKCHK>;
#[doc = "Writer for register RXCLKCHK"]
pub type W = crate::W<u32, super::RXCLKCHK>;
#[doc = "Register RXCLKCHK `reset()`'s with value 0"]
impl crate::ResetValue for super::RXCLKCHK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RCNT`"]
pub type RCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCNT`"]
pub struct RCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `RMAX`"]
pub type RMAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMAX`"]
pub struct RMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RMIN`"]
pub type RMIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMIN`"]
pub struct RMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RPS`"]
pub type RPS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPS`"]
pub struct RPS_W<'a> {
    w: &'a mut W,
}
impl<'a> RPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - RCV clock count value"]
    #[inline(always)]
    pub fn rcnt(&self) -> RCNT_R {
        RCNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RCV clock maximum boundary"]
    #[inline(always)]
    pub fn rmax(&self) -> RMAX_R {
        RMAX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RCV clock minimum boundary"]
    #[inline(always)]
    pub fn rmin(&self) -> RMIN_R {
        RMIN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:3 - RCV clock check prescaler 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7 0x8"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - RCV clock count value"]
    #[inline(always)]
    pub fn rcnt(&mut self) -> RCNT_W {
        RCNT_W { w: self }
    }
    #[doc = "Bits 16:23 - RCV clock maximum boundary"]
    #[inline(always)]
    pub fn rmax(&mut self) -> RMAX_W {
        RMAX_W { w: self }
    }
    #[doc = "Bits 8:15 - RCV clock minimum boundary"]
    #[inline(always)]
    pub fn rmin(&mut self) -> RMIN_W {
        RMIN_W { w: self }
    }
    #[doc = "Bits 0:3 - RCV clock check prescaler 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7 0x8"]
    #[inline(always)]
    pub fn rps(&mut self) -> RPS_W {
        RPS_W { w: self }
    }
}
