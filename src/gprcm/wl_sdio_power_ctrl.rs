#[doc = "Reader of register WL_SDIO_POWER_CTRL"]
pub type R = crate::R<u32, super::WL_SDIO_POWER_CTRL>;
#[doc = "Writer for register WL_SDIO_POWER_CTRL"]
pub type W = crate::W<u32, super::WL_SDIO_POWER_CTRL>;
#[doc = "Register WL_SDIO_POWER_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::WL_SDIO_POWER_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WL_SDIO_PD_STATUS`"]
pub type WL_SDIO_PD_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WL_SDIO_PD_STATUS`"]
pub struct WL_SDIO_PD_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WL_SDIO_PD_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:2 - WL_SDIO_PD_STATUS"]
    #[inline(always)]
    pub fn wl_sdio_pd_status(&self) -> WL_SDIO_PD_STATUS_R {
        WL_SDIO_PD_STATUS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - WL_SDIO_PD_STATUS"]
    #[inline(always)]
    pub fn wl_sdio_pd_status(&mut self) -> WL_SDIO_PD_STATUS_W {
        WL_SDIO_PD_STATUS_W { w: self }
    }
}
