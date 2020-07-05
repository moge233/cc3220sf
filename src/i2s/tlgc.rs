#[doc = "Reader of register TLGC"]
pub type R = crate::R<u32, super::TLGC>;
#[doc = "Writer for register TLGC"]
pub type W = crate::W<u32, super::TLGC>;
#[doc = "Register TLGC `reset()`'s with value 0"]
impl crate::ResetValue for super::TLGC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MT`"]
pub type MT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MT`"]
pub struct MT_W<'a> {
    w: &'a mut W,
}
impl<'a> MT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `MC`"]
pub type MC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MC`"]
pub struct MC_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `PC`"]
pub type PC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC`"]
pub struct PC_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:15 - MISR on/off trigger command 0x0 0x1 0x2 0x3"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - States of MISR 0x0 0x1 0x2 0x3"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 1:3 - Pattern code 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15 - MISR on/off trigger command 0x0 0x1 0x2 0x3"]
    #[inline(always)]
    pub fn mt(&mut self) -> MT_W {
        MT_W { w: self }
    }
    #[doc = "Bits 4:5 - States of MISR 0x0 0x1 0x2 0x3"]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W {
        MC_W { w: self }
    }
    #[doc = "Bits 1:3 - Pattern code 0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7"]
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W {
        PC_W { w: self }
    }
}
