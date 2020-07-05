#[doc = "Reader of register HIB_TMUX_CTRL"]
pub type R = crate::R<u32, super::HIB_TMUX_CTRL>;
#[doc = "Writer for register HIB_TMUX_CTRL"]
pub type W = crate::W<u32, super::HIB_TMUX_CTRL>;
#[doc = "Register HIB_TMUX_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::HIB_TMUX_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_HD_TMUX_CNTRL`"]
pub type MEM_HD_TMUX_CNTRL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_HD_TMUX_CNTRL`"]
pub struct MEM_HD_TMUX_CNTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_HD_TMUX_CNTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - MEM_HD_TMUX_CNTRL"]
    #[inline(always)]
    pub fn mem_hd_tmux_cntrl(&self) -> MEM_HD_TMUX_CNTRL_R {
        MEM_HD_TMUX_CNTRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - MEM_HD_TMUX_CNTRL"]
    #[inline(always)]
    pub fn mem_hd_tmux_cntrl(&mut self) -> MEM_HD_TMUX_CNTRL_W {
        MEM_HD_TMUX_CNTRL_W { w: self }
    }
}
