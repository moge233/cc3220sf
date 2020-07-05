#[doc = "Reader of register MCSPI_N1_POWER_CTRL"]
pub type R = crate::R<u32, super::MCSPI_N1_POWER_CTRL>;
#[doc = "Writer for register MCSPI_N1_POWER_CTRL"]
pub type W = crate::W<u32, super::MCSPI_N1_POWER_CTRL>;
#[doc = "Register MCSPI_N1_POWER_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MCSPI_N1_POWER_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCSPI_N1_PD_STATUS`"]
pub type MCSPI_N1_PD_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCSPI_N1_PD_STATUS`"]
pub struct MCSPI_N1_PD_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MCSPI_N1_PD_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:2 - 1 - MCSPI_N1-PD is ON ; 0 - MCSPI_N1-PD if OFF"]
    #[inline(always)]
    pub fn mcspi_n1_pd_status(&self) -> MCSPI_N1_PD_STATUS_R {
        MCSPI_N1_PD_STATUS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - 1 - MCSPI_N1-PD is ON ; 0 - MCSPI_N1-PD if OFF"]
    #[inline(always)]
    pub fn mcspi_n1_pd_status(&mut self) -> MCSPI_N1_PD_STATUS_W {
        MCSPI_N1_PD_STATUS_W { w: self }
    }
}
