#[doc = "Reader of register MEM_HIB_FSM_DEBUG"]
pub type R = crate::R<u32, super::MEM_HIB_FSM_DEBUG>;
#[doc = "Writer for register MEM_HIB_FSM_DEBUG"]
pub type W = crate::W<u32, super::MEM_HIB_FSM_DEBUG>;
#[doc = "Register MEM_HIB_FSM_DEBUG `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_HIB_FSM_DEBUG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRAM_PS`"]
pub type SRAM_PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRAM_PS`"]
pub struct SRAM_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `ANA_DCDC_PS`"]
pub type ANA_DCDC_PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ANA_DCDC_PS`"]
pub struct ANA_DCDC_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA_DCDC_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DIG_DCDC_PS`"]
pub type DIG_DCDC_PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIG_DCDC_PS`"]
pub struct DIG_DCDC_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_DCDC_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - SRAM_PS"]
    #[inline(always)]
    pub fn sram_ps(&self) -> SRAM_PS_R {
        SRAM_PS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - ANA_DCDC_PS"]
    #[inline(always)]
    pub fn ana_dcdc_ps(&self) -> ANA_DCDC_PS_R {
        ANA_DCDC_PS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - DIG_DCDC_PS"]
    #[inline(always)]
    pub fn dig_dcdc_ps(&self) -> DIG_DCDC_PS_R {
        DIG_DCDC_PS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - SRAM_PS"]
    #[inline(always)]
    pub fn sram_ps(&mut self) -> SRAM_PS_W {
        SRAM_PS_W { w: self }
    }
    #[doc = "Bits 4:7 - ANA_DCDC_PS"]
    #[inline(always)]
    pub fn ana_dcdc_ps(&mut self) -> ANA_DCDC_PS_W {
        ANA_DCDC_PS_W { w: self }
    }
    #[doc = "Bits 0:3 - DIG_DCDC_PS"]
    #[inline(always)]
    pub fn dig_dcdc_ps(&mut self) -> DIG_DCDC_PS_W {
        DIG_DCDC_PS_W { w: self }
    }
}
