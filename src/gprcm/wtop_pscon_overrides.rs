#[doc = "Reader of register WTOP_PSCON_OVERRIDES"]
pub type R = crate::R<u32, super::WTOP_PSCON_OVERRIDES>;
#[doc = "Writer for register WTOP_PSCON_OVERRIDES"]
pub type W = crate::W<u32, super::WTOP_PSCON_OVERRIDES>;
#[doc = "Register WTOP_PSCON_OVERRIDES `reset()`'s with value 0"]
impl crate::ResetValue for super::WTOP_PSCON_OVERRIDES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_WTOP_PSCON_MEM_OFF_OVERRIDE`"]
pub type MEM_WTOP_PSCON_MEM_OFF_OVERRIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_WTOP_PSCON_MEM_OFF_OVERRIDE`"]
pub struct MEM_WTOP_PSCON_MEM_OFF_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_WTOP_PSCON_MEM_OFF_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `MEM_WTOP_PSCON_MEM_RETAIN_OVERRIDE`"]
pub type MEM_WTOP_PSCON_MEM_RETAIN_OVERRIDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_WTOP_PSCON_MEM_RETAIN_OVERRIDE`"]
pub struct MEM_WTOP_PSCON_MEM_RETAIN_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_WTOP_PSCON_MEM_RETAIN_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - MEM_WTOP_PSCON_MEM_OFF_OVERRIDE"]
    #[inline(always)]
    pub fn mem_wtop_pscon_mem_off_override(&self) -> MEM_WTOP_PSCON_MEM_OFF_OVERRIDE_R {
        MEM_WTOP_PSCON_MEM_OFF_OVERRIDE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - MEM_WTOP_PSCON_MEM_RETAIN_OVERRIDE"]
    #[inline(always)]
    pub fn mem_wtop_pscon_mem_retain_override(&self) -> MEM_WTOP_PSCON_MEM_RETAIN_OVERRIDE_R {
        MEM_WTOP_PSCON_MEM_RETAIN_OVERRIDE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - MEM_WTOP_PSCON_MEM_OFF_OVERRIDE"]
    #[inline(always)]
    pub fn mem_wtop_pscon_mem_off_override(&mut self) -> MEM_WTOP_PSCON_MEM_OFF_OVERRIDE_W {
        MEM_WTOP_PSCON_MEM_OFF_OVERRIDE_W { w: self }
    }
    #[doc = "Bits 0:7 - MEM_WTOP_PSCON_MEM_RETAIN_OVERRIDE"]
    #[inline(always)]
    pub fn mem_wtop_pscon_mem_retain_override(&mut self) -> MEM_WTOP_PSCON_MEM_RETAIN_OVERRIDE_W {
        MEM_WTOP_PSCON_MEM_RETAIN_OVERRIDE_W { w: self }
    }
}
