#[doc = "Reader of register GPTMTBMR"]
pub type R = crate::R<u32, super::GPTMTBMR>;
#[doc = "Writer for register GPTMTBMR"]
pub type W = crate::W<u32, super::GPTMTBMR>;
#[doc = "Register GPTMTBMR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTMTBMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBPLO`"]
pub type TBPLO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBPLO`"]
pub struct TBPLO_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPLO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TBMRSU`"]
pub type TBMRSU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMRSU`"]
pub struct TBMRSU_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMRSU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TBPWMIE`"]
pub type TBPWMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBPWMIE`"]
pub struct TBPWMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPWMIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TBILD`"]
pub type TBILD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBILD`"]
pub struct TBILD_W<'a> {
    w: &'a mut W,
}
impl<'a> TBILD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TBMIE`"]
pub type TBMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMIE`"]
pub struct TBMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TBCDIR`"]
pub type TBCDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBCDIR`"]
pub struct TBCDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TBAMS`"]
pub type TBAMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBAMS`"]
pub struct TBAMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBAMS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TBCMIR`"]
pub type TBCMIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBCMIR`"]
pub struct TBCMIR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCMIR_W<'a> {
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
#[doc = "Reader of field `TBMR`"]
pub type TBMR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBMR`"]
pub struct TBMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - GPTM Timer B PWM Legacy operation"]
    #[inline(always)]
    pub fn tbplo(&self) -> TBPLO_R {
        TBPLO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B Match Register Update"]
    #[inline(always)]
    pub fn tbmrsu(&self) -> TBMRSU_R {
        TBMRSU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B PWM Interrupt Enable"]
    #[inline(always)]
    pub fn tbpwmie(&self) -> TBPWMIE_R {
        TBPWMIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Interval Load Write"]
    #[inline(always)]
    pub fn tbild(&self) -> TBILD_R {
        TBILD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer B Match Interrupt Enable"]
    #[inline(always)]
    pub fn tbmie(&self) -> TBMIE_R {
        TBMIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer B Count Direction"]
    #[inline(always)]
    pub fn tbcdir(&self) -> TBCDIR_R {
        TBCDIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline(always)]
    pub fn tbams(&self) -> TBAMS_R {
        TBAMS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline(always)]
    pub fn tbcmir(&self) -> TBCMIR_R {
        TBCMIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline(always)]
    pub fn tbmr(&self) -> TBMR_R {
        TBMR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - GPTM Timer B PWM Legacy operation"]
    #[inline(always)]
    pub fn tbplo(&mut self) -> TBPLO_W {
        TBPLO_W { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer B Match Register Update"]
    #[inline(always)]
    pub fn tbmrsu(&mut self) -> TBMRSU_W {
        TBMRSU_W { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B PWM Interrupt Enable"]
    #[inline(always)]
    pub fn tbpwmie(&mut self) -> TBPWMIE_W {
        TBPWMIE_W { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Interval Load Write"]
    #[inline(always)]
    pub fn tbild(&mut self) -> TBILD_W {
        TBILD_W { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer B Match Interrupt Enable"]
    #[inline(always)]
    pub fn tbmie(&mut self) -> TBMIE_W {
        TBMIE_W { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer B Count Direction"]
    #[inline(always)]
    pub fn tbcdir(&mut self) -> TBCDIR_W {
        TBCDIR_W { w: self }
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline(always)]
    pub fn tbams(&mut self) -> TBAMS_W {
        TBAMS_W { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline(always)]
    pub fn tbcmir(&mut self) -> TBCMIR_W {
        TBCMIR_W { w: self }
    }
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline(always)]
    pub fn tbmr(&mut self) -> TBMR_W {
        TBMR_W { w: self }
    }
}
