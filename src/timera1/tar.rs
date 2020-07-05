#[doc = "Reader of register TAR"]
pub type R = crate::R<u32, super::TAR>;
#[doc = "Writer for register TAR"]
pub type W = crate::W<u32, super::TAR>;
#[doc = "Register TAR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TARH`"]
pub type TARH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TARH`"]
pub struct TARH_W<'a> {
    w: &'a mut W,
}
impl<'a> TARH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TARL`"]
pub type TARL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TARL`"]
pub struct TARL_W<'a> {
    w: &'a mut W,
}
impl<'a> TARL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - GPTM Timer A Register High"]
    #[inline(always)]
    pub fn tarh(&self) -> TARH_R {
        TARH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - GPTM Timer A Register Low"]
    #[inline(always)]
    pub fn tarl(&self) -> TARL_R {
        TARL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - GPTM Timer A Register High"]
    #[inline(always)]
    pub fn tarh(&mut self) -> TARH_W {
        TARH_W { w: self }
    }
    #[doc = "Bits 0:15 - GPTM Timer A Register Low"]
    #[inline(always)]
    pub fn tarl(&mut self) -> TARL_W {
        TARL_W { w: self }
    }
}
