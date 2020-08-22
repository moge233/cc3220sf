#[doc = "Reader of register I2CMSA"]
pub type R = crate::R<u32, super::I2CMSA>;
#[doc = "Writer for register I2CMSA"]
pub type W = crate::W<u32, super::I2CMSA>;
#[doc = "Register I2CMSA `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CMSA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SA`"]
pub type SA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SA`"]
pub struct SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `R_S`"]
pub type R_S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R_S`"]
pub struct R_S_W<'a> {
    w: &'a mut W,
}
impl<'a> R_S_W<'a> {
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
    #[doc = "Bits 1:7 - I2C Slave Address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - I2C Receive/Send"]
    #[inline(always)]
    pub fn r_s(&self) -> R_S_R {
        R_S_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - I2C Slave Address"]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W {
        SA_W { w: self }
    }
    #[doc = "Bit 0 - I2C Receive/Send"]
    #[inline(always)]
    pub fn r_s(&mut self) -> R_S_W {
        R_S_W { w: self }
    }
}
