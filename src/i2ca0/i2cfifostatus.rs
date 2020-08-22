#[doc = "Reader of register I2CFIFOSTATUS"]
pub type R = crate::R<u32, super::I2CFIFOSTATUS>;
#[doc = "Writer for register I2CFIFOSTATUS"]
pub type W = crate::W<u32, super::I2CFIFOSTATUS>;
#[doc = "Register I2CFIFOSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CFIFOSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXABVTRIG`"]
pub type RXABVTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXABVTRIG`"]
pub struct RXABVTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RXABVTRIG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `RXFF`"]
pub type RXFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFF`"]
pub struct RXFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFF_W<'a> {
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
#[doc = "Reader of field `RXFE`"]
pub type RXFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFE`"]
pub struct RXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFE_W<'a> {
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
#[doc = "Reader of field `TXBLWTRIG`"]
pub type TXBLWTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBLWTRIG`"]
pub struct TXBLWTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBLWTRIG_W<'a> {
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
#[doc = "Reader of field `TXFF`"]
pub type TXFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFF`"]
pub struct TXFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFF_W<'a> {
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
#[doc = "Reader of field `TXFE`"]
pub type TXFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFE`"]
pub struct TXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFE_W<'a> {
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
    #[doc = "Bit 18 - RX FIFO above trigger level"]
    #[inline(always)]
    pub fn rxabvtrig(&self) -> RXABVTRIG_R {
        RXABVTRIG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RX FIFO full"]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RX FIFO empty"]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TX FIFO below trigger level"]
    #[inline(always)]
    pub fn txblwtrig(&self) -> TXBLWTRIG_R {
        TXBLWTRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TX FIFO full"]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TX FIFO empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - RX FIFO above trigger level"]
    #[inline(always)]
    pub fn rxabvtrig(&mut self) -> RXABVTRIG_W {
        RXABVTRIG_W { w: self }
    }
    #[doc = "Bit 17 - RX FIFO full"]
    #[inline(always)]
    pub fn rxff(&mut self) -> RXFF_W {
        RXFF_W { w: self }
    }
    #[doc = "Bit 16 - RX FIFO empty"]
    #[inline(always)]
    pub fn rxfe(&mut self) -> RXFE_W {
        RXFE_W { w: self }
    }
    #[doc = "Bit 2 - TX FIFO below trigger level"]
    #[inline(always)]
    pub fn txblwtrig(&mut self) -> TXBLWTRIG_W {
        TXBLWTRIG_W { w: self }
    }
    #[doc = "Bit 1 - TX FIFO full"]
    #[inline(always)]
    pub fn txff(&mut self) -> TXFF_W {
        TXFF_W { w: self }
    }
    #[doc = "Bit 0 - TX FIFO empty"]
    #[inline(always)]
    pub fn txfe(&mut self) -> TXFE_W {
        TXFE_W { w: self }
    }
}
