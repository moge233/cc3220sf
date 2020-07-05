#[doc = "Reader of register REF_SLICER_CONTROLS0"]
pub type R = crate::R<u32, super::REF_SLICER_CONTROLS0>;
#[doc = "Writer for register REF_SLICER_CONTROLS0"]
pub type W = crate::W<u32, super::REF_SLICER_CONTROLS0>;
#[doc = "Register REF_SLICER_CONTROLS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::REF_SLICER_CONTROLS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CM_TMUX_SEL_LOWV`"]
pub type CM_TMUX_SEL_LOWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CM_TMUX_SEL_LOWV`"]
pub struct CM_TMUX_SEL_LOWV_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_TMUX_SEL_LOWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Reader of field `SLICER_SPARE0`"]
pub type SLICER_SPARE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLICER_SPARE0`"]
pub struct SLICER_SPARE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLICER_SPARE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:9 - CM Test-mux select. Reaches on TOP_CLMM_REG1_IN\\[9:6\\]
port of gprcm"]
    #[inline(always)]
    pub fn cm_tmux_sel_lowv(&self) -> CM_TMUX_SEL_LOWV_R {
        CM_TMUX_SEL_LOWV_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Slicer spare0 control. Reaches on TOP_CLKM_REG1_IN\\[5:4\\]
port of gprcm"]
    #[inline(always)]
    pub fn slicer_spare0(&self) -> SLICER_SPARE0_R {
        SLICER_SPARE0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:9 - CM Test-mux select. Reaches on TOP_CLMM_REG1_IN\\[9:6\\]
port of gprcm"]
    #[inline(always)]
    pub fn cm_tmux_sel_lowv(&mut self) -> CM_TMUX_SEL_LOWV_W {
        CM_TMUX_SEL_LOWV_W { w: self }
    }
    #[doc = "Bits 4:5 - Slicer spare0 control. Reaches on TOP_CLKM_REG1_IN\\[5:4\\]
port of gprcm"]
    #[inline(always)]
    pub fn slicer_spare0(&mut self) -> SLICER_SPARE0_W {
        SLICER_SPARE0_W { w: self }
    }
}
