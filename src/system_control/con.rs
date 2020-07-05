#[doc = "Reader of register CON"]
pub type R = crate::R<u32, super::CON>;
#[doc = "Writer for register CON"]
pub type W = crate::W<u32, super::CON>;
#[doc = "Register CON `reset()`'s with value 0"]
impl crate::ResetValue for super::CON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DVAL`"]
pub type DVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DVAL`"]
pub struct DVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:10 - Debounce filter value All cards This register is used to define a debounce period to filter the card detect input signal (SDCD). The usage of the card detect input signal (SDCD) is optional and depends on the system integration and the type of the connector housing that accommodates the card. 0x0 33 us debounce period 0x1 231 us debounce period 0x2 1 ms debounce period 0x3 84 ms debounce period"]
    #[inline(always)]
    pub fn dval(&self) -> DVAL_R {
        DVAL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 9:10 - Debounce filter value All cards This register is used to define a debounce period to filter the card detect input signal (SDCD). The usage of the card detect input signal (SDCD) is optional and depends on the system integration and the type of the connector housing that accommodates the card. 0x0 33 us debounce period 0x1 231 us debounce period 0x2 1 ms debounce period 0x3 84 ms debounce period"]
    #[inline(always)]
    pub fn dval(&mut self) -> DVAL_W {
        DVAL_W { w: self }
    }
}
