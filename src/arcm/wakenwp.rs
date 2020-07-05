#[doc = "Reader of register WAKENWP"]
pub type R = crate::R<u32, super::WAKENWP>;
#[doc = "Writer for register WAKENWP"]
pub type W = crate::W<u32, super::WAKENWP>;
#[doc = "Register WAKENWP `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKENWP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAKENWP`"]
pub type WAKENWP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKENWP`"]
pub struct WAKENWP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKENWP_W<'a> {
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
    #[doc = "Bit 0 - APPS_TO_NWP_WAKEUP_REQUEST"]
    #[inline(always)]
    pub fn wakenwp(&self) -> WAKENWP_R {
        WAKENWP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - APPS_TO_NWP_WAKEUP_REQUEST"]
    #[inline(always)]
    pub fn wakenwp(&mut self) -> WAKENWP_W {
        WAKENWP_W { w: self }
    }
}
