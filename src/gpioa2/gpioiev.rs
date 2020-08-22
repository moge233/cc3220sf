#[doc = "Reader of register GPIOIEV"]
pub type R = crate::R<u32, super::GPIOIEV>;
#[doc = "Writer for register GPIOIEV"]
pub type W = crate::W<u32, super::GPIOIEV>;
#[doc = "Register GPIOIEV `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOIEV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IEV`"]
pub type IEV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IEV`"]
pub struct IEV_W<'a> {
    w: &'a mut W,
}
impl<'a> IEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO interrupt event on corresponding pin"]
    #[inline(always)]
    pub fn iev(&self) -> IEV_R {
        IEV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO interrupt event on corresponding pin"]
    #[inline(always)]
    pub fn iev(&mut self) -> IEV_W {
        IEV_W { w: self }
    }
}
