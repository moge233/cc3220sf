#[doc = "Reader of register GPIOIBE"]
pub type R = crate::R<u32, super::GPIOIBE>;
#[doc = "Writer for register GPIOIBE"]
pub type W = crate::W<u32, super::GPIOIBE>;
#[doc = "Register GPIOIBE `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOIBE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IBE`"]
pub type IBE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBE`"]
pub struct IBE_W<'a> {
    w: &'a mut W,
}
impl<'a> IBE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO interrupt on both edges on corresponding pin"]
    #[inline(always)]
    pub fn ibe(&self) -> IBE_R {
        IBE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO interrupt on both edges on corresponding pin"]
    #[inline(always)]
    pub fn ibe(&mut self) -> IBE_W {
        IBE_W { w: self }
    }
}
