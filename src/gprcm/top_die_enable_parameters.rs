#[doc = "Reader of register TOP_DIE_ENABLE_PARAMETERS"]
pub type R = crate::R<u32, super::TOP_DIE_ENABLE_PARAMETERS>;
#[doc = "Writer for register TOP_DIE_ENABLE_PARAMETERS"]
pub type W = crate::W<u32, super::TOP_DIE_ENABLE_PARAMETERS>;
#[doc = "Register TOP_DIE_ENABLE_PARAMETERS `reset()`'s with value 0"]
impl crate::ResetValue for super::TOP_DIE_ENABLE_PARAMETERS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_3P3_RSTN2D2D_POR_RSTN`"]
pub type FLASH_3P3_RSTN2D2D_POR_RSTN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLASH_3P3_RSTN2D2D_POR_RSTN`"]
pub struct FLASH_3P3_RSTN2D2D_POR_RSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_3P3_RSTN2D2D_POR_RSTN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `TOP_DIE_SW_EN2TOP_DIE_FLASH_3P3_RSTN`"]
pub type TOP_DIE_SW_EN2TOP_DIE_FLASH_3P3_RSTN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOP_DIE_SW_EN2TOP_DIE_FLASH_3P3_RSTN`"]
pub struct TOP_DIE_SW_EN2TOP_DIE_FLASH_3P3_RSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_DIE_SW_EN2TOP_DIE_FLASH_3P3_RSTN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TOP_DIE_POR_RSTN2BOTT_DIE_FMC_RSTN`"]
pub type TOP_DIE_POR_RSTN2BOTT_DIE_FMC_RSTN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOP_DIE_POR_RSTN2BOTT_DIE_FMC_RSTN`"]
pub struct TOP_DIE_POR_RSTN2BOTT_DIE_FMC_RSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_DIE_POR_RSTN2BOTT_DIE_FMC_RSTN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Configuration (in slow_clks) for number of clks between Flash-3p3-rstn to D2D POR Resetn."]
    #[inline(always)]
    pub fn flash_3p3_rstn2d2d_por_rstn(&self) -> FLASH_3P3_RSTN2D2D_POR_RSTN_R {
        FLASH_3P3_RSTN2D2D_POR_RSTN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Configuration (in slow_clks) for number of clks between Top-die Switch-Enable and Top-die Flash 3p3 Reset removal"]
    #[inline(always)]
    pub fn top_die_sw_en2top_die_flash_3p3_rstn(&self) -> TOP_DIE_SW_EN2TOP_DIE_FLASH_3P3_RSTN_R {
        TOP_DIE_SW_EN2TOP_DIE_FLASH_3P3_RSTN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Configuration (in slow_clks) for number of clks between D2D POR Reset removal and bottom die FMC reset removal"]
    #[inline(always)]
    pub fn top_die_por_rstn2bott_die_fmc_rstn(&self) -> TOP_DIE_POR_RSTN2BOTT_DIE_FMC_RSTN_R {
        TOP_DIE_POR_RSTN2BOTT_DIE_FMC_RSTN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Configuration (in slow_clks) for number of clks between Flash-3p3-rstn to D2D POR Resetn."]
    #[inline(always)]
    pub fn flash_3p3_rstn2d2d_por_rstn(&mut self) -> FLASH_3P3_RSTN2D2D_POR_RSTN_W {
        FLASH_3P3_RSTN2D2D_POR_RSTN_W { w: self }
    }
    #[doc = "Bits 16:23 - Configuration (in slow_clks) for number of clks between Top-die Switch-Enable and Top-die Flash 3p3 Reset removal"]
    #[inline(always)]
    pub fn top_die_sw_en2top_die_flash_3p3_rstn(
        &mut self,
    ) -> TOP_DIE_SW_EN2TOP_DIE_FLASH_3P3_RSTN_W {
        TOP_DIE_SW_EN2TOP_DIE_FLASH_3P3_RSTN_W { w: self }
    }
    #[doc = "Bits 0:7 - Configuration (in slow_clks) for number of clks between D2D POR Reset removal and bottom die FMC reset removal"]
    #[inline(always)]
    pub fn top_die_por_rstn2bott_die_fmc_rstn(&mut self) -> TOP_DIE_POR_RSTN2BOTT_DIE_FMC_RSTN_W {
        TOP_DIE_POR_RSTN2BOTT_DIE_FMC_RSTN_W { w: self }
    }
}
