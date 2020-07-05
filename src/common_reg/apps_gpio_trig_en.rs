#[doc = "Reader of register APPS_GPIO_TRIG_EN"]
pub type R = crate::R<u32, super::APPS_GPIO_TRIG_EN>;
#[doc = "Writer for register APPS_GPIO_TRIG_EN"]
pub type W = crate::W<u32, super::APPS_GPIO_TRIG_EN>;
#[doc = "Register APPS_GPIO_TRIG_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::APPS_GPIO_TRIG_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APPS_GPIO_TRIG_EN`"]
pub type APPS_GPIO_TRIG_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APPS_GPIO_TRIG_EN`"]
pub struct APPS_GPIO_TRIG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APPS_GPIO_TRIG_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - APPS GPIO Trigger EN control. Bit 0: when '1' enable GPIO 0 trigger. This bit enables trigger for all GPIO 0 pins (GPIO 0 to GPIO7). Bit 1: when '1' enable GPIO 1 trigger. This bit enables trigger for all GPIO 1 pins ( GPIO8 to GPIO15). Bit 2: when '1' enable GPIO 2 trigger. This bit enables trigger for all GPIO 2 pins (GPIO16 to GPIO23). Bit 3: when '1' enable GPIO 3 trigger. This bit enables trigger for all GPIO 3 pins (GPIO24 to GPIO31). Bit 4: when '1' enable GPIO 4 trigger. This bit enables trigger for all GPIO 4 pins.(GPIO32 to GPIO39)"]
    #[inline(always)]
    pub fn apps_gpio_trig_en(&self) -> APPS_GPIO_TRIG_EN_R {
        APPS_GPIO_TRIG_EN_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - APPS GPIO Trigger EN control. Bit 0: when '1' enable GPIO 0 trigger. This bit enables trigger for all GPIO 0 pins (GPIO 0 to GPIO7). Bit 1: when '1' enable GPIO 1 trigger. This bit enables trigger for all GPIO 1 pins ( GPIO8 to GPIO15). Bit 2: when '1' enable GPIO 2 trigger. This bit enables trigger for all GPIO 2 pins (GPIO16 to GPIO23). Bit 3: when '1' enable GPIO 3 trigger. This bit enables trigger for all GPIO 3 pins (GPIO24 to GPIO31). Bit 4: when '1' enable GPIO 4 trigger. This bit enables trigger for all GPIO 4 pins.(GPIO32 to GPIO39)"]
    #[inline(always)]
    pub fn apps_gpio_trig_en(&mut self) -> APPS_GPIO_TRIG_EN_W {
        APPS_GPIO_TRIG_EN_W { w: self }
    }
}
