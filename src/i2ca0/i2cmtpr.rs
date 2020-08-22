#[doc = "Reader of register I2CMTPR"]
pub type R = crate::R<u32, super::I2CMTPR>;
#[doc = "Writer for register I2CMTPR"]
pub type W = crate::W<u32, super::I2CMTPR>;
#[doc = "Register I2CMTPR `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CMTPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PULSEL`"]
pub type PULSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PULSEL`"]
pub struct PULSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `TPR`"]
pub type TPR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TPR`"]
pub struct TPR_W<'a> {
    w: &'a mut W,
}
impl<'a> TPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18 - Glitch suppression pulse width"]
    #[inline(always)]
    pub fn pulsel(&self) -> PULSEL_R {
        PULSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 0:6 - Timer period"]
    #[inline(always)]
    pub fn tpr(&self) -> TPR_R {
        TPR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - Glitch suppression pulse width"]
    #[inline(always)]
    pub fn pulsel(&mut self) -> PULSEL_W {
        PULSEL_W { w: self }
    }
    #[doc = "Bits 0:6 - Timer period"]
    #[inline(always)]
    pub fn tpr(&mut self) -> TPR_W {
        TPR_W { w: self }
    }
}
