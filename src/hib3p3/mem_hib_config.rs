#[doc = "Reader of register MEM_HIB_CONFIG"]
pub type R = crate::R<u32, super::MEM_HIB_CONFIG>;
#[doc = "Writer for register MEM_HIB_CONFIG"]
pub type W = crate::W<u32, super::MEM_HIB_CONFIG>;
#[doc = "Register MEM_HIB_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_HIB_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOP_MUX_CTRL_SOP_SPIO`"]
pub type TOP_MUX_CTRL_SOP_SPIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOP_MUX_CTRL_SOP_SPIO`"]
pub struct TOP_MUX_CTRL_SOP_SPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_MUX_CTRL_SOP_SPIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - TOP_MUX_CTRL_SOP_SPIO"]
    #[inline(always)]
    pub fn top_mux_ctrl_sop_spio(&self) -> TOP_MUX_CTRL_SOP_SPIO_R {
        TOP_MUX_CTRL_SOP_SPIO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - TOP_MUX_CTRL_SOP_SPIO"]
    #[inline(always)]
    pub fn top_mux_ctrl_sop_spio(&mut self) -> TOP_MUX_CTRL_SOP_SPIO_W {
        TOP_MUX_CTRL_SOP_SPIO_W { w: self }
    }
}
