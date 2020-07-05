#[doc = "Reader of register TEST"]
pub type R = crate::R<u32, super::TEST>;
#[doc = "Writer for register TEST"]
pub type W = crate::W<u32, super::TEST>;
#[doc = "Register TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STALL_EN`"]
pub type STALL_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STALL_EN`"]
pub struct STALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:11 - Watchdog stall enable"]
    #[inline(always)]
    pub fn stall_en(&self) -> STALL_EN_R {
        STALL_EN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 10:11 - Watchdog stall enable"]
    #[inline(always)]
    pub fn stall_en(&mut self) -> STALL_EN_W {
        STALL_EN_W { w: self }
    }
}
