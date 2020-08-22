#[doc = "Reader of register I2CMRIS"]
pub type R = crate::R<u32, super::I2CMRIS>;
#[doc = "Writer for register I2CMRIS"]
pub type W = crate::W<u32, super::I2CMRIS>;
#[doc = "Register I2CMRIS `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CMRIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXFFRIS`"]
pub type RXFFRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFFRIS`"]
pub struct RXFFRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFRIS_W<'a> {
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
#[doc = "Reader of field `TXFERIS`"]
pub type TXFERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFERIS`"]
pub struct TXFERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFERIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ARBLOSTRIS`"]
pub type ARBLOSTRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBLOSTRIS`"]
pub struct ARBLOSTRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLOSTRIS_W<'a> {
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
#[doc = "Reader of field `STOPRIS`"]
pub type STOPRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPRIS`"]
pub struct STOPRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPRIS_W<'a> {
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
#[doc = "Reader of field `STARTRIS`"]
pub type STARTRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTRIS`"]
pub struct STARTRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTRIS_W<'a> {
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
#[doc = "Reader of field `NACKRIS`"]
pub type NACKRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NACKRIS`"]
pub struct NACKRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLKRIS`"]
pub type CLKRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKRIS`"]
pub struct CLKRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKRIS_W<'a> {
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
#[doc = "Reader of field `RIS`"]
pub type RIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RIS`"]
pub struct RIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RIS_W<'a> {
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
    #[doc = "Bit 11 - Receive FIFO full raw interrupt status"]
    #[inline(always)]
    pub fn rxffris(&self) -> RXFFRIS_R {
        RXFFRIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO empty raw interrupt status"]
    #[inline(always)]
    pub fn txferis(&self) -> TXFERIS_R {
        TXFERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO request raw interrupt status"]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit FIFO request raw interrupt status"]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Arbitration lost raw interrupt status"]
    #[inline(always)]
    pub fn arblostris(&self) -> ARBLOSTRIS_R {
        ARBLOSTRIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stop detection raw interrupt status"]
    #[inline(always)]
    pub fn stopris(&self) -> STOPRIS_R {
        STOPRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Start detection raw interrupt status"]
    #[inline(always)]
    pub fn startris(&self) -> STARTRIS_R {
        STARTRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Address/data NACK raw interrupt status"]
    #[inline(always)]
    pub fn nackris(&self) -> NACKRIS_R {
        NACKRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA transmit raw interrupt status"]
    #[inline(always)]
    pub fn dmatxris(&self) -> DMATXRIS_R {
        DMATXRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA receive raw interrupt status"]
    #[inline(always)]
    pub fn dmarxris(&self) -> DMARXRIS_R {
        DMARXRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock timeout raw interrupt status"]
    #[inline(always)]
    pub fn clkris(&self) -> CLKRIS_R {
        CLKRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Master raw interrupt status"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Receive FIFO full raw interrupt status"]
    #[inline(always)]
    pub fn rxffris(&mut self) -> RXFFRIS_W {
        RXFFRIS_W { w: self }
    }
    #[doc = "Bit 10 - Transmit FIFO empty raw interrupt status"]
    #[inline(always)]
    pub fn txferis(&mut self) -> TXFERIS_W {
        TXFERIS_W { w: self }
    }
    #[doc = "Bit 9 - Receive FIFO request raw interrupt status"]
    #[inline(always)]
    pub fn rxris(&mut self) -> RXRIS_W {
        RXRIS_W { w: self }
    }
    #[doc = "Bit 8 - Transmit FIFO request raw interrupt status"]
    #[inline(always)]
    pub fn txris(&mut self) -> TXRIS_W {
        TXRIS_W { w: self }
    }
    #[doc = "Bit 7 - Arbitration lost raw interrupt status"]
    #[inline(always)]
    pub fn arblostris(&mut self) -> ARBLOSTRIS_W {
        ARBLOSTRIS_W { w: self }
    }
    #[doc = "Bit 6 - Stop detection raw interrupt status"]
    #[inline(always)]
    pub fn stopris(&mut self) -> STOPRIS_W {
        STOPRIS_W { w: self }
    }
    #[doc = "Bit 5 - Start detection raw interrupt status"]
    #[inline(always)]
    pub fn startris(&mut self) -> STARTRIS_W {
        STARTRIS_W { w: self }
    }
    #[doc = "Bit 4 - Address/data NACK raw interrupt status"]
    #[inline(always)]
    pub fn nackris(&mut self) -> NACKRIS_W {
        NACKRIS_W { w: self }
    }
    #[doc = "Bit 3 - DMA transmit raw interrupt status"]
    #[inline(always)]
    pub fn dmatxris(&mut self) -> DMATXRIS_W {
        DMATXRIS_W { w: self }
    }
    #[doc = "Bit 2 - DMA receive raw interrupt status"]
    #[inline(always)]
    pub fn dmarxris(&mut self) -> DMARXRIS_W {
        DMARXRIS_W { w: self }
    }
    #[doc = "Bit 1 - Clock timeout raw interrupt status"]
    #[inline(always)]
    pub fn clkris(&mut self) -> CLKRIS_W {
        CLKRIS_W { w: self }
    }
    #[doc = "Bit 0 - Master raw interrupt status"]
    #[inline(always)]
    pub fn ris(&mut self) -> RIS_W {
        RIS_W { w: self }
    }
}
