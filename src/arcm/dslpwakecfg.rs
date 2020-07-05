#[doc = "Reader of register DSLPWAKECFG"]
pub type R = crate::R<u32, super::DSLPWAKECFG>;
#[doc = "Writer for register DSLPWAKECFG"]
pub type W = crate::W<u32, super::DSLPWAKECFG>;
#[doc = "Register DSLPWAKECFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DSLPWAKECFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXITDSLPBYNWPEN`"]
pub type EXITDSLPBYNWPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXITDSLPBYNWPEN`"]
pub struct EXITDSLPBYNWPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXITDSLPBYNWPEN_W<'a> {
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
#[doc = "Reader of field `EXITDSLPBYTMREN`"]
pub type EXITDSLPBYTMREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXITDSLPBYTMREN`"]
pub struct EXITDSLPBYTMREN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXITDSLPBYTMREN_W<'a> {
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
    #[doc = "Bit 1 - DSLP_WAKE_FROM_NWP_ENABLE"]
    #[inline(always)]
    pub fn exitdslpbynwpen(&self) -> EXITDSLPBYNWPEN_R {
        EXITDSLPBYNWPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DLSP_WAKE_FROM_TIMER_ENABLE"]
    #[inline(always)]
    pub fn exitdslpbytmren(&self) -> EXITDSLPBYTMREN_R {
        EXITDSLPBYTMREN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DSLP_WAKE_FROM_NWP_ENABLE"]
    #[inline(always)]
    pub fn exitdslpbynwpen(&mut self) -> EXITDSLPBYNWPEN_W {
        EXITDSLPBYNWPEN_W { w: self }
    }
    #[doc = "Bit 0 - DLSP_WAKE_FROM_TIMER_ENABLE"]
    #[inline(always)]
    pub fn exitdslpbytmren(&mut self) -> EXITDSLPBYTMREN_W {
        EXITDSLPBYTMREN_W { w: self }
    }
}
