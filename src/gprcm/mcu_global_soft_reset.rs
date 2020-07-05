#[doc = "Reader of register MCU_GLOBAL_SOFT_RESET"]
pub type R = crate::R<u32, super::MCU_GLOBAL_SOFT_RESET>;
#[doc = "Writer for register MCU_GLOBAL_SOFT_RESET"]
pub type W = crate::W<u32, super::MCU_GLOBAL_SOFT_RESET>;
#[doc = "Register MCU_GLOBAL_SOFT_RESET `reset()`'s with value 0"]
impl crate::ResetValue for super::MCU_GLOBAL_SOFT_RESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
