#[doc = "Reader of register NWP_SRAM_DSLP_CFG"]
pub type R = crate::R<u32, super::NWP_SRAM_DSLP_CFG>;
#[doc = "Writer for register NWP_SRAM_DSLP_CFG"]
pub type W = crate::W<u32, super::NWP_SRAM_DSLP_CFG>;
#[doc = "Register NWP_SRAM_DSLP_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::NWP_SRAM_DSLP_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NWP_SRAM_DSLP_CFG`"]
pub type NWP_SRAM_DSLP_CFG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NWP_SRAM_DSLP_CFG`"]
pub struct NWP_SRAM_DSLP_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> NWP_SRAM_DSLP_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Configuration of NWP Memories during DSLP : 0 - SRAMs are OFF ; 1 - SRAMs are Retained. NWP SRAM Cluster information : \\[2\\]
- 3rd column in MEMSS (Applicable only when owned by NWP) ; \\[3\\]
- 4th column in MEMSS (Applicable only when owned by NWP) ; \\[4\\]
- 5th column in MEMSS (Applicable only when owned by NWP) ; \\[5\\]
- 6th column in MEMSS (Applicable only when owned by NWP) ; \\[6\\]
- 7th column in MEMSS (Applicable only when owned by NWP) ; \\[7\\]
- 8th column in MEMSS (Applicable only when owned by NWP) ; \\[8\\]
- 9th column in MEMSS (Applicable only when owned by NWP) ; \\[9\\]
- 10th column in MEMSS (Applicable only when owned by NWP) ; \\[10\\]
- 11th column in MEMSS (Applicable only when owned by NWP) ; \\[11\\]
- 12th column in MEMSS (Applicable only when owned by NWP) ; \\[12\\]
- 13th column in MEMSS (Applicable only when owned by NWP) ; \\[13\\]
- 14th column in MEMSS (Applicable only when owned by NWP) ; \\[14\\]
- 15th column in MEMSS (Applicable only when owned by NWP) ; \\[19:18\\]
- Reserved."]
    #[inline(always)]
    pub fn nwp_sram_dslp_cfg(&self) -> NWP_SRAM_DSLP_CFG_R {
        NWP_SRAM_DSLP_CFG_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Configuration of NWP Memories during DSLP : 0 - SRAMs are OFF ; 1 - SRAMs are Retained. NWP SRAM Cluster information : \\[2\\]
- 3rd column in MEMSS (Applicable only when owned by NWP) ; \\[3\\]
- 4th column in MEMSS (Applicable only when owned by NWP) ; \\[4\\]
- 5th column in MEMSS (Applicable only when owned by NWP) ; \\[5\\]
- 6th column in MEMSS (Applicable only when owned by NWP) ; \\[6\\]
- 7th column in MEMSS (Applicable only when owned by NWP) ; \\[7\\]
- 8th column in MEMSS (Applicable only when owned by NWP) ; \\[8\\]
- 9th column in MEMSS (Applicable only when owned by NWP) ; \\[9\\]
- 10th column in MEMSS (Applicable only when owned by NWP) ; \\[10\\]
- 11th column in MEMSS (Applicable only when owned by NWP) ; \\[11\\]
- 12th column in MEMSS (Applicable only when owned by NWP) ; \\[12\\]
- 13th column in MEMSS (Applicable only when owned by NWP) ; \\[13\\]
- 14th column in MEMSS (Applicable only when owned by NWP) ; \\[14\\]
- 15th column in MEMSS (Applicable only when owned by NWP) ; \\[19:18\\]
- Reserved."]
    #[inline(always)]
    pub fn nwp_sram_dslp_cfg(&mut self) -> NWP_SRAM_DSLP_CFG_W {
        NWP_SRAM_DSLP_CFG_W { w: self }
    }
}
