#[doc = "Reader of register LBCTL"]
pub type R = crate::R<u32, super::LBCTL>;
#[doc = "Writer for register LBCTL"]
pub type W = crate::W<u32, super::LBCTL>;
#[doc = "Register LBCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LBCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3 - Loop back clock source generator 0x0 0x1 0x2 0x3"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - Loop back clock source generator 0x0 0x1 0x2 0x3"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
