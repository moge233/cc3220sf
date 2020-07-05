#[doc = "Reader of register MODE"]
pub type R = crate::R<u32, super::MODE>;
#[doc = "Writer for register MODE"]
pub type W = crate::W<u32, super::MODE>;
#[doc = "Register MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALGO`"]
pub type ALGO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALGO`"]
pub struct ALGO_W<'a> {
    w: &'a mut W,
}
impl<'a> ALGO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:2 - These bits select the hash algorithm to be used for processing: 0x0 md5_128 algorithm 0x1 sha1_160 algorithm 0x2 sha2_224 algorithm 0x3 sha2_256 algorithm"]
    #[inline(always)]
    pub fn algo(&self) -> ALGO_R {
        ALGO_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - These bits select the hash algorithm to be used for processing: 0x0 md5_128 algorithm 0x1 sha1_160 algorithm 0x2 sha2_224 algorithm 0x3 sha2_256 algorithm"]
    #[inline(always)]
    pub fn algo(&mut self) -> ALGO_W {
        ALGO_W { w: self }
    }
}
