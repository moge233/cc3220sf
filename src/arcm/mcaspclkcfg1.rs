#[doc = "Reader of register MCASPCLKCFG1"]
pub type R = crate::R<u32, super::MCASPCLKCFG1>;
#[doc = "Writer for register MCASPCLKCFG1"]
pub type W = crate::W<u32, super::MCASPCLKCFG1>;
#[doc = "Register MCASPCLKCFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MCASPCLKCFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIVIDRSWRST`"]
pub type DIVIDRSWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVIDRSWRST`"]
pub struct DIVIDRSWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDRSWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPARE`"]
pub type SPARE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPARE`"]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - MCASP_FRAC_DIV_SOFT_RESET"]
    #[inline(always)]
    pub fn dividrswrst(&self) -> DIVIDRSWRST_R {
        DIVIDRSWRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:9 - MCASP_FRAC_DIV_PERIOD"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - MCASP_FRAC_DIV_SOFT_RESET"]
    #[inline(always)]
    pub fn dividrswrst(&mut self) -> DIVIDRSWRST_W {
        DIVIDRSWRST_W { w: self }
    }
    #[doc = "Bits 0:9 - MCASP_FRAC_DIV_PERIOD"]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
}
