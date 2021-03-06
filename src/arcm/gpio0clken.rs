#[doc = "Reader of register GPIO0CLKEN"]
pub type R = crate::R<u32, super::GPIO0CLKEN>;
#[doc = "Writer for register GPIO0CLKEN"]
pub type W = crate::W<u32, super::GPIO0CLKEN>;
#[doc = "Register GPIO0CLKEN `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO0CLKEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSLPCLKEN`"]
pub type DSLPCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSLPCLKEN`"]
pub struct DSLPCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSLPCLKEN_W<'a> {
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
#[doc = "Reader of field `NU1`"]
pub type NU1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NU1`"]
pub struct NU1_W<'a> {
    w: &'a mut W,
}
impl<'a> NU1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `SLPCLKEN`"]
pub type SLPCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLPCLKEN`"]
pub struct SLPCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLPCLKEN_W<'a> {
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
#[doc = "Reader of field `NU2`"]
pub type NU2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NU2`"]
pub struct NU2_W<'a> {
    w: &'a mut W,
}
impl<'a> NU2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `RUNCLKEN`"]
pub type RUNCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNCLKEN`"]
pub struct RUNCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNCLKEN_W<'a> {
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
    #[doc = "Bit 16 - GPIO_A_DSLP_CLK_ENABLE"]
    #[inline(always)]
    pub fn dslpclken(&self) -> DSLPCLKEN_R {
        DSLPCLKEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 9:15 - NU1"]
    #[inline(always)]
    pub fn nu1(&self) -> NU1_R {
        NU1_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - GPIO_A_SLP_CLK_ENABLE"]
    #[inline(always)]
    pub fn slpclken(&self) -> SLPCLKEN_R {
        SLPCLKEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - NU2"]
    #[inline(always)]
    pub fn nu2(&self) -> NU2_R {
        NU2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - GPIO_A_RUN_CLK_ENABLE"]
    #[inline(always)]
    pub fn runclken(&self) -> RUNCLKEN_R {
        RUNCLKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - GPIO_A_DSLP_CLK_ENABLE"]
    #[inline(always)]
    pub fn dslpclken(&mut self) -> DSLPCLKEN_W {
        DSLPCLKEN_W { w: self }
    }
    #[doc = "Bits 9:15 - NU1"]
    #[inline(always)]
    pub fn nu1(&mut self) -> NU1_W {
        NU1_W { w: self }
    }
    #[doc = "Bit 8 - GPIO_A_SLP_CLK_ENABLE"]
    #[inline(always)]
    pub fn slpclken(&mut self) -> SLPCLKEN_W {
        SLPCLKEN_W { w: self }
    }
    #[doc = "Bits 1:7 - NU2"]
    #[inline(always)]
    pub fn nu2(&mut self) -> NU2_W {
        NU2_W { w: self }
    }
    #[doc = "Bit 0 - GPIO_A_RUN_CLK_ENABLE"]
    #[inline(always)]
    pub fn runclken(&mut self) -> RUNCLKEN_W {
        RUNCLKEN_W { w: self }
    }
}
