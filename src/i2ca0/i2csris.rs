#[doc = "Reader of register I2CSRIS"]
pub type R = crate::R<u32, super::I2CSRIS>;
#[doc = "Writer for register I2CSRIS"]
pub type W = crate::W<u32, super::I2CSRIS>;
#[doc = "Register I2CSRIS `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CSRIS {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DATARIS`"]
pub type DATARIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATARIS`"]
pub struct DATARIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DATARIS_W<'a> {
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
    #[doc = "Bit 8 - Receive FIFO full raw interrupt status"]
    #[inline(always)]
    pub fn rxffris(&self) -> RXFFRIS_R {
        RXFFRIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty raw interrupt status"]
    #[inline(always)]
    pub fn txferis(&self) -> TXFERIS_R {
        TXFERIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO request raw interrupt status"]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO request raw interrupt status"]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit DMA raw interrupt status"]
    #[inline(always)]
    pub fn dmatxris(&self) -> DMATXRIS_R {
        DMATXRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive DMA raw interrupt status"]
    #[inline(always)]
    pub fn dmarxris(&self) -> DMARXRIS_R {
        DMARXRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop condition raw interrupt status"]
    #[inline(always)]
    pub fn stopris(&self) -> STOPRIS_R {
        STOPRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start condition raw interrupt status"]
    #[inline(always)]
    pub fn startris(&self) -> STARTRIS_R {
        STARTRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data raw interrupt status"]
    #[inline(always)]
    pub fn dataris(&self) -> DATARIS_R {
        DATARIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Receive FIFO full raw interrupt status"]
    #[inline(always)]
    pub fn rxffris(&mut self) -> RXFFRIS_W {
        RXFFRIS_W { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO empty raw interrupt status"]
    #[inline(always)]
    pub fn txferis(&mut self) -> TXFERIS_W {
        TXFERIS_W { w: self }
    }
    #[doc = "Bit 6 - Receive FIFO request raw interrupt status"]
    #[inline(always)]
    pub fn rxris(&mut self) -> RXRIS_W {
        RXRIS_W { w: self }
    }
    #[doc = "Bit 5 - Transmit FIFO request raw interrupt status"]
    #[inline(always)]
    pub fn txris(&mut self) -> TXRIS_W {
        TXRIS_W { w: self }
    }
    #[doc = "Bit 4 - Transmit DMA raw interrupt status"]
    #[inline(always)]
    pub fn dmatxris(&mut self) -> DMATXRIS_W {
        DMATXRIS_W { w: self }
    }
    #[doc = "Bit 3 - Receive DMA raw interrupt status"]
    #[inline(always)]
    pub fn dmarxris(&mut self) -> DMARXRIS_W {
        DMARXRIS_W { w: self }
    }
    #[doc = "Bit 2 - Stop condition raw interrupt status"]
    #[inline(always)]
    pub fn stopris(&mut self) -> STOPRIS_W {
        STOPRIS_W { w: self }
    }
    #[doc = "Bit 1 - Start condition raw interrupt status"]
    #[inline(always)]
    pub fn startris(&mut self) -> STARTRIS_W {
        STARTRIS_W { w: self }
    }
    #[doc = "Bit 1 - Data raw interrupt status"]
    #[inline(always)]
    pub fn dataris(&mut self) -> DATARIS_W {
        DATARIS_W { w: self }
    }
}
