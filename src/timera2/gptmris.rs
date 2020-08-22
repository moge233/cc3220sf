#[doc = "Reader of register GPTMRIS"]
pub type R = crate::R<u32, super::GPTMRIS>;
#[doc = "Writer for register GPTMRIS"]
pub type W = crate::W<u32, super::GPTMRIS>;
#[doc = "Register GPTMRIS `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTMRIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMABRIS`"]
pub type DMABRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMABRIS`"]
pub struct DMABRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMABRIS_W<'a> {
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
#[doc = "Reader of field `TBMRIS`"]
pub type TBMRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMRIS`"]
pub struct TBMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMRIS_W<'a> {
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
#[doc = "Reader of field `CBERIS`"]
pub type CBERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBERIS`"]
pub struct CBERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CBERIS_W<'a> {
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
#[doc = "Reader of field `CBMRIS`"]
pub type CBMRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBMRIS`"]
pub struct CBMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMRIS_W<'a> {
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
#[doc = "Reader of field `TBTORIS`"]
pub type TBTORIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBTORIS`"]
pub struct TBTORIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTORIS_W<'a> {
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
#[doc = "Reader of field `DMAARIS`"]
pub type DMAARIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAARIS`"]
pub struct DMAARIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAARIS_W<'a> {
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
#[doc = "Reader of field `TAMRIS`"]
pub type TAMRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMRIS`"]
pub struct TAMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMRIS_W<'a> {
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
#[doc = "Reader of field `CAERIS`"]
pub type CAERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAERIS`"]
pub struct CAERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAERIS_W<'a> {
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
#[doc = "Reader of field `CAMRIS`"]
pub type CAMRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAMRIS`"]
pub struct CAMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMRIS_W<'a> {
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
#[doc = "Reader of field `TATORIS`"]
pub type TATORIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TATORIS`"]
pub struct TATORIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TATORIS_W<'a> {
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
    #[doc = "Bit 13 - GPTM Timer B DMA Done Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmabris(&self) -> DMABRIS_R {
        DMABRIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Raw Interrupt Status"]
    #[inline(always)]
    pub fn tbmris(&self) -> TBMRIS_R {
        TBMRIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Raw Interrupt Status"]
    #[inline(always)]
    pub fn cberis(&self) -> CBERIS_R {
        CBERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Raw Interrupt Status"]
    #[inline(always)]
    pub fn cbmris(&self) -> CBMRIS_R {
        CBMRIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Time Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn tbtoris(&self) -> TBTORIS_R {
        TBTORIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmaaris(&self) -> DMAARIS_R {
        DMAARIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Raw Interrupt Status"]
    #[inline(always)]
    pub fn tamris(&self) -> TAMRIS_R {
        TAMRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Raw Interrupt Status"]
    #[inline(always)]
    pub fn caeris(&self) -> CAERIS_R {
        CAERIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Raw Interrupt Status"]
    #[inline(always)]
    pub fn camris(&self) -> CAMRIS_R {
        CAMRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPTM Timer A Time Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn tatoris(&self) -> TATORIS_R {
        TATORIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - GPTM Timer B DMA Done Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmabris(&mut self) -> DMABRIS_W {
        DMABRIS_W { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer B Match Raw Interrupt Status"]
    #[inline(always)]
    pub fn tbmris(&mut self) -> TBMRIS_W {
        TBMRIS_W { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Raw Interrupt Status"]
    #[inline(always)]
    pub fn cberis(&mut self) -> CBERIS_W {
        CBERIS_W { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Raw Interrupt Status"]
    #[inline(always)]
    pub fn cbmris(&mut self) -> CBMRIS_W {
        CBMRIS_W { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Time Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn tbtoris(&mut self) -> TBTORIS_W {
        TBTORIS_W { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmaaris(&mut self) -> DMAARIS_W {
        DMAARIS_W { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer A Match Raw Interrupt Status"]
    #[inline(always)]
    pub fn tamris(&mut self) -> TAMRIS_W {
        TAMRIS_W { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Raw Interrupt Status"]
    #[inline(always)]
    pub fn caeris(&mut self) -> CAERIS_W {
        CAERIS_W { w: self }
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Raw Interrupt Status"]
    #[inline(always)]
    pub fn camris(&mut self) -> CAMRIS_W {
        CAMRIS_W { w: self }
    }
    #[doc = "Bit 0 - GPTM Timer A Time Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn tatoris(&mut self) -> TATORIS_W {
        TATORIS_W { w: self }
    }
}
