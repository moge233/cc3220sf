#[doc = "Reader of register APLLMCS_MCU_RUN_CONFIG0_38P4"]
pub type R = crate::R<u32, super::APLLMCS_MCU_RUN_CONFIG0_38P4>;
#[doc = "Writer for register APLLMCS_MCU_RUN_CONFIG0_38P4"]
pub type W = crate::W<u32, super::APLLMCS_MCU_RUN_CONFIG0_38P4>;
#[doc = "Register APLLMCS_MCU_RUN_CONFIG0_38P4 `reset()`'s with value 0"]
impl crate::ResetValue for super::APLLMCS_MCU_RUN_CONFIG0_38P4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APLLMCS_MCU_POSTDIV`"]
pub type APLLMCS_MCU_POSTDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_MCU_POSTDIV`"]
pub struct APLLMCS_MCU_POSTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_MCU_POSTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Reader of field `APLLMCS_MCU_SPARE`"]
pub type APLLMCS_MCU_SPARE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_MCU_SPARE`"]
pub struct APLLMCS_MCU_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_MCU_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `APLLMCS_MCU_RUN_N_38P4`"]
pub type APLLMCS_MCU_RUN_N_38P4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_MCU_RUN_N_38P4`"]
pub struct APLLMCS_MCU_RUN_N_38P4_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_MCU_RUN_N_38P4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `APLLMCS_MCU_RUN_M_38P4`"]
pub type APLLMCS_MCU_RUN_M_38P4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_MCU_RUN_M_38P4`"]
pub struct APLLMCS_MCU_RUN_M_38P4_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_MCU_RUN_M_38P4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `APLLMCS_MCU_RUN_N_7_8_38P4`"]
pub type APLLMCS_MCU_RUN_N_7_8_38P4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APLLMCS_MCU_RUN_N_7_8_38P4`"]
pub struct APLLMCS_MCU_RUN_N_7_8_38P4_W<'a> {
    w: &'a mut W,
}
impl<'a> APLLMCS_MCU_RUN_N_7_8_38P4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:29 - APLLMCS_MCU_POSTDIV"]
    #[inline(always)]
    pub fn apllmcs_mcu_postdiv(&self) -> APLLMCS_MCU_POSTDIV_R {
        APLLMCS_MCU_POSTDIV_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - APLLMCS_MCU_SPARE"]
    #[inline(always)]
    pub fn apllmcs_mcu_spare(&self) -> APLLMCS_MCU_SPARE_R {
        APLLMCS_MCU_SPARE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:22 - Configuration for MCU-APLLMCS : N during RUN mode. Selected if the XTAL frequency is 38.4 MHz (from Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_n_38p4(&self) -> APLLMCS_MCU_RUN_N_38P4_R {
        APLLMCS_MCU_RUN_N_38P4_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Configuration for MCU-APLLMCS : M during RUN mode. Selected if the XTAL frequency is 38.4 MHz (from Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_m_38p4(&self) -> APLLMCS_MCU_RUN_M_38P4_R {
        APLLMCS_MCU_RUN_M_38P4_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:1 - Configuration for MCU-APLLMCS : N\\[8:7\\]
during RUN mode. Selected if the XTAL frequency is 38.4 MHz (From Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_n_7_8_38p4(&self) -> APLLMCS_MCU_RUN_N_7_8_38P4_R {
        APLLMCS_MCU_RUN_N_7_8_38P4_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 27:29 - APLLMCS_MCU_POSTDIV"]
    #[inline(always)]
    pub fn apllmcs_mcu_postdiv(&mut self) -> APLLMCS_MCU_POSTDIV_W {
        APLLMCS_MCU_POSTDIV_W { w: self }
    }
    #[doc = "Bits 24:26 - APLLMCS_MCU_SPARE"]
    #[inline(always)]
    pub fn apllmcs_mcu_spare(&mut self) -> APLLMCS_MCU_SPARE_W {
        APLLMCS_MCU_SPARE_W { w: self }
    }
    #[doc = "Bits 16:22 - Configuration for MCU-APLLMCS : N during RUN mode. Selected if the XTAL frequency is 38.4 MHz (from Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_n_38p4(&mut self) -> APLLMCS_MCU_RUN_N_38P4_W {
        APLLMCS_MCU_RUN_N_38P4_W { w: self }
    }
    #[doc = "Bits 8:15 - Configuration for MCU-APLLMCS : M during RUN mode. Selected if the XTAL frequency is 38.4 MHz (from Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_m_38p4(&mut self) -> APLLMCS_MCU_RUN_M_38P4_W {
        APLLMCS_MCU_RUN_M_38P4_W { w: self }
    }
    #[doc = "Bits 0:1 - Configuration for MCU-APLLMCS : N\\[8:7\\]
during RUN mode. Selected if the XTAL frequency is 38.4 MHz (From Efuse)"]
    #[inline(always)]
    pub fn apllmcs_mcu_run_n_7_8_38p4(&mut self) -> APLLMCS_MCU_RUN_N_7_8_38P4_W {
        APLLMCS_MCU_RUN_N_7_8_38P4_W { w: self }
    }
}
