#[doc = "Reader of register I2CMCS"]
pub type R = crate::R<u32, super::I2CMCS>;
#[doc = "Writer for register I2CMCS"]
pub type W = crate::W<u32, super::I2CMCS>;
#[doc = "Register I2CMCS `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CMCS {
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
#[doc = "Reader of field `CLKTO`"]
pub type CLKTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKTO`"]
pub struct CLKTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKTO_W<'a> {
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
#[doc = "Reader of field `BUSBUSY_OR_BURST`"]
pub type BUSBUSY_OR_BURST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSBUSY_OR_BURST`"]
pub struct BUSBUSY_OR_BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSBUSY_OR_BURST_W<'a> {
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
#[doc = "Reader of field `IDLE_OR_QCMD`"]
pub type IDLE_OR_QCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDLE_OR_QCMD`"]
pub struct IDLE_OR_QCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_OR_QCMD_W<'a> {
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
#[doc = "Reader of field `ARBLST_OR_HS`"]
pub type ARBLST_OR_HS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBLST_OR_HS`"]
pub struct ARBLST_OR_HS_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLST_OR_HS_W<'a> {
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
#[doc = "Reader of field `DATACK_OR_ACK`"]
pub type DATACK_OR_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATACK_OR_ACK`"]
pub struct DATACK_OR_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DATACK_OR_ACK_W<'a> {
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
#[doc = "Reader of field `ADRACK_OR_STOP`"]
pub type ADRACK_OR_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADRACK_OR_STOP`"]
pub struct ADRACK_OR_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRACK_OR_STOP_W<'a> {
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
#[doc = "Reader of field `ERROR_OR_START`"]
pub type ERROR_OR_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR_OR_START`"]
pub struct ERROR_OR_START_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_OR_START_W<'a> {
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
#[doc = "Reader of field `BUSY_OR_RUN`"]
pub type BUSY_OR_RUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSY_OR_RUN`"]
pub struct BUSY_OR_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_OR_RUN_W<'a> {
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
    #[doc = "Bit 31 - I2C DMA RX active status"]
    #[inline(always)]
    pub fn actdmarx(&self) -> ACTDMARX_R {
        ACTDMARX_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - I2C DMA TX active status"]
    #[inline(always)]
    pub fn actdmatx(&self) -> ACTDMATX_R {
        ACTDMATX_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C clock timeout error"]
    #[inline(always)]
    pub fn clkto(&self) -> CLKTO_R {
        CLKTO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C bus busy or burst enable"]
    #[inline(always)]
    pub fn busbusy_or_burst(&self) -> BUSBUSY_OR_BURST_R {
        BUSBUSY_OR_BURST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C idle or quick command"]
    #[inline(always)]
    pub fn idle_or_qcmd(&self) -> IDLE_OR_QCMD_R {
        IDLE_OR_QCMD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C arbitration lost (high speed not supported)"]
    #[inline(always)]
    pub fn arblst_or_hs(&self) -> ARBLST_OR_HS_R {
        ARBLST_OR_HS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C acknowledge data or data acknowledge enable"]
    #[inline(always)]
    pub fn datack_or_ack(&self) -> DATACK_OR_ACK_R {
        DATACK_OR_ACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C bus busy or burst enable"]
    #[inline(always)]
    pub fn adrack_or_stop(&self) -> ADRACK_OR_STOP_R {
        ADRACK_OR_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C error or generate start"]
    #[inline(always)]
    pub fn error_or_start(&self) -> ERROR_OR_START_R {
        ERROR_OR_START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - I2C busy or master enable"]
    #[inline(always)]
    pub fn busy_or_run(&self) -> BUSY_OR_RUN_R {
        BUSY_OR_RUN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - I2C DMA RX active status"]
    #[inline(always)]
    pub fn actdmarx(&mut self) -> ACTDMARX_W {
        ACTDMARX_W { w: self }
    }
    #[doc = "Bit 30 - I2C DMA TX active status"]
    #[inline(always)]
    pub fn actdmatx(&mut self) -> ACTDMATX_W {
        ACTDMATX_W { w: self }
    }
    #[doc = "Bit 7 - I2C clock timeout error"]
    #[inline(always)]
    pub fn clkto(&mut self) -> CLKTO_W {
        CLKTO_W { w: self }
    }
    #[doc = "Bit 6 - I2C bus busy or burst enable"]
    #[inline(always)]
    pub fn busbusy_or_burst(&mut self) -> BUSBUSY_OR_BURST_W {
        BUSBUSY_OR_BURST_W { w: self }
    }
    #[doc = "Bit 5 - I2C idle or quick command"]
    #[inline(always)]
    pub fn idle_or_qcmd(&mut self) -> IDLE_OR_QCMD_W {
        IDLE_OR_QCMD_W { w: self }
    }
    #[doc = "Bit 4 - I2C arbitration lost (high speed not supported)"]
    #[inline(always)]
    pub fn arblst_or_hs(&mut self) -> ARBLST_OR_HS_W {
        ARBLST_OR_HS_W { w: self }
    }
    #[doc = "Bit 3 - I2C acknowledge data or data acknowledge enable"]
    #[inline(always)]
    pub fn datack_or_ack(&mut self) -> DATACK_OR_ACK_W {
        DATACK_OR_ACK_W { w: self }
    }
    #[doc = "Bit 2 - I2C bus busy or burst enable"]
    #[inline(always)]
    pub fn adrack_or_stop(&mut self) -> ADRACK_OR_STOP_W {
        ADRACK_OR_STOP_W { w: self }
    }
    #[doc = "Bit 1 - I2C error or generate start"]
    #[inline(always)]
    pub fn error_or_start(&mut self) -> ERROR_OR_START_W {
        ERROR_OR_START_W { w: self }
    }
    #[doc = "Bit 0 - I2C busy or master enable"]
    #[inline(always)]
    pub fn busy_or_run(&mut self) -> BUSY_OR_RUN_W {
        BUSY_OR_RUN_W { w: self }
    }
}
