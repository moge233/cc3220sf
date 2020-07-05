#[doc = "Reader of register AHCLKRCTL"]
pub type R = crate::R<u32, super::AHCLKRCTL>;
#[doc = "Writer for register AHCLKRCTL"]
pub type W = crate::W<u32, super::AHCLKRCTL>;
#[doc = "Register AHCLKRCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::AHCLKRCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HCLKRADJ`"]
pub type HCLKRADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HCLKRADJ`"]
pub struct HCLKRADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLKRADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `HCLKRDIV`"]
pub type HCLKRDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HCLKRDIV`"]
pub struct HCLKRDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLKRDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - HCLKRADJ"]
    #[inline(always)]
    pub fn hclkradj(&self) -> HCLKRADJ_R {
        HCLKRADJ_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:11 - RCV clock Divide Ratio"]
    #[inline(always)]
    pub fn hclkrdiv(&self) -> HCLKRDIV_R {
        HCLKRDIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:17 - HCLKRADJ"]
    #[inline(always)]
    pub fn hclkradj(&mut self) -> HCLKRADJ_W {
        HCLKRADJ_W { w: self }
    }
    #[doc = "Bits 0:11 - RCV clock Divide Ratio"]
    #[inline(always)]
    pub fn hclkrdiv(&mut self) -> HCLKRDIV_W {
        HCLKRDIV_W { w: self }
    }
}
