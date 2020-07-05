#[doc = "Reader of register SH_SPI_CS_MASK"]
pub type R = crate::R<u32, super::SH_SPI_CS_MASK>;
#[doc = "Writer for register SH_SPI_CS_MASK"]
pub type W = crate::W<u32, super::SH_SPI_CS_MASK>;
#[doc = "Register SH_SPI_CS_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::SH_SPI_CS_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_SH_SPI_CS_MASK`"]
pub type MEM_SH_SPI_CS_MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SH_SPI_CS_MASK`"]
pub struct MEM_SH_SPI_CS_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SH_SPI_CS_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ( chip select 0 is unmasked after reset. When 1 : CS is unmasked or else masked. Valid configurations are 1000, 0100, 0010 or 0001. Any other setting can lead to unpredictable behavior."]
    #[inline(always)]
    pub fn mem_sh_spi_cs_mask(&self) -> MEM_SH_SPI_CS_MASK_R {
        MEM_SH_SPI_CS_MASK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ( chip select 0 is unmasked after reset. When 1 : CS is unmasked or else masked. Valid configurations are 1000, 0100, 0010 or 0001. Any other setting can lead to unpredictable behavior."]
    #[inline(always)]
    pub fn mem_sh_spi_cs_mask(&mut self) -> MEM_SH_SPI_CS_MASK_W {
        MEM_SH_SPI_CS_MASK_W { w: self }
    }
}
