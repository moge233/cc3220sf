#[doc = "Reader of register OBSMUXSEL1"]
pub type R = crate::R<u32, super::OBSMUXSEL1>;
#[doc = "Writer for register OBSMUXSEL1"]
pub type W = crate::W<u32, super::OBSMUXSEL1>;
#[doc = "Register OBSMUXSEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::OBSMUXSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LN7`"]
pub type LN7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN7`"]
pub struct LN7_W<'a> {
    w: &'a mut W,
}
impl<'a> LN7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `LN6`"]
pub type LN6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN6`"]
pub struct LN6_W<'a> {
    w: &'a mut W,
}
impl<'a> LN6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `LN5`"]
pub type LN5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN5`"]
pub struct LN5_W<'a> {
    w: &'a mut W,
}
impl<'a> LN5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `LN4`"]
pub type LN4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN4`"]
pub struct LN4_W<'a> {
    w: &'a mut W,
}
impl<'a> LN4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26 - Observation Mux Lane 7"]
    #[inline(always)]
    pub fn ln7(&self) -> LN7_R {
        LN7_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Observation Mux Lane 6"]
    #[inline(always)]
    pub fn ln6(&self) -> LN6_R {
        LN6_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Observation Mux Lane 5"]
    #[inline(always)]
    pub fn ln5(&self) -> LN5_R {
        LN5_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Observation Mux Lane 4"]
    #[inline(always)]
    pub fn ln4(&self) -> LN4_R {
        LN4_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Observation Mux Lane 7"]
    #[inline(always)]
    pub fn ln7(&mut self) -> LN7_W {
        LN7_W { w: self }
    }
    #[doc = "Bits 16:18 - Observation Mux Lane 6"]
    #[inline(always)]
    pub fn ln6(&mut self) -> LN6_W {
        LN6_W { w: self }
    }
    #[doc = "Bits 8:10 - Observation Mux Lane 5"]
    #[inline(always)]
    pub fn ln5(&mut self) -> LN5_W {
        LN5_W { w: self }
    }
    #[doc = "Bits 0:2 - Observation Mux Lane 4"]
    #[inline(always)]
    pub fn ln4(&mut self) -> LN4_W {
        LN4_W { w: self }
    }
}
