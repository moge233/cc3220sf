#[doc = "Reader of register AMUTE"]
pub type R = crate::R<u32, super::AMUTE>;
#[doc = "Writer for register AMUTE"]
pub type W = crate::W<u32, super::AMUTE>;
#[doc = "Register AMUTE `reset()`'s with value 0"]
impl crate::ResetValue for super::AMUTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MUTEN`"]
pub type MUTEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUTEN`"]
pub struct MUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - AMUTE pin enable 0x0 0x1 0x2"]
    #[inline(always)]
    pub fn muten(&self) -> MUTEN_R {
        MUTEN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AMUTE pin enable 0x0 0x1 0x2"]
    #[inline(always)]
    pub fn muten(&mut self) -> MUTEN_W {
        MUTEN_W { w: self }
    }
}
