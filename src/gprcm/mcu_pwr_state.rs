#[doc = "Reader of register MCU_PWR_STATE"]
pub type R = crate::R<u32, super::MCU_PWR_STATE>;
#[doc = "Writer for register MCU_PWR_STATE"]
pub type W = crate::W<u32, super::MCU_PWR_STATE>;
#[doc = "Register MCU_PWR_STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::MCU_PWR_STATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCU_OPP_PS`"]
pub type MCU_OPP_PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCU_OPP_PS`"]
pub struct MCU_OPP_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_OPP_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - TBD"]
    #[inline(always)]
    pub fn mcu_opp_ps(&self) -> MCU_OPP_PS_R {
        MCU_OPP_PS_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - TBD"]
    #[inline(always)]
    pub fn mcu_opp_ps(&mut self) -> MCU_OPP_PS_W {
        MCU_OPP_PS_W { w: self }
    }
}
