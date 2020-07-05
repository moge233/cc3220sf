#[doc = "Reader of register SRAM_SKA_LDO_FSM_PARAMETERS"]
pub type R = crate::R<u32, super::SRAM_SKA_LDO_FSM_PARAMETERS>;
#[doc = "Writer for register SRAM_SKA_LDO_FSM_PARAMETERS"]
pub type W = crate::W<u32, super::SRAM_SKA_LDO_FSM_PARAMETERS>;
#[doc = "Register SRAM_SKA_LDO_FSM_PARAMETERS `reset()`'s with value 0"]
impl crate::ResetValue for super::SRAM_SKA_LDO_FSM_PARAMETERS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_SKA_LDO_EN_TO_SRAM_LDO_DIS`"]
pub type MEM_SKA_LDO_EN_TO_SRAM_LDO_DIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SKA_LDO_EN_TO_SRAM_LDO_DIS`"]
pub struct MEM_SKA_LDO_EN_TO_SRAM_LDO_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SKA_LDO_EN_TO_SRAM_LDO_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `MEM_SRAM_LDO_EN_TO_SKA_LDO_DIS`"]
pub type MEM_SRAM_LDO_EN_TO_SKA_LDO_DIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SRAM_LDO_EN_TO_SKA_LDO_DIS`"]
pub struct MEM_SRAM_LDO_EN_TO_SKA_LDO_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SRAM_LDO_EN_TO_SKA_LDO_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5 - MEM_SKA_LDO_EN_TO_SRAM_LDO_DIS"]
    #[inline(always)]
    pub fn mem_ska_ldo_en_to_sram_ldo_dis(&self) -> MEM_SKA_LDO_EN_TO_SRAM_LDO_DIS_R {
        MEM_SKA_LDO_EN_TO_SRAM_LDO_DIS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - MEM_SRAM_LDO_EN_TO_SKA_LDO_DIS"]
    #[inline(always)]
    pub fn mem_sram_ldo_en_to_ska_ldo_dis(&self) -> MEM_SRAM_LDO_EN_TO_SKA_LDO_DIS_R {
        MEM_SRAM_LDO_EN_TO_SKA_LDO_DIS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - MEM_SKA_LDO_EN_TO_SRAM_LDO_DIS"]
    #[inline(always)]
    pub fn mem_ska_ldo_en_to_sram_ldo_dis(&mut self) -> MEM_SKA_LDO_EN_TO_SRAM_LDO_DIS_W {
        MEM_SKA_LDO_EN_TO_SRAM_LDO_DIS_W { w: self }
    }
    #[doc = "Bits 0:2 - MEM_SRAM_LDO_EN_TO_SKA_LDO_DIS"]
    #[inline(always)]
    pub fn mem_sram_ldo_en_to_ska_ldo_dis(&mut self) -> MEM_SRAM_LDO_EN_TO_SKA_LDO_DIS_W {
        MEM_SRAM_LDO_EN_TO_SKA_LDO_DIS_W { w: self }
    }
}
