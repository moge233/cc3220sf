#[doc = "Reader of register GPIOMIS"]
pub type R = crate::R<u32, super::GPIOMIS>;
#[doc = "Writer for register GPIOMIS"]
pub type W = crate::W<u32, super::GPIOMIS>;
#[doc = "Register GPIOMIS `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOMIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RIS`"]
pub type RIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RIS`"]
pub struct RIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO masked interrupt status on corresponding pin"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO masked interrupt status on corresponding pin"]
    #[inline(always)]
    pub fn ris(&mut self) -> RIS_W {
        RIS_W { w: self }
    }
}
