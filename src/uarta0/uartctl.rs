#[doc = "Reader of register UARTCTL"]
pub type R = crate::R<u32, super::UARTCTL>;
#[doc = "Writer for register UARTCTL"]
pub type W = crate::W<u32, super::UARTCTL>;
#[doc = "Register UARTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTSEN`"]
pub type CTSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSEN`"]
pub struct CTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RTSEN`"]
pub type RTSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTSEN`"]
pub struct RTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RTS`"]
pub type RTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTS`"]
pub struct RTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RXE`"]
pub type RXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXE`"]
pub struct RXE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TXE`"]
pub type TXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXE`"]
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `LBE`"]
pub type LBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBE`"]
pub struct LBE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `HSE`"]
pub type HSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSE`"]
pub struct HSE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EOT`"]
pub type EOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOT`"]
pub struct EOT_W<'a> {
    w: &'a mut W,
}
impl<'a> EOT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `UARTEN`"]
pub type UARTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UARTEN`"]
pub struct UARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - UART enable clear to send"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - UART enable request to send"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 11 - UART request to send"]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART receive enable"]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART transmit enable"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART loop back enable"]
    #[inline(always)]
    pub fn lbe(&self) -> LBE_R {
        LBE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART high speed enable"]
    #[inline(always)]
    pub fn hse(&self) -> HSE_R {
        HSE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART end of transmission"]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - UART enable"]
    #[inline(always)]
    pub fn uarten(&self) -> UARTEN_R {
        UARTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - UART enable clear to send"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W { w: self }
    }
    #[doc = "Bit 14 - UART enable request to send"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W {
        RTSEN_W { w: self }
    }
    #[doc = "Bit 11 - UART request to send"]
    #[inline(always)]
    pub fn rts(&mut self) -> RTS_W {
        RTS_W { w: self }
    }
    #[doc = "Bit 9 - UART receive enable"]
    #[inline(always)]
    pub fn rxe(&mut self) -> RXE_W {
        RXE_W { w: self }
    }
    #[doc = "Bit 8 - UART transmit enable"]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
    #[doc = "Bit 7 - UART loop back enable"]
    #[inline(always)]
    pub fn lbe(&mut self) -> LBE_W {
        LBE_W { w: self }
    }
    #[doc = "Bit 5 - UART high speed enable"]
    #[inline(always)]
    pub fn hse(&mut self) -> HSE_W {
        HSE_W { w: self }
    }
    #[doc = "Bit 4 - UART end of transmission"]
    #[inline(always)]
    pub fn eot(&mut self) -> EOT_W {
        EOT_W { w: self }
    }
    #[doc = "Bit 0 - UART enable"]
    #[inline(always)]
    pub fn uarten(&mut self) -> UARTEN_W {
        UARTEN_W { w: self }
    }
}
