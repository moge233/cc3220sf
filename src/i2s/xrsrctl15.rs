#[doc = "Reader of register XRSRCTL15"]
pub type R = crate::R<u32, super::XRSRCTL15>;
#[doc = "Writer for register XRSRCTL15"]
pub type W = crate::W<u32, super::XRSRCTL15>;
#[doc = "Register XRSRCTL15 `reset()`'s with value 0"]
impl crate::ResetValue for super::XRSRCTL15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DISMOD`"]
pub type DISMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DISMOD`"]
pub struct DISMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SRMOD`"]
pub type SRMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRMOD`"]
pub struct SRMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3 - Serializer drive state 0x0 Tri state 0x1 Reserved 0x2 Drive pin low 0x3 Drive pin high"]
    #[inline(always)]
    pub fn dismod(&self) -> DISMOD_R {
        DISMOD_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Serializer Mode 0x0 InActive mode 0x1 Transmit mode 0x2 Receive mode"]
    #[inline(always)]
    pub fn srmod(&self) -> SRMOD_R {
        SRMOD_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - Serializer drive state 0x0 Tri state 0x1 Reserved 0x2 Drive pin low 0x3 Drive pin high"]
    #[inline(always)]
    pub fn dismod(&mut self) -> DISMOD_W {
        DISMOD_W { w: self }
    }
    #[doc = "Bits 0:1 - Serializer Mode 0x0 InActive mode 0x1 Transmit mode 0x2 Receive mode"]
    #[inline(always)]
    pub fn srmod(&mut self) -> SRMOD_W {
        SRMOD_W { w: self }
    }
}
