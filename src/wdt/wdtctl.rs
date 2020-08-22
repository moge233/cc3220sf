#[doc = "Reader of register WDTCTL"]
pub type R = crate::R<u32, super::WDTCTL>;
#[doc = "Writer for register WDTCTL"]
pub type W = crate::W<u32, super::WDTCTL>;
#[doc = "Register WDTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRC`"]
pub type WRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRC`"]
pub struct WRC_W<'a> {
    w: &'a mut W,
}
impl<'a> WRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `INTTYPE`"]
pub type INTTYPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTTYPE`"]
pub struct INTTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTYPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `INTEN`"]
pub type INTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN`"]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
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
    #[doc = "Bit 31 - Write Complete"]
    #[inline(always)]
    pub fn wrc(&self) -> WRC_R {
        WRC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Watchdog Interrupt Type"]
    #[inline(always)]
    pub fn inttype(&self) -> INTTYPE_R {
        INTTYPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Write Complete"]
    #[inline(always)]
    pub fn wrc(&mut self) -> WRC_W {
        WRC_W { w: self }
    }
    #[doc = "Bit 2 - Watchdog Interrupt Type"]
    #[inline(always)]
    pub fn inttype(&mut self) -> INTTYPE_W {
        INTTYPE_W { w: self }
    }
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
}
