#[doc = "Reader of register I2CMICR"]
pub type R = crate::R<u32, super::I2CMICR>;
#[doc = "Writer for register I2CMICR"]
pub type W = crate::W<u32, super::I2CMICR>;
#[doc = "Register I2CMICR `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CMICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXFFIC`"]
pub type RXFFIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFFIC`"]
pub struct RXFFIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFIC_W<'a> {
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
#[doc = "Reader of field `TXFEIC`"]
pub type TXFEIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFEIC`"]
pub struct TXFEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFEIC_W<'a> {
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
#[doc = "Reader of field `RXIC`"]
pub type RXIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIC`"]
pub struct RXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIC_W<'a> {
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
#[doc = "Reader of field `TXIC`"]
pub type TXIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXIC`"]
pub struct TXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIC_W<'a> {
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
#[doc = "Reader of field `ARBLOSTIC`"]
pub type ARBLOSTIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBLOSTIC`"]
pub struct ARBLOSTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLOSTIC_W<'a> {
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
#[doc = "Reader of field `STOPIC`"]
pub type STOPIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPIC`"]
pub struct STOPIC_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPIC_W<'a> {
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
#[doc = "Reader of field `STARTIC`"]
pub type STARTIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTIC`"]
pub struct STARTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTIC_W<'a> {
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
#[doc = "Reader of field `NACKIC`"]
pub type NACKIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NACKIC`"]
pub struct NACKIC_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKIC_W<'a> {
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
#[doc = "Reader of field `DMATXIC`"]
pub type DMATXIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMATXIC`"]
pub struct DMATXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATXIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DMARXIC`"]
pub type DMARXIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARXIC`"]
pub struct DMARXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARXIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLKIC`"]
pub type CLKIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKIC`"]
pub struct CLKIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKIC_W<'a> {
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
#[doc = "Reader of field `IC`"]
pub type IC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC`"]
pub struct IC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_W<'a> {
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
    #[doc = "Bit 11 - Receive FIFO full interrupt clear"]
    #[inline(always)]
    pub fn rxffic(&self) -> RXFFIC_R {
        RXFFIC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO empty interrupt clear"]
    #[inline(always)]
    pub fn txfeic(&self) -> TXFEIC_R {
        TXFEIC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO request interrupt clear"]
    #[inline(always)]
    pub fn rxic(&self) -> RXIC_R {
        RXIC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit FIFO request interrupt clear"]
    #[inline(always)]
    pub fn txic(&self) -> TXIC_R {
        TXIC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Arbitration lost interrupt clear"]
    #[inline(always)]
    pub fn arblostic(&self) -> ARBLOSTIC_R {
        ARBLOSTIC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stop detection interrupt clear"]
    #[inline(always)]
    pub fn stopic(&self) -> STOPIC_R {
        STOPIC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Start detection interrupt clear"]
    #[inline(always)]
    pub fn startic(&self) -> STARTIC_R {
        STARTIC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Address/data NACK interrupt clear"]
    #[inline(always)]
    pub fn nackic(&self) -> NACKIC_R {
        NACKIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA transmit interrupt clear"]
    #[inline(always)]
    pub fn dmatxic(&self) -> DMATXIC_R {
        DMATXIC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA receive interrupt clear"]
    #[inline(always)]
    pub fn dmarxic(&self) -> DMARXIC_R {
        DMARXIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock timeout interrupt clear"]
    #[inline(always)]
    pub fn clkic(&self) -> CLKIC_R {
        CLKIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Master interrupt clear"]
    #[inline(always)]
    pub fn ic(&self) -> IC_R {
        IC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Receive FIFO full interrupt clear"]
    #[inline(always)]
    pub fn rxffic(&mut self) -> RXFFIC_W {
        RXFFIC_W { w: self }
    }
    #[doc = "Bit 10 - Transmit FIFO empty interrupt clear"]
    #[inline(always)]
    pub fn txfeic(&mut self) -> TXFEIC_W {
        TXFEIC_W { w: self }
    }
    #[doc = "Bit 9 - Receive FIFO request interrupt clear"]
    #[inline(always)]
    pub fn rxic(&mut self) -> RXIC_W {
        RXIC_W { w: self }
    }
    #[doc = "Bit 8 - Transmit FIFO request interrupt clear"]
    #[inline(always)]
    pub fn txic(&mut self) -> TXIC_W {
        TXIC_W { w: self }
    }
    #[doc = "Bit 7 - Arbitration lost interrupt clear"]
    #[inline(always)]
    pub fn arblostic(&mut self) -> ARBLOSTIC_W {
        ARBLOSTIC_W { w: self }
    }
    #[doc = "Bit 6 - Stop detection interrupt clear"]
    #[inline(always)]
    pub fn stopic(&mut self) -> STOPIC_W {
        STOPIC_W { w: self }
    }
    #[doc = "Bit 5 - Start detection interrupt clear"]
    #[inline(always)]
    pub fn startic(&mut self) -> STARTIC_W {
        STARTIC_W { w: self }
    }
    #[doc = "Bit 4 - Address/data NACK interrupt clear"]
    #[inline(always)]
    pub fn nackic(&mut self) -> NACKIC_W {
        NACKIC_W { w: self }
    }
    #[doc = "Bit 3 - DMA transmit interrupt clear"]
    #[inline(always)]
    pub fn dmatxic(&mut self) -> DMATXIC_W {
        DMATXIC_W { w: self }
    }
    #[doc = "Bit 2 - DMA receive interrupt clear"]
    #[inline(always)]
    pub fn dmarxic(&mut self) -> DMARXIC_W {
        DMARXIC_W { w: self }
    }
    #[doc = "Bit 1 - Clock timeout interrupt clear"]
    #[inline(always)]
    pub fn clkic(&mut self) -> CLKIC_W {
        CLKIC_W { w: self }
    }
    #[doc = "Bit 0 - Master interrupt clear"]
    #[inline(always)]
    pub fn ic(&mut self) -> IC_W {
        IC_W { w: self }
    }
}
