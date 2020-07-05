#[doc = "Reader of register REF_LDO_CONTROLS"]
pub type R = crate::R<u32, super::REF_LDO_CONTROLS>;
#[doc = "Writer for register REF_LDO_CONTROLS"]
pub type W = crate::W<u32, super::REF_LDO_CONTROLS>;
#[doc = "Register REF_LDO_CONTROLS `reset()`'s with value 0"]
impl crate::ResetValue for super::REF_LDO_CONTROLS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REF_SPARE_CONTROL`"]
pub type REF_SPARE_CONTROL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REF_SPARE_CONTROL`"]
pub struct REF_SPARE_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_SPARE_CONTROL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `REF_TLOAD_ENABLE`"]
pub type REF_TLOAD_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REF_TLOAD_ENABLE`"]
pub struct REF_TLOAD_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_TLOAD_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `REF_LDO_TMUX_CONTROL`"]
pub type REF_LDO_TMUX_CONTROL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REF_LDO_TMUX_CONTROL`"]
pub struct REF_LDO_TMUX_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_LDO_TMUX_CONTROL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `REF_BW_CONTROL`"]
pub type REF_BW_CONTROL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REF_BW_CONTROL`"]
pub struct REF_BW_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_BW_CONTROL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `REF_VTRIM_CONTROL`"]
pub type REF_VTRIM_CONTROL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REF_VTRIM_CONTROL`"]
pub struct REF_VTRIM_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_VTRIM_CONTROL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:15 - Spare bits for REF_CTRL_FSM. Reaches directly on port TOP_PM_REG2\\[15:14\\]
of gprcm."]
    #[inline(always)]
    pub fn ref_spare_control(&self) -> REF_SPARE_CONTROL_R {
        REF_SPARE_CONTROL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 11:13 - REF TLOAD Enable. Reaches directly on port TOP_PM_REG2\\[13:11\\]
of gprcm."]
    #[inline(always)]
    pub fn ref_tload_enable(&self) -> REF_TLOAD_ENABLE_R {
        REF_TLOAD_ENABLE_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - REF_LDO Test-mux control. Reaches directly on port TOP_PM_REG2\\[10:8\\]
of gprcm."]
    #[inline(always)]
    pub fn ref_ldo_tmux_control(&self) -> REF_LDO_TMUX_CONTROL_R {
        REF_LDO_TMUX_CONTROL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - REF BW Control. Reaches directly on port TOP_PM_REG2\\[7:6\\]
of gprcm."]
    #[inline(always)]
    pub fn ref_bw_control(&self) -> REF_BW_CONTROL_R {
        REF_BW_CONTROL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 2:5 - REF VTRIM Control. Reaches directly on port TOP_PM_REG2\\[5:2\\]
of gprcm."]
    #[inline(always)]
    pub fn ref_vtrim_control(&self) -> REF_VTRIM_CONTROL_R {
        REF_VTRIM_CONTROL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15 - Spare bits for REF_CTRL_FSM. Reaches directly on port TOP_PM_REG2\\[15:14\\]
of gprcm."]
    #[inline(always)]
    pub fn ref_spare_control(&mut self) -> REF_SPARE_CONTROL_W {
        REF_SPARE_CONTROL_W { w: self }
    }
    #[doc = "Bits 11:13 - REF TLOAD Enable. Reaches directly on port TOP_PM_REG2\\[13:11\\]
of gprcm."]
    #[inline(always)]
    pub fn ref_tload_enable(&mut self) -> REF_TLOAD_ENABLE_W {
        REF_TLOAD_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:10 - REF_LDO Test-mux control. Reaches directly on port TOP_PM_REG2\\[10:8\\]
of gprcm."]
    #[inline(always)]
    pub fn ref_ldo_tmux_control(&mut self) -> REF_LDO_TMUX_CONTROL_W {
        REF_LDO_TMUX_CONTROL_W { w: self }
    }
    #[doc = "Bits 6:7 - REF BW Control. Reaches directly on port TOP_PM_REG2\\[7:6\\]
of gprcm."]
    #[inline(always)]
    pub fn ref_bw_control(&mut self) -> REF_BW_CONTROL_W {
        REF_BW_CONTROL_W { w: self }
    }
    #[doc = "Bits 2:5 - REF VTRIM Control. Reaches directly on port TOP_PM_REG2\\[5:2\\]
of gprcm."]
    #[inline(always)]
    pub fn ref_vtrim_control(&mut self) -> REF_VTRIM_CONTROL_W {
        REF_VTRIM_CONTROL_W { w: self }
    }
}
