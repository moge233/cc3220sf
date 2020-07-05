#[doc = "Reader of register HIB_COMP_TRIM"]
pub type R = crate::R<u32, super::HIB_COMP_TRIM>;
#[doc = "Writer for register HIB_COMP_TRIM"]
pub type W = crate::W<u32, super::HIB_COMP_TRIM>;
#[doc = "Register HIB_COMP_TRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::HIB_COMP_TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_HD_COMP_TRIM`"]
pub type MEM_HD_COMP_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_HD_COMP_TRIM`"]
pub struct MEM_HD_COMP_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_HD_COMP_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - MEM_HD_COMP_TRIM"]
    #[inline(always)]
    pub fn mem_hd_comp_trim(&self) -> MEM_HD_COMP_TRIM_R {
        MEM_HD_COMP_TRIM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - MEM_HD_COMP_TRIM"]
    #[inline(always)]
    pub fn mem_hd_comp_trim(&mut self) -> MEM_HD_COMP_TRIM_W {
        MEM_HD_COMP_TRIM_W { w: self }
    }
}
