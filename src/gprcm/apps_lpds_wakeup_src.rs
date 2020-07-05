#[doc = "Reader of register APPS_LPDS_WAKEUP_SRC"]
pub type R = crate::R<u32, super::APPS_LPDS_WAKEUP_SRC>;
#[doc = "Writer for register APPS_LPDS_WAKEUP_SRC"]
pub type W = crate::W<u32, super::APPS_LPDS_WAKEUP_SRC>;
#[doc = "Register APPS_LPDS_WAKEUP_SRC `reset()`'s with value 0"]
impl crate::ResetValue for super::APPS_LPDS_WAKEUP_SRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APPS_LPDS_WAKEUP_SRC`"]
pub type APPS_LPDS_WAKEUP_SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APPS_LPDS_WAKEUP_SRC`"]
pub struct APPS_LPDS_WAKEUP_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> APPS_LPDS_WAKEUP_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Indicates the cause for wakeup from LPDS : \\[7\\]
- Host IRQ from NWP \\[6\\]
- NWP_LPDS_Wake_irq (TRUE_LPDS) \\[5\\]
- NWP Wake-request to APPS \\[4\\]
- GPIO \\[3:1\\]
- Reserved \\[0\\]
- LPDS Wakeup-timer"]
    #[inline(always)]
    pub fn apps_lpds_wakeup_src(&self) -> APPS_LPDS_WAKEUP_SRC_R {
        APPS_LPDS_WAKEUP_SRC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates the cause for wakeup from LPDS : \\[7\\]
- Host IRQ from NWP \\[6\\]
- NWP_LPDS_Wake_irq (TRUE_LPDS) \\[5\\]
- NWP Wake-request to APPS \\[4\\]
- GPIO \\[3:1\\]
- Reserved \\[0\\]
- LPDS Wakeup-timer"]
    #[inline(always)]
    pub fn apps_lpds_wakeup_src(&mut self) -> APPS_LPDS_WAKEUP_SRC_W {
        APPS_LPDS_WAKEUP_SRC_W { w: self }
    }
}
