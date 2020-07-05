#[doc = "Reader of register APLLMCS_MCU_RUN_CONFIG1_38P4"]
pub type R = crate::R<u32, super::APLLMCS_MCU_RUN_CONFIG1_38P4>;
#[doc = "Writer for register APLLMCS_MCU_RUN_CONFIG1_38P4"]
pub type W = crate::W<u32, super::APLLMCS_MCU_RUN_CONFIG1_38P4>;
#[doc = "Register APLLMCS_MCU_RUN_CONFIG1_38P4 `reset()`'s with value 0"]
impl crate::ResetValue for super::APLLMCS_MCU_RUN_CONFIG1_38P4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APLLMCS_MCU_RUN_SELINPFREQ_38P4`"]
pub type APLLMCS_MCU_RUN_SELINPFREQ_38P4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_MCU_RUN_SELINPFREQ_38P4`"]
pub struct APLLMCS_MCU_RUN_SELINPFREQ_38P4_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_MCU_RUN_SELINPFREQ_38P4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Configuration for MCU-APLLMCS : SELINPFREQ during RUN mode. Selected if the XTAL frequency is 38.4 MHz (from Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_selinpfreq_38p4(&self) -> APLLMCS_MCU_RUN_SELINPFREQ_38P4_R {
        APLLMCS_MCU_RUN_SELINPFREQ_38P4_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configuration for MCU-APLLMCS : SELINPFREQ during RUN mode. Selected if the XTAL frequency is 38.4 MHz (from Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_selinpfreq_38p4(&mut self) -> APLLMCS_MCU_RUN_SELINPFREQ_38P4_W {
        APLLMCS_MCU_RUN_SELINPFREQ_38P4_W { w: self }
    }
}
