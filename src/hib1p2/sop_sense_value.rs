#[doc = "Reader of register SOP_SENSE_VALUE"]
pub type R = crate::R<u32, super::SOP_SENSE_VALUE>;
#[doc = "Writer for register SOP_SENSE_VALUE"]
pub type W = crate::W<u32, super::SOP_SENSE_VALUE>;
#[doc = "Register SOP_SENSE_VALUE `reset()`'s with value 0"]
impl crate::ResetValue for super::SOP_SENSE_VALUE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOP_SENSE_VALUE`"]
pub type SOP_SENSE_VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SOP_SENSE_VALUE`"]
pub struct SOP_SENSE_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOP_SENSE_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SOP_SENSE_VALUE"]
    #[inline(always)]
    pub fn sop_sense_value(&self) -> SOP_SENSE_VALUE_R {
        SOP_SENSE_VALUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SOP_SENSE_VALUE"]
    #[inline(always)]
    pub fn sop_sense_value(&mut self) -> SOP_SENSE_VALUE_W {
        SOP_SENSE_VALUE_W { w: self }
    }
}
