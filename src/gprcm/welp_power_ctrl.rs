#[doc = "Reader of register WELP_POWER_CTRL"]
pub type R = crate::R<u32, super::WELP_POWER_CTRL>;
#[doc = "Writer for register WELP_POWER_CTRL"]
pub type W = crate::W<u32, super::WELP_POWER_CTRL>;
#[doc = "Register WELP_POWER_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::WELP_POWER_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTOP_PD_STATUS`"]
pub type WTOP_PD_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WTOP_PD_STATUS`"]
pub struct WTOP_PD_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WTOP_PD_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `WELP_PD_STATUS`"]
pub type WELP_PD_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WELP_PD_STATUS`"]
pub struct WELP_PD_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WELP_PD_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:12 - WTOP_PD_STATUS"]
    #[inline(always)]
    pub fn wtop_pd_status(&self) -> WTOP_PD_STATUS_R {
        WTOP_PD_STATUS_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 1:2 - WELP_PD_STATUS"]
    #[inline(always)]
    pub fn welp_pd_status(&self) -> WELP_PD_STATUS_R {
        WELP_PD_STATUS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 10:12 - WTOP_PD_STATUS"]
    #[inline(always)]
    pub fn wtop_pd_status(&mut self) -> WTOP_PD_STATUS_W {
        WTOP_PD_STATUS_W { w: self }
    }
    #[doc = "Bits 1:2 - WELP_PD_STATUS"]
    #[inline(always)]
    pub fn welp_pd_status(&mut self) -> WELP_PD_STATUS_W {
        WELP_PD_STATUS_W { w: self }
    }
}
