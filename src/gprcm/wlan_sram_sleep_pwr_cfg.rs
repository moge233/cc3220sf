#[doc = "Reader of register WLAN_SRAM_SLEEP_PWR_CFG"]
pub type R = crate::R<u32, super::WLAN_SRAM_SLEEP_PWR_CFG>;
#[doc = "Writer for register WLAN_SRAM_SLEEP_PWR_CFG"]
pub type W = crate::W<u32, super::WLAN_SRAM_SLEEP_PWR_CFG>;
#[doc = "Register WLAN_SRAM_SLEEP_PWR_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::WLAN_SRAM_SLEEP_PWR_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WLAN_SRAM_SLEEP_PWR_CFG`"]
pub type WLAN_SRAM_SLEEP_PWR_CFG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WLAN_SRAM_SLEEP_PWR_CFG`"]
pub struct WLAN_SRAM_SLEEP_PWR_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> WLAN_SRAM_SLEEP_PWR_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - SRAM (WTOP+DRP) state during Sleep-mode : 1 - SRAMs are RET ; 0 - SRAMs are OFF. Cluster information : \\[0\\]
- 1st column of MEMSS (Applicable only when owned by WTOP/PHY) \\[1\\]
- 2nd column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[2\\]
- 3rd column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[3\\]
- 4th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[4\\]
- 5th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[5\\]
- 6th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[6\\]
- 7th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[7\\]
- 8th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[8\\]
- 9th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[9\\]
- 10th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[10\\]
- 11th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[11\\]
- 12th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[12\\]
- 13th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[13\\]
- 14th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[14\\]
- 15th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[15\\]
- 16th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[23:16\\]
- Internal to WTOP Cluster"]
    #[inline(always)]
    pub fn wlan_sram_sleep_pwr_cfg(&self) -> WLAN_SRAM_SLEEP_PWR_CFG_R {
        WLAN_SRAM_SLEEP_PWR_CFG_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - SRAM (WTOP+DRP) state during Sleep-mode : 1 - SRAMs are RET ; 0 - SRAMs are OFF. Cluster information : \\[0\\]
- 1st column of MEMSS (Applicable only when owned by WTOP/PHY) \\[1\\]
- 2nd column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[2\\]
- 3rd column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[3\\]
- 4th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[4\\]
- 5th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[5\\]
- 6th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[6\\]
- 7th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[7\\]
- 8th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[8\\]
- 9th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[9\\]
- 10th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[10\\]
- 11th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[11\\]
- 12th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[12\\]
- 13th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[13\\]
- 14th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[14\\]
- 15th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[15\\]
- 16th column of MEMSS (Applicable only when owned by WTOP/PHY) ; \\[23:16\\]
- Internal to WTOP Cluster"]
    #[inline(always)]
    pub fn wlan_sram_sleep_pwr_cfg(&mut self) -> WLAN_SRAM_SLEEP_PWR_CFG_W {
        WLAN_SRAM_SLEEP_PWR_CFG_W { w: self }
    }
}
