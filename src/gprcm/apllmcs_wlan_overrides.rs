#[doc = "Reader of register APLLMCS_WLAN_OVERRIDES"]
pub type R = crate::R<u32, super::APLLMCS_WLAN_OVERRIDES>;
#[doc = "Writer for register APLLMCS_WLAN_OVERRIDES"]
pub type W = crate::W<u32, super::APLLMCS_WLAN_OVERRIDES>;
#[doc = "Register APLLMCS_WLAN_OVERRIDES `reset()`'s with value 0"]
impl crate::ResetValue for super::APLLMCS_WLAN_OVERRIDES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APLLMCS_WLAN_POSTDIV_OVERRIDE`"]
pub type APLLMCS_WLAN_POSTDIV_OVERRIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_WLAN_POSTDIV_OVERRIDE`"]
pub struct APLLMCS_WLAN_POSTDIV_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_WLAN_POSTDIV_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `APLLMCS_WLAN_SPARE`"]
pub type APLLMCS_WLAN_SPARE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_WLAN_SPARE`"]
pub struct APLLMCS_WLAN_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_WLAN_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `APLLMCS_WLAN_N_7_8_OVERRIDE`"]
pub type APLLMCS_WLAN_N_7_8_OVERRIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_WLAN_N_7_8_OVERRIDE`"]
pub struct APLLMCS_WLAN_N_7_8_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_WLAN_N_7_8_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18 - APLLMCS_WLAN_POSTDIV_OVERRIDE"]
    #[inline(always)]
    pub fn apllmcs_wlan_postdiv_override(&self) -> APLLMCS_WLAN_POSTDIV_OVERRIDE_R {
        APLLMCS_WLAN_POSTDIV_OVERRIDE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - APLLMCS_WLAN_SPARE"]
    #[inline(always)]
    pub fn apllmcs_wlan_spare(&self) -> APLLMCS_WLAN_SPARE_R {
        APLLMCS_WLAN_SPARE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:1 - Override value for WLAN_APLLMCS_N\\[8:7\\]
bits. Applicable only when bit \\[1\\]
is set to 1. (Else controlled from WTOP)"]
    #[inline(always)]
    pub fn apllmcs_wlan_n_7_8_override(&self) -> APLLMCS_WLAN_N_7_8_OVERRIDE_R {
        APLLMCS_WLAN_N_7_8_OVERRIDE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - APLLMCS_WLAN_POSTDIV_OVERRIDE"]
    #[inline(always)]
    pub fn apllmcs_wlan_postdiv_override(&mut self) -> APLLMCS_WLAN_POSTDIV_OVERRIDE_W {
        APLLMCS_WLAN_POSTDIV_OVERRIDE_W { w: self }
    }
    #[doc = "Bits 8:10 - APLLMCS_WLAN_SPARE"]
    #[inline(always)]
    pub fn apllmcs_wlan_spare(&mut self) -> APLLMCS_WLAN_SPARE_W {
        APLLMCS_WLAN_SPARE_W { w: self }
    }
    #[doc = "Bits 0:1 - Override value for WLAN_APLLMCS_N\\[8:7\\]
bits. Applicable only when bit \\[1\\]
is set to 1. (Else controlled from WTOP)"]
    #[inline(always)]
    pub fn apllmcs_wlan_n_7_8_override(&mut self) -> APLLMCS_WLAN_N_7_8_OVERRIDE_W {
        APLLMCS_WLAN_N_7_8_OVERRIDE_W { w: self }
    }
}
