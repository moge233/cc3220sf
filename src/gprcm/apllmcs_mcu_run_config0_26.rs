#[doc = "Reader of register APLLMCS_MCU_RUN_CONFIG0_26"]
pub type R = crate::R<u32, super::APLLMCS_MCU_RUN_CONFIG0_26>;
#[doc = "Writer for register APLLMCS_MCU_RUN_CONFIG0_26"]
pub type W = crate::W<u32, super::APLLMCS_MCU_RUN_CONFIG0_26>;
#[doc = "Register APLLMCS_MCU_RUN_CONFIG0_26 `reset()`'s with value 0"]
impl crate::ResetValue for super::APLLMCS_MCU_RUN_CONFIG0_26 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APLLMCS_MCU_RUN_N_26`"]
pub type APLLMCS_MCU_RUN_N_26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_MCU_RUN_N_26`"]
pub struct APLLMCS_MCU_RUN_N_26_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_MCU_RUN_N_26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `APLLMCS_MCU_RUN_M_26`"]
pub type APLLMCS_MCU_RUN_M_26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_MCU_RUN_M_26`"]
pub struct APLLMCS_MCU_RUN_M_26_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_MCU_RUN_M_26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `APLLMCS_MCU_RUN_N_7_8_26`"]
pub type APLLMCS_MCU_RUN_N_7_8_26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_MCU_RUN_N_7_8_26`"]
pub struct APLLMCS_MCU_RUN_N_7_8_26_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_MCU_RUN_N_7_8_26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:22 - Configuration for MCU-APLLMCS : N during RUN mode. Selected if the XTAL frequency is 26 MHz (from Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_n_26(&self) -> APLLMCS_MCU_RUN_N_26_R {
        APLLMCS_MCU_RUN_N_26_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Configuration for MCU-APLLMCS : M during RUN mode. Selected if the XTAL frequency is 26 MHz (from Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_m_26(&self) -> APLLMCS_MCU_RUN_M_26_R {
        APLLMCS_MCU_RUN_M_26_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:1 - Configuration for MCU-APLLMCS : N\\[8:7\\]
during RUN mode. Selected if the XTAL frequency is 26 MHz (From Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_n_7_8_26(&self) -> APLLMCS_MCU_RUN_N_7_8_26_R {
        APLLMCS_MCU_RUN_N_7_8_26_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:22 - Configuration for MCU-APLLMCS : N during RUN mode. Selected if the XTAL frequency is 26 MHz (from Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_n_26(&mut self) -> APLLMCS_MCU_RUN_N_26_W {
        APLLMCS_MCU_RUN_N_26_W { w: self }
    }
    #[doc = "Bits 8:15 - Configuration for MCU-APLLMCS : M during RUN mode. Selected if the XTAL frequency is 26 MHz (from Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_m_26(&mut self) -> APLLMCS_MCU_RUN_M_26_W {
        APLLMCS_MCU_RUN_M_26_W { w: self }
    }
    #[doc = "Bits 0:1 - Configuration for MCU-APLLMCS : N\\[8:7\\]
during RUN mode. Selected if the XTAL frequency is 26 MHz (From Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_n_7_8_26(&mut self) -> APLLMCS_MCU_RUN_N_7_8_26_W {
        APLLMCS_MCU_RUN_N_7_8_26_W { w: self }
    }
}
