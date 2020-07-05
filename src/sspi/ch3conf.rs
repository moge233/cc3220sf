#[doc = "Reader of register CH3CONF"]
pub type R = crate::R<u32, super::CH3CONF>;
#[doc = "Writer for register CH3CONF"]
pub type W = crate::W<u32, super::CH3CONF>;
#[doc = "Register CH3CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCS3`"]
pub type TCS3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCS3`"]
pub struct TCS3_W<'a> {
    w: &'a mut W,
}
impl<'a> TCS3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `TRM`"]
pub type TRM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRM`"]
pub struct TRM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `WL`"]
pub type WL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WL`"]
pub struct WL_W<'a> {
    w: &'a mut W,
}
impl<'a> WL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | (((value as u32) & 0x1f) << 7);
        self.w
    }
}
#[doc = "Reader of field `CLKD`"]
pub type CLKD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKD`"]
pub struct CLKD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:26 - Chip Select Time Control This 2-bits field defines the number of interface clock cycles between CS toggling and first or last edge of SPI clock. 0x0 0.5 clock cycle 0x1 1.5 clock cycle 0x2 2.5 clock cycle 0x3 3.5 clock cycle"]
    #[inline(always)]
    pub fn tcs3(&self) -> TCS3_R {
        TCS3_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Transmit/Receive modes 0x0 Transmit and Receive mode 0x1 Receive only mode 0x2 Transmit only mode 0x3 Reserved"]
    #[inline(always)]
    pub fn trm(&self) -> TRM_R {
        TRM_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 7:11 - SPI word length 0x00 Reserved 0x01 Reserved 0x02 Reserved 0x03 The SPI word is 4-bits long 0x04 The SPI word is 5-bits long 0x05 The SPI word is 6-bits long 0x06 The SPI word is 7-bits long 0x07 The SPI word is 8-bits long 0x08 The SPI word is 9-bits long 0x09 The SPI word is 10-bits long 0x0A The SPI word is 11-bits long 0x0B The SPI word is 12-bits long 0x0C The SPI word is 13-bits long 0x0D The SPI word is 14-bits long 0x0E The SPI word is 15-bits long 0x0F The SPI word is 16-bits long 0x10 The SPI word is 17-bits long 0x11 The SPI word is 18-bits long 0x12 The SPI word is 19-bits long 0x13 The SPI word is 20-bits long 0x14 The SPI word is 21-bits long 0x15 The SPI word is 22-bits long 0x16 The SPI word is 23-bits long 0x17 The SPI word is 24-bits long 0x18 The SPI word is 25-bits long 0x19 The SPI word is 26-bits long 0x1A The SPI word is 27-bits long 0x1B The SPI word is 28-bits long 0x1C The SPI word is 29-bits long 0x1D The SPI word is 30-bits long 0x1E The SPI word is 31-bits long 0x1F The SPI word is 32-bits long"]
    #[inline(always)]
    pub fn wl(&self) -> WL_R {
        WL_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 2:5 - Frequency divider for SPICLK. (only when the module is a Master SPI device). A programmable clock divider divides the SPI reference clock (CLKSPIREF) with a 4-bit value and results in a new clock SPICLK available to shift-in and shift-out data. By default the clock divider ratio has a power of two granularity when MCSPI_CHCONF\\[CLKG\\]
is cleared Otherwise this register is the 4 LSB bit of a 12-bit register concatenated with clock divider extension MCSPI_CHCTRL\\[EXTCLK\\]
register.The value description below defines the clock ratio when MCSPI_CHCONF\\[CLKG\\]
is set to 0. 0x0 1 0x1 2 0x2 4 0x3 8 0x4 16 0x5 32 0x6 64 0x7 128 0x8 256 0x9 512 0xA 1024 0xB 2048 0xC 4096 0xD 8192 0xE 16384 0xF 32768"]
    #[inline(always)]
    pub fn clkd(&self) -> CLKD_R {
        CLKD_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:26 - Chip Select Time Control This 2-bits field defines the number of interface clock cycles between CS toggling and first or last edge of SPI clock. 0x0 0.5 clock cycle 0x1 1.5 clock cycle 0x2 2.5 clock cycle 0x3 3.5 clock cycle"]
    #[inline(always)]
    pub fn tcs3(&mut self) -> TCS3_W {
        TCS3_W { w: self }
    }
    #[doc = "Bits 12:13 - Transmit/Receive modes 0x0 Transmit and Receive mode 0x1 Receive only mode 0x2 Transmit only mode 0x3 Reserved"]
    #[inline(always)]
    pub fn trm(&mut self) -> TRM_W {
        TRM_W { w: self }
    }
    #[doc = "Bits 7:11 - SPI word length 0x00 Reserved 0x01 Reserved 0x02 Reserved 0x03 The SPI word is 4-bits long 0x04 The SPI word is 5-bits long 0x05 The SPI word is 6-bits long 0x06 The SPI word is 7-bits long 0x07 The SPI word is 8-bits long 0x08 The SPI word is 9-bits long 0x09 The SPI word is 10-bits long 0x0A The SPI word is 11-bits long 0x0B The SPI word is 12-bits long 0x0C The SPI word is 13-bits long 0x0D The SPI word is 14-bits long 0x0E The SPI word is 15-bits long 0x0F The SPI word is 16-bits long 0x10 The SPI word is 17-bits long 0x11 The SPI word is 18-bits long 0x12 The SPI word is 19-bits long 0x13 The SPI word is 20-bits long 0x14 The SPI word is 21-bits long 0x15 The SPI word is 22-bits long 0x16 The SPI word is 23-bits long 0x17 The SPI word is 24-bits long 0x18 The SPI word is 25-bits long 0x19 The SPI word is 26-bits long 0x1A The SPI word is 27-bits long 0x1B The SPI word is 28-bits long 0x1C The SPI word is 29-bits long 0x1D The SPI word is 30-bits long 0x1E The SPI word is 31-bits long 0x1F The SPI word is 32-bits long"]
    #[inline(always)]
    pub fn wl(&mut self) -> WL_W {
        WL_W { w: self }
    }
    #[doc = "Bits 2:5 - Frequency divider for SPICLK. (only when the module is a Master SPI device). A programmable clock divider divides the SPI reference clock (CLKSPIREF) with a 4-bit value and results in a new clock SPICLK available to shift-in and shift-out data. By default the clock divider ratio has a power of two granularity when MCSPI_CHCONF\\[CLKG\\]
is cleared Otherwise this register is the 4 LSB bit of a 12-bit register concatenated with clock divider extension MCSPI_CHCTRL\\[EXTCLK\\]
register.The value description below defines the clock ratio when MCSPI_CHCONF\\[CLKG\\]
is set to 0. 0x0 1 0x1 2 0x2 4 0x3 8 0x4 16 0x5 32 0x6 64 0x7 128 0x8 256 0x9 512 0xA 1024 0xB 2048 0xC 4096 0xD 8192 0xE 16384 0xF 32768"]
    #[inline(always)]
    pub fn clkd(&mut self) -> CLKD_W {
        CLKD_W { w: self }
    }
}
