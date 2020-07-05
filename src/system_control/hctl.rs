#[doc = "Reader of register HCTL"]
pub type R = crate::R<u32, super::HCTL>;
#[doc = "Writer for register HCTL"]
pub type W = crate::W<u32, super::HCTL>;
#[doc = "Register HCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::HCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDVS`"]
pub type SDVS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDVS`"]
pub struct SDVS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `DMAS`"]
pub type DMAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMAS`"]
pub struct DMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:11 - SD bus voltage select All cards. The host driver should set to these bits to select the voltage level for the card according to the voltage supported by the system (MMCHS_CAPA\\[VS18VS30VS33\\]) before starting a transfer. 0x5 1.8V (Typical) 0x6 3.0V (Typical) 0x7 3.3V (Typical)"]
    #[inline(always)]
    pub fn sdvs(&self) -> SDVS_R {
        SDVS_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - DMA Select Mode: One of supported DMA modes can be selected. The host driver shall check support of DMA modes by referring the Capabilities register. Use of selected DMA is determined by DMA Enable of the Transfer Mode register. This register is only meaningful when MADMA_EN is set to 1. When MADMA_EN is set to 0 the bit field is read only and returned value is 0. 0x0 Reserved 0x1 Reserved 0x2 32-bit Address ADMA2 is selected 0x3 Reserved"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 9:11 - SD bus voltage select All cards. The host driver should set to these bits to select the voltage level for the card according to the voltage supported by the system (MMCHS_CAPA\\[VS18VS30VS33\\]) before starting a transfer. 0x5 1.8V (Typical) 0x6 3.0V (Typical) 0x7 3.3V (Typical)"]
    #[inline(always)]
    pub fn sdvs(&mut self) -> SDVS_W {
        SDVS_W { w: self }
    }
    #[doc = "Bits 3:4 - DMA Select Mode: One of supported DMA modes can be selected. The host driver shall check support of DMA modes by referring the Capabilities register. Use of selected DMA is determined by DMA Enable of the Transfer Mode register. This register is only meaningful when MADMA_EN is set to 1. When MADMA_EN is set to 0 the bit field is read only and returned value is 0. 0x0 Reserved 0x1 Reserved 0x2 32-bit Address ADMA2 is selected 0x3 Reserved"]
    #[inline(always)]
    pub fn dmas(&mut self) -> DMAS_W {
        DMAS_W { w: self }
    }
}
