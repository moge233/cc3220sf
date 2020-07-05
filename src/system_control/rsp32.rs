#[doc = "Reader of register RSP32"]
pub type R = crate::R<u32, super::RSP32>;
#[doc = "Writer for register RSP32"]
pub type W = crate::W<u32, super::RSP32>;
#[doc = "Register RSP32 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSP32 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSP3`"]
pub type RSP3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RSP3`"]
pub struct RSP3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSP3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RSP2`"]
pub type RSP2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RSP2`"]
pub struct RSP2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Command Response \\[63:48\\]"]
    #[inline(always)]
    pub fn rsp3(&self) -> RSP3_R {
        RSP3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Command Response \\[47:32\\]"]
    #[inline(always)]
    pub fn rsp2(&self) -> RSP2_R {
        RSP2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Command Response \\[63:48\\]"]
    #[inline(always)]
    pub fn rsp3(&mut self) -> RSP3_W {
        RSP3_W { w: self }
    }
    #[doc = "Bits 0:15 - Command Response \\[47:32\\]"]
    #[inline(always)]
    pub fn rsp2(&mut self) -> RSP2_W {
        RSP2_W { w: self }
    }
}
