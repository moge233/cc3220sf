#[doc = "Reader of register NWP_GPIO_WAKE_CONF"]
pub type R = crate::R<u32, super::NWP_GPIO_WAKE_CONF>;
#[doc = "Writer for register NWP_GPIO_WAKE_CONF"]
pub type W = crate::W<u32, super::NWP_GPIO_WAKE_CONF>;
#[doc = "Register NWP_GPIO_WAKE_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::NWP_GPIO_WAKE_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NWP_GPIO_WAKE_CONF`"]
pub type NWP_GPIO_WAKE_CONF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NWP_GPIO_WAKE_CONF`"]
pub struct NWP_GPIO_WAKE_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> NWP_GPIO_WAKE_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - &quot;00&quot; - Wakeup on level0 of the selected GPIO (GPIO gets selected inside HIB3P3-module); &quot;01&quot; - Wakeup on fall-edge of selected GPIO."]
    #[inline(always)]
    pub fn nwp_gpio_wake_conf(&self) -> NWP_GPIO_WAKE_CONF_R {
        NWP_GPIO_WAKE_CONF_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - &quot;00&quot; - Wakeup on level0 of the selected GPIO (GPIO gets selected inside HIB3P3-module); &quot;01&quot; - Wakeup on fall-edge of selected GPIO."]
    #[inline(always)]
    pub fn nwp_gpio_wake_conf(&mut self) -> NWP_GPIO_WAKE_CONF_W {
        NWP_GPIO_WAKE_CONF_W { w: self }
    }
}
