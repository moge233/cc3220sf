#[doc = "Reader of register REF_ANA_SPARE_CONTROLS1"]
pub type R = crate::R<u32, super::REF_ANA_SPARE_CONTROLS1>;
#[doc = "Writer for register REF_ANA_SPARE_CONTROLS1"]
pub type W = crate::W<u32, super::REF_ANA_SPARE_CONTROLS1>;
#[doc = "Register REF_ANA_SPARE_CONTROLS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::REF_ANA_SPARE_CONTROLS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_TOP_CLKM_REG3`"]
pub type MEM_TOP_CLKM_REG3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_TOP_CLKM_REG3`"]
pub struct MEM_TOP_CLKM_REG3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TOP_CLKM_REG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEM_TOP_CLKM_REG4`"]
pub type MEM_TOP_CLKM_REG4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_TOP_CLKM_REG4`"]
pub struct MEM_TOP_CLKM_REG4_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TOP_CLKM_REG4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Spare control. Reaches on TOP_CLKM_REG3 \\[15:0\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_top_clkm_reg3(&self) -> MEM_TOP_CLKM_REG3_R {
        MEM_TOP_CLKM_REG3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Spare control. Reaches on TOP_CLKM_REG4 \\[15:0\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_top_clkm_reg4(&self) -> MEM_TOP_CLKM_REG4_R {
        MEM_TOP_CLKM_REG4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Spare control. Reaches on TOP_CLKM_REG3 \\[15:0\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_top_clkm_reg3(&mut self) -> MEM_TOP_CLKM_REG3_W {
        MEM_TOP_CLKM_REG3_W { w: self }
    }
    #[doc = "Bits 0:15 - Spare control. Reaches on TOP_CLKM_REG4 \\[15:0\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_top_clkm_reg4(&mut self) -> MEM_TOP_CLKM_REG4_W {
        MEM_TOP_CLKM_REG4_W { w: self }
    }
}
