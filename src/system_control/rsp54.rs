#[doc = "Reader of register RSP54"]
pub type R = crate::R<u32, super::RSP54>;
#[doc = "Writer for register RSP54"]
pub type W = crate::W<u32, super::RSP54>;
#[doc = "Register RSP54 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSP54 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSP5`"]
pub type RSP5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RSP5`"]
pub struct RSP5_W<'a> {
    w: &'a mut W,
}
impl<'a> RSP5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RSP4`"]
pub type RSP4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RSP4`"]
pub struct RSP4_W<'a> {
    w: &'a mut W,
}
impl<'a> RSP4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Command Response \\[95:80\\]"]
    #[inline(always)]
    pub fn rsp5(&self) -> RSP5_R {
        RSP5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Command Response \\[79:64\\]"]
    #[inline(always)]
    pub fn rsp4(&self) -> RSP4_R {
        RSP4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Command Response \\[95:80\\]"]
    #[inline(always)]
    pub fn rsp5(&mut self) -> RSP5_W {
        RSP5_W { w: self }
    }
    #[doc = "Bits 0:15 - Command Response \\[79:64\\]"]
    #[inline(always)]
    pub fn rsp4(&mut self) -> RSP4_W {
        RSP4_W { w: self }
    }
}
