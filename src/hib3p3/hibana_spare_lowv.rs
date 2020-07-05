#[doc = "Reader of register HIBANA_SPARE_LOWV"]
pub type R = crate::R<u32, super::HIBANA_SPARE_LOWV>;
#[doc = "Writer for register HIBANA_SPARE_LOWV"]
pub type W = crate::W<u32, super::HIBANA_SPARE_LOWV>;
#[doc = "Register HIBANA_SPARE_LOWV `reset()`'s with value 0"]
impl crate::ResetValue for super::HIBANA_SPARE_LOWV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_HIBANA_SPARE1`"]
pub type MEM_HIBANA_SPARE1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEM_HIBANA_SPARE1`"]
pub struct MEM_HIBANA_SPARE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_HIBANA_SPARE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 22)) | (((value as u32) & 0x03ff) << 22);
        self.w
    }
}
#[doc = "Reader of field `MEM_HIBANA_SPARE0`"]
pub type MEM_HIBANA_SPARE0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MEM_HIBANA_SPARE0`"]
pub struct MEM_HIBANA_SPARE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_HIBANA_SPARE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:31 - MEM_HIBANA_SPARE1"]
    #[inline(always)]
    pub fn mem_hibana_spare1(&self) -> MEM_HIBANA_SPARE1_R {
        MEM_HIBANA_SPARE1_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:16 - MEM_HIBANA_SPARE0"]
    #[inline(always)]
    pub fn mem_hibana_spare0(&self) -> MEM_HIBANA_SPARE0_R {
        MEM_HIBANA_SPARE0_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 22:31 - MEM_HIBANA_SPARE1"]
    #[inline(always)]
    pub fn mem_hibana_spare1(&mut self) -> MEM_HIBANA_SPARE1_W {
        MEM_HIBANA_SPARE1_W { w: self }
    }
    #[doc = "Bits 0:16 - MEM_HIBANA_SPARE0"]
    #[inline(always)]
    pub fn mem_hibana_spare0(&mut self) -> MEM_HIBANA_SPARE0_W {
        MEM_HIBANA_SPARE0_W { w: self }
    }
}
