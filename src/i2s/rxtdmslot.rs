#[doc = "Reader of register RXTDMSLOT"]
pub type R = crate::R<u32, super::RXTDMSLOT>;
#[doc = "Writer for register RXTDMSLOT"]
pub type W = crate::W<u32, super::RXTDMSLOT>;
#[doc = "Register RXTDMSLOT `reset()`'s with value 0"]
impl crate::ResetValue for super::RXTDMSLOT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSLOTCNT`"]
pub type RSLOTCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RSLOTCNT`"]
pub struct RSLOTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSLOTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Current RCV time slot count"]
    #[inline(always)]
    pub fn rslotcnt(&self) -> RSLOTCNT_R {
        RSLOTCNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Current RCV time slot count"]
    #[inline(always)]
    pub fn rslotcnt(&mut self) -> RSLOTCNT_W {
        RSLOTCNT_W { w: self }
    }
}
