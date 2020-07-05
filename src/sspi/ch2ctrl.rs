#[doc = "Reader of register CH2CTRL"]
pub type R = crate::R<u32, super::CH2CTRL>;
#[doc = "Writer for register CH2CTRL"]
pub type W = crate::W<u32, super::CH2CTRL>;
#[doc = "Register CH2CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CH2CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTCLK`"]
pub type EXTCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTCLK`"]
pub struct EXTCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Clock ratio extension: This register is used to concatenate with MCSPI_CHCONF\\[CLKD\\]
register for clock ratio only when granularity is one clock cycle (MCSPI_CHCONF\\[CLKG\\]
set to 1). Then the max value reached is 4096 clock divider ratio. 0x00 Clock ratio is CLKD + 1 0x01 Clock ratio is CLKD + 1 + 16 0xFF Clock ratio is CLKD + 1 + 4080"]
    #[inline(always)]
    pub fn extclk(&self) -> EXTCLK_R {
        EXTCLK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Clock ratio extension: This register is used to concatenate with MCSPI_CHCONF\\[CLKD\\]
register for clock ratio only when granularity is one clock cycle (MCSPI_CHCONF\\[CLKG\\]
set to 1). Then the max value reached is 4096 clock divider ratio. 0x00 Clock ratio is CLKD + 1 0x01 Clock ratio is CLKD + 1 + 16 0xFF Clock ratio is CLKD + 1 + 4080"]
    #[inline(always)]
    pub fn extclk(&mut self) -> EXTCLK_W {
        EXTCLK_W { w: self }
    }
}
