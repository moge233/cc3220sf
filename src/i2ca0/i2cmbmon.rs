#[doc = "Reader of register I2CMBMON"]
pub type R = crate::R<u32, super::I2CMBMON>;
#[doc = "Writer for register I2CMBMON"]
pub type W = crate::W<u32, super::I2CMBMON>;
#[doc = "Register I2CMBMON `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CMBMON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDA`"]
pub type SDA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDA`"]
pub struct SDA_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_W<'a> {
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
#[doc = "Reader of field `SCL`"]
pub type SCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCL`"]
pub struct SCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_W<'a> {
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
    #[doc = "Bit 1 - I2C SDA status"]
    #[inline(always)]
    pub fn sda(&self) -> SDA_R {
        SDA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - I2C SCL status"]
    #[inline(always)]
    pub fn scl(&self) -> SCL_R {
        SCL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - I2C SDA status"]
    #[inline(always)]
    pub fn sda(&mut self) -> SDA_W {
        SDA_W { w: self }
    }
    #[doc = "Bit 0 - I2C SCL status"]
    #[inline(always)]
    pub fn scl(&mut self) -> SCL_W {
        SCL_W { w: self }
    }
}
