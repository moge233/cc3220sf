#[doc = "Reader of register SLPWAKEEN"]
pub type R = crate::R<u32, super::SLPWAKEEN>;
#[doc = "Writer for register SLPWAKEEN"]
pub type W = crate::W<u32, super::SLPWAKEEN>;
#[doc = "Register SLPWAKEEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SLPWAKEEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EITBYNWP`"]
pub type EITBYNWP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EITBYNWP`"]
pub struct EITBYNWP_W<'a> {
    w: &'a mut W,
}
impl<'a> EITBYNWP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EXITBYTIMR`"]
pub type EXITBYTIMR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXITBYTIMR`"]
pub struct EXITBYTIMR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXITBYTIMR_W<'a> {
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
    #[doc = "Bit 1 - SLP_WAKE_FROM_NWP_ENABLE"]
    #[inline(always)]
    pub fn eitbynwp(&self) -> EITBYNWP_R {
        EITBYNWP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SLP_WAKE_TIMER_ENABLE"]
    #[inline(always)]
    pub fn exitbytimr(&self) -> EXITBYTIMR_R {
        EXITBYTIMR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SLP_WAKE_FROM_NWP_ENABLE"]
    #[inline(always)]
    pub fn eitbynwp(&mut self) -> EITBYNWP_W {
        EITBYNWP_W { w: self }
    }
    #[doc = "Bit 0 - SLP_WAKE_TIMER_ENABLE"]
    #[inline(always)]
    pub fn exitbytimr(&mut self) -> EXITBYTIMR_W {
        EXITBYTIMR_W { w: self }
    }
}
