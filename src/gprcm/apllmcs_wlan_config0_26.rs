#[doc = "Reader of register APLLMCS_WLAN_CONFIG0_26"]
pub type R = crate::R<u32, super::APLLMCS_WLAN_CONFIG0_26>;
#[doc = "Writer for register APLLMCS_WLAN_CONFIG0_26"]
pub type W = crate::W<u32, super::APLLMCS_WLAN_CONFIG0_26>;
#[doc = "Register APLLMCS_WLAN_CONFIG0_26 `reset()`'s with value 0"]
impl crate::ResetValue for super::APLLMCS_WLAN_CONFIG0_26 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APLLMCS_WLAN_N_26`"]
pub type APLLMCS_WLAN_N_26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_WLAN_N_26`"]
pub struct APLLMCS_WLAN_N_26_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_WLAN_N_26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `APLLMCS_WLAN_M_26`"]
pub type APLLMCS_WLAN_M_26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_WLAN_M_26`"]
pub struct APLLMCS_WLAN_M_26_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_WLAN_M_26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:14 - Configuration for WLAN APLLMCS - N\\[6:0\\], if the XTAL frequency is 26 MHz (Selected by efuse)"]
    #[inline(always)]
    pub fn apllmcs_wlan_n_26(&self) -> APLLMCS_WLAN_N_26_R {
        APLLMCS_WLAN_N_26_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:7 - Configuration for WLAN APLLMCS - M\\[7:0\\], if the XTAL frequency is 26 MHz (Selected by efuse)"]
    #[inline(always)]
    pub fn apllmcs_wlan_m_26(&self) -> APLLMCS_WLAN_M_26_R {
        APLLMCS_WLAN_M_26_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - Configuration for WLAN APLLMCS - N\\[6:0\\], if the XTAL frequency is 26 MHz (Selected by efuse)"]
    #[inline(always)]
    pub fn apllmcs_wlan_n_26(&mut self) -> APLLMCS_WLAN_N_26_W {
        APLLMCS_WLAN_N_26_W { w: self }
    }
    #[doc = "Bits 0:7 - Configuration for WLAN APLLMCS - M\\[7:0\\], if the XTAL frequency is 26 MHz (Selected by efuse)"]
    #[inline(always)]
    pub fn apllmcs_wlan_m_26(&mut self) -> APLLMCS_WLAN_M_26_W {
        APLLMCS_WLAN_M_26_W { w: self }
    }
}
