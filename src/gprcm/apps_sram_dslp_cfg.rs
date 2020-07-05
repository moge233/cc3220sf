#[doc = "Reader of register APPS_SRAM_DSLP_CFG"]
pub type R = crate::R<u32, super::APPS_SRAM_DSLP_CFG>;
#[doc = "Writer for register APPS_SRAM_DSLP_CFG"]
pub type W = crate::W<u32, super::APPS_SRAM_DSLP_CFG>;
#[doc = "Register APPS_SRAM_DSLP_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::APPS_SRAM_DSLP_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APPS_SRAM_DSLP_CFG`"]
pub type APPS_SRAM_DSLP_CFG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APPS_SRAM_DSLP_CFG`"]
pub struct APPS_SRAM_DSLP_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> APPS_SRAM_DSLP_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Configuration of APPS Memories during Deep-sleep : 0 - SRAMs are OFF ; 1 - SRAMs are Retained. APPS SRAM Cluster information : \\[0\\]
- 1st column in MEMSS (Applicable only when owned by APPS); \\[1\\]
- 2nd column in MEMSS (Applicable only when owned by APPS); \\[2\\]
- 3rd column in MEMSS (Applicable only when owned by APPS) ; \\[3\\]
- 4th column in MEMSS (Applicable only when owned by APPS) ; \\[16\\]
- MCU-PD - Apps cluster 0 (TBD); \\[19:18\\]
- Reserved."]
    #[inline(always)]
    pub fn apps_sram_dslp_cfg(&self) -> APPS_SRAM_DSLP_CFG_R {
        APPS_SRAM_DSLP_CFG_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Configuration of APPS Memories during Deep-sleep : 0 - SRAMs are OFF ; 1 - SRAMs are Retained. APPS SRAM Cluster information : \\[0\\]
- 1st column in MEMSS (Applicable only when owned by APPS); \\[1\\]
- 2nd column in MEMSS (Applicable only when owned by APPS); \\[2\\]
- 3rd column in MEMSS (Applicable only when owned by APPS) ; \\[3\\]
- 4th column in MEMSS (Applicable only when owned by APPS) ; \\[16\\]
- MCU-PD - Apps cluster 0 (TBD); \\[19:18\\]
- Reserved."]
    #[inline(always)]
    pub fn apps_sram_dslp_cfg(&mut self) -> APPS_SRAM_DSLP_CFG_W {
        APPS_SRAM_DSLP_CFG_W { w: self }
    }
}
