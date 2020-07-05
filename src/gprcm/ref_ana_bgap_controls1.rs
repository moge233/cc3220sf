#[doc = "Reader of register REF_ANA_BGAP_CONTROLS1"]
pub type R = crate::R<u32, super::REF_ANA_BGAP_CONTROLS1>;
#[doc = "Writer for register REF_ANA_BGAP_CONTROLS1"]
pub type W = crate::W<u32, super::REF_ANA_BGAP_CONTROLS1>;
#[doc = "Register REF_ANA_BGAP_CONTROLS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::REF_ANA_BGAP_CONTROLS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_REF_BG_SPARE`"]
pub type MEM_REF_BG_SPARE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_REF_BG_SPARE`"]
pub struct MEM_REF_BG_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_REF_BG_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `MEM_REF_BGAP_TMUX_CTRL`"]
pub type MEM_REF_BGAP_TMUX_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_REF_BGAP_TMUX_CTRL`"]
pub struct MEM_REF_BGAP_TMUX_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_REF_BGAP_TMUX_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 9)) | (((value as u32) & 0x1f) << 9);
        self.w
    }
}
#[doc = "Reader of field `MEM_REF_FILT_TRIM`"]
pub type MEM_REF_FILT_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_REF_FILT_TRIM`"]
pub struct MEM_REF_FILT_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_REF_FILT_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
#[doc = "Reader of field `MEM_REF_MAG_TRIM`"]
pub type MEM_REF_MAG_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_REF_MAG_TRIM`"]
pub struct MEM_REF_MAG_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_REF_MAG_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:15 - REF_BGAP_SPARE. Reaches on port TOP_PM_REG1\\[15:14\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_ref_bg_spare(&self) -> MEM_REF_BG_SPARE_R {
        MEM_REF_BG_SPARE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 9:13 - REF_BGAP_TMUX_CTRL. Reaches on port TOP_PM_REG1\\[13:9\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_ref_bgap_tmux_ctrl(&self) -> MEM_REF_BGAP_TMUX_CTRL_R {
        MEM_REF_BGAP_TMUX_CTRL_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - REF_FILT_TRIM. Reaches on port TOP_PM_REG1\\[8:5\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_ref_filt_trim(&self) -> MEM_REF_FILT_TRIM_R {
        MEM_REF_FILT_TRIM_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 0:4 - REF_MAG_TRIM Override. Applicable when bit\\[22\\]
of REF_ANA_BGAP_CONTROLS0 \\[0x084C\\]
set to 1 (of efc_done = 0). Note : Final REF_MAG_TRIM reaches on port TOP_PM_REG1\\[4:0\\]
of gprcm"]
    #[inline(always)]
    pub fn mem_ref_mag_trim(&self) -> MEM_REF_MAG_TRIM_R {
        MEM_REF_MAG_TRIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15 - REF_BGAP_SPARE. Reaches on port TOP_PM_REG1\\[15:14\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_ref_bg_spare(&mut self) -> MEM_REF_BG_SPARE_W {
        MEM_REF_BG_SPARE_W { w: self }
    }
    #[doc = "Bits 9:13 - REF_BGAP_TMUX_CTRL. Reaches on port TOP_PM_REG1\\[13:9\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_ref_bgap_tmux_ctrl(&mut self) -> MEM_REF_BGAP_TMUX_CTRL_W {
        MEM_REF_BGAP_TMUX_CTRL_W { w: self }
    }
    #[doc = "Bits 5:8 - REF_FILT_TRIM. Reaches on port TOP_PM_REG1\\[8:5\\]
of gprcm."]
    #[inline(always)]
    pub fn mem_ref_filt_trim(&mut self) -> MEM_REF_FILT_TRIM_W {
        MEM_REF_FILT_TRIM_W { w: self }
    }
    #[doc = "Bits 0:4 - REF_MAG_TRIM Override. Applicable when bit\\[22\\]
of REF_ANA_BGAP_CONTROLS0 \\[0x084C\\]
set to 1 (of efc_done = 0). Note : Final REF_MAG_TRIM reaches on port TOP_PM_REG1\\[4:0\\]
of gprcm"]
    #[inline(always)]
    pub fn mem_ref_mag_trim(&mut self) -> MEM_REF_MAG_TRIM_W {
        MEM_REF_MAG_TRIM_W { w: self }
    }
}
