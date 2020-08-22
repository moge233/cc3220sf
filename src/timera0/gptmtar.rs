#[doc = "Reader of register GPTMTAR"]
pub type R = crate::R<u32, super::GPTMTAR>;
#[doc = "Writer for register GPTMTAR"]
pub type W = crate::W<u32, super::GPTMTAR>;
#[doc = "Register GPTMTAR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTMTAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAR`"]
pub type TAR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPTM Timer A Register"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {}
