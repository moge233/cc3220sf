#[doc = "Reader of register RESET_CAUSE"]
pub type R = crate::R<u32, super::RESET_CAUSE>;
#[doc = "Writer for register RESET_CAUSE"]
pub type W = crate::W<u32, super::RESET_CAUSE>;
#[doc = "Register RESET_CAUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::RESET_CAUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
