#[doc = "Reader of register HL_HWINFO"]
pub type R = crate::R<u32, super::HL_HWINFO>;
#[doc = "Writer for register HL_HWINFO"]
pub type W = crate::W<u32, super::HL_HWINFO>;
#[doc = "Register HL_HWINFO `reset()`'s with value 0"]
impl crate::ResetValue for super::HL_HWINFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FFNBYTE`"]
pub type FFNBYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FFNBYTE`"]
pub struct FFNBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> FFNBYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:5 - FFNBYTE"]
    #[inline(always)]
    pub fn ffnbyte(&self) -> FFNBYTE_R {
        FFNBYTE_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:5 - FFNBYTE"]
    #[inline(always)]
    pub fn ffnbyte(&mut self) -> FFNBYTE_W {
        FFNBYTE_W { w: self }
    }
}
