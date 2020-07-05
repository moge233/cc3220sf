#[doc = "Reader of register LPDSREQ"]
pub type R = crate::R<u32, super::LPDSREQ>;
#[doc = "Writer for register LPDSREQ"]
pub type W = crate::W<u32, super::LPDSREQ>;
#[doc = "Register LPDSREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::LPDSREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPDSREQ`"]
pub type LPDSREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPDSREQ`"]
pub struct LPDSREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDSREQ_W<'a> {
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
    #[doc = "Bit 0 - APPS_LPDS_REQ"]
    #[inline(always)]
    pub fn lpdsreq(&self) -> LPDSREQ_R {
        LPDSREQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - APPS_LPDS_REQ"]
    #[inline(always)]
    pub fn lpdsreq(&mut self) -> LPDSREQ_W {
        LPDSREQ_W { w: self }
    }
}
