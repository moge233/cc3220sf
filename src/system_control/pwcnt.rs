#[doc = "Reader of register PWCNT"]
pub type R = crate::R<u32, super::PWCNT>;
#[doc = "Writer for register PWCNT"]
pub type W = crate::W<u32, super::PWCNT>;
#[doc = "Register PWCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::PWCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWRCNT`"]
pub type PWRCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PWRCNT`"]
pub struct PWRCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Power counter register. This register is used to introduce a delay between the PAD ACTIVE pin assertion and the command issued. 0x0000 No additional delay added 0x0001 TCF delay (card clock period) 0x0002 TCF x 2 delay (card clock period) 0xFFFE TCF x 65534 delay (card clock period) 0xFFFF TCF x 65535 delay (card clock period)"]
    #[inline(always)]
    pub fn pwrcnt(&self) -> PWRCNT_R {
        PWRCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Power counter register. This register is used to introduce a delay between the PAD ACTIVE pin assertion and the command issued. 0x0000 No additional delay added 0x0001 TCF delay (card clock period) 0x0002 TCF x 2 delay (card clock period) 0xFFFE TCF x 65534 delay (card clock period) 0xFFFF TCF x 65535 delay (card clock period)"]
    #[inline(always)]
    pub fn pwrcnt(&mut self) -> PWRCNT_W {
        PWRCNT_W { w: self }
    }
}
