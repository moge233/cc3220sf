#[doc = "Reader of register GPTMIMR"]
pub type R = crate::R<u32, super::GPTMIMR>;
#[doc = "Writer for register GPTMIMR"]
pub type W = crate::W<u32, super::GPTMIMR>;
#[doc = "Register GPTMIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTMIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMABIM`"]
pub type DMABIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMABIM`"]
pub struct DMABIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMABIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TBMIM`"]
pub type TBMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMIM`"]
pub struct TBMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMIM_W<'a> {
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
#[doc = "Reader of field `CBEIM`"]
pub type CBEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBEIM`"]
pub struct CBEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CBEIM_W<'a> {
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
#[doc = "Reader of field `CBMIM`"]
pub type CBMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBMIM`"]
pub struct CBMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMIM_W<'a> {
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
#[doc = "Reader of field `TBTOIM`"]
pub type TBTOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBTOIM`"]
pub struct TBTOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTOIM_W<'a> {
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
#[doc = "Reader of field `DMAAIM`"]
pub type DMAAIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAAIM`"]
pub struct DMAAIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAAIM_W<'a> {
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
#[doc = "Reader of field `TAMIM`"]
pub type TAMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMIM`"]
pub struct TAMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMIM_W<'a> {
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
#[doc = "Reader of field `CAEIM`"]
pub type CAEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAEIM`"]
pub struct CAEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEIM_W<'a> {
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
#[doc = "Reader of field `CAMIM`"]
pub type CAMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAMIM`"]
pub struct CAMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMIM_W<'a> {
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
#[doc = "Reader of field `TATOIM`"]
pub type TATOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TATOIM`"]
pub struct TATOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TATOIM_W<'a> {
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
    #[doc = "Bit 13 - GPTM Timer B DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn dmabim(&self) -> DMABIM_R {
        DMABIM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Interrupt Mask"]
    #[inline(always)]
    pub fn tbmim(&self) -> TBMIM_R {
        TBMIM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn cbeim(&self) -> CBEIM_R {
        CBEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn cbmim(&self) -> CBMIM_R {
        CBMIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Time Out Interrupt Mask"]
    #[inline(always)]
    pub fn tbtoim(&self) -> TBTOIM_R {
        TBTOIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn dmaaim(&self) -> DMAAIM_R {
        DMAAIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Interrupt Mask"]
    #[inline(always)]
    pub fn tamim(&self) -> TAMIM_R {
        TAMIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn caeim(&self) -> CAEIM_R {
        CAEIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn camim(&self) -> CAMIM_R {
        CAMIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPTM Timer A Time Out Interrupt Mask"]
    #[inline(always)]
    pub fn tatoim(&self) -> TATOIM_R {
        TATOIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - GPTM Timer B DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn dmabim(&mut self) -> DMABIM_W {
        DMABIM_W { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer B Match Interrupt Mask"]
    #[inline(always)]
    pub fn tbmim(&mut self) -> TBMIM_W {
        TBMIM_W { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn cbeim(&mut self) -> CBEIM_W {
        CBEIM_W { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn cbmim(&mut self) -> CBMIM_W {
        CBMIM_W { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Time Out Interrupt Mask"]
    #[inline(always)]
    pub fn tbtoim(&mut self) -> TBTOIM_W {
        TBTOIM_W { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn dmaaim(&mut self) -> DMAAIM_W {
        DMAAIM_W { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer A Match Interrupt Mask"]
    #[inline(always)]
    pub fn tamim(&mut self) -> TAMIM_W {
        TAMIM_W { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn caeim(&mut self) -> CAEIM_W {
        CAEIM_W { w: self }
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn camim(&mut self) -> CAMIM_W {
        CAMIM_W { w: self }
    }
    #[doc = "Bit 0 - GPTM Timer A Time Out Interrupt Mask"]
    #[inline(always)]
    pub fn tatoim(&mut self) -> TATOIM_W {
        TATOIM_W { w: self }
    }
}
