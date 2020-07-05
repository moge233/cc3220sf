#[doc = "Reader of register SPARE_REG_4"]
pub type R = crate::R<u32, super::SPARE_REG_4>;
#[doc = "Writer for register SPARE_REG_4"]
pub type W = crate::W<u32, super::SPARE_REG_4>;
#[doc = "Register SPARE_REG_4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPARE_REG_4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_SPARE_REG_4`"]
pub type MEM_SPARE_REG_4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MEM_SPARE_REG_4`"]
pub struct MEM_SPARE_REG_4_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SPARE_REG_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:31 - HW register"]
    #[inline(always)]
    pub fn mem_spare_reg_4(&self) -> MEM_SPARE_REG_4_R {
        MEM_SPARE_REG_4_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 1:31 - HW register"]
    #[inline(always)]
    pub fn mem_spare_reg_4(&mut self) -> MEM_SPARE_REG_4_W {
        MEM_SPARE_REG_4_W { w: self }
    }
}
