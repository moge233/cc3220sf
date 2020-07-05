#[doc = "Reader of register MEM_HIB_LPDS_GPIO_SEL"]
pub type R = crate::R<u32, super::MEM_HIB_LPDS_GPIO_SEL>;
#[doc = "Writer for register MEM_HIB_LPDS_GPIO_SEL"]
pub type W = crate::W<u32, super::MEM_HIB_LPDS_GPIO_SEL>;
#[doc = "Register MEM_HIB_LPDS_GPIO_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_HIB_LPDS_GPIO_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HIB_LPDS_GPIO_SEL`"]
pub type HIB_LPDS_GPIO_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HIB_LPDS_GPIO_SEL`"]
pub struct HIB_LPDS_GPIO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_LPDS_GPIO_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - HIB_LPDS_GPIO_SEL"]
    #[inline(always)]
    pub fn hib_lpds_gpio_sel(&self) -> HIB_LPDS_GPIO_SEL_R {
        HIB_LPDS_GPIO_SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HIB_LPDS_GPIO_SEL"]
    #[inline(always)]
    pub fn hib_lpds_gpio_sel(&mut self) -> HIB_LPDS_GPIO_SEL_W {
        HIB_LPDS_GPIO_SEL_W { w: self }
    }
}
