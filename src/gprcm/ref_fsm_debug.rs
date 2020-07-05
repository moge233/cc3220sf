#[doc = "Reader of register REF_FSM_DEBUG"]
pub type R = crate::R<u32, super::REF_FSM_DEBUG>;
#[doc = "Writer for register REF_FSM_DEBUG"]
pub type W = crate::W<u32, super::REF_FSM_DEBUG>;
#[doc = "Register REF_FSM_DEBUG `reset()`'s with value 0"]
impl crate::ResetValue for super::REF_FSM_DEBUG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FREF_MODE`"]
pub type FREF_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FREF_MODE`"]
pub struct FREF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREF_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `REF_FSM_PS`"]
pub type REF_FSM_PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REF_FSM_PS`"]
pub struct REF_FSM_PS_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_FSM_PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - 01 - HV Mode ; 10 - LV Mode ; 11 - XTAL Mode"]
    #[inline(always)]
    pub fn fref_mode(&self) -> FREF_MODE_R {
        FREF_MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - constant FREF_CLK_OFF = &quot;00000&quot;; constant FREF_EN_BGAP = &quot;00001&quot;; constant FREF_EN_LDO = &quot;00010&quot;; constant FREF_EN_SLI_HV = &quot;00011&quot;; constant FREF_EN_SLI_HV_PD = &quot;00100&quot;; constant FREF_EN_DIG_BUF = &quot;00101&quot;; constant FREF_EN_OSC = &quot;00110&quot;; constant FREF_EN_SLI_LV = &quot;00111&quot;; constant FREF_EN_CLK_REQ = &quot;01000&quot;; constant FREF_CLK_VALID = &quot;01001&quot;; constant FREF_MODE_DET0 = &quot;01010&quot;; constant FREF_MODE_DET1 = &quot;01011&quot;; constant FREF_MODE_DET2 = &quot;10010&quot;; constant FREF_MODE_DET3 = &quot;10011&quot;; constant FREF_VALID = &quot;01100&quot;; constant FREF_VALID0 = &quot;01101&quot;; constant FREF_VALID1 = &quot;01110&quot;; constant FREF_VALID2 = &quot;01111&quot;; constant FREF_WAIT_EXT_TCXO0 = &quot;10000&quot;; constant FREF_WAIT_EXT_TCXO1 = &quot;10001&quot;;"]
    #[inline(always)]
    pub fn ref_fsm_ps(&self) -> REF_FSM_PS_R {
        REF_FSM_PS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - 01 - HV Mode ; 10 - LV Mode ; 11 - XTAL Mode"]
    #[inline(always)]
    pub fn fref_mode(&mut self) -> FREF_MODE_W {
        FREF_MODE_W { w: self }
    }
    #[doc = "Bits 0:3 - constant FREF_CLK_OFF = &quot;00000&quot;; constant FREF_EN_BGAP = &quot;00001&quot;; constant FREF_EN_LDO = &quot;00010&quot;; constant FREF_EN_SLI_HV = &quot;00011&quot;; constant FREF_EN_SLI_HV_PD = &quot;00100&quot;; constant FREF_EN_DIG_BUF = &quot;00101&quot;; constant FREF_EN_OSC = &quot;00110&quot;; constant FREF_EN_SLI_LV = &quot;00111&quot;; constant FREF_EN_CLK_REQ = &quot;01000&quot;; constant FREF_CLK_VALID = &quot;01001&quot;; constant FREF_MODE_DET0 = &quot;01010&quot;; constant FREF_MODE_DET1 = &quot;01011&quot;; constant FREF_MODE_DET2 = &quot;10010&quot;; constant FREF_MODE_DET3 = &quot;10011&quot;; constant FREF_VALID = &quot;01100&quot;; constant FREF_VALID0 = &quot;01101&quot;; constant FREF_VALID1 = &quot;01110&quot;; constant FREF_VALID2 = &quot;01111&quot;; constant FREF_WAIT_EXT_TCXO0 = &quot;10000&quot;; constant FREF_WAIT_EXT_TCXO1 = &quot;10001&quot;;"]
    #[inline(always)]
    pub fn ref_fsm_ps(&mut self) -> REF_FSM_PS_W {
        REF_FSM_PS_W { w: self }
    }
}
