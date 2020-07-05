#[doc = "Reader of register SYSCTL"]
pub type R = crate::R<u32, super::SYSCTL>;
#[doc = "Writer for register SYSCTL"]
pub type W = crate::W<u32, super::SYSCTL>;
#[doc = "Register SYSCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTO`"]
pub type DTO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTO`"]
pub struct DTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CLKD`"]
pub type CLKD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLKD`"]
pub struct CLKD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 6)) | (((value as u32) & 0x03ff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - Data timeout counter value and busy timeout. This value determines the interval by which DAT lines timeouts are detected. The host driver needs to set this bitfield based on - the maximum read access time (NAC) (Refer to the SD Specification Part1 Physical Layer) - the data read access time values (TAAC and NSAC) in the card specific data register (CSD) of the card - the timeout clock base frequency (MMCHS_CAPA\\[TCF\\]). If the card does not respond within the specified number of cycles a data timeout error occurs (MMCHS_STA\\[DTO\\]). The MMCHS_SYSCTL\\[DTO\\]
register is also used to check busy duration to generate busy timeout for commands with busy response or for busy programming during a write command. Timeout on CRC status is generated if no CRC token is present after a block write. 0x0 TCF x 2^13 0x1 TCF x 2^14 0xE TCF x 2^27 0xF Reserved"]
    #[inline(always)]
    pub fn dto(&self) -> DTO_R {
        DTO_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 6:15 - Clock frequency select These bits define the ratio between a reference clock frequency (system dependant) and the output clock frequency on the CLK pin of either the memory card (MMC SD or SDIO). 0x000 Clock Ref bypass 0x001 Clock Ref bypass 0x002 Clock Ref / 2 0x003 Clock Ref / 3 0x3FF Clock Ref / 1023"]
    #[inline(always)]
    pub fn clkd(&self) -> CLKD_R {
        CLKD_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:19 - Data timeout counter value and busy timeout. This value determines the interval by which DAT lines timeouts are detected. The host driver needs to set this bitfield based on - the maximum read access time (NAC) (Refer to the SD Specification Part1 Physical Layer) - the data read access time values (TAAC and NSAC) in the card specific data register (CSD) of the card - the timeout clock base frequency (MMCHS_CAPA\\[TCF\\]). If the card does not respond within the specified number of cycles a data timeout error occurs (MMCHS_STA\\[DTO\\]). The MMCHS_SYSCTL\\[DTO\\]
register is also used to check busy duration to generate busy timeout for commands with busy response or for busy programming during a write command. Timeout on CRC status is generated if no CRC token is present after a block write. 0x0 TCF x 2^13 0x1 TCF x 2^14 0xE TCF x 2^27 0xF Reserved"]
    #[inline(always)]
    pub fn dto(&mut self) -> DTO_W {
        DTO_W { w: self }
    }
    #[doc = "Bits 6:15 - Clock frequency select These bits define the ratio between a reference clock frequency (system dependant) and the output clock frequency on the CLK pin of either the memory card (MMC SD or SDIO). 0x000 Clock Ref bypass 0x001 Clock Ref bypass 0x002 Clock Ref / 2 0x003 Clock Ref / 3 0x3FF Clock Ref / 1023"]
    #[inline(always)]
    pub fn clkd(&mut self) -> CLKD_W {
        CLKD_W { w: self }
    }
}
