#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMACHANS`"]
pub type DMACHANS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMACHANS`"]
pub struct DMACHANS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACHANS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATE`"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:20 - Available uDMA Channels Minus 1"]
    #[inline(always)]
    pub fn dmachans(&self) -> DMACHANS_R {
        DMACHANS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 4:7 - Control State Machine Status 0x00000090 : UDMA_STAT_STATE_DONE : Done 0x00000000 : UDMA_STAT_STATE_IDLE : Idle 0x00000010 : UDMA_STAT_STATE_RD_CTRL : Reading channel controller data 0x00000030 : UDMA_STAT_STATE_RD_DSTENDP : Reading destination end pointer 0x00000040 : UDMA_STAT_STATE_RD_SRCDAT : Reading source data 0x00000020 : UDMA_STAT_STATE_RD_SRCENDP : Reading source end pointer 0x00000080 : UDMA_STAT_STATE_STALL : Stalled 0x000000A0 : UDMA_STAT_STATE_UNDEF : Undefined 0x00000060 : UDMA_STAT_STATE_WAIT : Waiting for uDMA request to clear 0x00000070 : UDMA_STAT_STATE_WR_CTRL : Writing channel controller data 0x00000050 : UDMA_STAT_STATE_WR_DSTDAT : Writing destination data"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - Available uDMA Channels Minus 1"]
    #[inline(always)]
    pub fn dmachans(&mut self) -> DMACHANS_W {
        DMACHANS_W { w: self }
    }
    #[doc = "Bits 4:7 - Control State Machine Status 0x00000090 : UDMA_STAT_STATE_DONE : Done 0x00000000 : UDMA_STAT_STATE_IDLE : Idle 0x00000010 : UDMA_STAT_STATE_RD_CTRL : Reading channel controller data 0x00000030 : UDMA_STAT_STATE_RD_DSTENDP : Reading destination end pointer 0x00000040 : UDMA_STAT_STATE_RD_SRCDAT : Reading source data 0x00000020 : UDMA_STAT_STATE_RD_SRCENDP : Reading source end pointer 0x00000080 : UDMA_STAT_STATE_STALL : Stalled 0x000000A0 : UDMA_STAT_STATE_UNDEF : Undefined 0x00000060 : UDMA_STAT_STATE_WAIT : Waiting for uDMA request to clear 0x00000070 : UDMA_STAT_STATE_WR_CTRL : Writing channel controller data 0x00000050 : UDMA_STAT_STATE_WR_DSTDAT : Writing destination data"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
}
