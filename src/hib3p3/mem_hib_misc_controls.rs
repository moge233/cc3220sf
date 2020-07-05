#[doc = "Reader of register MEM_HIB_MISC_CONTROLS"]
pub type R = crate::R<u32, super::MEM_HIB_MISC_CONTROLS>;
#[doc = "Writer for register MEM_HIB_MISC_CONTROLS"]
pub type W = crate::W<u32, super::MEM_HIB_MISC_CONTROLS>;
#[doc = "Register MEM_HIB_MISC_CONTROLS `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_HIB_MISC_CONTROLS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_HIB_POK_POR_COMP_TRIM`"]
pub type MEM_HIB_POK_POR_COMP_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_HIB_POK_POR_COMP_TRIM`"]
pub struct MEM_HIB_POK_POR_COMP_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_HIB_POK_POR_COMP_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:8 - MEM_HIB_POK_POR_COMP_TRIM"]
    #[inline(always)]
    pub fn mem_hib_pok_por_comp_trim(&self) -> MEM_HIB_POK_POR_COMP_TRIM_R {
        MEM_HIB_POK_POR_COMP_TRIM_R::new(((self.bits >> 6) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 6:8 - MEM_HIB_POK_POR_COMP_TRIM"]
    #[inline(always)]
    pub fn mem_hib_pok_por_comp_trim(&mut self) -> MEM_HIB_POK_POR_COMP_TRIM_W {
        MEM_HIB_POK_POR_COMP_TRIM_W { w: self }
    }
}
