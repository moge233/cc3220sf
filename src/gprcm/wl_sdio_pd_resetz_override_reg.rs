#[doc = "Reader of register WL_SDIO_PD_RESETZ_OVERRIDE_REG"]
pub type R = crate::R<u32, super::WL_SDIO_PD_RESETZ_OVERRIDE_REG>;
#[doc = "Writer for register WL_SDIO_PD_RESETZ_OVERRIDE_REG"]
pub type W = crate::W<u32, super::WL_SDIO_PD_RESETZ_OVERRIDE_REG>;
#[doc = "Register WL_SDIO_PD_RESETZ_OVERRIDE_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::WL_SDIO_PD_RESETZ_OVERRIDE_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
