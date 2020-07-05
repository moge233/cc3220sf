#[doc = "Reader of register MEM_GPIO_WAKE_EN"]
pub type R = crate::R<u32, super::MEM_GPIO_WAKE_EN>;
#[doc = "Writer for register MEM_GPIO_WAKE_EN"]
pub type W = crate::W<u32, super::MEM_GPIO_WAKE_EN>;
#[doc = "Register MEM_GPIO_WAKE_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_GPIO_WAKE_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_GPIO_WAKE_EN`"]
pub type MEM_GPIO_WAKE_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_GPIO_WAKE_EN`"]
pub struct MEM_GPIO_WAKE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_GPIO_WAKE_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 1 - Enable the GPIO-Autonomous mode wakeup during Hibernate mode ; This is an auto-clear bit, once programmed to 1, it will latched into an internal register which remain asserted until the Hib-wakeup is initiated."]
    #[inline(always)]
    pub fn mem_gpio_wake_en(&self) -> MEM_GPIO_WAKE_EN_R {
        MEM_GPIO_WAKE_EN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 1 - Enable the GPIO-Autonomous mode wakeup during Hibernate mode ; This is an auto-clear bit, once programmed to 1, it will latched into an internal register which remain asserted until the Hib-wakeup is initiated."]
    #[inline(always)]
    pub fn mem_gpio_wake_en(&mut self) -> MEM_GPIO_WAKE_EN_W {
        MEM_GPIO_WAKE_EN_W { w: self }
    }
}
