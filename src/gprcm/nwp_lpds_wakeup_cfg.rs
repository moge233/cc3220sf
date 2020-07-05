#[doc = "Reader of register NWP_LPDS_WAKEUP_CFG"]
pub type R = crate::R<u32, super::NWP_LPDS_WAKEUP_CFG>;
#[doc = "Writer for register NWP_LPDS_WAKEUP_CFG"]
pub type W = crate::W<u32, super::NWP_LPDS_WAKEUP_CFG>;
#[doc = "Register NWP_LPDS_WAKEUP_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::NWP_LPDS_WAKEUP_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NWP_LPDS_WAKEUP_CFG`"]
pub type NWP_LPDS_WAKEUP_CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NWP_LPDS_WAKEUP_CFG`"]
pub struct NWP_LPDS_WAKEUP_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> NWP_LPDS_WAKEUP_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Mask for LPDS Wakeup interrupt : 7 - WLAN Host Interrupt ; 6 - WLAN to NWP Wake request ; 5 - APPS to NWP Wake request; 4 - GPIO Wakeup ; 3 - Autonomous UART Wakeup ; 2 - SSDIO Wakeup ; 1 - Autonomous SPI Wakeup ; 0 - LPDS Wakeup-timer"]
    #[inline(always)]
    pub fn nwp_lpds_wakeup_cfg(&self) -> NWP_LPDS_WAKEUP_CFG_R {
        NWP_LPDS_WAKEUP_CFG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mask for LPDS Wakeup interrupt : 7 - WLAN Host Interrupt ; 6 - WLAN to NWP Wake request ; 5 - APPS to NWP Wake request; 4 - GPIO Wakeup ; 3 - Autonomous UART Wakeup ; 2 - SSDIO Wakeup ; 1 - Autonomous SPI Wakeup ; 0 - LPDS Wakeup-timer"]
    #[inline(always)]
    pub fn nwp_lpds_wakeup_cfg(&mut self) -> NWP_LPDS_WAKEUP_CFG_W {
        NWP_LPDS_WAKEUP_CFG_W { w: self }
    }
}
