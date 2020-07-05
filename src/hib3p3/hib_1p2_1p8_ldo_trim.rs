#[doc = "Reader of register HIB_1P2_1P8_LDO_TRIM"]
pub type R = crate::R<u32, super::HIB_1P2_1P8_LDO_TRIM>;
#[doc = "Writer for register HIB_1P2_1P8_LDO_TRIM"]
pub type W = crate::W<u32, super::HIB_1P2_1P8_LDO_TRIM>;
#[doc = "Register HIB_1P2_1P8_LDO_TRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::HIB_1P2_1P8_LDO_TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_HD_1P2_LDO_VTRIM`"]
pub type MEM_HD_1P2_LDO_VTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_HD_1P2_LDO_VTRIM`"]
pub struct MEM_HD_1P2_LDO_VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_HD_1P2_LDO_VTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MEM_HD_1P8_LDO_VTRIM`"]
pub type MEM_HD_1P8_LDO_VTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_HD_1P8_LDO_VTRIM`"]
pub struct MEM_HD_1P8_LDO_VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_HD_1P8_LDO_VTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - MEM_HD_1P2_LDO_VTRIM"]
    #[inline(always)]
    pub fn mem_hd_1p2_ldo_vtrim(&self) -> MEM_HD_1P2_LDO_VTRIM_R {
        MEM_HD_1P2_LDO_VTRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - MEM_HD_1P8_LDO_VTRIM"]
    #[inline(always)]
    pub fn mem_hd_1p8_ldo_vtrim(&self) -> MEM_HD_1P8_LDO_VTRIM_R {
        MEM_HD_1P8_LDO_VTRIM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - MEM_HD_1P2_LDO_VTRIM"]
    #[inline(always)]
    pub fn mem_hd_1p2_ldo_vtrim(&mut self) -> MEM_HD_1P2_LDO_VTRIM_W {
        MEM_HD_1P2_LDO_VTRIM_W { w: self }
    }
    #[doc = "Bits 0:3 - MEM_HD_1P8_LDO_VTRIM"]
    #[inline(always)]
    pub fn mem_hd_1p8_ldo_vtrim(&mut self) -> MEM_HD_1P8_LDO_VTRIM_W {
        MEM_HD_1P8_LDO_VTRIM_W { w: self }
    }
}
