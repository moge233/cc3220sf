#[doc = "Reader of register RCM_IS"]
pub type R = crate::R<u32, super::RCM_IS>;
#[doc = "Writer for register RCM_IS"]
pub type W = crate::W<u32, super::RCM_IS>;
#[doc = "Register RCM_IS `reset()`'s with value 0"]
impl crate::ResetValue for super::RCM_IS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAKETIMRIRQ`"]
pub type WAKETIMRIRQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLLOCK`"]
pub type PLLLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXITDSLPBYTMR`"]
pub type EXITDSLPBYTMR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXITSLPBYTMR`"]
pub type EXITSLPBYTMR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXITDSLPBYNWP`"]
pub type EXITDSLPBYNWP_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXITSLPBYNWP`"]
pub type EXITSLPBYNWP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 14 - WAKETIMRIRQ"]
    #[inline(always)]
    pub fn waketimrirq(&self) -> WAKETIMRIRQ_R {
        WAKETIMRIRQ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PLLLOCK"]
    #[inline(always)]
    pub fn plllock(&self) -> PLLLOCK_R {
        PLLLOCK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EXITDSLPBYTMR"]
    #[inline(always)]
    pub fn exitdslpbytmr(&self) -> EXITDSLPBYTMR_R {
        EXITDSLPBYTMR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EXITSLPBYTMR"]
    #[inline(always)]
    pub fn exitslpbytmr(&self) -> EXITSLPBYTMR_R {
        EXITSLPBYTMR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXITDSLPBYNWP"]
    #[inline(always)]
    pub fn exitdslpbynwp(&self) -> EXITDSLPBYNWP_R {
        EXITDSLPBYNWP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - EXITSLPBYNWP"]
    #[inline(always)]
    pub fn exitslpbynwp(&self) -> EXITSLPBYNWP_R {
        EXITSLPBYNWP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {}
