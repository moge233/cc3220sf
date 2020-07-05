#[doc = "Reader of register CM_SPARE"]
pub type R = crate::R<u32, super::CM_SPARE>;
#[doc = "Writer for register CM_SPARE"]
pub type W = crate::W<u32, super::CM_SPARE>;
#[doc = "Register CM_SPARE `reset()`'s with value 0"]
impl crate::ResetValue for super::CM_SPARE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CM_SPARE_OUT`"]
pub type CM_SPARE_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CM_SPARE_OUT`"]
pub struct CM_SPARE_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_SPARE_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `MEM_CM_TEST_CTRL`"]
pub type MEM_CM_TEST_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_CM_TEST_CTRL`"]
pub struct MEM_CM_TEST_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CM_TEST_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEM_CM_SPARE`"]
pub type MEM_CM_SPARE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_CM_SPARE`"]
pub struct MEM_CM_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CM_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - CM_SPARE_OUT"]
    #[inline(always)]
    pub fn cm_spare_out(&self) -> CM_SPARE_OUT_R {
        CM_SPARE_OUT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MEM_CM_TEST_CTRL"]
    #[inline(always)]
    pub fn mem_cm_test_ctrl(&self) -> MEM_CM_TEST_CTRL_R {
        MEM_CM_TEST_CTRL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - MEM_CM_SPARE"]
    #[inline(always)]
    pub fn mem_cm_spare(&self) -> MEM_CM_SPARE_R {
        MEM_CM_SPARE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31 - CM_SPARE_OUT"]
    #[inline(always)]
    pub fn cm_spare_out(&mut self) -> CM_SPARE_OUT_W {
        CM_SPARE_OUT_W { w: self }
    }
    #[doc = "Bits 16:23 - MEM_CM_TEST_CTRL"]
    #[inline(always)]
    pub fn mem_cm_test_ctrl(&mut self) -> MEM_CM_TEST_CTRL_W {
        MEM_CM_TEST_CTRL_W { w: self }
    }
    #[doc = "Bits 0:15 - MEM_CM_SPARE"]
    #[inline(always)]
    pub fn mem_cm_spare(&mut self) -> MEM_CM_SPARE_W {
        MEM_CM_SPARE_W { w: self }
    }
}
