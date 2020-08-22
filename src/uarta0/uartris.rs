#[doc = "Reader of register UARTRIS"]
pub type R = crate::R<u32, super::UARTRIS>;
#[doc = "Writer for register UARTRIS"]
pub type W = crate::W<u32, super::UARTRIS>;
#[doc = "Register UARTRIS `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTRIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMATXRIS`"]
pub type DMATXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMATXRIS`"]
pub struct DMATXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATXRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `DMARXRIS`"]
pub type DMARXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARXRIS`"]
pub struct DMARXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARXRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `OERIS`"]
pub type OERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OERIS`"]
pub struct OERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OERIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `BERIS`"]
pub type BERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BERIS`"]
pub struct BERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BERIS_W<'a> {
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
#[doc = "Reader of field `PERIS`"]
pub type PERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERIS`"]
pub struct PERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIS_W<'a> {
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
#[doc = "Reader of field `FERIS`"]
pub type FERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FERIS`"]
pub struct FERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FERIS_W<'a> {
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
#[doc = "Reader of field `RTRIS`"]
pub type RTRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTRIS`"]
pub struct RTRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TXRIS`"]
pub type TXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXRIS`"]
pub struct TXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRIS_W<'a> {
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
#[doc = "Reader of field `RXRIS`"]
pub type RXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXRIS`"]
pub struct RXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRIS_W<'a> {
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
#[doc = "Reader of field `CTSRIS`"]
pub type CTSRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSRIS`"]
pub struct CTSRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - UART transmit DMA raw interrupt status"]
    #[inline(always)]
    pub fn dmatxris(&self) -> DMATXRIS_R {
        DMATXRIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - UART receive DMA raw interrupt status"]
    #[inline(always)]
    pub fn dmarxris(&self) -> DMARXRIS_R {
        DMARXRIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART overrun error raw interrupt status"]
    #[inline(always)]
    pub fn oeris(&self) -> OERIS_R {
        OERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART break error raw interrupt status"]
    #[inline(always)]
    pub fn beris(&self) -> BERIS_R {
        BERIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART parity error raw interrupt status"]
    #[inline(always)]
    pub fn peris(&self) -> PERIS_R {
        PERIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART framing error raw interrupt status"]
    #[inline(always)]
    pub fn feris(&self) -> FERIS_R {
        FERIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART receive time out raw interrupt status"]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART transmit raw interrupt status"]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART receive raw interrupt status"]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART clear to send raw interrupt status"]
    #[inline(always)]
    pub fn ctsris(&self) -> CTSRIS_R {
        CTSRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - UART transmit DMA raw interrupt status"]
    #[inline(always)]
    pub fn dmatxris(&mut self) -> DMATXRIS_W {
        DMATXRIS_W { w: self }
    }
    #[doc = "Bit 16 - UART receive DMA raw interrupt status"]
    #[inline(always)]
    pub fn dmarxris(&mut self) -> DMARXRIS_W {
        DMARXRIS_W { w: self }
    }
    #[doc = "Bit 10 - UART overrun error raw interrupt status"]
    #[inline(always)]
    pub fn oeris(&mut self) -> OERIS_W {
        OERIS_W { w: self }
    }
    #[doc = "Bit 9 - UART break error raw interrupt status"]
    #[inline(always)]
    pub fn beris(&mut self) -> BERIS_W {
        BERIS_W { w: self }
    }
    #[doc = "Bit 8 - UART parity error raw interrupt status"]
    #[inline(always)]
    pub fn peris(&mut self) -> PERIS_W {
        PERIS_W { w: self }
    }
    #[doc = "Bit 7 - UART framing error raw interrupt status"]
    #[inline(always)]
    pub fn feris(&mut self) -> FERIS_W {
        FERIS_W { w: self }
    }
    #[doc = "Bit 6 - UART receive time out raw interrupt status"]
    #[inline(always)]
    pub fn rtris(&mut self) -> RTRIS_W {
        RTRIS_W { w: self }
    }
    #[doc = "Bit 5 - UART transmit raw interrupt status"]
    #[inline(always)]
    pub fn txris(&mut self) -> TXRIS_W {
        TXRIS_W { w: self }
    }
    #[doc = "Bit 4 - UART receive raw interrupt status"]
    #[inline(always)]
    pub fn rxris(&mut self) -> RXRIS_W {
        RXRIS_W { w: self }
    }
    #[doc = "Bit 1 - UART clear to send raw interrupt status"]
    #[inline(always)]
    pub fn ctsris(&mut self) -> CTSRIS_W {
        CTSRIS_W { w: self }
    }
}
