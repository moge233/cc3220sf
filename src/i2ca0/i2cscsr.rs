#[doc = "Reader of register I2CSCSR"]
pub type R = crate::R<u32, super::I2CSCSR>;
#[doc = "Writer for register I2CSCSR"]
pub type W = crate::W<u32, super::I2CSCSR>;
#[doc = "Register I2CSCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CSCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACTDMARX`"]
pub type ACTDMARX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTDMARX`"]
pub struct ACTDMARX_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTDMARX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `ACTDMATX`"]
pub type ACTDMATX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTDMATX`"]
pub struct ACTDMATX_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTDMATX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `QCMDRW`"]
pub type QCMDRW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QCMDRW`"]
pub struct QCMDRW_W<'a> {
    w: &'a mut W,
}
impl<'a> QCMDRW_W<'a> {
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
#[doc = "Reader of field `QCMDST`"]
pub type QCMDST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QCMDST`"]
pub struct QCMDST_W<'a> {
    w: &'a mut W,
}
impl<'a> QCMDST_W<'a> {
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
#[doc = "Reader of field `OAR2SEL`"]
pub type OAR2SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OAR2SEL`"]
pub struct OAR2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OAR2SEL_W<'a> {
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
#[doc = "Reader of field `FBR_OR_RXFIFO`"]
pub type FBR_OR_RXFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBR_OR_RXFIFO`"]
pub struct FBR_OR_RXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> FBR_OR_RXFIFO_W<'a> {
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
#[doc = "Reader of field `TREQ_OR_TXFIFO`"]
pub type TREQ_OR_TXFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TREQ_OR_TXFIFO`"]
pub struct TREQ_OR_TXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> TREQ_OR_TXFIFO_W<'a> {
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
#[doc = "Reader of field `RREQ_OR_DA`"]
pub type RREQ_OR_DA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RREQ_OR_DA`"]
pub struct RREQ_OR_DA_W<'a> {
    w: &'a mut W,
}
impl<'a> RREQ_OR_DA_W<'a> {
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
    #[doc = "Bit 31 - DMA RX active status"]
    #[inline(always)]
    pub fn actdmarx(&self) -> ACTDMARX_R {
        ACTDMARX_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA TX active status"]
    #[inline(always)]
    pub fn actdmatx(&self) -> ACTDMATX_R {
        ACTDMATX_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Quick command read/write"]
    #[inline(always)]
    pub fn qcmdrw(&self) -> QCMDRW_R {
        QCMDRW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Quick command status"]
    #[inline(always)]
    pub fn qcmdst(&self) -> QCMDST_R {
        QCMDST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OAR2 address matched"]
    #[inline(always)]
    pub fn oar2sel(&self) -> OAR2SEL_R {
        OAR2SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - First byte received or RX FIFO enabled"]
    #[inline(always)]
    pub fn fbr_or_rxfifo(&self) -> FBR_OR_RXFIFO_R {
        FBR_OR_RXFIFO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit request or TX FIFO enabled"]
    #[inline(always)]
    pub fn treq_or_txfifo(&self) -> TREQ_OR_TXFIFO_R {
        TREQ_OR_TXFIFO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Receive request or device active"]
    #[inline(always)]
    pub fn rreq_or_da(&self) -> RREQ_OR_DA_R {
        RREQ_OR_DA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - DMA RX active status"]
    #[inline(always)]
    pub fn actdmarx(&mut self) -> ACTDMARX_W {
        ACTDMARX_W { w: self }
    }
    #[doc = "Bit 30 - DMA TX active status"]
    #[inline(always)]
    pub fn actdmatx(&mut self) -> ACTDMATX_W {
        ACTDMATX_W { w: self }
    }
    #[doc = "Bit 5 - Quick command read/write"]
    #[inline(always)]
    pub fn qcmdrw(&mut self) -> QCMDRW_W {
        QCMDRW_W { w: self }
    }
    #[doc = "Bit 4 - Quick command status"]
    #[inline(always)]
    pub fn qcmdst(&mut self) -> QCMDST_W {
        QCMDST_W { w: self }
    }
    #[doc = "Bit 3 - OAR2 address matched"]
    #[inline(always)]
    pub fn oar2sel(&mut self) -> OAR2SEL_W {
        OAR2SEL_W { w: self }
    }
    #[doc = "Bit 2 - First byte received or RX FIFO enabled"]
    #[inline(always)]
    pub fn fbr_or_rxfifo(&mut self) -> FBR_OR_RXFIFO_W {
        FBR_OR_RXFIFO_W { w: self }
    }
    #[doc = "Bit 1 - Transmit request or TX FIFO enabled"]
    #[inline(always)]
    pub fn treq_or_txfifo(&mut self) -> TREQ_OR_TXFIFO_W {
        TREQ_OR_TXFIFO_W { w: self }
    }
    #[doc = "Bit 0 - Receive request or device active"]
    #[inline(always)]
    pub fn rreq_or_da(&mut self) -> RREQ_OR_DA_W {
        RREQ_OR_DA_W { w: self }
    }
}
