#[doc = "Reader of register MEM_BGAP_PARAMETERS1"]
pub type R = crate::R<u32, super::MEM_BGAP_PARAMETERS1>;
#[doc = "Writer for register MEM_BGAP_PARAMETERS1"]
pub type W = crate::W<u32, super::MEM_BGAP_PARAMETERS1>;
#[doc = "Register MEM_BGAP_PARAMETERS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_BGAP_PARAMETERS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_BGAP_ACT_IREF_ITRIM`"]
pub type MEM_BGAP_ACT_IREF_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_BGAP_ACT_IREF_ITRIM`"]
pub struct MEM_BGAP_ACT_IREF_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_BGAP_ACT_IREF_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28 - MEM_BGAP_ACT_IREF_ITRIM"]
    #[inline(always)]
    pub fn mem_bgap_act_iref_itrim(&self) -> MEM_BGAP_ACT_IREF_ITRIM_R {
        MEM_BGAP_ACT_IREF_ITRIM_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - MEM_BGAP_ACT_IREF_ITRIM"]
    #[inline(always)]
    pub fn mem_bgap_act_iref_itrim(&mut self) -> MEM_BGAP_ACT_IREF_ITRIM_W {
        MEM_BGAP_ACT_IREF_ITRIM_W { w: self }
    }
}
