#[doc = "Reader of register CHANNEL2FIFODATA"]
pub type R = crate::R<u32, super::CHANNEL2FIFODATA>;
#[doc = "Writer for register CHANNEL2FIFODATA"]
pub type W = crate::W<u32, super::CHANNEL2FIFODATA>;
#[doc = "Register CHANNEL2FIFODATA `reset()`'s with value 0"]
impl crate::ResetValue for super::CHANNEL2FIFODATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMESTAMP`"]
pub type TIMESTAMP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMESTAMP`"]
pub struct TIMESTAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMESTAMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 14)) | (((value as u32) & 0x0001_ffff) << 14);
        self.w
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 2)) | (((value as u32) & 0x0fff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:30 - ADC FIFO timestamp."]
    #[inline(always)]
    pub fn timestamp(&self) -> TIMESTAMP_R {
        TIMESTAMP_R::new(((self.bits >> 14) & 0x0001_ffff) as u32)
    }
    #[doc = "Bits 2:13 - ADC FIFO data."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 2) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 14:30 - ADC FIFO timestamp."]
    #[inline(always)]
    pub fn timestamp(&mut self) -> TIMESTAMP_W {
        TIMESTAMP_W { w: self }
    }
    #[doc = "Bits 2:13 - ADC FIFO data."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
