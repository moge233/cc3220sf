#[doc = "Reader of register I2COBSMUXSEL0"]
pub type R = crate::R<u32, super::I2COBSMUXSEL0>;
#[doc = "Writer for register I2COBSMUXSEL0"]
pub type W = crate::W<u32, super::I2COBSMUXSEL0>;
#[doc = "Register I2COBSMUXSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::I2COBSMUXSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LN3`"]
pub type LN3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN3`"]
pub struct LN3_W<'a> {
    w: &'a mut W,
}
impl<'a> LN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `LN2`"]
pub type LN2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN2`"]
pub struct LN2_W<'a> {
    w: &'a mut W,
}
impl<'a> LN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `LN1`"]
pub type LN1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN1`"]
pub struct LN1_W<'a> {
    w: &'a mut W,
}
impl<'a> LN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `LN0`"]
pub type LN0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LN0`"]
pub struct LN0_W<'a> {
    w: &'a mut W,
}
impl<'a> LN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26 - Observation Mux Lane 3"]
    #[inline(always)]
    pub fn ln3(&self) -> LN3_R {
        LN3_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Observation Mux Lane 2"]
    #[inline(always)]
    pub fn ln2(&self) -> LN2_R {
        LN2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Observation Mux Lane 1"]
    #[inline(always)]
    pub fn ln1(&self) -> LN1_R {
        LN1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Observation Mux Lane 0"]
    #[inline(always)]
    pub fn ln0(&self) -> LN0_R {
        LN0_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Observation Mux Lane 3"]
    #[inline(always)]
    pub fn ln3(&mut self) -> LN3_W {
        LN3_W { w: self }
    }
    #[doc = "Bits 16:18 - Observation Mux Lane 2"]
    #[inline(always)]
    pub fn ln2(&mut self) -> LN2_W {
        LN2_W { w: self }
    }
    #[doc = "Bits 8:10 - Observation Mux Lane 1"]
    #[inline(always)]
    pub fn ln1(&mut self) -> LN1_W {
        LN1_W { w: self }
    }
    #[doc = "Bits 0:2 - Observation Mux Lane 0"]
    #[inline(always)]
    pub fn ln0(&mut self) -> LN0_W {
        LN0_W { w: self }
    }
}
