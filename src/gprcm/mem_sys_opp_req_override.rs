#[doc = "Reader of register MEM_SYS_OPP_REQ_OVERRIDE"]
pub type R = crate::R<u32, super::MEM_SYS_OPP_REQ_OVERRIDE>;
#[doc = "Writer for register MEM_SYS_OPP_REQ_OVERRIDE"]
pub type W = crate::W<u32, super::MEM_SYS_OPP_REQ_OVERRIDE>;
#[doc = "Register MEM_SYS_OPP_REQ_OVERRIDE `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_SYS_OPP_REQ_OVERRIDE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_SYS_OPP_REQ_OVERRIDE`"]
pub type MEM_SYS_OPP_REQ_OVERRIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_SYS_OPP_REQ_OVERRIDE`"]
pub struct MEM_SYS_OPP_REQ_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SYS_OPP_REQ_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - &quot;0001&quot; - RUN ; &quot;0010&quot; - DSLP ; &quot;0100&quot; - LPDS ; Others - NA"]
    #[inline(always)]
    pub fn mem_sys_opp_req_override(&self) -> MEM_SYS_OPP_REQ_OVERRIDE_R {
        MEM_SYS_OPP_REQ_OVERRIDE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - &quot;0001&quot; - RUN ; &quot;0010&quot; - DSLP ; &quot;0100&quot; - LPDS ; Others - NA"]
    #[inline(always)]
    pub fn mem_sys_opp_req_override(&mut self) -> MEM_SYS_OPP_REQ_OVERRIDE_W {
        MEM_SYS_OPP_REQ_OVERRIDE_W { w: self }
    }
}
