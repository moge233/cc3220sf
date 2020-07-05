#[doc = "Reader of register DIG_DCDC_VTRIM_CFG"]
pub type R = crate::R<u32, super::DIG_DCDC_VTRIM_CFG>;
#[doc = "Writer for register DIG_DCDC_VTRIM_CFG"]
pub type W = crate::W<u32, super::DIG_DCDC_VTRIM_CFG>;
#[doc = "Register DIG_DCDC_VTRIM_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DIG_DCDC_VTRIM_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_RUN_VTRIM`"]
pub type MEM_DCDC_DIG_RUN_VTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_RUN_VTRIM`"]
pub struct MEM_DCDC_DIG_RUN_VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_RUN_VTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_DSLP_VTRIM`"]
pub type MEM_DCDC_DIG_DSLP_VTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_DSLP_VTRIM`"]
pub struct MEM_DCDC_DIG_DSLP_VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_DSLP_VTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `MEM_DCDC_DIG_LPDS_VTRIM`"]
pub type MEM_DCDC_DIG_LPDS_VTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_DCDC_DIG_LPDS_VTRIM`"]
pub struct MEM_DCDC_DIG_LPDS_VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_DCDC_DIG_LPDS_VTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPARE_RW`"]
pub type SPARE_RW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPARE_RW`"]
pub struct SPARE_RW_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_RW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:23 - MEM_DCDC_DIG_RUN_VTRIM"]
    #[inline(always)]
    pub fn mem_dcdc_dig_run_vtrim(&self) -> MEM_DCDC_DIG_RUN_VTRIM_R {
        MEM_DCDC_DIG_RUN_VTRIM_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - MEM_DCDC_DIG_DSLP_VTRIM"]
    #[inline(always)]
    pub fn mem_dcdc_dig_dslp_vtrim(&self) -> MEM_DCDC_DIG_DSLP_VTRIM_R {
        MEM_DCDC_DIG_DSLP_VTRIM_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - MEM_DCDC_DIG_LPDS_VTRIM"]
    #[inline(always)]
    pub fn mem_dcdc_dig_lpds_vtrim(&self) -> MEM_DCDC_DIG_LPDS_VTRIM_R {
        MEM_DCDC_DIG_LPDS_VTRIM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - SPARE_RW"]
    #[inline(always)]
    pub fn spare_rw(&self) -> SPARE_RW_R {
        SPARE_RW_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 18:23 - MEM_DCDC_DIG_RUN_VTRIM"]
    #[inline(always)]
    pub fn mem_dcdc_dig_run_vtrim(&mut self) -> MEM_DCDC_DIG_RUN_VTRIM_W {
        MEM_DCDC_DIG_RUN_VTRIM_W { w: self }
    }
    #[doc = "Bits 12:17 - MEM_DCDC_DIG_DSLP_VTRIM"]
    #[inline(always)]
    pub fn mem_dcdc_dig_dslp_vtrim(&mut self) -> MEM_DCDC_DIG_DSLP_VTRIM_W {
        MEM_DCDC_DIG_DSLP_VTRIM_W { w: self }
    }
    #[doc = "Bits 6:11 - MEM_DCDC_DIG_LPDS_VTRIM"]
    #[inline(always)]
    pub fn mem_dcdc_dig_lpds_vtrim(&mut self) -> MEM_DCDC_DIG_LPDS_VTRIM_W {
        MEM_DCDC_DIG_LPDS_VTRIM_W { w: self }
    }
    #[doc = "Bits 0:5 - SPARE_RW"]
    #[inline(always)]
    pub fn spare_rw(&mut self) -> SPARE_RW_W {
        SPARE_RW_W { w: self }
    }
}
