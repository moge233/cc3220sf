#[doc = "Reader of register PMBIST_PARAMETERS0"]
pub type R = crate::R<u32, super::PMBIST_PARAMETERS0>;
#[doc = "Writer for register PMBIST_PARAMETERS0"]
pub type W = crate::W<u32, super::PMBIST_PARAMETERS0>;
#[doc = "Register PMBIST_PARAMETERS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PMBIST_PARAMETERS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_PM_BIST_CTRL_LOWV`"]
pub type MEM_PM_BIST_CTRL_LOWV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MEM_PM_BIST_CTRL_LOWV`"]
pub struct MEM_PM_BIST_CTRL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_PM_BIST_CTRL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 11)) | (((value as u32) & 0x000f_ffff) << 11);
        self.w
    }
}
#[doc = "Reader of field `NA21`"]
pub type NA21_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NA21`"]
pub struct NA21_W<'a> {
    w: &'a mut W,
}
impl<'a> NA21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:30 - MEM_PM_BIST_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_pm_bist_ctrl_lowv(&self) -> MEM_PM_BIST_CTRL_LOWV_R {
        MEM_PM_BIST_CTRL_LOWV_R::new(((self.bits >> 11) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 0:10 - NA21"]
    #[inline(always)]
    pub fn na21(&self) -> NA21_R {
        NA21_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 11:30 - MEM_PM_BIST_CTRL_LOWV"]
    #[inline(always)]
    pub fn mem_pm_bist_ctrl_lowv(&mut self) -> MEM_PM_BIST_CTRL_LOWV_W {
        MEM_PM_BIST_CTRL_LOWV_W { w: self }
    }
    #[doc = "Bits 0:10 - NA21"]
    #[inline(always)]
    pub fn na21(&mut self) -> NA21_W {
        NA21_W { w: self }
    }
}
