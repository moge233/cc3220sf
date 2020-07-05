#[doc = "Reader of register REF_ANA_BGAP_CONTROLS0"]
pub type R = crate::R<u32, super::REF_ANA_BGAP_CONTROLS0>;
#[doc = "Writer for register REF_ANA_BGAP_CONTROLS0"]
pub type W = crate::W<u32, super::REF_ANA_BGAP_CONTROLS0>;
#[doc = "Register REF_ANA_BGAP_CONTROLS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::REF_ANA_BGAP_CONTROLS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_REF_TEMP_TRIM`"]
pub type MEM_REF_TEMP_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_REF_TEMP_TRIM`"]
pub struct MEM_REF_TEMP_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_REF_TEMP_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `MEM_REF_V2I_TRIM`"]
pub type MEM_REF_V2I_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_REF_V2I_TRIM`"]
pub struct MEM_REF_V2I_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_REF_V2I_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Reader of field `NU1`"]
pub type NU1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NU1`"]
pub struct NU1_W<'a> {
    w: &'a mut W,
}
impl<'a> NU1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:15 - REF_TEMP_TRIM override. Applicable when bit \\[20\\]
of this register set to 1. (or efc_done = 0) Note : Final REF_TEMP_TRIM reaches on port TOP_PM_REG0\\[15:10\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_ref_temp_trim(&self) -> MEM_REF_TEMP_TRIM_R {
        MEM_REF_TEMP_TRIM_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - REF_V2I_TRIM Override. Applicable when bit \\[21\\]
of this register set to 1 . (of efc_done = 0) Note : Final REF_V2I_TRIM reaches on port TOP_PM_REG0\\[9:6\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_ref_v2i_trim(&self) -> MEM_REF_V2I_TRIM_R {
        MEM_REF_V2I_TRIM_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - NU1"]
    #[inline(always)]
    pub fn nu1(&self) -> NU1_R {
        NU1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 10:15 - REF_TEMP_TRIM override. Applicable when bit \\[20\\]
of this register set to 1. (or efc_done = 0) Note : Final REF_TEMP_TRIM reaches on port TOP_PM_REG0\\[15:10\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_ref_temp_trim(&mut self) -> MEM_REF_TEMP_TRIM_W {
        MEM_REF_TEMP_TRIM_W { w: self }
    }
    #[doc = "Bits 6:9 - REF_V2I_TRIM Override. Applicable when bit \\[21\\]
of this register set to 1 . (of efc_done = 0) Note : Final REF_V2I_TRIM reaches on port TOP_PM_REG0\\[9:6\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_ref_v2i_trim(&mut self) -> MEM_REF_V2I_TRIM_W {
        MEM_REF_V2I_TRIM_W { w: self }
    }
    #[doc = "Bits 4:5 - NU1"]
    #[inline(always)]
    pub fn nu1(&mut self) -> NU1_W {
        NU1_W { w: self }
    }
}
