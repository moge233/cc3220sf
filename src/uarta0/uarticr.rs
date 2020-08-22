#[doc = "Reader of register UARTICR"]
pub type R = crate::R<u32, super::UARTICR>;
#[doc = "Writer for register UARTICR"]
pub type W = crate::W<u32, super::UARTICR>;
#[doc = "Register UARTICR `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMATXICR`"]
pub type DMATXICR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMATXICR`"]
pub struct DMATXICR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATXICR_W<'a> {
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
#[doc = "Reader of field `DMARXICR`"]
pub type DMARXICR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARXICR`"]
pub struct DMARXICR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARXICR_W<'a> {
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
#[doc = "Reader of field `OEICR`"]
pub type OEICR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEICR`"]
pub struct OEICR_W<'a> {
    w: &'a mut W,
}
impl<'a> OEICR_W<'a> {
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
#[doc = "Reader of field `BEICR`"]
pub type BEICR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEICR`"]
pub struct BEICR_W<'a> {
    w: &'a mut W,
}
impl<'a> BEICR_W<'a> {
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
#[doc = "Reader of field `PEICR`"]
pub type PEICR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEICR`"]
pub struct PEICR_W<'a> {
    w: &'a mut W,
}
impl<'a> PEICR_W<'a> {
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
#[doc = "Reader of field `FEICR`"]
pub type FEICR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEICR`"]
pub struct FEICR_W<'a> {
    w: &'a mut W,
}
impl<'a> FEICR_W<'a> {
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
#[doc = "Reader of field `RTICR`"]
pub type RTICR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTICR`"]
pub struct RTICR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTICR_W<'a> {
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
#[doc = "Reader of field `TXICR`"]
pub type TXICR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXICR`"]
pub struct TXICR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXICR_W<'a> {
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
#[doc = "Reader of field `RXICR`"]
pub type RXICR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXICR`"]
pub struct RXICR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXICR_W<'a> {
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
#[doc = "Reader of field `CTSICR`"]
pub type CTSICR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSICR`"]
pub struct CTSICR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSICR_W<'a> {
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
    #[doc = "Bit 17 - UART transmit DMA interrupt clear"]
    #[inline(always)]
    pub fn dmatxicr(&self) -> DMATXICR_R {
        DMATXICR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - UART receive DMA interrupt clear"]
    #[inline(always)]
    pub fn dmarxicr(&self) -> DMARXICR_R {
        DMARXICR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART overrun error interrupt clear"]
    #[inline(always)]
    pub fn oeicr(&self) -> OEICR_R {
        OEICR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART break error interrupt clear"]
    #[inline(always)]
    pub fn beicr(&self) -> BEICR_R {
        BEICR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART parity error interrupt clear"]
    #[inline(always)]
    pub fn peicr(&self) -> PEICR_R {
        PEICR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART framing error interrupt clear"]
    #[inline(always)]
    pub fn feicr(&self) -> FEICR_R {
        FEICR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART receive time out interrupt clear"]
    #[inline(always)]
    pub fn rticr(&self) -> RTICR_R {
        RTICR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART transmit interrupt clear"]
    #[inline(always)]
    pub fn txicr(&self) -> TXICR_R {
        TXICR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART receive interrupt clear"]
    #[inline(always)]
    pub fn rxicr(&self) -> RXICR_R {
        RXICR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART clear to send interrupt clear"]
    #[inline(always)]
    pub fn ctsicr(&self) -> CTSICR_R {
        CTSICR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - UART transmit DMA interrupt clear"]
    #[inline(always)]
    pub fn dmatxicr(&mut self) -> DMATXICR_W {
        DMATXICR_W { w: self }
    }
    #[doc = "Bit 16 - UART receive DMA interrupt clear"]
    #[inline(always)]
    pub fn dmarxicr(&mut self) -> DMARXICR_W {
        DMARXICR_W { w: self }
    }
    #[doc = "Bit 10 - UART overrun error interrupt clear"]
    #[inline(always)]
    pub fn oeicr(&mut self) -> OEICR_W {
        OEICR_W { w: self }
    }
    #[doc = "Bit 9 - UART break error interrupt clear"]
    #[inline(always)]
    pub fn beicr(&mut self) -> BEICR_W {
        BEICR_W { w: self }
    }
    #[doc = "Bit 8 - UART parity error interrupt clear"]
    #[inline(always)]
    pub fn peicr(&mut self) -> PEICR_W {
        PEICR_W { w: self }
    }
    #[doc = "Bit 7 - UART framing error interrupt clear"]
    #[inline(always)]
    pub fn feicr(&mut self) -> FEICR_W {
        FEICR_W { w: self }
    }
    #[doc = "Bit 6 - UART receive time out interrupt clear"]
    #[inline(always)]
    pub fn rticr(&mut self) -> RTICR_W {
        RTICR_W { w: self }
    }
    #[doc = "Bit 5 - UART transmit interrupt clear"]
    #[inline(always)]
    pub fn txicr(&mut self) -> TXICR_W {
        TXICR_W { w: self }
    }
    #[doc = "Bit 4 - UART receive interrupt clear"]
    #[inline(always)]
    pub fn rxicr(&mut self) -> RXICR_W {
        RXICR_W { w: self }
    }
    #[doc = "Bit 1 - UART clear to send interrupt clear"]
    #[inline(always)]
    pub fn ctsicr(&mut self) -> CTSICR_W {
        CTSICR_W { w: self }
    }
}
