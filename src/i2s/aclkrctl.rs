#[doc = "Reader of register ACLKRCTL"]
pub type R = crate::R<u32, super::ACLKRCTL>;
#[doc = "Writer for register ACLKRCTL"]
pub type W = crate::W<u32, super::ACLKRCTL>;
#[doc = "Register ACLKRCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ACLKRCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKRADJ`"]
pub type CLKRADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKRADJ`"]
pub struct CLKRADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKRADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CLKRDIV`"]
pub type CLKRDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKRDIV`"]
pub struct CLKRDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKRDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - CLKRADJ"]
    #[inline(always)]
    pub fn clkradj(&self) -> CLKRADJ_R {
        CLKRADJ_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:4 - RCV clock devide ratio"]
    #[inline(always)]
    pub fn clkrdiv(&self) -> CLKRDIV_R {
        CLKRDIV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - CLKRADJ"]
    #[inline(always)]
    pub fn clkradj(&mut self) -> CLKRADJ_W {
        CLKRADJ_W { w: self }
    }
    #[doc = "Bits 0:4 - RCV clock devide ratio"]
    #[inline(always)]
    pub fn clkrdiv(&mut self) -> CLKRDIV_W {
        CLKRDIV_W { w: self }
    }
}
