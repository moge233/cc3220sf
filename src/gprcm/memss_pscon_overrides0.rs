#[doc = "Reader of register MEMSS_PSCON_OVERRIDES0"]
pub type R = crate::R<u32, super::MEMSS_PSCON_OVERRIDES0>;
#[doc = "Writer for register MEMSS_PSCON_OVERRIDES0"]
pub type W = crate::W<u32, super::MEMSS_PSCON_OVERRIDES0>;
#[doc = "Register MEMSS_PSCON_OVERRIDES0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MEMSS_PSCON_OVERRIDES0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_MEMSS_PSCON_MEM_OFF_OVERRIDE`"]
pub type MEM_MEMSS_PSCON_MEM_OFF_OVERRIDE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_MEMSS_PSCON_MEM_OFF_OVERRIDE`"]
pub struct MEM_MEMSS_PSCON_MEM_OFF_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MEMSS_PSCON_MEM_OFF_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEM_MEMSS_PSCON_MEM_RETAIN_OVERRIDE`"]
pub type MEM_MEMSS_PSCON_MEM_RETAIN_OVERRIDE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_MEMSS_PSCON_MEM_RETAIN_OVERRIDE`"]
pub struct MEM_MEMSS_PSCON_MEM_RETAIN_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MEMSS_PSCON_MEM_RETAIN_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - MEM_MEMSS_PSCON_MEM_OFF_OVERRIDE"]
    #[inline(always)]
    pub fn mem_memss_pscon_mem_off_override(&self) -> MEM_MEMSS_PSCON_MEM_OFF_OVERRIDE_R {
        MEM_MEMSS_PSCON_MEM_OFF_OVERRIDE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - MEM_MEMSS_PSCON_MEM_RETAIN_OVERRIDE"]
    #[inline(always)]
    pub fn mem_memss_pscon_mem_retain_override(&self) -> MEM_MEMSS_PSCON_MEM_RETAIN_OVERRIDE_R {
        MEM_MEMSS_PSCON_MEM_RETAIN_OVERRIDE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - MEM_MEMSS_PSCON_MEM_OFF_OVERRIDE"]
    #[inline(always)]
    pub fn mem_memss_pscon_mem_off_override(&mut self) -> MEM_MEMSS_PSCON_MEM_OFF_OVERRIDE_W {
        MEM_MEMSS_PSCON_MEM_OFF_OVERRIDE_W { w: self }
    }
    #[doc = "Bits 0:15 - MEM_MEMSS_PSCON_MEM_RETAIN_OVERRIDE"]
    #[inline(always)]
    pub fn mem_memss_pscon_mem_retain_override(&mut self) -> MEM_MEMSS_PSCON_MEM_RETAIN_OVERRIDE_W {
        MEM_MEMSS_PSCON_MEM_RETAIN_OVERRIDE_W { w: self }
    }
}
