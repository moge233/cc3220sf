#[doc = "Reader of register FIFOCTL"]
pub type R = crate::R<u32, super::FIFOCTL>;
#[doc = "Writer for register FIFOCTL"]
pub type W = crate::W<u32, super::FIFOCTL>;
#[doc = "Register FIFOCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXTRIG`"]
pub type RXTRIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXTRIG`"]
pub struct RXTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTRIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `TXTRIG`"]
pub type TXTRIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXTRIG`"]
pub struct TXTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTRIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18 - RX FIFO Trigger"]
    #[inline(always)]
    pub fn rxtrig(&self) -> RXTRIG_R {
        RXTRIG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - TX FIFO Trigger"]
    #[inline(always)]
    pub fn txtrig(&self) -> TXTRIG_R {
        TXTRIG_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - RX FIFO Trigger"]
    #[inline(always)]
    pub fn rxtrig(&mut self) -> RXTRIG_W {
        RXTRIG_W { w: self }
    }
    #[doc = "Bits 0:2 - TX FIFO Trigger"]
    #[inline(always)]
    pub fn txtrig(&mut self) -> TXTRIG_W {
        TXTRIG_W { w: self }
    }
}
