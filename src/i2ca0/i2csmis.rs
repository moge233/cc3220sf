#[doc = "Reader of register I2CSMIS"]
pub type R = crate::R<u32, super::I2CSMIS>;
#[doc = "Writer for register I2CSMIS"]
pub type W = crate::W<u32, super::I2CSMIS>;
#[doc = "Register I2CSMIS `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CSMIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXFFMIS`"]
pub type RXFFMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFFMIS`"]
pub struct RXFFMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFMIS_W<'a> {
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
#[doc = "Reader of field `TXFEMIS`"]
pub type TXFEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFEMIS`"]
pub struct TXFEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFEMIS_W<'a> {
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
#[doc = "Reader of field `RXMIS`"]
pub type RXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXMIS`"]
pub struct RXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMIS_W<'a> {
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
#[doc = "Reader of field `TXMIS`"]
pub type TXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMIS`"]
pub struct TXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMIS_W<'a> {
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
#[doc = "Reader of field `DMATXMIS`"]
pub type DMATXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMATXMIS`"]
pub struct DMATXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATXMIS_W<'a> {
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
#[doc = "Reader of field `DMARXMIS`"]
pub type DMARXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARXMIS`"]
pub struct DMARXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARXMIS_W<'a> {
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
#[doc = "Reader of field `STOPMIS`"]
pub type STOPMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPMIS`"]
pub struct STOPMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPMIS_W<'a> {
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
#[doc = "Reader of field `STARTMIS`"]
pub type STARTMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTMIS`"]
pub struct STARTMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTMIS_W<'a> {
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
#[doc = "Reader of field `DATAMIS`"]
pub type DATAMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAMIS`"]
pub struct DATAMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAMIS_W<'a> {
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
    #[doc = "Bit 8 - Receive FIFO full masked interrupt status"]
    #[inline(always)]
    pub fn rxffmis(&self) -> RXFFMIS_R {
        RXFFMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty masked interrupt status"]
    #[inline(always)]
    pub fn txfemis(&self) -> TXFEMIS_R {
        TXFEMIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO request masked interrupt status"]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO request masked interrupt status"]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit DMA masked interrupt status"]
    #[inline(always)]
    pub fn dmatxmis(&self) -> DMATXMIS_R {
        DMATXMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive DMA masked interrupt status"]
    #[inline(always)]
    pub fn dmarxmis(&self) -> DMARXMIS_R {
        DMARXMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop condition masked interrupt status"]
    #[inline(always)]
    pub fn stopmis(&self) -> STOPMIS_R {
        STOPMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start condition masked interrupt status"]
    #[inline(always)]
    pub fn startmis(&self) -> STARTMIS_R {
        STARTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data masked interrupt status"]
    #[inline(always)]
    pub fn datamis(&self) -> DATAMIS_R {
        DATAMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Receive FIFO full masked interrupt status"]
    #[inline(always)]
    pub fn rxffmis(&mut self) -> RXFFMIS_W {
        RXFFMIS_W { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO empty masked interrupt status"]
    #[inline(always)]
    pub fn txfemis(&mut self) -> TXFEMIS_W {
        TXFEMIS_W { w: self }
    }
    #[doc = "Bit 6 - Receive FIFO request masked interrupt status"]
    #[inline(always)]
    pub fn rxmis(&mut self) -> RXMIS_W {
        RXMIS_W { w: self }
    }
    #[doc = "Bit 5 - Transmit FIFO request masked interrupt status"]
    #[inline(always)]
    pub fn txmis(&mut self) -> TXMIS_W {
        TXMIS_W { w: self }
    }
    #[doc = "Bit 4 - Transmit DMA masked interrupt status"]
    #[inline(always)]
    pub fn dmatxmis(&mut self) -> DMATXMIS_W {
        DMATXMIS_W { w: self }
    }
    #[doc = "Bit 3 - Receive DMA masked interrupt status"]
    #[inline(always)]
    pub fn dmarxmis(&mut self) -> DMARXMIS_W {
        DMARXMIS_W { w: self }
    }
    #[doc = "Bit 2 - Stop condition masked interrupt status"]
    #[inline(always)]
    pub fn stopmis(&mut self) -> STOPMIS_W {
        STOPMIS_W { w: self }
    }
    #[doc = "Bit 1 - Start condition masked interrupt status"]
    #[inline(always)]
    pub fn startmis(&mut self) -> STARTMIS_W {
        STARTMIS_W { w: self }
    }
    #[doc = "Bit 1 - Data masked interrupt status"]
    #[inline(always)]
    pub fn datamis(&mut self) -> DATAMIS_W {
        DATAMIS_W { w: self }
    }
}
