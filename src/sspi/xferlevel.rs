#[doc = "Reader of register XFERLEVEL"]
pub type R = crate::R<u32, super::XFERLEVEL>;
#[doc = "Writer for register XFERLEVEL"]
pub type W = crate::W<u32, super::XFERLEVEL>;
#[doc = "Register XFERLEVEL `reset()`'s with value 0"]
impl crate::ResetValue for super::XFERLEVEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WCNT`"]
pub type WCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WCNT`"]
pub struct WCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> WCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `AFL`"]
pub type AFL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFL`"]
pub struct AFL_W<'a> {
    w: &'a mut W,
}
impl<'a> AFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `AEL`"]
pub type AEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AEL`"]
pub struct AEL_W<'a> {
    w: &'a mut W,
}
impl<'a> AEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Spi word counterThis register holds the programmable value of number of SPI word to be transferred on channel which is using the FIFO buffer.When transfer had started a read back in this register returns the current SPI word transfer index. 0x0000 Counter not used 0x0001 one word 0xFFFE 65534 spi word 0xFFFF 65535 spi word"]
    #[inline(always)]
    pub fn wcnt(&self) -> WCNT_R {
        WCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - Buffer Almost Full This register holds the programmable almost full level value used to determine almost full buffer condition. If the user wants an interrupt or a DMA read request to be issued during a receive operation when the data buffer holds at least n bytes then the buffer MCSPI_MODULCTRL\\[AFL\\]
must be set with n-1.The size of this register is defined by the generic parameter FFNBYTE. 0x00 one byte 0x01 2 bytes 0xFE 255bytes 0xFF 256bytes"]
    #[inline(always)]
    pub fn afl(&self) -> AFL_R {
        AFL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Buffer Almost EmptyThis register holds the programmable almost empty level value used to determine almost empty buffer condition. If the user wants an interrupt or a DMA write request to be issued during a transmit operation when the data buffer is able to receive n bytes then the buffer MCSPI_MODULCTRL\\[AEL\\]
must be set with n-1. 0x00 one byte 0x01 2 bytes 0xFE 255 bytes 0xFF 256bytes"]
    #[inline(always)]
    pub fn ael(&self) -> AEL_R {
        AEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - Spi word counterThis register holds the programmable value of number of SPI word to be transferred on channel which is using the FIFO buffer.When transfer had started a read back in this register returns the current SPI word transfer index. 0x0000 Counter not used 0x0001 one word 0xFFFE 65534 spi word 0xFFFF 65535 spi word"]
    #[inline(always)]
    pub fn wcnt(&mut self) -> WCNT_W {
        WCNT_W { w: self }
    }
    #[doc = "Bits 8:15 - Buffer Almost Full This register holds the programmable almost full level value used to determine almost full buffer condition. If the user wants an interrupt or a DMA read request to be issued during a receive operation when the data buffer holds at least n bytes then the buffer MCSPI_MODULCTRL\\[AFL\\]
must be set with n-1.The size of this register is defined by the generic parameter FFNBYTE. 0x00 one byte 0x01 2 bytes 0xFE 255bytes 0xFF 256bytes"]
    #[inline(always)]
    pub fn afl(&mut self) -> AFL_W {
        AFL_W { w: self }
    }
    #[doc = "Bits 0:7 - Buffer Almost EmptyThis register holds the programmable almost empty level value used to determine almost empty buffer condition. If the user wants an interrupt or a DMA write request to be issued during a transmit operation when the data buffer is able to receive n bytes then the buffer MCSPI_MODULCTRL\\[AEL\\]
must be set with n-1. 0x00 one byte 0x01 2 bytes 0xFE 255 bytes 0xFF 256bytes"]
    #[inline(always)]
    pub fn ael(&mut self) -> AEL_W {
        AEL_W { w: self }
    }
}
