#[doc = "Reader of register MCU_PSCON_OVERRIDES"]
pub type R = crate::R<u32, super::MCU_PSCON_OVERRIDES>;
#[doc = "Writer for register MCU_PSCON_OVERRIDES"]
pub type W = crate::W<u32, super::MCU_PSCON_OVERRIDES>;
#[doc = "Register MCU_PSCON_OVERRIDES `reset()`'s with value 0"]
impl crate::ResetValue for super::MCU_PSCON_OVERRIDES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NU1`"]
pub type NU1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NU1`"]
pub struct NU1_W<'a> {
    w: &'a mut W,
}
impl<'a> NU1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `MEM_MCU_PSCON_MEM_OFF_OVERRIDE`"]
pub type MEM_MCU_PSCON_MEM_OFF_OVERRIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_MCU_PSCON_MEM_OFF_OVERRIDE`"]
pub struct MEM_MCU_PSCON_MEM_OFF_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MCU_PSCON_MEM_OFF_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MEM_MCU_PSCON_MEM_RETAIN_OVERRIDE`"]
pub type MEM_MCU_PSCON_MEM_RETAIN_OVERRIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_MCU_PSCON_MEM_RETAIN_OVERRIDE`"]
pub struct MEM_MCU_PSCON_MEM_RETAIN_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MCU_PSCON_MEM_RETAIN_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:15 - NU1"]
    #[inline(always)]
    pub fn nu1(&self) -> NU1_R {
        NU1_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 4:7 - MEM_MCU_PSCON_MEM_OFF_OVERRIDE"]
    #[inline(always)]
    pub fn mem_mcu_pscon_mem_off_override(&self) -> MEM_MCU_PSCON_MEM_OFF_OVERRIDE_R {
        MEM_MCU_PSCON_MEM_OFF_OVERRIDE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - MEM_MCU_PSCON_MEM_RETAIN_OVERRIDE"]
    #[inline(always)]
    pub fn mem_mcu_pscon_mem_retain_override(&self) -> MEM_MCU_PSCON_MEM_RETAIN_OVERRIDE_R {
        MEM_MCU_PSCON_MEM_RETAIN_OVERRIDE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 10:15 - NU1"]
    #[inline(always)]
    pub fn nu1(&mut self) -> NU1_W {
        NU1_W { w: self }
    }
    #[doc = "Bits 4:7 - MEM_MCU_PSCON_MEM_OFF_OVERRIDE"]
    #[inline(always)]
    pub fn mem_mcu_pscon_mem_off_override(&mut self) -> MEM_MCU_PSCON_MEM_OFF_OVERRIDE_W {
        MEM_MCU_PSCON_MEM_OFF_OVERRIDE_W { w: self }
    }
    #[doc = "Bits 0:3 - MEM_MCU_PSCON_MEM_RETAIN_OVERRIDE"]
    #[inline(always)]
    pub fn mem_mcu_pscon_mem_retain_override(&mut self) -> MEM_MCU_PSCON_MEM_RETAIN_OVERRIDE_W {
        MEM_MCU_PSCON_MEM_RETAIN_OVERRIDE_W { w: self }
    }
}
