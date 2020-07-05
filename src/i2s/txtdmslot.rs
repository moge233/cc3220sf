#[doc = "Reader of register TXTDMSLOT"]
pub type R = crate::R<u32, super::TXTDMSLOT>;
#[doc = "Writer for register TXTDMSLOT"]
pub type W = crate::W<u32, super::TXTDMSLOT>;
#[doc = "Register TXTDMSLOT `reset()`'s with value 0"]
impl crate::ResetValue for super::TXTDMSLOT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XSLOTCNT`"]
pub type XSLOTCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XSLOTCNT`"]
pub struct XSLOTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> XSLOTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Current XMT time slot count during reset the value of this register is 0b0101111111 (0x17f) and after reset 0"]
    #[inline(always)]
    pub fn xslotcnt(&self) -> XSLOTCNT_R {
        XSLOTCNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Current XMT time slot count during reset the value of this register is 0b0101111111 (0x17f) and after reset 0"]
    #[inline(always)]
    pub fn xslotcnt(&mut self) -> XSLOTCNT_W {
        XSLOTCNT_W { w: self }
    }
}
