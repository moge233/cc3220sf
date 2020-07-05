#[doc = "Reader of register MEM_GPIO_WAKE_CONF"]
pub type R = crate::R<u32, super::MEM_GPIO_WAKE_CONF>;
#[doc = "Writer for register MEM_GPIO_WAKE_CONF"]
pub type W = crate::W<u32, super::MEM_GPIO_WAKE_CONF>;
#[doc = "Register MEM_GPIO_WAKE_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_GPIO_WAKE_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_GPIO_WAKE_CONF`"]
pub type MEM_GPIO_WAKE_CONF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_GPIO_WAKE_CONF`"]
pub struct MEM_GPIO_WAKE_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_GPIO_WAKE_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Configuration to say whether the GPIO wakeup has to happen on Level0 or falling-edge for the given group. 00? Level0 01? Level1 10?- Fall-edge 11?- Rise-edge \\[1:0\\]
Conf for GPIO0 \\[3:2\\]
Conf for GPIO1 \\[5:4\\]
Conf for GPIO2 \\[7:6\\]
Conf for GPIO3 \\[9:8\\]
Conf for GPIO4 \\[11:10\\]
Conf for GPIO5 \\[13:12\\]
Conf for GPIO6"]
    #[inline(always)]
    pub fn mem_gpio_wake_conf(&self) -> MEM_GPIO_WAKE_CONF_R {
        MEM_GPIO_WAKE_CONF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configuration to say whether the GPIO wakeup has to happen on Level0 or falling-edge for the given group. 00? Level0 01? Level1 10?- Fall-edge 11?- Rise-edge \\[1:0\\]
Conf for GPIO0 \\[3:2\\]
Conf for GPIO1 \\[5:4\\]
Conf for GPIO2 \\[7:6\\]
Conf for GPIO3 \\[9:8\\]
Conf for GPIO4 \\[11:10\\]
Conf for GPIO5 \\[13:12\\]
Conf for GPIO6"]
    #[inline(always)]
    pub fn mem_gpio_wake_conf(&mut self) -> MEM_GPIO_WAKE_CONF_W {
        MEM_GPIO_WAKE_CONF_W { w: self }
    }
}
