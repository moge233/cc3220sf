#[doc = "Reader of register APLLMCS_WLAN_CONFIG1_40"]
pub type R = crate::R<u32, super::APLLMCS_WLAN_CONFIG1_40>;
#[doc = "Writer for register APLLMCS_WLAN_CONFIG1_40"]
pub type W = crate::W<u32, super::APLLMCS_WLAN_CONFIG1_40>;
#[doc = "Register APLLMCS_WLAN_CONFIG1_40 `reset()`'s with value 0"]
impl crate::ResetValue for super::APLLMCS_WLAN_CONFIG1_40 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APLLMCS_SELINPFREQ_40`"]
pub type APLLMCS_SELINPFREQ_40_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_SELINPFREQ_40`"]
pub struct APLLMCS_SELINPFREQ_40_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_SELINPFREQ_40_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Configuration for WLAN APLLMCS - Selinpfreq, if the XTAL frequency is 40 MHz (Selected by Efuse)"]
    #[inline(always)]
    pub fn apllmcs_selinpfreq_40(&self) -> APLLMCS_SELINPFREQ_40_R {
        APLLMCS_SELINPFREQ_40_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configuration for WLAN APLLMCS - Selinpfreq, if the XTAL frequency is 40 MHz (Selected by Efuse)"]
    #[inline(always)]
    pub fn apllmcs_selinpfreq_40(&mut self) -> APLLMCS_SELINPFREQ_40_W {
        APLLMCS_SELINPFREQ_40_W { w: self }
    }
}
