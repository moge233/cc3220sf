#[doc = "Reader of register APLLMCS_LOCK_TIME_CONF"]
pub type R = crate::R<u32, super::APLLMCS_LOCK_TIME_CONF>;
#[doc = "Writer for register APLLMCS_LOCK_TIME_CONF"]
pub type W = crate::W<u32, super::APLLMCS_LOCK_TIME_CONF>;
#[doc = "Register APLLMCS_LOCK_TIME_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::APLLMCS_LOCK_TIME_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_APLLMCS_WLAN_LOCK_TIME`"]
pub type MEM_APLLMCS_WLAN_LOCK_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_APLLMCS_WLAN_LOCK_TIME`"]
pub struct MEM_APLLMCS_WLAN_LOCK_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_APLLMCS_WLAN_LOCK_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `MEM_APLLMCS_MCU_LOCK_TIME`"]
pub type MEM_APLLMCS_MCU_LOCK_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_APLLMCS_MCU_LOCK_TIME`"]
pub struct MEM_APLLMCS_MCU_LOCK_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_APLLMCS_MCU_LOCK_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - MEM_APLLMCS_WLAN_LOCK_TIME"]
    #[inline(always)]
    pub fn mem_apllmcs_wlan_lock_time(&self) -> MEM_APLLMCS_WLAN_LOCK_TIME_R {
        MEM_APLLMCS_WLAN_LOCK_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - MEM_APLLMCS_MCU_LOCK_TIME"]
    #[inline(always)]
    pub fn mem_apllmcs_mcu_lock_time(&self) -> MEM_APLLMCS_MCU_LOCK_TIME_R {
        MEM_APLLMCS_MCU_LOCK_TIME_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - MEM_APLLMCS_WLAN_LOCK_TIME"]
    #[inline(always)]
    pub fn mem_apllmcs_wlan_lock_time(&mut self) -> MEM_APLLMCS_WLAN_LOCK_TIME_W {
        MEM_APLLMCS_WLAN_LOCK_TIME_W { w: self }
    }
    #[doc = "Bits 0:7 - MEM_APLLMCS_MCU_LOCK_TIME"]
    #[inline(always)]
    pub fn mem_apllmcs_mcu_lock_time(&mut self) -> MEM_APLLMCS_MCU_LOCK_TIME_W {
        MEM_APLLMCS_MCU_LOCK_TIME_W { w: self }
    }
}
