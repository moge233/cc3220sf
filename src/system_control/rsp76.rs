#[doc = "Reader of register RSP76"]
pub type R = crate::R<u32, super::RSP76>;
#[doc = "Writer for register RSP76"]
pub type W = crate::W<u32, super::RSP76>;
#[doc = "Register RSP76 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSP76 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSP7`"]
pub type RSP7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RSP7`"]
pub struct RSP7_W<'a> {
    w: &'a mut W,
}
impl<'a> RSP7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RSP6`"]
pub type RSP6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RSP6`"]
pub struct RSP6_W<'a> {
    w: &'a mut W,
}
impl<'a> RSP6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Command Response \\[127:112\\]"]
    #[inline(always)]
    pub fn rsp7(&self) -> RSP7_R {
        RSP7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Command Response \\[111:96\\]"]
    #[inline(always)]
    pub fn rsp6(&self) -> RSP6_R {
        RSP6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Command Response \\[127:112\\]"]
    #[inline(always)]
    pub fn rsp7(&mut self) -> RSP7_W {
        RSP7_W { w: self }
    }
    #[doc = "Bits 0:15 - Command Response \\[111:96\\]"]
    #[inline(always)]
    pub fn rsp6(&mut self) -> RSP6_W {
        RSP6_W { w: self }
    }
}
