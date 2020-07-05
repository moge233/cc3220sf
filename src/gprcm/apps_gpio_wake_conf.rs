#[doc = "Reader of register APPS_GPIO_WAKE_CONF"]
pub type R = crate::R<u32, super::APPS_GPIO_WAKE_CONF>;
#[doc = "Writer for register APPS_GPIO_WAKE_CONF"]
pub type W = crate::W<u32, super::APPS_GPIO_WAKE_CONF>;
#[doc = "Register APPS_GPIO_WAKE_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::APPS_GPIO_WAKE_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APPS_GPIO_WAKE_CONF`"]
pub type APPS_GPIO_WAKE_CONF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APPS_GPIO_WAKE_CONF`"]
pub struct APPS_GPIO_WAKE_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> APPS_GPIO_WAKE_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - &quot;00&quot; - Wake on Level0 on selected GPIO pin (GPIO is selected inside the HIB3p3 module); &quot;01&quot; - Wakeup on fall-edge of GPIO pin."]
    #[inline(always)]
    pub fn apps_gpio_wake_conf(&self) -> APPS_GPIO_WAKE_CONF_R {
        APPS_GPIO_WAKE_CONF_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - &quot;00&quot; - Wake on Level0 on selected GPIO pin (GPIO is selected inside the HIB3p3 module); &quot;01&quot; - Wakeup on fall-edge of GPIO pin."]
    #[inline(always)]
    pub fn apps_gpio_wake_conf(&mut self) -> APPS_GPIO_WAKE_CONF_W {
        APPS_GPIO_WAKE_CONF_W { w: self }
    }
}
