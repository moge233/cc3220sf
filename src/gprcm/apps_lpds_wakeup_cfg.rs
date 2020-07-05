#[doc = "Reader of register APPS_LPDS_WAKEUP_CFG"]
pub type R = crate::R<u32, super::APPS_LPDS_WAKEUP_CFG>;
#[doc = "Writer for register APPS_LPDS_WAKEUP_CFG"]
pub type W = crate::W<u32, super::APPS_LPDS_WAKEUP_CFG>;
#[doc = "Register APPS_LPDS_WAKEUP_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::APPS_LPDS_WAKEUP_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APPS_LPDS_WAKEUP_CFG`"]
pub type APPS_LPDS_WAKEUP_CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APPS_LPDS_WAKEUP_CFG`"]
pub struct APPS_LPDS_WAKEUP_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> APPS_LPDS_WAKEUP_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Mask for LPDS Wakeup interrupt : \\[7\\]
- Host IRQ from NWP \\[6\\]
- NWP_LPDS_Wake_irq (TRUE_LPDS) \\[5\\]
- NWP Wake-request to APPS \\[4\\]
- GPIO \\[3:1\\]
- Reserved \\[0\\]
- LPDS Wakeup-timer"]
    #[inline(always)]
    pub fn apps_lpds_wakeup_cfg(&self) -> APPS_LPDS_WAKEUP_CFG_R {
        APPS_LPDS_WAKEUP_CFG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Mask for LPDS Wakeup interrupt : \\[7\\]
- Host IRQ from NWP \\[6\\]
- NWP_LPDS_Wake_irq (TRUE_LPDS) \\[5\\]
- NWP Wake-request to APPS \\[4\\]
- GPIO \\[3:1\\]
- Reserved \\[0\\]
- LPDS Wakeup-timer"]
    #[inline(always)]
    pub fn apps_lpds_wakeup_cfg(&mut self) -> APPS_LPDS_WAKEUP_CFG_W {
        APPS_LPDS_WAKEUP_CFG_W { w: self }
    }
}
