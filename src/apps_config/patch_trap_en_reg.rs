#[doc = "Reader of register PATCH_TRAP_EN_REG"]
pub type R = crate::R<u32, super::PATCH_TRAP_EN_REG>;
#[doc = "Writer for register PATCH_TRAP_EN_REG"]
pub type W = crate::W<u32, super::PATCH_TRAP_EN_REG>;
#[doc = "Register PATCH_TRAP_EN_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::PATCH_TRAP_EN_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PATCH_TRAP_EN`"]
pub type PATCH_TRAP_EN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PATCH_TRAP_EN`"]
pub struct PATCH_TRAP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH_TRAP_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - When PATCH_TRAP_EN\\[n\\]
is set bus fault is generated for the address PATCH_TRAP_ADD\\[n\\]\\[31:0\\]
from Idcode bus. The exception routine should take care to jump to the location where the patch correspond to this address is kept."]
    #[inline(always)]
    pub fn patch_trap_en(&self) -> PATCH_TRAP_EN_R {
        PATCH_TRAP_EN_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - When PATCH_TRAP_EN\\[n\\]
is set bus fault is generated for the address PATCH_TRAP_ADD\\[n\\]\\[31:0\\]
from Idcode bus. The exception routine should take care to jump to the location where the patch correspond to this address is kept."]
    #[inline(always)]
    pub fn patch_trap_en(&mut self) -> PATCH_TRAP_EN_W {
        PATCH_TRAP_EN_W { w: self }
    }
}
