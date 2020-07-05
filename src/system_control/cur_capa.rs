#[doc = "Reader of register CUR_CAPA"]
pub type R = crate::R<u32, super::CUR_CAPA>;
#[doc = "Writer for register CUR_CAPA"]
pub type W = crate::W<u32, super::CUR_CAPA>;
#[doc = "Register CUR_CAPA `reset()`'s with value 0"]
impl crate::ResetValue for super::CUR_CAPA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CUR_1V8`"]
pub type CUR_1V8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CUR_1V8`"]
pub struct CUR_1V8_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_1V8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CUR_3V0`"]
pub type CUR_3V0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CUR_3V0`"]
pub struct CUR_3V0_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_3V0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CUR_3V3`"]
pub type CUR_3V3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CUR_3V3`"]
pub struct CUR_3V3_W<'a> {
    w: &'a mut W,
}
impl<'a> CUR_3V3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - CUR_1V8"]
    #[inline(always)]
    pub fn cur_1v8(&self) -> CUR_1V8_R {
        CUR_1V8_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CUR_3V0"]
    #[inline(always)]
    pub fn cur_3v0(&self) -> CUR_3V0_R {
        CUR_3V0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - CUR_3V3"]
    #[inline(always)]
    pub fn cur_3v3(&self) -> CUR_3V3_R {
        CUR_3V3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - CUR_1V8"]
    #[inline(always)]
    pub fn cur_1v8(&mut self) -> CUR_1V8_W {
        CUR_1V8_W { w: self }
    }
    #[doc = "Bits 8:15 - CUR_3V0"]
    #[inline(always)]
    pub fn cur_3v0(&mut self) -> CUR_3V0_W {
        CUR_3V0_W { w: self }
    }
    #[doc = "Bits 0:7 - CUR_3V3"]
    #[inline(always)]
    pub fn cur_3v3(&mut self) -> CUR_3V3_W {
        CUR_3V3_W { w: self }
    }
}
