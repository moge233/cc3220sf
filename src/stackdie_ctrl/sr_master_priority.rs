#[doc = "Reader of register SR_MASTER_PRIORITY"]
pub type R = crate::R<u32, super::SR_MASTER_PRIORITY>;
#[doc = "Writer for register SR_MASTER_PRIORITY"]
pub type W = crate::W<u32, super::SR_MASTER_PRIORITY>;
#[doc = "Register SR_MASTER_PRIORITY `reset()`'s with value 0"]
impl crate::ResetValue for super::SR_MASTER_PRIORITY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRIORITY`"]
pub type PRIORITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIORITY`"]
pub struct PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - 00 : Equal Priority 01 : Stack Processor have priority 10 : Base Processor have priority 11 : Unused"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00 : Equal Priority 01 : Stack Processor have priority 10 : Base Processor have priority 11 : Unused"]
    #[inline(always)]
    pub fn priority(&mut self) -> PRIORITY_W {
        PRIORITY_W { w: self }
    }
}
