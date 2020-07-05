#[doc = "Reader of register RSP10"]
pub type R = crate::R<u32, super::RSP10>;
#[doc = "Writer for register RSP10"]
pub type W = crate::W<u32, super::RSP10>;
#[doc = "Register RSP10 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSP10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSP1`"]
pub type RSP1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RSP1`"]
pub struct RSP1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RSP0`"]
pub type RSP0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RSP0`"]
pub struct RSP0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Command Response \\[31:16\\]"]
    #[inline(always)]
    pub fn rsp1(&self) -> RSP1_R {
        RSP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Command Response \\[15:0\\]"]
    #[inline(always)]
    pub fn rsp0(&self) -> RSP0_R {
        RSP0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Command Response \\[31:16\\]"]
    #[inline(always)]
    pub fn rsp1(&mut self) -> RSP1_W {
        RSP1_W { w: self }
    }
    #[doc = "Bits 0:15 - Command Response \\[15:0\\]"]
    #[inline(always)]
    pub fn rsp0(&mut self) -> RSP0_W {
        RSP0_W { w: self }
    }
}
