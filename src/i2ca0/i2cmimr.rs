#[doc = "Reader of register I2CMIMR"]
pub type R = crate::R<u32, super::I2CMIMR>;
#[doc = "Writer for register I2CMIMR"]
pub type W = crate::W<u32, super::I2CMIMR>;
#[doc = "Register I2CMIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CMIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXFFIM`"]
pub type RXFFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFFIM`"]
pub struct RXFFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFIM_W<'a> {
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
#[doc = "Reader of field `TXFEIM`"]
pub type TXFEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFEIM`"]
pub struct TXFEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFEIM_W<'a> {
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
#[doc = "Reader of field `RXIM`"]
pub type RXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIM`"]
pub struct RXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIM_W<'a> {
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
#[doc = "Reader of field `TXIM`"]
pub type TXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXIM`"]
pub struct TXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIM_W<'a> {
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
#[doc = "Reader of field `ARBLOSTIM`"]
pub type ARBLOSTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBLOSTIM`"]
pub struct ARBLOSTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLOSTIM_W<'a> {
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
#[doc = "Reader of field `STOPIM`"]
pub type STOPIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPIM`"]
pub struct STOPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPIM_W<'a> {
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
#[doc = "Reader of field `STARTIM`"]
pub type STARTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTIM`"]
pub struct STARTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTIM_W<'a> {
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
#[doc = "Reader of field `NACKIM`"]
pub type NACKIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NACKIM`"]
pub struct NACKIM_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKIM_W<'a> {
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
#[doc = "Reader of field `DMATXIM`"]
pub type DMATXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMATXIM`"]
pub struct DMATXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATXIM_W<'a> {
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
#[doc = "Reader of field `DMARXIM`"]
pub type DMARXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARXIM`"]
pub struct DMARXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARXIM_W<'a> {
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
#[doc = "Reader of field `CLKIM`"]
pub type CLKIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKIM`"]
pub struct CLKIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKIM_W<'a> {
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
#[doc = "Reader of field `IM`"]
pub type IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM`"]
pub struct IM_W<'a> {
    w: &'a mut W,
}
impl<'a> IM_W<'a> {
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
    #[doc = "Bit 11 - Receive FIFO full interrupt mask"]
    #[inline(always)]
    pub fn rxffim(&self) -> RXFFIM_R {
        RXFFIM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO empty interrupt mask"]
    #[inline(always)]
    pub fn txfeim(&self) -> TXFEIM_R {
        TXFEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO request interrupt mask"]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit FIFO request interrupt mask"]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Arbitration lost interrupt mask"]
    #[inline(always)]
    pub fn arblostim(&self) -> ARBLOSTIM_R {
        ARBLOSTIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stop detection interrupt mask"]
    #[inline(always)]
    pub fn stopim(&self) -> STOPIM_R {
        STOPIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Start detection interrupt mask"]
    #[inline(always)]
    pub fn startim(&self) -> STARTIM_R {
        STARTIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Address/data NACK interrupt mask"]
    #[inline(always)]
    pub fn nackim(&self) -> NACKIM_R {
        NACKIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA transmit interrupt mask"]
    #[inline(always)]
    pub fn dmatxim(&self) -> DMATXIM_R {
        DMATXIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA receive interrupt mask"]
    #[inline(always)]
    pub fn dmarxim(&self) -> DMARXIM_R {
        DMARXIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock timeout interrupt mask"]
    #[inline(always)]
    pub fn clkim(&self) -> CLKIM_R {
        CLKIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Master interrupt mask"]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Receive FIFO full interrupt mask"]
    #[inline(always)]
    pub fn rxffim(&mut self) -> RXFFIM_W {
        RXFFIM_W { w: self }
    }
    #[doc = "Bit 10 - Transmit FIFO empty interrupt mask"]
    #[inline(always)]
    pub fn txfeim(&mut self) -> TXFEIM_W {
        TXFEIM_W { w: self }
    }
    #[doc = "Bit 9 - Receive FIFO request interrupt mask"]
    #[inline(always)]
    pub fn rxim(&mut self) -> RXIM_W {
        RXIM_W { w: self }
    }
    #[doc = "Bit 8 - Transmit FIFO request interrupt mask"]
    #[inline(always)]
    pub fn txim(&mut self) -> TXIM_W {
        TXIM_W { w: self }
    }
    #[doc = "Bit 7 - Arbitration lost interrupt mask"]
    #[inline(always)]
    pub fn arblostim(&mut self) -> ARBLOSTIM_W {
        ARBLOSTIM_W { w: self }
    }
    #[doc = "Bit 6 - Stop detection interrupt mask"]
    #[inline(always)]
    pub fn stopim(&mut self) -> STOPIM_W {
        STOPIM_W { w: self }
    }
    #[doc = "Bit 5 - Start detection interrupt mask"]
    #[inline(always)]
    pub fn startim(&mut self) -> STARTIM_W {
        STARTIM_W { w: self }
    }
    #[doc = "Bit 4 - Address/data NACK interrupt mask"]
    #[inline(always)]
    pub fn nackim(&mut self) -> NACKIM_W {
        NACKIM_W { w: self }
    }
    #[doc = "Bit 3 - DMA transmit interrupt mask"]
    #[inline(always)]
    pub fn dmatxim(&mut self) -> DMATXIM_W {
        DMATXIM_W { w: self }
    }
    #[doc = "Bit 2 - DMA receive interrupt mask"]
    #[inline(always)]
    pub fn dmarxim(&mut self) -> DMARXIM_W {
        DMARXIM_W { w: self }
    }
    #[doc = "Bit 1 - Clock timeout interrupt mask"]
    #[inline(always)]
    pub fn clkim(&mut self) -> CLKIM_W {
        CLKIM_W { w: self }
    }
    #[doc = "Bit 0 - Master interrupt mask"]
    #[inline(always)]
    pub fn im(&mut self) -> IM_W {
        IM_W { w: self }
    }
}
