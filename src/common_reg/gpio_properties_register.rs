#[doc = "Reader of register GPIO_PROPERTIES_REGISTER"]
pub type R = crate::R<u32, super::GPIO_PROPERTIES_REGISTER>;
#[doc = "Writer for register GPIO_PROPERTIES_REGISTER"]
pub type W = crate::W<u32, super::GPIO_PROPERTIES_REGISTER>;
#[doc = "Register GPIO_PROPERTIES_REGISTER `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_PROPERTIES_REGISTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMON_REG_GPIO_properties_register_GPIO_properties_register`"]
pub type COMMON_REG_GPIO_PROPERTIES_REGISTER_GPIO_PROPERTIES_REGISTER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMMON_REG_GPIO_properties_register_GPIO_properties_register`"]
pub struct COMMON_REG_GPIO_PROPERTIES_REGISTER_GPIO_PROPERTIES_REGISTER_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMON_REG_GPIO_PROPERTIES_REGISTER_GPIO_PROPERTIES_REGISTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Shared GPIO configuration register. Bit \\[1:0\\]
to configure GPIO0 Bit \\[3:2\\]
to configure GPIO1 Bit \\[5:4\\]
to configure GPIO2 Bit \\[7:6\\]
to configure GPIO3 Bit \\[9:8\\]
to configure GPIO4 each GPIO can be individully selected. When 00? GPIO is free resource. When 01? GPIO is APPS resource. When 10? GPIO is NWP resource. Writing 11 doesnt have any affect, i.e. If one write only relevant gpio semaphore and other bits are 1s, it'll not disturb the other semaphore bits. For example : Say If NW wants to take control of gpio-1, one should write 10'b11_1111_1011 and if one wants to release it write 10'b11_1111_0011."]
    #[inline(always)]
    pub fn common_reg_gpio_properties_register_gpio_properties_register(
        &self,
    ) -> COMMON_REG_GPIO_PROPERTIES_REGISTER_GPIO_PROPERTIES_REGISTER_R {
        COMMON_REG_GPIO_PROPERTIES_REGISTER_GPIO_PROPERTIES_REGISTER_R::new(
            (self.bits & 0x03ff) as u16,
        )
    }
}
impl W {
    #[doc = "Bits 0:9 - Shared GPIO configuration register. Bit \\[1:0\\]
to configure GPIO0 Bit \\[3:2\\]
to configure GPIO1 Bit \\[5:4\\]
to configure GPIO2 Bit \\[7:6\\]
to configure GPIO3 Bit \\[9:8\\]
to configure GPIO4 each GPIO can be individully selected. When 00? GPIO is free resource. When 01? GPIO is APPS resource. When 10? GPIO is NWP resource. Writing 11 doesnt have any affect, i.e. If one write only relevant gpio semaphore and other bits are 1s, it'll not disturb the other semaphore bits. For example : Say If NW wants to take control of gpio-1, one should write 10'b11_1111_1011 and if one wants to release it write 10'b11_1111_0011."]
    #[inline(always)]
    pub fn common_reg_gpio_properties_register_gpio_properties_register(
        &mut self,
    ) -> COMMON_REG_GPIO_PROPERTIES_REGISTER_GPIO_PROPERTIES_REGISTER_W {
        COMMON_REG_GPIO_PROPERTIES_REGISTER_GPIO_PROPERTIES_REGISTER_W { w: self }
    }
}
