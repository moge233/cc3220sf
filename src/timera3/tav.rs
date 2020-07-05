#[doc = "Reader of register TAV"]
pub type R = crate::R<u32, super::TAV>;
#[doc = "Writer for register TAV"]
pub type W = crate::W<u32, super::TAV>;
#[doc = "Register TAV `reset()`'s with value 0"]
impl crate::ResetValue for super::TAV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAVH`"]
pub type TAVH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TAVH`"]
pub struct TAVH_W<'a> {
    w: &'a mut W,
}
impl<'a> TAVH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TAVL`"]
pub type TAVL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TAVL`"]
pub struct TAVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TAVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - GPTM Timer A Value High"]
    #[inline(always)]
    pub fn tavh(&self) -> TAVH_R {
        TAVH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - GPTM Timer A Register Low"]
    #[inline(always)]
    pub fn tavl(&self) -> TAVL_R {
        TAVL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - GPTM Timer A Value High"]
    #[inline(always)]
    pub fn tavh(&mut self) -> TAVH_W {
        TAVH_W { w: self }
    }
    #[doc = "Bits 0:15 - GPTM Timer A Register Low"]
    #[inline(always)]
    pub fn tavl(&mut self) -> TAVL_W {
        TAVL_W { w: self }
    }
}
