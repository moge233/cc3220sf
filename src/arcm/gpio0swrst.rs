#[doc = "Reader of register GPIO0SWRST"]
pub type R = crate::R<u32, super::GPIO0SWRST>;
#[doc = "Writer for register GPIO0SWRST"]
pub type W = crate::W<u32, super::GPIO0SWRST>;
#[doc = "Register GPIO0SWRST `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO0SWRST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENSTS`"]
pub type ENSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - GPIO_A_ENABLED_STATUS"]
    #[inline(always)]
    pub fn ensts(&self) -> ENSTS_R {
        ENSTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPIO_A_SOFT_RESET"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO_A_SOFT_RESET"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
}
