#[doc = "Reader of register FAULT_STATUS_REG"]
pub type R = crate::R<u32, super::FAULT_STATUS_REG>;
#[doc = "Writer for register FAULT_STATUS_REG"]
pub type W = crate::W<u32, super::FAULT_STATUS_REG>;
#[doc = "Register FAULT_STATUS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::FAULT_STATUS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PATCH_ERR_INDEX`"]
pub type PATCH_ERR_INDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PATCH_ERR_INDEX`"]
pub struct PATCH_ERR_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH_ERR_INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:5 - This field shows because of which patch trap address the bus_fault is generated. If the PATCH_ERR bit is set, then it means the bus fault is generated because of PATCH_TRAP_ADDR_REG\\[2^PATCH_ERR_INDEX\\]"]
    #[inline(always)]
    pub fn patch_err_index(&self) -> PATCH_ERR_INDEX_R {
        PATCH_ERR_INDEX_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:5 - This field shows because of which patch trap address the bus_fault is generated. If the PATCH_ERR bit is set, then it means the bus fault is generated because of PATCH_TRAP_ADDR_REG\\[2^PATCH_ERR_INDEX\\]"]
    #[inline(always)]
    pub fn patch_err_index(&mut self) -> PATCH_ERR_INDEX_W {
        PATCH_ERR_INDEX_W { w: self }
    }
}
