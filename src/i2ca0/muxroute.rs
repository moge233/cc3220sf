#[doc = "Reader of register MUXROUTE"]
pub type R = crate::R<u32, super::MUXROUTE>;
#[doc = "Writer for register MUXROUTE"]
pub type W = crate::W<u32, super::MUXROUTE>;
#[doc = "Register MUXROUTE `reset()`'s with value 0"]
impl crate::ResetValue for super::MUXROUTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LN7ROUTE`"]
pub type LN7ROUTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN7ROUTE`"]
pub struct LN7ROUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> LN7ROUTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `LN6ROUTE`"]
pub type LN6ROUTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN6ROUTE`"]
pub struct LN6ROUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> LN6ROUTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `LN5ROUTE`"]
pub type LN5ROUTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN5ROUTE`"]
pub struct LN5ROUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> LN5ROUTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `LN4ROUTE`"]
pub type LN4ROUTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN4ROUTE`"]
pub struct LN4ROUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> LN4ROUTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `LN3ROUTE`"]
pub type LN3ROUTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN3ROUTE`"]
pub struct LN3ROUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> LN3ROUTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `LN2ROUTE`"]
pub type LN2ROUTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN2ROUTE`"]
pub struct LN2ROUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> LN2ROUTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `LN1ROUTE`"]
pub type LN1ROUTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN1ROUTE`"]
pub struct LN1ROUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> LN1ROUTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `LN0ROUTE`"]
pub type LN0ROUTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN0ROUTE`"]
pub struct LN0ROUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> LN0ROUTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30 - Lane 7 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln7route(&self) -> LN7ROUTE_R {
        LN7ROUTE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Lane 6 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln6route(&self) -> LN6ROUTE_R {
        LN6ROUTE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Lane 5 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln5route(&self) -> LN5ROUTE_R {
        LN5ROUTE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Lane 4 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln4route(&self) -> LN4ROUTE_R {
        LN4ROUTE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Lane 3 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln3route(&self) -> LN3ROUTE_R {
        LN3ROUTE_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Lane 2 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln2route(&self) -> LN2ROUTE_R {
        LN2ROUTE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Lane 1 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln1route(&self) -> LN1ROUTE_R {
        LN1ROUTE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Lane 0 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln0route(&self) -> LN0ROUTE_R {
        LN0ROUTE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30 - Lane 7 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln7route(&mut self) -> LN7ROUTE_W {
        LN7ROUTE_W { w: self }
    }
    #[doc = "Bits 24:26 - Lane 6 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln6route(&mut self) -> LN6ROUTE_W {
        LN6ROUTE_W { w: self }
    }
    #[doc = "Bits 20:22 - Lane 5 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln5route(&mut self) -> LN5ROUTE_W {
        LN5ROUTE_W { w: self }
    }
    #[doc = "Bits 16:18 - Lane 4 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln4route(&mut self) -> LN4ROUTE_W {
        LN4ROUTE_W { w: self }
    }
    #[doc = "Bits 12:14 - Lane 3 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln3route(&mut self) -> LN3ROUTE_W {
        LN3ROUTE_W { w: self }
    }
    #[doc = "Bits 8:10 - Lane 2 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln2route(&mut self) -> LN2ROUTE_W {
        LN2ROUTE_W { w: self }
    }
    #[doc = "Bits 4:6 - Lane 1 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln1route(&mut self) -> LN1ROUTE_W {
        LN1ROUTE_W { w: self }
    }
    #[doc = "Bits 0:2 - Lane 0 output is routed to the lane pointed to by the offset in this bit field"]
    #[inline(always)]
    pub fn ln0route(&mut self) -> LN0ROUTE_W {
        LN0ROUTE_W { w: self }
    }
}
