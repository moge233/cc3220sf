#[doc = "Reader of register MEM_SLDO_VNWA_SW_CTRL"]
pub type R = crate::R<u32, super::MEM_SLDO_VNWA_SW_CTRL>;
#[doc = "Writer for register MEM_SLDO_VNWA_SW_CTRL"]
pub type W = crate::W<u32, super::MEM_SLDO_VNWA_SW_CTRL>;
#[doc = "Register MEM_SLDO_VNWA_SW_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_SLDO_VNWA_SW_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_SLDO_VNWA_SW_CTRL`"]
pub type MEM_SLDO_VNWA_SW_CTRL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MEM_SLDO_VNWA_SW_CTRL`"]
pub struct MEM_SLDO_VNWA_SW_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SLDO_VNWA_SW_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - MEM_SLDO_VNWA_SW_CTRL"]
    #[inline(always)]
    pub fn mem_sldo_vnwa_sw_ctrl(&self) -> MEM_SLDO_VNWA_SW_CTRL_R {
        MEM_SLDO_VNWA_SW_CTRL_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - MEM_SLDO_VNWA_SW_CTRL"]
    #[inline(always)]
    pub fn mem_sldo_vnwa_sw_ctrl(&mut self) -> MEM_SLDO_VNWA_SW_CTRL_W {
        MEM_SLDO_VNWA_SW_CTRL_W { w: self }
    }
}
