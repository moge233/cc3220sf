#[doc = "Reader of register APPS_SH_RESOURCE_INTERRUPT_ENABLE"]
pub type R = crate::R<u32, super::APPS_SH_RESOURCE_INTERRUPT_ENABLE>;
#[doc = "Writer for register APPS_SH_RESOURCE_INTERRUPT_ENABLE"]
pub type W = crate::W<u32, super::APPS_SH_RESOURCE_INTERRUPT_ENABLE>;
#[doc = "Register APPS_SH_RESOURCE_INTERRUPT_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::APPS_SH_RESOURCE_INTERRUPT_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMON_REG_APPS_sh_resource_Interrupt_enable_APPS_sh_resource_Interrupt_enable`"]
pub type COMMON_REG_APPS_SH_RESOURCE_INTERRUPT_ENABLE_APPS_SH_RESOURCE_INTERRUPT_ENABLE_R =
    crate::R<u8, u8>;
#[doc = "Write proxy for field `COMMON_REG_APPS_sh_resource_Interrupt_enable_APPS_sh_resource_Interrupt_enable`"]
pub struct COMMON_REG_APPS_SH_RESOURCE_INTERRUPT_ENABLE_APPS_SH_RESOURCE_INTERRUPT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMON_REG_APPS_SH_RESOURCE_INTERRUPT_ENABLE_APPS_SH_RESOURCE_INTERRUPT_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Interrupt enable APPS bit 0 -> when '1' enable I2C interrupt bit 1 -> when '1' enable SPI interrupt bit 3 -> when '1' enable GPIO interrupt"]
    #[inline(always)]
    pub fn common_reg_apps_sh_resource_interrupt_enable_apps_sh_resource_interrupt_enable(
        &self,
    ) -> COMMON_REG_APPS_SH_RESOURCE_INTERRUPT_ENABLE_APPS_SH_RESOURCE_INTERRUPT_ENABLE_R {
        COMMON_REG_APPS_SH_RESOURCE_INTERRUPT_ENABLE_APPS_SH_RESOURCE_INTERRUPT_ENABLE_R::new(
            (self.bits & 0x0f) as u8,
        )
    }
}
impl W {
    #[doc = "Bits 0:3 - Interrupt enable APPS bit 0 -> when '1' enable I2C interrupt bit 1 -> when '1' enable SPI interrupt bit 3 -> when '1' enable GPIO interrupt"]
    #[inline(always)]
    pub fn common_reg_apps_sh_resource_interrupt_enable_apps_sh_resource_interrupt_enable(
        &mut self,
    ) -> COMMON_REG_APPS_SH_RESOURCE_INTERRUPT_ENABLE_APPS_SH_RESOURCE_INTERRUPT_ENABLE_W {
        COMMON_REG_APPS_SH_RESOURCE_INTERRUPT_ENABLE_APPS_SH_RESOURCE_INTERRUPT_ENABLE_W { w: self }
    }
}
