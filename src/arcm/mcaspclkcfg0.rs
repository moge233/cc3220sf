#[doc = "Reader of register MCASPCLKCFG0"]
pub type R = crate::R<u32, super::MCASPCLKCFG0>;
#[doc = "Writer for register MCASPCLKCFG0"]
pub type W = crate::W<u32, super::MCASPCLKCFG0>;
#[doc = "Register MCASPCLKCFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MCASPCLKCFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIVISR`"]
pub type DIVISR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIVISR`"]
pub struct DIVISR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVISR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `FRACTN`"]
pub type FRACTN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRACTN`"]
pub struct FRACTN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACTN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:25 - MCASP_FRAC_DIV_DIVISOR"]
    #[inline(always)]
    pub fn divisr(&self) -> DIVISR_R {
        DIVISR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:15 - MCASP_FRAC_DIV_FRACTION"]
    #[inline(always)]
    pub fn fractn(&self) -> FRACTN_R {
        FRACTN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25 - MCASP_FRAC_DIV_DIVISOR"]
    #[inline(always)]
    pub fn divisr(&mut self) -> DIVISR_W {
        DIVISR_W { w: self }
    }
    #[doc = "Bits 0:15 - MCASP_FRAC_DIV_FRACTION"]
    #[inline(always)]
    pub fn fractn(&mut self) -> FRACTN_W {
        FRACTN_W { w: self }
    }
}
