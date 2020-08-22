#[doc = "Reader of register GPTMTBR"]
pub type R = crate::R<u32, super::GPTMTBR>;
#[doc = "Writer for register GPTMTBR"]
pub type W = crate::W<u32, super::GPTMTBR>;
#[doc = "Register GPTMTBR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTMTBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBR`"]
pub type TBR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B"]
    #[inline(always)]
    pub fn tbr(&self) -> TBR_R {
        TBR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
