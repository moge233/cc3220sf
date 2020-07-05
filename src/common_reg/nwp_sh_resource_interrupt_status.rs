#[doc = "Reader of register NWP_SH_RESOURCE_INTERRUPT_STATUS"]
pub type R = crate::R<u32, super::NWP_SH_RESOURCE_INTERRUPT_STATUS>;
#[doc = "Writer for register NWP_SH_RESOURCE_INTERRUPT_STATUS"]
pub type W = crate::W<u32, super::NWP_SH_RESOURCE_INTERRUPT_STATUS>;
#[doc = "Register NWP_SH_RESOURCE_INTERRUPT_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::NWP_SH_RESOURCE_INTERRUPT_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMON_REG_NWP_sh_resource_Interrupt_status_NWP_sh_resource_Interrupt_status`"]
pub type COMMON_REG_NWP_SH_RESOURCE_INTERRUPT_STATUS_NWP_SH_RESOURCE_INTERRUPT_STATUS_R =
    crate::R<u8, u8>;
#[doc = "Write proxy for field `COMMON_REG_NWP_sh_resource_Interrupt_status_NWP_sh_resource_Interrupt_status`"]
pub struct COMMON_REG_NWP_SH_RESOURCE_INTERRUPT_STATUS_NWP_SH_RESOURCE_INTERRUPT_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMON_REG_NWP_SH_RESOURCE_INTERRUPT_STATUS_NWP_SH_RESOURCE_INTERRUPT_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Interrupt enable NWP bit 0 -> when '1' enable I2C interrupt bit 1 -> when '1' enable SPI interrupt bit 3 -> when '1' enable GPIO interrupt"]
    #[inline(always)]
    pub fn common_reg_nwp_sh_resource_interrupt_status_nwp_sh_resource_interrupt_status(
        &self,
    ) -> COMMON_REG_NWP_SH_RESOURCE_INTERRUPT_STATUS_NWP_SH_RESOURCE_INTERRUPT_STATUS_R {
        COMMON_REG_NWP_SH_RESOURCE_INTERRUPT_STATUS_NWP_SH_RESOURCE_INTERRUPT_STATUS_R::new(
            (self.bits & 0x0f) as u8,
        )
    }
}
impl W {
    #[doc = "Bits 0:3 - Interrupt enable NWP bit 0 -> when '1' enable I2C interrupt bit 1 -> when '1' enable SPI interrupt bit 3 -> when '1' enable GPIO interrupt"]
    #[inline(always)]
    pub fn common_reg_nwp_sh_resource_interrupt_status_nwp_sh_resource_interrupt_status(
        &mut self,
    ) -> COMMON_REG_NWP_SH_RESOURCE_INTERRUPT_STATUS_NWP_SH_RESOURCE_INTERRUPT_STATUS_W {
        COMMON_REG_NWP_SH_RESOURCE_INTERRUPT_STATUS_NWP_SH_RESOURCE_INTERRUPT_STATUS_W { w: self }
    }
}
