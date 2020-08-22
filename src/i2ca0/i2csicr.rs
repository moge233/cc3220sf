#[doc = "Reader of register I2CSICR"]
pub type R = crate::R<u32, super::I2CSICR>;
#[doc = "Writer for register I2CSICR"]
pub type W = crate::W<u32, super::I2CSICR>;
#[doc = "Register I2CSICR `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CSICR {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DATAIC`"]
pub type DATAIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAIC`"]
pub struct DATAIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAIC_W<'a> {
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
    #[doc = "Bit 8 - Receive FIFO full interrupt clear"]
    #[inline(always)]
    pub fn rxffic(&self) -> RXFFIC_R {
        RXFFIC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty interrupt clear"]
    #[inline(always)]
    pub fn txfeic(&self) -> TXFEIC_R {
        TXFEIC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO request interrupt clear"]
    #[inline(always)]
    pub fn rxic(&self) -> RXIC_R {
        RXIC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO request interrupt clear"]
    #[inline(always)]
    pub fn txic(&self) -> TXIC_R {
        TXIC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit DMA interrupt clear"]
    #[inline(always)]
    pub fn dmatxic(&self) -> DMATXIC_R {
        DMATXIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive DMA interrupt clear"]
    #[inline(always)]
    pub fn dmarxic(&self) -> DMARXIC_R {
        DMARXIC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop condition interrupt clear"]
    #[inline(always)]
    pub fn stopic(&self) -> STOPIC_R {
        STOPIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start condition interrupt clear"]
    #[inline(always)]
    pub fn startic(&self) -> STARTIC_R {
        STARTIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data interrupt clear"]
    #[inline(always)]
    pub fn dataic(&self) -> DATAIC_R {
        DATAIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Receive FIFO full interrupt clear"]
    #[inline(always)]
    pub fn rxffic(&mut self) -> RXFFIC_W {
        RXFFIC_W { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO empty interrupt clear"]
    #[inline(always)]
    pub fn txfeic(&mut self) -> TXFEIC_W {
        TXFEIC_W { w: self }
    }
    #[doc = "Bit 6 - Receive FIFO request interrupt clear"]
    #[inline(always)]
    pub fn rxic(&mut self) -> RXIC_W {
        RXIC_W { w: self }
    }
    #[doc = "Bit 5 - Transmit FIFO request interrupt clear"]
    #[inline(always)]
    pub fn txic(&mut self) -> TXIC_W {
        TXIC_W { w: self }
    }
    #[doc = "Bit 4 - Transmit DMA interrupt clear"]
    #[inline(always)]
    pub fn dmatxic(&mut self) -> DMATXIC_W {
        DMATXIC_W { w: self }
    }
    #[doc = "Bit 3 - Receive DMA interrupt clear"]
    #[inline(always)]
    pub fn dmarxic(&mut self) -> DMARXIC_W {
        DMARXIC_W { w: self }
    }
    #[doc = "Bit 2 - Stop condition interrupt clear"]
    #[inline(always)]
    pub fn stopic(&mut self) -> STOPIC_W {
        STOPIC_W { w: self }
    }
    #[doc = "Bit 1 - Start condition interrupt clear"]
    #[inline(always)]
    pub fn startic(&mut self) -> STARTIC_W {
        STARTIC_W { w: self }
    }
    #[doc = "Bit 1 - Data interrupt clear"]
    #[inline(always)]
    pub fn dataic(&mut self) -> DATAIC_W {
        DATAIC_W { w: self }
    }
}
