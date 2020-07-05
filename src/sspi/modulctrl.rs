#[doc = "Reader of register MODULCTRL"]
pub type R = crate::R<u32, super::MODULCTRL>;
#[doc = "Writer for register MODULCTRL"]
pub type W = crate::W<u32, super::MODULCTRL>;
#[doc = "Register MODULCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MODULCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INITDLY`"]
pub type INITDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INITDLY`"]
pub struct INITDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> INITDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - Initial spi delay for first transfer: This register is an option only available in SINGLE master mode The controller waits for a delay to transmit the first spi word after channel enabled and corresponding TX register filled. This Delay is based on SPI output frequency clock No clock output provided to the boundary and chip select is not active in 4 pin mode within this period. 0x0 No delay for first spi transfer. 0x1 The controller wait 4 spi bus clock 0x2 The controller wait 8 spi bus clock 0x3 The controller wait 16 spi bus clock 0x4 The controller wait 32 spi bus clock"]
    #[inline(always)]
    pub fn initdly(&self) -> INITDLY_R {
        INITDLY_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Initial spi delay for first transfer: This register is an option only available in SINGLE master mode The controller waits for a delay to transmit the first spi word after channel enabled and corresponding TX register filled. This Delay is based on SPI output frequency clock No clock output provided to the boundary and chip select is not active in 4 pin mode within this period. 0x0 No delay for first spi transfer. 0x1 The controller wait 4 spi bus clock 0x2 The controller wait 8 spi bus clock 0x3 The controller wait 16 spi bus clock 0x4 The controller wait 32 spi bus clock"]
    #[inline(always)]
    pub fn initdly(&mut self) -> INITDLY_W {
        INITDLY_W { w: self }
    }
}
