#[doc = "Reader of register TURBOREQ"]
pub type R = crate::R<u32, super::TURBOREQ>;
#[doc = "Writer for register TURBOREQ"]
pub type W = crate::W<u32, super::TURBOREQ>;
#[doc = "Register TURBOREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::TURBOREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TURBOREQ`"]
pub type TURBOREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TURBOREQ`"]
pub struct TURBOREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TURBOREQ_W<'a> {
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
    #[doc = "Bit 0 - APPS_TURBO_REQ"]
    #[inline(always)]
    pub fn turboreq(&self) -> TURBOREQ_R {
        TURBOREQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - APPS_TURBO_REQ"]
    #[inline(always)]
    pub fn turboreq(&mut self) -> TURBOREQ_W {
        TURBOREQ_W { w: self }
    }
}
