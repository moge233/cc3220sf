#[doc = "Reader of register REF_ANA_SPARE_CONTROLS0"]
pub type R = crate::R<u32, super::REF_ANA_SPARE_CONTROLS0>;
#[doc = "Writer for register REF_ANA_SPARE_CONTROLS0"]
pub type W = crate::W<u32, super::REF_ANA_SPARE_CONTROLS0>;
#[doc = "Register REF_ANA_SPARE_CONTROLS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::REF_ANA_SPARE_CONTROLS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_TOP_PM_REG3`"]
pub type MEM_TOP_PM_REG3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_TOP_PM_REG3`"]
pub struct MEM_TOP_PM_REG3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TOP_PM_REG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Spare control. Reaches on TOP_PM_REG3 \\[15:0\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_top_pm_reg3(&self) -> MEM_TOP_PM_REG3_R {
        MEM_TOP_PM_REG3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Spare control. Reaches on TOP_PM_REG3 \\[15:0\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_top_pm_reg3(&mut self) -> MEM_TOP_PM_REG3_W {
        MEM_TOP_PM_REG3_W { w: self }
    }
}
